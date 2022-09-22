use std::mem;

use bytemuck::{Pod, Zeroable};
use na::Vector3;

use crate::byteops;

pub struct Chunk {
  blocks: Vec<Block>, // blocks: [Block; 16 * 16 * 16],
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Block {
  points: [Vertex; 8],
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
  position: Vector3<f32>,
  tex_coords: Vector3<f32>,
}
// https://github.com/rust-lang/rust/issues/92476
pub fn gen_indices() -> Vec<u8> {
  let mut res = Vec::with_capacity(36 * 16 * 16 * 16);
  for index in 0..16 * 16 * 16u32 {
    for num in gen_indices_base(index * 32) {
      res.push(num)
    }
  }
  byteops::u32vec_to_u8vec(res)
}
#[test]
fn test() {
  let v = gen_indices();
  println!("{} kb", v.capacity() * 4 / 1024)
}

#[rustfmt::skip]
const fn gen_indices_base(base:u32)->[u32;36]{
  [
    base+0,base+3,base+1,  base+1,base+3,base+2, //Down
    base+4,base+5,base+6,  base+4,base+6,base+7, //Up
    base+0,base+1,base+5,  base+0,base+5,base+4, //Front
    base+2,base+3,base+7,  base+2,base+7,base+6, //Back
    base+0,base+7,base+3,  base+0,base+4,base+7, //Left
    base+1,base+2,base+6,  base+1,base+6,base+5  //Right
  ]
}
#[test]
pub fn test_align() {
  debug_assert_eq!(mem::align_of::<Block>(), mem::align_of::<u32>());
}
impl Chunk {
  pub fn to_bytes(self) -> Vec<u8> {
    let mut vec32: Vec<u32> = bytemuck::allocation::cast_vec(self.blocks);
    let vec8 = unsafe {
      let ratio = mem::size_of::<u32>() / mem::size_of::<u8>();

      let length = vec32.len() * ratio;
      let capacity = vec32.capacity() * ratio;
      let ptr = vec32.as_mut_ptr() as *mut u8;

      // Don't run the destructor for vec32
      mem::forget(vec32);

      // Construct new Vec
      Vec::from_raw_parts(ptr, length, capacity)
    };
    vec8
  }

  pub fn random() -> Chunk {
    let mut blocks: Vec<Block> = Vec::with_capacity(16 * 16 * 16);

    for z in 0..=15u8 {
      for y in 0..=15u8 {
        for x in 0..=15u8 {
          let index = (z as usize * 16 * 16) + (y as usize * 16) + x as usize;
          debug_assert_eq!(index, blocks.len());
          blocks.push(Block::new_with_index(
            &Vector3::<f32>::new(0.0, 0.0, 0.0),
            x,
            y,
            z,
          ));
        }
      }
    }
    debug_assert_eq!(blocks.len(), 16 * 16 * 16);
    Chunk { blocks }
  }
}
#[test]
fn size() {
  Chunk::random();
  println!("{}kb", std::mem::size_of::<Chunk>() / 1024)
}

impl Block {
  pub fn new_with_index(base: &Vector3<f32>, index_x: u8, index_y: u8, index_z: u8) -> Self {
    debug_assert!(index_x < 16 && index_y < 16 && index_z < 16);
    // Down-Front-Left
    let mut position1 = base.to_owned();
    position1.x = position1.x + index_x as f32;
    position1.y = position1.y + index_y as f32;
    position1.z = position1.z + index_z as f32;
    let position1 = Vertex {
      position: position1,
      tex_coords: Vector3::<f32>::new(0.0, 0.0, 0.0),
    };
    // Down-Front-Right
    let mut position2 = base.to_owned();
    position2.x = position2.x + index_x as f32 + 1.0;
    position2.y = position2.y + index_y as f32;
    position2.z = position2.z + index_z as f32;
    let position2 = Vertex {
      position: position2,
      tex_coords: Vector3::<f32>::new(1.0, 0.0, 0.0),
    };
    // Down-Back-Right
    let mut position3 = base.to_owned();
    position3.x = position3.x + index_x as f32 + 1.0;
    position3.y = position3.y + index_y as f32;
    position3.z = position3.z + index_z as f32 + 1.0;
    let position3 = Vertex {
      position: position3,
      tex_coords: Vector3::<f32>::new(1.0, 1.0, 0.0),
    };
    // Down-Back-Left
    let mut position4 = base.to_owned();
    position4.x = position4.x + index_x as f32;
    position4.y = position4.y + index_y as f32;
    position4.z = position4.z + index_z as f32 + 1.0;
    let position4 = Vertex {
      position: position4,
      tex_coords: Vector3::<f32>::new(0.0, 1.0, 0.0),
    };
    // Up-Front-Left
    let mut position5 = base.to_owned();
    position5.x = position5.x + index_x as f32;
    position5.y = position5.y + index_y as f32 + 1.0;
    position5.z = position5.z + index_z as f32;
    let position5 = Vertex {
      position: position5,
      tex_coords: Vector3::<f32>::new(1.0, 0.0, 1.0),
    };
    // Up-Front-Right
    let mut position6 = base.to_owned();
    position6.x = position6.x + index_x as f32 + 1.0;
    position6.y = position6.y + index_y as f32 + 1.0;
    position6.z = position6.z + index_z as f32;
    let position6 = Vertex {
      position: position6,
      tex_coords: Vector3::<f32>::new(1.0, 0.0, 1.0),
    };
    // Up-Back-Right
    let mut position7 = base.to_owned();
    position7.x = position7.x + index_x as f32 + 1.0;
    position7.y = position7.y + index_y as f32 + 1.0;
    position7.z = position7.z + index_z as f32 + 1.0;
    let position7 = Vertex {
      position: position7,
      tex_coords: Vector3::<f32>::new(1.0, 0.0, 1.0),
    };
    // Up-Back-Left
    let mut position8 = base.to_owned();
    position8.x = position8.x + index_x as f32;
    position8.y = position8.y + index_y as f32 + 1.0;
    position8.z = position8.z + index_z as f32 + 1.0;
    let position8 = Vertex {
      position: position8,
      tex_coords: Vector3::<f32>::new(1.0, 0.0, 1.0),
    };
    let points = [
      position1, position2, position3, position4, position5, position6, position7, position8,
    ];
    Block { points }
  }
}

impl Vertex {
  const ATTRIBS: [wgpu::VertexAttribute; 2] =
    wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

  pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
    wgpu::VertexBufferLayout {
      // array_stride 定义了每个顶点的宽度
      array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
      // step_model步长模式 告诉 pipeline 应以怎样的频率移动到下一个顶点
      step_mode: wgpu::VertexStepMode::Vertex,
      attributes: &Self::ATTRIBS,
    }
  }
}
