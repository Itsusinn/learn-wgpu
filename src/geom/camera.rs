use na::{Matrix4, Point3, Vector3};
use winit::event::VirtualKeyCode as Keycode;

use crate::{input, time};
pub struct Camera {
  // 摄像机的位置
  pub eye: Point3<f32>,
  // 摄像机的看向
  toward: Vector3<f32>,
  // 摄像机朝上的方向
  up: Vector3<f32>,
  // 俯仰角
  pitch: f32,
  // 偏航角
  yaw: f32,
  // 近平面距离
  znear: f32,
  // 远平面距离
  zfar: f32,
  // 视域(角度)
  fov: f32,
}
impl Camera {
  pub fn new(eye: Point3<f32>) -> Self {
    // 格拉姆—施密特正交化(Gram-Schmidt Process)。 <https://en.wikipedia.org/wiki/Gram-Schmidt_process>
    let toward = Vector3::new(0.0, 0.0, 1.0);
    let right = Vector3::y_axis().cross(&toward);
    let up = toward.cross(&right).normalize();
    Camera {
      eye,
      toward,
      up,
      pitch: 0.0,
      yaw: -90.0,
      znear: 0.1,
      zfar: 100.0,
      fov: 45.0,
    }
  }

  pub fn move_forward_and_backward(&mut self, distance: f32) {
    let change = Vector3::new(self.toward.x, 0.0, self.toward.z).normalize();
    let delta = change * distance;
    self.eye += delta
  }

  pub fn move_left_and_right(&mut self, distance: f32) {
    let right = Vector3::y_axis().cross(&self.toward).normalize();
    let delta = right * distance;
    self.eye += delta;
  }

  pub fn move_upward_and_downward(&mut self, distance: f32) {
    self.eye.y += distance;
  }

  pub fn turn_right_and_left(&mut self, angle: f32) {
    self.yaw += angle;
    self.toward.x = self.pitch.to_radians().cos() * self.yaw.to_radians().cos();
    self.toward.z = self.pitch.to_radians().cos() * self.yaw.to_radians().sin();
    self.gs_process();
  }

  pub fn turn_up_and_down(&mut self, angle: f32) {
    self.pitch += angle;
    if self.pitch >= 89.0 {
      self.pitch = 89.0
    } else if self.pitch <= -89.0 {
      self.pitch = -89.0
    }
    self.toward.x = self.pitch.to_radians().cos() * self.yaw.to_radians().cos();
    self.toward.y = self.pitch.to_radians().sin();
    self.toward.z = self.pitch.to_radians().cos() * self.yaw.to_radians().sin();
    self.gs_process()
  }

  // Gram-Schmidt Process, 正交化
  fn gs_process(&mut self) {
    let right = Vector3::y_axis().cross(&self.toward);
    self.up = self.toward.cross(&right).normalize();
  }

  // 获取摄像机的视图矩阵
  pub fn get_view_mat(&self) -> Matrix4<f32> {
    Matrix4::look_at_lh(&self.eye, &(&self.eye + &self.toward), &self.up)
  }

  // 获得透视投影矩阵
  // aspect: 宽高比
  pub fn get_proj_mat(&self, aspect: f32) -> Matrix4<f32> {
    Matrix4::new_perspective(aspect, self.fov, self.znear, self.zfar)
  }

  pub fn get_vp_mat(&self, aspect: f32) -> Matrix4<f32> {
    // OPENGL_TO_WGPU_MATRIX * self.get_proj_mat(aspect) * self.get_view_mat()
    self.get_proj_mat(aspect) * self.get_view_mat()
  }

  pub fn handle_input(&mut self) {
    let (dx, dy) = input::fetch_motion();
    let rate = time::get_delta() * 100.0;
    if dx != 0 {
      self.turn_right_and_left((dx as f32) / 10.0);
    }
    if dy != 0 {
      self.turn_up_and_down((dy as f32) / 10.0);
    }
    if input::get_key(Keycode::W) {
      self.move_forward_and_backward(-rate);
    }
    if input::get_key(Keycode::S) {
      self.move_forward_and_backward(rate);
    }
    if input::get_key(Keycode::A) {
      self.move_left_and_right(-rate);
    }
    if input::get_key(Keycode::D) {
      self.move_left_and_right(rate);
    }
    if input::get_key(Keycode::Space) {
      self.move_upward_and_downward(rate);
    }
    if input::get_key(Keycode::LShift) {
      self.move_upward_and_downward(-rate);
    }
  }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
  view_proj: Matrix4<f32>,
}
impl CameraUniform {
  pub fn new() -> Self {
    Self {
      view_proj: Matrix4::identity(),
    }
  }

  pub fn update_view_proj(&mut self, camera: &Camera, aspect: f32) {
    self.view_proj = camera.get_vp_mat(aspect)
  }
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);
