pub mod ext;
pub mod exts;
pub mod geom;
pub mod input;
pub mod instance;
mod light;
mod log;
pub mod model;
pub mod res;
pub mod state;
pub mod texture;
pub mod time;
mod world;

use std::sync::Arc;

use color_eyre::eyre::Result;
use state::State;
use winit::{
  event::*,
  event_loop::EventLoop,
  keyboard::{self, KeyCode, NamedKey},
  window::{CursorGrabMode, WindowBuilder},
};

#[tokio::main]
async fn main() -> Result<()> {
  #[cfg(debug_assertions)]
  std::env::set_var("RUST_BACKTRACE", "full");
  #[cfg(not(debug_assertions))]
  std::env::set_var("RUST_BACKTRACE", "1");

  color_eyre::install()?;

  crate::log::init().await?;
  time::get_now();
  let event_loop = EventLoop::new()?;
  let window = WindowBuilder::new().build(&event_loop)?;
  let window = Arc::new(window);
  let mut state = State::new(window.clone()).await?;

  let mut focus = false;
  let mut cursor_visible = true;

  event_loop.run(move |event, elwt| match event {
    Event::DeviceEvent { device_id: _, event } => {
      // handle mouse input
      input::handle_device_event(&event)
    }
    Event::WindowEvent { event, window_id: _ } => {
      // handle keyboarc input
      input::handle_window_event(&event);
      match event {
        WindowEvent::CloseRequested
        | WindowEvent::KeyboardInput {
          event:
            KeyEvent {
              state: ElementState::Pressed,
              logical_key: keyboard::Key::Named(NamedKey::Escape),
              ..
            },
          ..
        } => elwt.exit(),
        WindowEvent::Resized(physical_size) => {
          state.resize(physical_size);
        }
        WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
          // state.resize(*new_inner_size);
        }
        WindowEvent::Focused(v) => {
          focus = v;
        }
        WindowEvent::RedrawRequested => {
          state.update();
          match state.render() {
            Ok(_) => {}
            // 如果发生上下文丢失，就重新配置 surface
            Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
            // 系统内存不足，此时应该退出
            Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
            // 所有其他错误（如过时、超时等）都应在下一帧解决
            Err(e) => eprintln!("{:?}", e),
          }
          // 除非手动请求，否则 RedrawRequested 只会触发一次
          window.request_redraw();
        }
        _ => {}
      }
      if input::get_key_with_cooldown(KeyCode::ControlLeft, 0.3) {
        cursor_visible = !cursor_visible;
        window.set_cursor_visible(cursor_visible);
        if cursor_visible {
          window.set_cursor_grab(CursorGrabMode::None).unwrap();
        } else {
          window
            .set_cursor_grab(CursorGrabMode::Confined)
            .or_else(|_| window.set_cursor_grab(CursorGrabMode::Locked))
            .unwrap();
        }
      };
    }
    _ => {}
  })?;
  Ok(())
}
