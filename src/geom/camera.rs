use glam::{vec3, Mat4, Vec3};
use winit::keyboard::KeyCode;

use crate::{input, time};
pub struct Camera {
  // 摄像机的位置
  pub eye: Vec3,
  pub target: Vec3,
  pub up: Vec3,
}
impl Camera {
  pub fn new(eye: Vec3) -> Self {
    Camera {
      eye,
      target: vec3(eye.x, eye.y, eye.z + 1.0),
      up: vec3(0.0, 1.0, 0.0),
    }
  }

  pub fn move_forward_and_backward(&mut self, distance: f32) {
    self.eye.z += distance;
    self.target.x = self.eye.x;
    self.target.y = self.eye.y;
    self.target.z = self.eye.z + 1.0;
  }

  pub fn move_left_and_right(&mut self, distance: f32) {
    self.eye.x += distance;
    self.target.x = self.eye.x;
    self.target.y = self.eye.y;
  }

  pub fn move_upward_and_downward(&mut self, distance: f32) {
    self.eye.y += distance;
    self.target.x = self.eye.x;
    self.target.y = self.eye.y;
  }

  // 获取摄像机的视图矩阵
  pub fn get_view_mat(&self) -> Mat4 {
    Mat4::look_at_lh(self.eye, self.target, self.up)
    // Matrix4::look_at_lh(&na::Point3::new(0.0, 0.0, -1.0), &na::Point3::new(0.0, 0.0, 1.0), &na::Vector3::new(0.0, 1.0, 0.0))
  }

  // 获得透视投影矩阵
  // aspect: 宽高比
  pub fn get_proj_mat(&self, _aspect: f32) -> Mat4 {
    // could control zoom
    Mat4::orthographic_lh(-8.0, 8.0, -6.0, 6.0, 0.1, 4.0)
    // Matrix4::new_perspective(4.0/3.0, 80.0,  0.1, 60.0)
  }

  pub fn get_vp_mat(&self, aspect: f32) -> Mat4 {
    self.get_proj_mat(aspect) * self.get_view_mat()
    // self.get_view_mat()
    // self.get_proj_mat(aspect) * self.get_view_mat()
  }

  pub fn handle_input(&mut self) {
    let rate = time::get_delta() * 100.0;

    if input::get_key(KeyCode::Space) {
      self.move_forward_and_backward(-rate);
    }
    if input::get_key(KeyCode::ShiftLeft) {
      self.move_forward_and_backward(rate);
    }
    if input::get_key(KeyCode::KeyA) {
      self.move_left_and_right(-rate);
    }
    if input::get_key(KeyCode::KeyD) {
      self.move_left_and_right(rate);
    }
    if input::get_key(KeyCode::KeyW) {
      self.move_upward_and_downward(rate);
    }
    if input::get_key(KeyCode::KeyS) {
      self.move_upward_and_downward(-rate);
    }
  }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
  view_proj: Mat4,
}
impl CameraUniform {
  pub fn new() -> Self {
    Self {
      view_proj: Mat4::IDENTITY,
    }
  }

  pub fn update_view_proj(&mut self, camera: &Camera, aspect: f32) {
    self.view_proj = camera.get_vp_mat(aspect)
  }
}

#[test]
fn test_ort() {
  let point = glam::f32::vec4(3.0, 4.0, 5.0, 1.0);
  let view = glam::f32::Mat4::look_at_lh(
    glam::f32::vec3(0.0, 0.0, -1.0),
    glam::f32::vec3(0.0, 0.0, 1.0),
    glam::f32::vec3(0.0, 1.0, 0.0),
  );
  let ort = glam::f32::Mat4::orthographic_lh(-8.0, 8.0, -6.0, 6.0, -10.0, 10.0);
  println!("{}", view * point);
  println!("ort {}", ort * view * point);
}
