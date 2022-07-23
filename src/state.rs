use color_eyre::eyre::Result;
use na::{Vector2, Vector3};
use nalgebra as na;
use wgpu::{include_wgsl, util::DeviceExt, Backends};
// lib.rs
use winit::{
  event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent},
  window::Window,
};

use crate::texture;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
  position: [f32; 3],
  tex_coords: Vector2<f32>,
}
impl Vertex {
  const ATTRIBS: [wgpu::VertexAttribute; 2] =
    wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

  fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
    wgpu::VertexBufferLayout {
      // array_stride 定义了每个顶点的宽度
      array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
      // step_mode 告诉 pipeline 应以怎样的频率移动到下一个顶点
      step_mode: wgpu::VertexStepMode::Vertex,
      attributes: &Self::ATTRIBS,
    }
  }
}
const VERTICES: &[Vertex] = &[
  Vertex {
    position: [-0.0868241, 0.49240386, 0.0],
    tex_coords: Vector2::new(0.4131759, 0.99240386),
  }, // A
  Vertex {
    position: [-0.49513406, 0.06958647, 0.0],
    tex_coords: Vector2::new(0.0048659444, 0.56958647),
  }, // B
  Vertex {
    position: [-0.21918549, -0.44939706, 0.0],
    tex_coords: Vector2::new(0.28081453, 0.05060294),
  }, // C
  Vertex {
    position: [0.35966998, -0.3473291, 0.0],
    tex_coords: Vector2::new(0.85967, 0.1526709),
  }, // D
  Vertex {
    position: [0.44147372, 0.2347359, 0.0],
    tex_coords: Vector2::new(0.9414737, 0.7347359),
  }, // E
];

const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

pub struct State {
  surface: wgpu::Surface,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  pub size: winit::dpi::PhysicalSize<u32>,
  render_pipeline: wgpu::RenderPipeline,
  vertex_buffer: wgpu::Buffer,

  clear_color: na::Vector3<f64>,
  index_buffer: wgpu::Buffer,
  num_indices: u32,
  diffuse_bind_group: wgpu::BindGroup,
  diffuse_texture: texture::Texture,
}

