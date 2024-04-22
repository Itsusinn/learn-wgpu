use std::{
  ops::Deref,
  sync::atomic::{AtomicI32, Ordering::SeqCst},
};

use dashmap::DashMap;
use once_cell::sync::Lazy;
use winit::{
  event::{DeviceEvent, ElementState, KeyEvent, WindowEvent},
  keyboard::{KeyCode, PhysicalKey},
};

use crate::time;

static KEYMAP: Lazy<KeyMap> = Lazy::new(|| KeyMap::new());
static MOUSE: Lazy<Mouse> = Lazy::new(|| Mouse::new());
static COOLDOWN_MAP: Lazy<DashMap<KeyCode, f32>> = Lazy::new(|| DashMap::new());

struct Mouse {
  dx: AtomicI32,
  dy: AtomicI32,
}
impl Mouse {
  fn new() -> Self {
    Mouse {
      dx: AtomicI32::new(0),
      dy: AtomicI32::new(0),
    }
  }

  fn store_motion(&self, x: f64, y: f64) {
    self.dx.fetch_add(x as i32, SeqCst);
    self.dy.fetch_add(y as i32, SeqCst);
  }
}
struct KeyMap {
  inner: DashMap<KeyCode, bool>,
}
impl KeyMap {
  fn new() -> Self {
    Self {
      inner: DashMap::new(),
    }
  }
}
impl Deref for KeyMap {
  type Target = DashMap<KeyCode, bool>;

  fn deref(&self) -> &DashMap<KeyCode, bool> {
    &self.inner
  }
}

pub fn fetch_motion() -> (i32, i32) {
  let dx = MOUSE.dx.swap(0, SeqCst);
  let dy = MOUSE.dy.swap(0, SeqCst);
  (dx, dy)
}
pub fn get_key(keycode: KeyCode) -> bool {
  let pair = KEYMAP.inner.get(&keycode);
  match pair {
    None => return false,
    Some(pair) => return *pair,
  }
}
pub fn get_key_with_cooldown(keycode: KeyCode, cooltime: f32) -> bool {
  let tv = get_key(keycode);
  // 若按键本就未按下，则返回false
  if !tv {
    return false;
  }
  if !COOLDOWN_MAP.contains_key(&keycode) {
    // 不含该键，说明第一次按下该键
    let now = time::get_now();
    COOLDOWN_MAP.insert(keycode, now);
    return true;
  } else {
    let mut last = COOLDOWN_MAP.get_mut(&keycode).unwrap();
    let now = time::get_now();
    if (now - *last) > cooltime {
      *last = now;
      return true;
    } else {
      return false;
    }
  }
}
pub fn handle_window_event(event: &WindowEvent) {
  match event {
    // WindowEvent::CursorMoved { delta } => {
    //   MOUSE.store_motion(delta.0, delta.1);
    // }
    WindowEvent::KeyboardInput {
      device_id: _,
      event:
        KeyEvent {
          state,
          physical_key: PhysicalKey::Code(keycode),
          logical_key: _,
          ..
        },
      ..
    } => {
      match state {
        ElementState::Pressed => KEYMAP.insert(keycode.to_owned(), true),
        ElementState::Released => KEYMAP.insert(keycode.to_owned(), false),
      };
    }
    _ => {}
  }
}

pub fn handle_device_event(event: &DeviceEvent) {}
