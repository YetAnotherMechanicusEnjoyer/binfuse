use std::path::PathBuf;

use crate::{
    Language, ProjectMetadata, Result, cli,
    core::{
        config::{Config, parser::parse_config},
        detect::run_detect,
        error::BinfuseError,
    },
};

pub mod c;
pub mod go;
pub mod nodejs;
pub mod python;
pub mod rust;
pub mod staticb;
pub mod zig;

pub fn run_build(
    source: String,
    output: String,
    config_path: Option<String>,
    compress_assets: bool,
) -> Result<()> {
    let args = cli::Commands::Build {
        source: source.clone(),
        output: output.clone(),
        config: config_path.clone(),
        no_compress: !compress_assets,
    };
    let config = parse_config(config_path.as_deref(), &args)?;

    let metadata = run_detect(&source, false)?;

    let builder = get_builder(&metadata.language)?;
    let binary_path = builder.build(&config, &metadata)?;

    if let Some(parent) = config.build.output.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            BinfuseError::BuildError(format!(
                "Failed to create output directory {}: {}",
                parent.display(),
                e
            ))
        })?;
    }
    std::fs::copy(&binary_path, &config.build.output).map_err(|e| {
        BinfuseError::BuildError(format!(
            "Failed to copy binary from {} to {}: {}",
            binary_path.display(),
            config.build.output.display(),
            e
        ))
    })?;
    log::info!("Build successful: {}", config.build.output.display());

    Ok(())
}

fn get_builder(language: &Language) -> Result<Box<dyn Builder>> {
    match language {
        Language::Rust => Ok(Box::new(rust::RustBuilder::new())),
        Language::Zig => Ok(Box::new(zig::ZigBuilder::new())),
        Language::NodeJs => Ok(Box::new(nodejs::NodeJsBuilder::new())),
        Language::Go => Ok(Box::new(go::GoBuilder::new())),
        Language::C => Ok(Box::new(c::CBuilder::new())),
        Language::Python => Ok(Box::new(python::PythonBuilder::new())),
        Language::Static => Ok(Box::new(staticb::StaticBuilder::new())),
    }
}

pub trait Builder {
    fn build(&self, config: &Config, metadata: &ProjectMetadata) -> Result<PathBuf>;
}
