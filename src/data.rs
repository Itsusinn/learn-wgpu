// lib.rs
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
  relative_pos: [f32; 2],
}

impl Vertex {
  const ATTRIBS: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x2];

  pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
    use std::mem;
    wgpu::VertexBufferLayout {
      array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
      // 我们需要从把 Vertex 的 step mode 切换为 Instance
      // 这样着色器只有在开始处理一次新实例化绘制时，才会接受下一份实例
      step_mode: wgpu::VertexStepMode::Vertex,
      attributes: &Self::ATTRIBS,
    }
  }
}

// 2   3

// 0   1
pub const VERTICES: &[Vertex] = &[
  Vertex {
    relative_pos: [0f32, 0f32],
  },
  Vertex {
    relative_pos: [1f32, 0f32],
  },
  Vertex {
    relative_pos: [0f32, 1f32],
  },
  Vertex {
    relative_pos: [1f32, 1f32],
  },
];

pub const INDICES: &[u16] = &[0, 1, 2, 1, 3, 2];
