use crate::{
    ProjectMetadata, Result,
    core::{build::Builder, config::Config, error::BinfuseError},
};
use std::path::PathBuf;

#[derive(Default)]
pub struct PythonBuilder;

impl Builder for PythonBuilder {
    fn build(&self, _config: &Config, _metadata: &ProjectMetadata) -> Result<PathBuf> {
        Err(BinfuseError::BuildError("Python builder not yet implemented".to_string()).into())
    }
}
