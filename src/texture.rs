use color_eyre::eyre::Result;
use image::GenericImageView;

pub struct Texture {
  pub texture: wgpu::Texture,
  pub view: wgpu::TextureView,
  pub sampler: wgpu::Sampler,
}

impl Texture {
  pub fn from_bytes(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    bytes: &[u8],
    label: &str,
  ) -> Result<Self> {
    let img = image::load_from_memory(bytes)?;
    Self::from_image(device, queue, &img, Some(label))
  }

  pub fn from_image(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    img: &image::DynamicImage,
    label: Option<&str>,
  ) -> Result<Self> {
    let rgba = img.as_rgba8().unwrap();
    let dimensions = img.dimensions();

    let size = wgpu::Extent3d {
      width: dimensions.0,
      height: dimensions.1,
      depth_or_array_layers: 1,
    };
    let texture = device.create_texture(&wgpu::TextureDescriptor {
      // 所有纹理都会以三维数组形式存储，我们通过设置深度为 1 来表示这是二维的纹理
      size,
      mip_level_count: 1, // 我们后面会介绍这里的细节
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      // 多数图像都使用 sRGB 格式，所以我们需要在此将其体现出来
      format: wgpu::TextureFormat::Rgba8UnormSrgb,
      // TEXTURE_BINDING 告诉 wgpu 我们想在着色器中使用这个纹理
      // COPY_DST 则表示我们想把数据复制到这个纹理
      usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
      label,
    });

    queue.write_texture(
      // 告诉 wgpu 从何处复制像素数据
      wgpu::ImageCopyTexture {
        texture: &texture,
        mip_level: 0,
        origin: wgpu::Origin3d::ZERO,
        aspect: wgpu::TextureAspect::All,
      },
      // 实际的像素数据
      rgba,
      // 纹理的内存布局
      wgpu::ImageDataLayout {
        offset: 0,
        bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0),
        rows_per_image: std::num::NonZeroU32::new(dimensions.1),
      },
      size,
    );
    // 我们无需手动配置纹理视图，让 wgpu 定义它即可
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
      address_mode_u: wgpu::AddressMode::ClampToEdge,
      address_mode_v: wgpu::AddressMode::ClampToEdge,
      address_mode_w: wgpu::AddressMode::ClampToEdge,
      mag_filter: wgpu::FilterMode::Linear,
      min_filter: wgpu::FilterMode::Nearest,
      mipmap_filter: wgpu::FilterMode::Nearest,
      ..Default::default()
    });
    Ok(Self {
      texture,
      view,
      sampler,
    })
  }
}
