use std::path::Path;

use color_eyre::eyre::Result;
use image::GenericImageView;

use crate::exts::state::DeviceTrait;

pub struct Texture {
  pub texture: wgpu::Texture,
  pub view: wgpu::TextureView,
  pub sampler: wgpu::Sampler,
}

impl Texture {
  pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

  pub fn load<P: AsRef<Path>, T: DeviceTrait>(
    device: &T,
    queue: &wgpu::Queue,
    path: P,
  ) -> Result<Self> {
    let path_copy = path.as_ref().to_path_buf();
    let label = path_copy.to_str();
    let img = image::open(path)?;
    Self::from_image(device, queue, &img, label)
  }

  pub fn from_bytes<T: DeviceTrait>(
    device: &T,
    queue: &wgpu::Queue,
    bytes: &[u8],
    label: &str,
  ) -> Result<Self> {
    let img = image::load_from_memory(bytes)?;
    Self::from_image(device, queue, &img, Some(label))
  }

  pub fn from_image<T: DeviceTrait>(
    device: &T,
    queue: &wgpu::Queue,
    img: &image::DynamicImage,
    label: Option<&str>,
  ) -> Result<Self> {
    let rgba = img.to_rgba8();
    let dimensions = img.dimensions();

    let size = wgpu::Extent3d {
      width: dimensions.0,
      height: dimensions.1,
      depth_or_array_layers: 1,
    };
    let texture = device
      .get_device()
      .create_texture(&wgpu::TextureDescriptor {
        label,
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
        view_formats: &[],
      });

    // 填充数据到纹理中
    queue.write_texture(
      // 告诉 wgpu 从何处复制像素数据
      wgpu::ImageCopyTexture {
        texture: &texture,
        mip_level: 0,
        origin: wgpu::Origin3d::ZERO,
        aspect: wgpu::TextureAspect::All,
      },
      // 实际的像素数据
      &rgba,
      // 纹理的内存布局
      wgpu::ImageDataLayout {
        offset: 0,
        bytes_per_row: Some(4 * dimensions.0),
        rows_per_image: Some(dimensions.1),
      },
      size,
    );
    // 我们无需手动配置纹理视图，让 wgpu 定义它即可
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = device
      .get_device()
      .create_sampler(&wgpu::SamplerDescriptor {
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

  pub fn create_depth_texture<T>(
    device: &T,
    config: &wgpu::SurfaceConfiguration,
    label: &str,
  ) -> Self
  where
    T: DeviceTrait,
  {
    let device = device.get_device();

    let size = wgpu::Extent3d {
      // 如果想得到正确的渲染效果，深度纹理需要和屏幕一样大。我们可以用 config 来确保深度纹理与
      // surface 纹理的尺寸相同
      width: config.width,
      height: config.height,
      depth_or_array_layers: 1,
    };
    let desc = wgpu::TextureDescriptor {
      label: Some(label),
      size,
      mip_level_count: 1,
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      format: Self::DEPTH_FORMAT,
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT // 对这个纹理做渲染，因此需要给它添加 RENDER_ATTACHMENT 配置
            | wgpu::TextureUsages::TEXTURE_BINDING,
      view_formats: &[],
    };
    let texture = device.create_texture(&desc);

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
      // 从技术上而言，我们不需要深度纹理的采样器，但 Texture struct
      // 需要它。并且如果我们想自己对深度纹理做采样，这时也会需要使用采样器。
      address_mode_u: wgpu::AddressMode::ClampToEdge,
      address_mode_v: wgpu::AddressMode::ClampToEdge,
      address_mode_w: wgpu::AddressMode::ClampToEdge,
      mag_filter: wgpu::FilterMode::Linear,
      min_filter: wgpu::FilterMode::Linear,
      mipmap_filter: wgpu::FilterMode::Nearest,
      compare: Some(wgpu::CompareFunction::LessEqual), // 5.
      lod_min_clamp: 0.0,
      lod_max_clamp: 100.0,
      ..Default::default()
    });

    Self {
      texture,
      view,
      sampler,
    }
  }
}
