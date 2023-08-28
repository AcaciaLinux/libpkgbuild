use std::path::Path;

use crate::StdIOErrorExt;

/// Ensures a directory exists
/// # Arguments
/// * `path` - The path to ensure
pub fn ensure_dir(path: &Path) -> Result<(), std::io::Error> {
    info!("Ensuring directory {}", path.to_string_lossy());

    Ok(std::fs::create_dir_all(path).err_prepend(&format!(
        "When ensuring directory {}",
        path.to_string_lossy()
    ))?)
}

/// Ensures a clean (empty) directory exists, removes an old one if necessary
/// # Arguments
/// * `path` - The path to check
pub fn clean_dir(path: &Path) -> Result<(), std::io::Error> {
    if path.exists() {
        debug!("Removing old directory at {}", path.to_string_lossy());
        std::fs::remove_dir_all(path)?;
    }

    ensure_dir(path)
}
