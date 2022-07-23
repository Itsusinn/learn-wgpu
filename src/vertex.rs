use na::{Vector2, Vector3};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
  position: Vector3<f32>,
  tex_coords: Vector2<f32>,
}
impl Vertex {
  const ATTRIBS: [wgpu::VertexAttribute; 2] =
    wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

  pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
    wgpu::VertexBufferLayout {
      // array_stride 定义了每个顶点的宽度
      array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
      // step_mode 告诉 pipeline 应以怎样的频率移动到下一个顶点
      step_mode: wgpu::VertexStepMode::Vertex,
      attributes: &Self::ATTRIBS,
    }
  }
}

pub const VERTICES: &[Vertex] = &[
  Vertex {
    position: Vector3::new(-0.0868241, 0.49240386, 0.0),
    tex_coords: Vector2::new(0.4131759, 0.99240386),
  }, // A
  Vertex {
    position: Vector3::new(-0.49513406, 0.06958647, 0.0),
    tex_coords: Vector2::new(0.0048659444, 0.56958647),
  }, // B
  Vertex {
    position: Vector3::new(-0.21918549, -0.44939706, 0.0),
    tex_coords: Vector2::new(0.28081453, 0.05060294),
  }, // C
  Vertex {
    position: Vector3::new(0.35966998, -0.3473291, 0.0),
    tex_coords: Vector2::new(0.85967, 0.1526709),
  }, // D
  Vertex {
    position: Vector3::new(0.44147372, 0.2347359, 0.0),
    tex_coords: Vector2::new(0.9414737, 0.7347359),
  }, // E
];

pub const INDICES: &[u16] = &[0, 4, 1, 1, 4, 2, 2, 4, 3];
