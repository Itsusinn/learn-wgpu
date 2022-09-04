use na::{Matrix4, Point3, UnitQuaternion};

pub struct Instance {
  pub position: Point3<f32>,
  pub rotation: UnitQuaternion<f32>,
}
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
  model: Matrix4<f32>,
}
impl Instance {
  pub fn to_raw(&self) -> InstanceRaw {
    let euler = self.rotation.euler_angles();
    let model = Matrix4::new_translation(&self.position.coords)
      * Matrix4::from_euler_angles(euler.0, euler.1, euler.2);
    InstanceRaw { model }
  }
}

impl InstanceRaw {
  const ATTRIBS: [wgpu::VertexAttribute; 4] =
    wgpu::vertex_attr_array![5 => Float32x4, 6 => Float32x4,7 => Float32x4,8 => Float32x4];

  pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
    use std::mem;
    wgpu::VertexBufferLayout {
      array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
      // 我们需要从把 Vertex 的 step mode 切换为 Instance
      // 这样着色器只有在开始处理一次新实例化绘制时，才会接受下一份实例
      step_mode: wgpu::VertexStepMode::Instance,
      attributes: &Self::ATTRIBS,
    }
  }
}
