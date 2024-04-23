pub struct TextureArray {
  pub texture: wgpu::Texture,
  pub texture_view: wgpu::TextureView,
  pub sampler: wgpu::Sampler,
  pub bind_group_layout: wgpu::BindGroupLayout,
  pub bind_group: wgpu::BindGroup
}
impl TextureArray {
  pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
    let diffuse_bytes = include_bytes!("../assets/crawl-tiles/dc-dngn/floor/bog_green0.png");
    let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();
    let diffuse_rgba = diffuse_image.to_rgba8();

    use image::GenericImageView;
    let dimensions = diffuse_image.dimensions();

    let texture = device.create_texture(&wgpu::TextureDescriptor {
      // 所有纹理都是以 3D 形式存储的，我们通过设置深度 1 来表示 2D 纹理
      size: wgpu::Extent3d {
        width: dimensions.0,
        height: dimensions.1,
        depth_or_array_layers: 6,
      },
      mip_level_count: 1, // 后面会详细介绍此字段
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      // 大多数图像都是使用 sRGB 来存储的，我们需要在这里指定。
      format: wgpu::TextureFormat::Rgba8UnormSrgb,
      // TEXTURE_BINDING 表示我们要在着色器中使用这个纹理。
      // COPY_DST 表示我们能将数据复制到这个纹理上。
      usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
      label: Some("texture_array"),
      view_formats: &[],
    });
    for index in 0..6u32 {
      queue.write_texture(
        // 告诉 wgpu 从何处复制像素数据
        wgpu::ImageCopyTexture {
          texture: &texture,
          mip_level: 0,
          origin: wgpu::Origin3d {
            x: 0,
            y: 0,
            z: index,
          },
          aspect: wgpu::TextureAspect::All,
        },
        // 实际像素数据
        &diffuse_rgba,
        // 纹理的内存布局
        wgpu::ImageDataLayout {
          offset: 0,
          bytes_per_row: Some(4 * dimensions.0),
          rows_per_image: Some(dimensions.1),
        },
        wgpu::Extent3d {
          width: dimensions.0,
          height: dimensions.1,
          depth_or_array_layers: 1,
        },
      );
    }

    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
      label: Some(&format!("array view")),
      dimension: Some(wgpu::TextureViewDimension::D2Array),
      ..wgpu::TextureViewDescriptor::default()
    });
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
      address_mode_u: wgpu::AddressMode::ClampToEdge,
      address_mode_v: wgpu::AddressMode::ClampToEdge,
      address_mode_w: wgpu::AddressMode::ClampToEdge,
      mag_filter: wgpu::FilterMode::Linear,
      min_filter: wgpu::FilterMode::Nearest,
      mipmap_filter: wgpu::FilterMode::Nearest,
      ..Default::default()
    });
    let bind_group_layout = Self::new_bind_group_layout(device);
    let bind_group = Self::new_bind_group(device, &bind_group_layout, &texture_view, &sampler);
    Self {
      texture,
      texture_view,
      sampler,
      bind_group_layout,
      bind_group,
    }
  }

  fn new_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      label: Some("texture_bind_group_layout"),
      entries: &[
        wgpu::BindGroupLayoutEntry {
          binding: 0,
          visibility: wgpu::ShaderStages::FRAGMENT,
          ty: wgpu::BindingType::Texture {
            multisampled: false,
            view_dimension: wgpu::TextureViewDimension::D2Array,
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
    })
  }

  fn new_bind_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    view: &wgpu::TextureView,
    sampler: &wgpu::Sampler,
  ) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &layout,
      entries: &[
        wgpu::BindGroupEntry {
          binding: 0,
          resource: wgpu::BindingResource::TextureView(view),
        },
        wgpu::BindGroupEntry {
          binding: 1,
          resource: wgpu::BindingResource::Sampler(sampler),
        },
      ],
      label: Some("texture_bind_group"),
    })
  }
}
