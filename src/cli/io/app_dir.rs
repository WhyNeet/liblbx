use std::{fs, path::PathBuf};

use crate::utils::path::path_exists;

pub fn get_app_dir() -> anyhow::Result<PathBuf> {
    let home_dir = dirs::config_dir().ok_or(anyhow::anyhow!("failed to find config directory"))?;
    let app_dir = home_dir.join("lockbox");

    if path_exists(app_dir.as_path()) {
        return Ok(app_dir);
    }

    fs::create_dir(&app_dir).or(Err(anyhow::anyhow!("failed to create app directory")))?;

    Ok(app_dir)
}
