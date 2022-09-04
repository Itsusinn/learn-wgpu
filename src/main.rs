pub mod exts;
pub mod geom;
pub mod input;
pub mod instance;
pub mod ext;
pub mod model;
pub mod res;
pub mod state;
pub mod texture;
pub mod time;
mod log;
mod world;
mod light;

use color_eyre::eyre::Result;
use state::State;
use winit::{
  event::{VirtualKeyCode as Keycode, *},
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
};

#[macro_use]
extern crate tracing;

#[tokio::main]
async fn main() -> Result<()> {
  #[cfg(debug_assertions)]
  std::env::set_var("RUST_BACKTRACE", "full");
  #[cfg(not(debug_assertions))]
  std::env::set_var("RUST_BACKTRACE", "1");

  if cfg!(feature = "color") {
    color_eyre::install()?;
  } else {
    color_eyre::config::HookBuilder::new()
      .theme(color_eyre::config::Theme::new())
      .install()?;
  }
  crate::log::init().await?;
  time::get_now();
  let event_loop = EventLoop::new();
  let window = WindowBuilder::new().build(&event_loop)?;
  let mut state = State::new(&window).await?;

  let mut focus = false;
  let mut cursor_visible = true;

  event_loop.run(move |event, _, control_flow| match event {
    Event::WindowEvent { event, window_id } if window_id == window.id() => {
      if state.input(&event) {
        return;
      }
      match event {
        WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
          input:
            KeyboardInput {
              state: ElementState::Pressed,
              virtual_keycode: Some(VirtualKeyCode::Escape),
              ..
            },
          ..
        } => *control_flow = ControlFlow::Exit,
        WindowEvent::Resized(physical_size) => {
          state.resize(physical_size);
        }
        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
          state.resize(*new_inner_size);
        }
        WindowEvent::Focused(v) => {
          focus = v;
        }
        _ => {}
      }
      if input::get_key_with_cooldown(Keycode::LControl, 0.3) {
        cursor_visible = !cursor_visible;
        window.set_cursor_visible(cursor_visible);
        window.set_cursor_grab(!cursor_visible).unwrap();
      };
    }
    Event::DeviceEvent {
      device_id: _,
      event,
    } => {
      if focus {
        input::handle_input(&event);
      }
    }
    Event::RedrawRequested(window_id) if window_id == window.id() => {
      state.update();
      match state.render() {
        Ok(_) => {}
        // 如果发生上下文丢失，就重新配置 surface
        Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
        // 系统内存不足，此时应该退出
        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
        // 所有其他错误（如过时、超时等）都应在下一帧解决
        Err(e) => eprintln!("{:?}", e),
      }
    }
    Event::MainEventsCleared => {
      // 除非手动请求，否则 RedrawRequested 只会触发一次
      window.request_redraw();
    }
    _ => {}
  });
}
