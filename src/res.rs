use std::path::Path;

use color_eyre::eyre::Result;

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