impl State {
  // Creating some of the wgpu types requires async code
  pub async fn new(window: &Window) -> Result<Self> {
    let size = window.inner_size();
    // instance is a handle to gpu
    let instance = wgpu::Instance::new(Backends::all());
    let surface = unsafe { instance.create_surface(&window) };
    let adapter = instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        force_fallback_adapter: false,
        compatible_surface: Some(&surface),
      })
      .await
      .unwrap();
    let (device, queue) = adapter
      .request_device(
        &wgpu::DeviceDescriptor {
          label: None,
          features: wgpu::Features::empty(),
          limits: wgpu::Limits::default(),
        },
        None,
      )
      .await?;
    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface.get_supported_formats(&adapter)[0],
      width: size.width,
      height: size.height,
      present_mode: wgpu::PresentMode::Fifo,
    };
    surface.configure(&device, &config);

    let diffuse_bytes = include_bytes!("../assets/happy-tree.png");
    let diffuse_texture = texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "happy tree")?;

    let texture_bind_group_layout =
      device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
          wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
              multisampled: false,
              view_dimension: wgpu::TextureViewDimension::D2,
              sample_type: wgpu::TextureSampleType::Float { filterable: true },
            },
            count: None,
          },
          wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(
              // SamplerBindingType::Comparison 仅可供 TextureSampleType::Depth 使用
              // 如果纹理的 sample_type 是 TextureSampleType::Float { filterable: true }
              // 那么就应当使用 SamplerBindingType::Filtering
              // 否则会报错
              wgpu::SamplerBindingType::Filtering,
            ),
            count: None,
          },
        ],
        label: Some("texture_bind_group_layout"),
      });
    let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &texture_bind_group_layout,
      entries: &[
        wgpu::BindGroupEntry {
          binding: 0,
          resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
        },
        wgpu::BindGroupEntry {
          binding: 1,
          resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
        },
      ],
      label: Some("diffuse_bind_group"),
    });

    let shader = device.create_shader_module(include_wgsl!("../assets/shader.wgsl"));
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: Some("Render Pipeline Layout"),
      bind_group_layouts: &[&texture_bind_group_layout],
      push_constant_ranges: &[],
    });
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: Some("Render Pipline"),
      layout: Some(&render_pipeline_layout),
      vertex: wgpu::VertexState {
        module: &shader,
        // 指定应将着色器中的哪个函数作为 entry_point
        entry_point: "vs_main",
        // buffers 字段用于告知 wgpu 我们要传递给顶点着色器的顶点类型
        buffers: &[Vertex::desc()],
      },
      fragment: Some(wgpu::FragmentState {
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
      }),
      // primitive 字段描述了应如何将我们所提供的顶点数据转为三角形
      primitive: wgpu::PrimitiveState {
        // PrimitiveTopology::TriangleList 表示每三个顶点将对应一个三角形
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        // front_face 和 cull_mode 字段告诉 wgpu 应如何确定某个三角形是否朝前
        // FrontFace::Ccw 表示如果顶点按逆时针方向排列，则判定三角形是朝前的
        front_face: wgpu::FrontFace::Ccw,
        // 不满足朝前条件的三角形会被剔除（即不被渲染），这是用 CullMode::Back 所确定的
        cull_mode: Some(wgpu::Face::Back),
        // 如果将该字段设置为除了 Fill 之外的任何值，都需要 Features::NON_FILL_POLYGON_MODE
        polygon_mode: wgpu::PolygonMode::Fill,
        // 需要 Features::DEPTH_CLIP_ENABLE
        unclipped_depth: false,
        // 需要 Features::CONSERVATIVE_RASTERIZATION
        conservative: false,
      },
      // 深度 / 模板缓冲区
      depth_stencil: None,
      multisample: wgpu::MultisampleState {
        // count 决定了 pipeline 将使用多少次采样
        count: 1,
        // mask 指定了哪些采样应被设为活跃。目前我们将使用所有的采样
        mask: !0,
        // 抗锯齿
        alpha_to_coverage_enabled: false,
      },
      multiview: None, // 5.
    });
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Vertex Buffer"),
      contents: bytemuck::cast_slice(VERTICES),
      usage: wgpu::BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Index Buffer"),
      contents: bytemuck::cast_slice(INDICES),
      usage: wgpu::BufferUsages::INDEX,
    });
    let num_indices = INDICES.len() as u32;
    Ok(Self {
      surface,
      device,
      queue,
      config,
      size,
      clear_color: na::Vector3::new(0.0, 0.0, 0.0),
      render_pipeline,
      vertex_buffer,
      index_buffer,
      num_indices,
      diffuse_bind_group,
      diffuse_texture
    })
  }

  pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    if new_size.width > 0 && new_size.height > 0 {
      self.size = new_size;
      self.config.width = new_size.width;
      self.config.height = new_size.height;
      self.surface.configure(&self.device, &self.config);
    };
  }

  pub fn input(&mut self, event: &WindowEvent) -> bool {
    match event {
      WindowEvent::CursorMoved { position: _, .. } => {
        // let x = position.x / self.size.width as f64;
        // let y = position.y / self.size.height as f64;
        // let z = (position.x + position.y) / (self.size.width +
        // self.size.height) as f64; self.clear_color =
        // na::Vector3::new(x, y, z);
      }
      WindowEvent::KeyboardInput {
        input:
          KeyboardInput {
            state: ElementState::Pressed,
            virtual_keycode: Some(VirtualKeyCode::Space),
            ..
          },
        ..
      } => {}
      _ => {}
    }
    false
  }

  pub fn update(&mut self) {}

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
          store: true,
        },
      })],
      depth_stencil_attachment: None,
    });

    render_pass.set_pipeline(&self.render_pipeline);
    render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
    render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
    render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    drop(render_pass);

    // submit 方法能传入任何实现了 IntoIter 的参数
    self.queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
  }
}
