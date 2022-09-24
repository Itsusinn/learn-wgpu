use std::num::NonZeroU32;

use wgpu::{
  util::{BufferInitDescriptor, DeviceExt},
  *,
};

#[derive(Debug)]
pub struct DeviceWarp<'a> {
  pub inner: &'a wgpu::Device,
}
impl DeviceWarp<'_> {
  pub fn wrap<'a>(inner: &'a wgpu::Device) -> DeviceWarp<'a> {
    DeviceWarp { inner }
  }
}
impl DeviceTrait for DeviceWarp<'_> {
  #[inline(always)]
  fn get_device(&self) -> &wgpu::Device {
    &self.inner
  }
}

pub trait DeviceTrait {
  fn get_device(&self) -> &wgpu::Device;
  #[inline(always)]
  fn create_bind_group_layout<'a>(
    &self,
    label: &str,
    entries: &'a [BindGroupLayoutEntry],
  ) -> BindGroupLayout {
    self
      .get_device()
      .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some(label),
        entries,
      })
  }
  #[inline(always)]
  fn create_bind_group<'a>(
    &self,
    label: &str,
    layout: &'a BindGroupLayout,
    entries: &'a [BindGroupEntry<'a>],
  ) -> BindGroup {
    self.get_device().create_bind_group(&BindGroupDescriptor {
      label: if label.is_empty() { None } else { Some(label) },
      layout,
      entries,
    })
  }
  #[inline(always)]
  fn create_shader_module(&self, desc: ShaderModuleDescriptor) -> ShaderModule {
    self.get_device().create_shader_module(desc)
  }
  #[inline(always)]
  fn create_buffer_init<'a>(&self, label: &str, contents: &'a [u8], usage: BufferUsages) -> Buffer {
    self.get_device().create_buffer_init(&BufferInitDescriptor {
      label: Some(label),
      contents,
      usage,
    })
  }
  #[inline(always)]
  fn create_pipeline_layout<'a>(
    &self,
    label: &str,
    bind_group_layouts: &'a [&'a BindGroupLayout],
    push_constant_ranges: &'a [PushConstantRange],
  ) -> PipelineLayout {
    self
      .get_device()
      .create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some(label),
        bind_group_layouts,
        push_constant_ranges,
      })
  }
  #[inline(always)]
  fn create_render_pipeline<'a>(
    &self,
    label: &str,
    layout: Option<&'a PipelineLayout>,
    vertex: VertexState<'a>,
    primitive: PrimitiveState,
    depth_stencil: Option<DepthStencilState>,
    multisample: MultisampleState,
    fragment: FragmentState<'a>,
    multiview: Option<NonZeroU32>,
  ) -> RenderPipeline {
    self
      .get_device()
      .create_render_pipeline(&RenderPipelineDescriptor {
        label: Some(label),
        layout,
        vertex,
        primitive,
        depth_stencil,
        multisample,
        fragment: Some(fragment),
        multiview,
      })
  }
  #[inline(always)]
  fn create_texture(
    &self,
    label: &str,
    size: Extent3d,
    mip_level_count: u32,
    sample_count: u32,
    dimension: TextureDimension,
    format: TextureFormat,
    usage: TextureUsages,
  ) -> Texture {
    self.get_device().create_texture(&TextureDescriptor {
      label: Some(label),
      size,
      mip_level_count,
      sample_count,
      dimension,
      format,
      usage,
    })
  }
}
