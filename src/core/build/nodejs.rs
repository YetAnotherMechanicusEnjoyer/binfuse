use crate::{
    ProjectMetadata, Result,
    core::{build::Builder, config::Config, error::BinfuseError},
};
use std::path::PathBuf;

pub struct NodeJsBuilder;

impl NodeJsBuilder {
    pub fn new() -> Self {
        Self
    }
}

impl Builder for NodeJsBuilder {
    fn build(&self, _config: &Config, _metadata: &ProjectMetadata) -> Result<PathBuf> {
        Err(BinfuseError::BuildError("Node.js builder not yet implemented".to_string()).into())
    }
}
