use super::error::BinfuseError;
use crate::{ProjectMetadata, Result};
use std::path::Path;

pub mod assets;
pub mod component;
pub mod language;
pub mod project_type;

pub fn run_detect(source: &str, verbose: bool) -> Result<ProjectMetadata> {
    let source_path = Path::new(source).canonicalize()?;

    let language = language::detect_language(&source_path)
        .ok_or_else(|| BinfuseError::NoProjectDetected(source.to_string()))?;

    let project_type = project_type::detect_project_type(&source_path, &language)?;

    let assets = assets::detect_assets(&source_path, &project_type)?;

    let configs = detect_configs(&source_path)?;

    if verbose {
        log::info!("Detected language: {language:?}");
        log::info!("Detected project type: {project_type:?}");
        log::info!("Detected assets: {assets:?}");
        log::info!("Detected configs: {configs:?}");
    }

    Ok(ProjectMetadata {
        source_path,
        language,
        project_type,
        assets,
        configs,
    })
}

fn detect_configs(source_path: &Path) -> Result<Vec<std::path::PathBuf>> {
    let mut configs = Vec::new();

    let binfuse_toml = source_path.join("binfuse.toml");
    if binfuse_toml.exists() {
        configs.push(binfuse_toml);
    }

    for entry in walkdir::WalkDir::new(source_path) {
        let entry = entry?;
        if entry.file_name().to_string_lossy().ends_with(".env") {
            configs.push(entry.path().to_path_buf());
        }
    }

    Ok(configs)
}
