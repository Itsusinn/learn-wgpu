use std::io::{BufReader, Cursor};

use color_eyre::eyre::Result;

use crate::{exts::state::DeviceTrait, model, texture};
pub async fn load_str(filename: &str) -> Result<String> {
  let path = std::path::Path::new(env!("OUT_DIR"))
    .join("assets")
    .join(filename);
  let text = tokio::fs::read_to_string(path).await?;
  Ok(text)
}

pub async fn load_binary(filename: &str) -> Result<Vec<u8>> {
  let path = std::path::Path::new(env!("OUT_DIR"))
    .join("assets")
    .join(filename);
  let data = tokio::fs::read(path).await?;
  Ok(data)
}
pub async fn load_texture<T: DeviceTrait>(
  filename: &str,
  device: &T,
  queue: &wgpu::Queue,
) -> Result<texture::Texture> {
  let data = load_binary(filename).await?;
  texture::Texture::from_bytes(device, queue, &data, filename)
}

pub async fn load_model<T: DeviceTrait>(
  filename: &str,
  device: &T,
  queue: &wgpu::Queue,
  layout: &wgpu::BindGroupLayout,
) -> Result<model::Model> {
  let obj_text = load_str(filename).await?;
  let obj_cursor = Cursor::new(obj_text);
  let mut obj_reader = BufReader::new(obj_cursor);
  let (models, obj_materials) = tobj::load_obj_buf_async(
    &mut obj_reader,
    &tobj::LoadOptions {
      triangulate: true,
      single_index: true,
      ..Default::default()
    },
    |p| async move {
      let mat_text = load_str(&p).await.unwrap();
      tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
    },
  )
  .await?;
  let mut materials = Vec::new();
  for m in obj_materials? {
    let diffuse_texture = load_texture(&m.diffuse_texture, device, queue).await?;
    let bind_group = device.create_bind_group(
      "",
      layout,
      &[
        wgpu::BindGroupEntry {
          binding: 0,
          resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
        },
        wgpu::BindGroupEntry {
          binding: 1,
          resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
        },
      ],
    );
    materials.push(model::Material {
      name: m.name,
      diffuse_texture,
      bind_group,
    });
  }
  let meshes = models
    .into_iter()
    .map(|m| {
      let vertices = (0..m.mesh.positions.len() / 3)
        .map(|i| model::ModelVertex {
          position: na::Point3::new(
            m.mesh.positions[i * 3],
            m.mesh.positions[i * 3 + 1],
            m.mesh.positions[i * 3 + 2],
          ),
          tex_coords: na::Point2::new(m.mesh.texcoords[i * 2], m.mesh.texcoords[i * 2 + 1]),
          normal: na::Vector3::new(
            m.mesh.normals[i * 3],
            m.mesh.normals[i * 3 + 1],
            m.mesh.normals[i * 3 + 2],
          ),
        })
        .collect::<Vec<_>>();

      let vertex_buffer = device.create_buffer_init(
        &format!("{} Vertex Buffer", filename),
        bytemuck::cast_slice(&vertices),
        wgpu::BufferUsages::VERTEX,
      );
      let index_buffer = device.create_buffer_init(
        &format!("{} Index Buffer", filename),
        bytemuck::cast_slice(&m.mesh.indices),
        wgpu::BufferUsages::INDEX,
      );
      model::Mesh {
        name: filename.to_string(),
        vertex_buffer,
        index_buffer,
        num_elements: m.mesh.indices.len() as u32,
        material: m.mesh.material_id.unwrap_or(0),
      }
    })
    .collect::<Vec<_>>();
  Ok(model::Model { meshes, materials })
}
