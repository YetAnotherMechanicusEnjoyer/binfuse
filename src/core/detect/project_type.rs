use super::BinfuseError;
use crate::{Language, ProjectType};
use std::{fs, path::Path};

pub fn detect_project_type(
    source_path: &Path,
    language: &Language,
) -> Result<ProjectType, BinfuseError> {
    match language {
        Language::Rust => detect_rust_project_type(source_path),
        Language::Zig => detect_zig_project_type(source_path),
        Language::NodeJs => detect_nodejs_project_type(source_path),
        Language::Go => detect_go_project_type(source_path),
        Language::C => detect_c_project_type(source_path),
        Language::Python => detect_python_project_type(source_path),
        Language::Static => Ok(ProjectType::StaticSite),
    }
}

fn detect_rust_project_type(source_path: &Path) -> Result<ProjectType, BinfuseError> {
    let cargo_toml_path = source_path.join("Cargo.toml");
    let cargo_toml_content = fs::read_to_string(&cargo_toml_path)
        .map_err(|_| BinfuseError::DetectionError("Failed to read Cargo.toml".to_string()))?;

    let has_axum = cargo_toml_content.contains("axum");
    let has_actix_web = cargo_toml_content.contains("actix-web");
    let has_rocket = cargo_toml_content.contains("rocket");
    let has_crossterm = cargo_toml_content.contains("crossterm");
    let has_ratatui = cargo_toml_content.contains("ratatui");

    if has_axum || has_actix_web || has_rocket {
        Ok(ProjectType::WebBackend)
    } else if has_crossterm || has_ratatui {
        Ok(ProjectType::Tui)
    } else {
        Ok(ProjectType::Cli)
    }
}

fn detect_zig_project_type(_source_path: &Path) -> Result<ProjectType, BinfuseError> {
    // TODO: Implement Zig support.
    Ok(ProjectType::Cli)
}

fn detect_nodejs_project_type(_source_path: &Path) -> Result<ProjectType, BinfuseError> {
    // TODO: Implement NodeJS support.
    Ok(ProjectType::WebBackend)
}

fn detect_go_project_type(_source_path: &Path) -> Result<ProjectType, BinfuseError> {
    // TODO: Implement Go support.
    Ok(ProjectType::Cli)
}

fn detect_c_project_type(_source_path: &Path) -> Result<ProjectType, BinfuseError> {
    // TODO: Implement C support.
    Ok(ProjectType::Cli)
}

fn detect_python_project_type(_source_path: &Path) -> Result<ProjectType, BinfuseError> {
    // TODO: Implement Python support.
    Ok(ProjectType::Cli)
}
