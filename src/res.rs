use std::{
  io::{BufReader, Cursor},
  path::Path,
};

use color_eyre::eyre::Result;
use tracing::{debug, instrument};

use crate::{exts::state::DeviceTrait, texture};

#[instrument]
pub async fn load_str(filepath: &Path) -> Result<String> {
  debug!("loading file {}", filepath.display());
  let filepath = std::path::Path::new(env!("OUT_DIR"))
    .join("assets")
    .join(filepath);

  let text = tokio::fs::read_to_string(filepath).await?;
  Ok(text)
}

#[instrument]
pub async fn load_binary(filepath: &Path) -> Result<Vec<u8>> {
  debug!("loading file {}", filepath.display());
  let filepath = std::path::Path::new(env!("OUT_DIR"))
    .join("assets")
    .join(filepath);

  let data = tokio::fs::read(filepath).await?;
  Ok(data)
}
pub async fn load_texture<T: DeviceTrait>(
  filename: &Path,
  device: &T,
  queue: &wgpu::Queue,
) -> Result<texture::Texture> {
  let data = load_binary(filename).await?;
  texture::Texture::from_bytes(device, queue, &data, &filename.to_string_lossy())
}
