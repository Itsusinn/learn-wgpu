use na::Point2;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
  pub position: Point2<f32>,
  pub tex_id: u32,
}

impl Instance {
  const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![5 => Float32x2, 6 => Uint32];

  pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
    use std::mem;
    wgpu::VertexBufferLayout {
      array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
      // 我们需要从把 Vertex 的 step mode 切换为 Instance
      // 这样着色器只有在开始处理一次新实例化绘制时，才会接受下一份实例
      step_mode: wgpu::VertexStepMode::Instance,
      attributes: &Self::ATTRIBS,
    }
  }
}
