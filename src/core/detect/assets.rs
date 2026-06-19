use super::BinfuseError;
use crate::{ProjectType, Result};
use std::path::Path;

pub fn detect_assets(
    source_path: &Path,
    project_type: &ProjectType,
) -> Result<Vec<std::path::PathBuf>> {
    let mut assets = Vec::new();

    let asset_dirs = ["static", "public", "assets", "dist", "web", "frontend"];

    for dir in asset_dirs {
        let dir_path = source_path.join(dir);
        if dir_path.exists() {
            for entry in walkdir::WalkDir::new(&dir_path) {
                let entry = entry.map_err(|e| {
                    BinfuseError::DetectionError(format!("Failed to walk directory {}: {}", dir, e))
                })?;
                if entry.file_type().is_file() {
                    assets.push(entry.path().to_path_buf());
                }
            }
        }
    }

    if matches!(
        project_type,
        ProjectType::StaticSite | ProjectType::WebFrontend
    ) {
        for entry in walkdir::WalkDir::new(source_path) {
            let entry = entry.map_err(|e| {
                BinfuseError::DetectionError(format!("Failed to walk directory: {}", e))
            })?;
            let path = entry.path();
            if entry.file_type().is_file() {
                let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
                if matches!(
                    ext,
                    "html" | "css" | "js" | "png" | "jpg" | "jpeg" | "gif" | "svg" | "ico"
                ) {
                    assets.push(path.to_path_buf());
                }
            }
        }
    }

    Ok(assets)
}
