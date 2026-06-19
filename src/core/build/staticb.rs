use crate::{
    ProjectMetadata, Result,
    core::{build::Builder, config::Config, error::BinfuseError},
};
use std::path::PathBuf;

pub struct StaticBuilder;

impl StaticBuilder {
    pub fn new() -> Self {
        Self
    }
}

impl Builder for StaticBuilder {
    fn build(&self, _config: &Config, _metadata: &ProjectMetadata) -> Result<PathBuf> {
        Err(BinfuseError::BuildError("Static builder not yet implemented".to_string()).into())
    }
}
