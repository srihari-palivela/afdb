use anyhow::Result;
use std::path::PathBuf;

pub fn save_json<T: serde::Serialize>(path: PathBuf, value: &T) -> Result<()> {
    if let Some(parent) = path.parent() { std::fs::create_dir_all(parent)?; }
    let data = serde_json::to_vec_pretty(value)?;
    std::fs::write(path, data)?;
    Ok(())
}

pub fn load_json<T: for<'de> serde::Deserialize<'de>>(path: PathBuf) -> Result<T> {
    let data = std::fs::read(path)?;
    let v = serde_json::from_slice::<T>(&data)?;
    Ok(v)
}
