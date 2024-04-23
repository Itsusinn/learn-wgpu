use std::sync::Arc;

use color_eyre::eyre::Result;
use glam::Vec3;

use wgpu::include_wgsl;
use winit::window::Window;

use crate::{
  data,
  exts::state::{DeviceTrait, DeviceWarp},
  geom::{
    self,
    camera::{Camera, CameraUniform},
  },
  instance::{self, Instance}, texture,
};

pub struct State {
  surface: wgpu::Surface<'static>,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  pub size: winit::dpi::PhysicalSize<u32>,
  render_pipeline: wgpu::RenderPipeline,

  clear_color: na::Vector3<f64>,
  camera: geom::camera::Camera,
  camera_uniform: CameraUniform,
  camera_buffer: wgpu::Buffer,
  camera_bind_group: wgpu::BindGroup,

  instances: Vec<instance::Instance>,
  instance_buffer: wgpu::Buffer,

  depth_texture: texture::Texture,

  vertex_buffer: wgpu::Buffer,

  index_buffer: wgpu::Buffer,
  num_indices: u32,

  texture: crate::texture_array::TextureArray
}
const NUM_INSTANCES_PER_ROW: u32 = 10;

impl DeviceTrait for State {
  #[inline(always)]
  fn get_device(&self) -> &wgpu::Device {
    &self.device
  }
}
impl State {
  // Creating some of the wgpu types requires async code
  pub async fn new(window: Arc<Window>) -> Result<Self> {
    let size = window.inner_size();
    // instance is a handle to gpu
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
      backends: wgpu::Backends::all(),
      ..Default::default()
    });
    let surface = instance.create_surface(window.clone())?;
    let adapter = instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        force_fallback_adapter: false,
        compatible_surface: Some(&surface),
      })
      .await
      .unwrap();
    let (rdevice, queue) = adapter
      .request_device(
        &wgpu::DeviceDescriptor {
          label: None,
          required_features: wgpu::Features::empty(),
          required_limits: wgpu::Limits::default(),
        },
        None,
      )
      .await?;
    let device = DeviceWarp { inner: &rdevice };
    let caps = surface.get_capabilities(&adapter);
    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: caps.formats[0],
      width: size.width,
      height: size.height,
      present_mode: wgpu::PresentMode::Fifo,
      alpha_mode: caps.alpha_modes[0],
      view_formats: vec![],
      desired_maximum_frame_latency: 2,
    };
    surface.configure(&device.inner, &config);

    let texture = crate::texture_array::TextureArray::new(&device.get_device(), &queue);

    let camera = Camera::new(Vec3::new(0.0, 0.0, -1.0));
    let mut camera_uniform = CameraUniform::new();
    camera_uniform.update_view_proj(&camera, config.width as f32 / config.height as f32);

    let camera_buffer = device.create_buffer_init(
      "Camera Buffer",
      bytemuck::cast_slice(&[camera_uniform]),
      wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    );
    let camera_bind_group_layout = device.create_bind_group_layout(
      "camera_bind_group_layout",
      &[wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::VERTEX,
        ty: wgpu::BindingType::Buffer {
          ty: wgpu::BufferBindingType::Uniform,
          has_dynamic_offset: false,
          min_binding_size: None,
        },
        count: None,
      }],
    );
    let camera_bind_group = device.create_bind_group(
      "camera_bind_group",
      &camera_bind_group_layout,
      &[wgpu::BindGroupEntry {
        binding: 0,
        resource: camera_buffer.as_entire_binding(),
      }],
    );
    let depth_texture = texture::Texture::create_depth_texture(&device, &config, "depth_texture");

    let shader = device.create_shader_module(include_wgsl!("../assets/shader.wgsl"));
    let render_pipeline_layout = device.create_pipeline_layout(
      "Render Pipeline Layout",
      &[&texture.bind_group_layout, &camera_bind_group_layout],
      &[],
    );
    let render_pipeline = device.create_render_pipeline(
      "Render Pipline",
      Some(&render_pipeline_layout),
      wgpu::VertexState {
        module: &shader,
        // 指定应将着色器中的哪个函数作为 entry_point
        entry_point: "vs_main",
        // buffers 字段用于告知 wgpu 我们要传递给顶点着色器的顶点类型
        buffers: &[data::Vertex::desc(), Instance::desc()],
      },
      // primitive 字段描述了应如何将我们所提供的顶点数据转为三角形
      wgpu::PrimitiveState {
        // PrimitiveTopology::TriangleList 表示每三个顶点将对应一个三角形
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        // front_face 和 cull_mode 字段告诉 wgpu 应如何确定某个三角形是否朝前
        // FrontFace::Ccw 表示如果顶点按逆时针方向排列，则判定三角形是朝前的
        front_face: wgpu::FrontFace::Ccw,
        // 不满足朝前条件的三角形会被剔除（即不被渲染），这是用 CullMode::Back 所确定的
        cull_mode: Some(wgpu::Face::Back),
        // cull_mode: None,
        // 如果将该字段设置为除了 Fill 之外的任何值，都需要 Features::NON_FILL_POLYGON_MODE
        polygon_mode: wgpu::PolygonMode::Fill,
        // 需要 Features::DEPTH_CLIP_ENABLE
        unclipped_depth: false,
        // 需要 Features::CONSERVATIVE_RASTERIZATION
        conservative: false,
      },
      // 深度 / 模板缓冲区
      Some(wgpu::DepthStencilState {
        format: texture::Texture::DEPTH_FORMAT,
        depth_write_enabled: true,
        // 用于确定何时丢弃一个新像素，使用 LESS 意味着像素将从前往后绘制
        depth_compare: wgpu::CompareFunction::Less,
        stencil: wgpu::StencilState::default(),
        bias: wgpu::DepthBiasState::default(),
      }),
      wgpu::MultisampleState {
        // count 决定了 pipeline 将使用多少次采样
        count: 1,
        // mask 指定了哪些采样应被设为活跃。目前我们将使用所有的采样
        mask: !0,
        // 抗锯齿
        alpha_to_coverage_enabled: false,
      },
      wgpu::FragmentState {
        module: &shader,
        // 指定应将着色器中的哪个函数作为 entry_point
        entry_point: "fs_main",
        // targets 字段告诉 wgpu 应该设置哪些颜色输出
        targets: &[Some(wgpu::ColorTargetState {
          format: config.format,
          // 指定混合模式（blending）为仅用新数据替换旧像素数据
          blend: Some(wgpu::BlendState::REPLACE),
          // 要求 wgpu 写入所有像素通道的颜色，即红、蓝、绿和 alpha
          write_mask: wgpu::ColorWrites::ALL,
        })],
      },
      None,
    );

    let instances = (0..NUM_INSTANCES_PER_ROW)
      .flat_map(|y| {
        (0..NUM_INSTANCES_PER_ROW).map(move |x| {
          let position = na::Point2::new(x as f32, y as f32);

          Instance {
            position,
            tex_id: 0,
          }
        })
      })
      .collect::<Vec<_>>();

    let instance_buffer = device.create_buffer_init(
      "Instance Buffer",
      bytemuck::cast_slice(&instances),
      wgpu::BufferUsages::VERTEX,
    );
    let vertex_buffer = device.create_buffer_init(
      "Vertex Buffer",
      bytemuck::cast_slice(data::VERTICES),
      wgpu::BufferUsages::VERTEX,
    );

    let index_buffer = device.create_buffer_init(
      "Index Buffer",
      bytemuck::cast_slice(data::INDICES),
      wgpu::BufferUsages::INDEX,
    );
    let num_indices = crate::data::INDICES.len() as u32;

    Ok(Self {
      surface,
      device: rdevice,
      queue,
      config,
      size,
      clear_color: na::Vector3::new(0.1, 0.7, 0.2),
      render_pipeline,
      camera,
      camera_uniform,
      camera_buffer,
      camera_bind_group,
      instances,
      instance_buffer,
      depth_texture,
      vertex_buffer,
      index_buffer,
      num_indices,
      texture
    })
  }

  pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    if new_size.width > 0 && new_size.height > 0 {
      self.size = new_size;
      self.config.width = new_size.width;
      self.config.height = new_size.height;
      self.depth_texture =
        texture::Texture::create_depth_texture(self, &self.config, "depth_texture");
      self.surface.configure(&self.device, &self.config);
    };
  }

  pub fn update(&mut self) {
    self.camera.handle_input();
    self.camera_uniform.update_view_proj(
      &self.camera,
      self.config.width as f32 / self.config.height as f32,
    );
    self.queue.write_buffer(
      &self.camera_buffer,
      0,
      bytemuck::cast_slice(&[self.camera_uniform]),
    );
  }

  pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
    let output = self.surface.get_current_texture()?;
    let view = output
      .texture
      .create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = self
      .device
      .create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
      });

    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
      label: Some("Render Pass"),
      color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        // 用于告知 wgpu 应将颜色存储到哪个纹理
        view: &view,
        // 用于接收多重采样解析后所输出内容的纹理
        resolve_target: None,
        // 用于告知 wgpu 应如何处理屏幕上的颜色
        ops: wgpu::Operations {
          // load 字段告诉 wgpu 该如何处理存储在前一帧的颜色
          load: wgpu::LoadOp::Clear(wgpu::Color {
            r: self.clear_color.x,
            g: self.clear_color.y,
            b: self.clear_color.z,
            a: 1.0,
          }),
          // store 字段用于告知 wgpu 是否应将渲染的结果存储到 TextureView 下层的 Texture
          store: wgpu::StoreOp::Store,
        },
      })],
      depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
        view: &self.depth_texture.view,
        depth_ops: Some(wgpu::Operations {
          load: wgpu::LoadOp::Clear(1.0),
          store: wgpu::StoreOp::Store,
        }),
        stencil_ops: None,
      }),
      ..Default::default()
    });
    render_pass.set_pipeline(&self.render_pipeline);
    render_pass.set_bind_group(0, &self.texture.bind_group, &[]);
    render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
    render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
    render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
    render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    render_pass.draw_indexed(0..self.num_indices, 0, 0..self.instances.len() as u32);

    drop(render_pass);

    // submit 方法能传入任何实现了 IntoIter 的参数
    self.queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
  }
}
