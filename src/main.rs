pub mod lib;

use lib::State;
use winit::{
  event::*,
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
};

#[tokio::main]
async fn main() {
  env_logger::init();

  let event_loop = EventLoop::new();
  let window = WindowBuilder::new().build(&event_loop).unwrap();
  let mut state = State::new(&window).await;
  event_loop.run(move |event, _, control_flow| match event {
    Event::WindowEvent {
      ref event,
      window_id,
    } if window_id == window.id() => {
      if state.input(event) {
        return;
      }
      match event {
        WindowEvent::CloseRequested
        | WindowEvent::KeyboardInput {
          input:
            KeyboardInput {
              state: ElementState::Pressed,
              virtual_keycode: Some(VirtualKeyCode::Escape),
              ..
            },
          ..
        } => *control_flow = ControlFlow::Exit,
        WindowEvent::Resized(physical_size) => {
          state.resize(*physical_size);
        }
        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
          state.resize(**new_inner_size);
        }

        _ => {}
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
