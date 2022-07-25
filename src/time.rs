use std::{
  sync::Mutex,
  time::{Duration, Instant},
};

use once_cell::sync::Lazy;

static PREVIOUS_TIME: Lazy<Mutex<Instant>> = Lazy::new(|| Mutex::new(Instant::now()));
static DELTA_TIME: Lazy<Mutex<Duration>> = Lazy::new(|| Mutex::new(Duration::from_millis(1)));

pub fn update() {
  let mut previous_guard = PREVIOUS_TIME.lock().unwrap();
  let now = Instant::now();
  let previous = *previous_guard;
  *previous_guard = now;
  let delta = now.duration_since(previous);
  let mut delta_guard = DELTA_TIME.lock().unwrap();
  *delta_guard = delta;
}
pub fn get_delta() -> f32 {
  let delta_guard = DELTA_TIME.lock().unwrap();
  let delta = *delta_guard;
  delta.as_secs_f32()
}

static START: Lazy<Instant> = Lazy::new(|| Instant::now());
pub fn get_now() -> f32 {
  START.elapsed().as_secs_f32()
}
