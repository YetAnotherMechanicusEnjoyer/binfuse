use crate::{
    ProjectMetadata, Result,
    core::{build::Builder, config::Config, error::BinfuseError},
};
use std::path::PathBuf;

pub struct GoBuilder;

impl GoBuilder {
    pub fn new() -> Self {
        Self
    }
}

impl Builder for GoBuilder {
    fn build(&self, _config: &Config, _metadata: &ProjectMetadata) -> Result<PathBuf> {
        Err(BinfuseError::BuildError("Go builder not yet implemented".to_string()).into())
    }
}
