use std::num::NonZeroU32;

use wgpu::{
  util::{BufferInitDescriptor, DeviceExt},
  *,
};

pub struct DeviceWarp {
  pub inner: wgpu::Device,
}
impl From<Device> for DeviceWarp {
  fn from(v: Device) -> Self {
    Self { inner: v }
  }
}
impl DeviceTrait for DeviceWarp {
  #[inline(always)]
  fn get_device(&self) -> &wgpu::Device {
    &self.inner
  }

  #[inline(always)]
  fn take(self) -> wgpu::Device {
    self.inner
  }
}
pub trait DeviceTrait {
  fn get_device(&self) -> &wgpu::Device;
  fn take(self) -> wgpu::Device;
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
      label: Some(label),
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
}
