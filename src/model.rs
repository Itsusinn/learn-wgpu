use std::{ops::Range, path::Path};

use na::{Point2, Point3, Vector3};
use tobj::LoadOptions;
use wgpu::{vertex_attr_array, VertexAttribute};

use crate::{exts::state::DeviceTrait, texture};

pub trait VertexTrait {
  fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex {
  pub position: Point3<f32>,
  pub tex_coords: Point2<f32>,
  pub normal: Vector3<f32>,
}
impl ModelVertex {
  const ATTRI: [VertexAttribute; 3] = vertex_attr_array![0=> Float32x3,1=> Float32x2,2=> Float32x3];
}
impl VertexTrait for ModelVertex {
  fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
    wgpu::VertexBufferLayout {
      array_stride: std::mem::size_of::<ModelVertex>() as wgpu::BufferAddress,
      step_mode: wgpu::VertexStepMode::Vertex,
      attributes: &Self::ATTRI,
    }
  }
}

pub struct Model {
  pub meshes: Vec<Mesh>,
  pub materials: Vec<Material>,
}

pub struct Material {
  pub name: String,
  pub diffuse_texture: texture::Texture,
  pub bind_group: wgpu::BindGroup,
}

pub struct Mesh {
  pub name: String,
  pub vertex_buffer: wgpu::Buffer,
  pub index_buffer: wgpu::Buffer,
  pub num_elements: u32,
  // 在绘制时用于索引 materials 列表
  pub material: usize,
}

pub trait DrawModel<'a> {
  fn draw_mesh(
    &mut self,
    mesh: &'a Mesh,
    material: &'a Material,
    camera_bind_group: &'a wgpu::BindGroup,
  );
  fn draw_mesh_instanced(
    &mut self,
    mesh: &'a Mesh,
    material: &'a Material,
    instances: Range<u32>,
    camera_bind_group: &'a wgpu::BindGroup,
  );
  fn draw_model(&mut self, model: &'a Model, camera_bind_group: &'a wgpu::BindGroup);
  fn draw_model_instanced(
    &mut self,
    model: &'a Model,
    instances: Range<u32>,
    camera_bind_group: &'a wgpu::BindGroup,
  );
}

impl<'a, 'b> DrawModel<'b> for wgpu::RenderPass<'a>
where
  'b: 'a,
{
  fn draw_mesh(
    &mut self,
    mesh: &'b Mesh,
    material: &'b Material,
    camera_bind_group: &'b wgpu::BindGroup,
  ) {
    self.draw_mesh_instanced(mesh, material, 0..1, camera_bind_group);
  }

  fn draw_mesh_instanced(
    &mut self,
    mesh: &'b Mesh,
    material: &'b Material,
    instances: Range<u32>,
    camera_bind_group: &'b wgpu::BindGroup,
  ) {
    self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
    self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
    self.set_bind_group(0, &material.bind_group, &[]);
    self.set_bind_group(1, camera_bind_group, &[]);
    self.draw_indexed(0..mesh.num_elements, 0, instances);
  }

  fn draw_model(&mut self, model: &'b Model, camera_bind_group: &'b wgpu::BindGroup) {
    self.draw_model_instanced(model, 0..1, camera_bind_group);
  }

  fn draw_model_instanced(
    &mut self,
    model: &'b Model,
    instances: Range<u32>,
    camera_bind_group: &'b wgpu::BindGroup,
  ) {
    for mesh in &model.meshes {
      let material = &model.materials[mesh.material];
      self.draw_mesh_instanced(mesh, material, instances.clone(), camera_bind_group);
    }
  }
}
