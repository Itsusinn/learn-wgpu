use na::Vector3;
use nalgebra as na;
use wgpu::{include_wgsl, Backends};
// lib.rs
use winit::{
  event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent},
  window::Window,
};

pub struct State {
  surface: wgpu::Surface,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  pub size: winit::dpi::PhysicalSize<u32>,
  render_pipeline: wgpu::RenderPipeline,
  render_pipeline2: wgpu::RenderPipeline,
  switch: bool,
  clear_color: na::Vector3<f64>,
}

impl State {
  // Creating some of the wgpu types requires async code
  pub async fn new(window: &Window) -> Self {
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
      .await
      .unwrap();
    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface.get_supported_formats(&adapter)[0],
      width: size.width,
      height: size.height,
      present_mode: wgpu::PresentMode::Fifo,
    };
    surface.configure(&device, &config);
    let shader = device.create_shader_module(include_wgsl!("../assets/shader.wgsl"));
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: Some("Render Pipeline Layout"),
      bind_group_layouts: &[],
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
        buffers: &[],
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


    let shader2 = device.create_shader_module(include_wgsl!("../assets/shader2.wgsl"));
    let render_pipeline2 = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: Some("Render Pipline"),
      layout: Some(&render_pipeline_layout),
      vertex: wgpu::VertexState {
        module: &shader2,
        // 指定应将着色器中的哪个函数作为 entry_point
        entry_point: "vs_main",
        // buffers 字段用于告知 wgpu 我们要传递给顶点着色器的顶点类型
        buffers: &[],
      },
      fragment: Some(wgpu::FragmentState {
        module: &shader2,
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
    Self {
      surface,
      device,
      queue,
      config,
      size,
      clear_color: na::Vector3::new(1.0, 1.0, 1.0),
      render_pipeline,
      render_pipeline2,
      switch: false
    }
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
      WindowEvent::CursorMoved { position, .. } => {
        let x = position.x / self.size.width as f64;
        let y = position.y / self.size.height as f64;
        let z = (position.x + position.y) / (self.size.width + self.size.height) as f64;
        self.clear_color = na::Vector3::new(x, y, z);
      }
      WindowEvent::KeyboardInput {
        input:
          KeyboardInput {
            state: ElementState::Pressed,
            virtual_keycode: Some(VirtualKeyCode::Space),
            ..
          },
        ..
      } => {
        self.switch = !self.switch;
      }
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
    if self.switch {
      render_pass.set_pipeline(&self.render_pipeline);
    } else {
      render_pass.set_pipeline(&self.render_pipeline2);
    }
    render_pass.draw(0..3, 0..1);
    drop(render_pass);

    // submit 方法能传入任何实现了 IntoIter 的参数
    self.queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
  }
}
