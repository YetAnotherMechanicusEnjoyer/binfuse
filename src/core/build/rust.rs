use crate::{
    ProjectMetadata, Result,
    core::{build::Builder, config::Config, error::BinfuseError},
};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub struct RustBuilder;

impl RustBuilder {
    pub fn new() -> Self {
        Self
    }
}

impl Builder for RustBuilder {
    fn build(&self, _config: &Config, metadata: &ProjectMetadata) -> Result<PathBuf> {
        let project_dir = &metadata.source_path;

        let output = Command::new("cargo")
            .arg("build")
            .arg("--release")
            .current_dir(project_dir)
            .output()
            .map_err(|e| {
                BinfuseError::BuildError(format!(
                    "Failed to run `cargo build --release` in {}: {}",
                    project_dir.display(),
                    e
                ))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(
                BinfuseError::BuildError(format!("Cargo build failed:\n{}", stderr)).into(),
            );
        }

        let binary_name = get_binary_name(project_dir)?;

        let binary_path = project_dir.join("target").join("release").join(binary_name);
        Ok(binary_path)
    }
}

fn get_binary_name(project_dir: &Path) -> Result<String> {
    let cargo_toml_path = project_dir.join("Cargo.toml");
    let cargo_toml_content = std::fs::read_to_string(&cargo_toml_path).map_err(|e| {
        BinfuseError::BuildError(format!(
            "Failed to read Cargo.toml in {}: {}",
            project_dir.display(),
            e
        ))
    })?;

    let name_re = regex::Regex::new(r#"name\s*=\s*"([^"]+)"#)
        .map_err(|_| BinfuseError::BuildError("Failed to compile regex".to_string()))?;
    let name = name_re
        .captures(&cargo_toml_content)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string())
        .ok_or_else(|| {
            BinfuseError::BuildError("Failed to find `name` in Cargo.toml".to_string())
        })?;

    Ok(name)
}
