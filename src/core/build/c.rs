use crate::{
    ProjectMetadata, Result,
    core::{build::Builder, config::Config, error::BinfuseError},
};
use std::path::PathBuf;

pub struct CBuilder;

impl CBuilder {
    pub fn new() -> Self {
        Self
    }
}

impl Builder for CBuilder {
    fn build(&self, _config: &Config, _metadata: &ProjectMetadata) -> Result<PathBuf> {
        Err(BinfuseError::BuildError("C builder not yet implemented".to_string()).into())
    }
}
