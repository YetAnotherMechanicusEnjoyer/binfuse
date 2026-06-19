use crate::{Language, ProjectMetadata, Result};
use std::path::Path;

pub fn detect_components(source_path: &Path, language: &Language) -> Result<Vec<ProjectMetadata>> {
    // TODO: Detect multi-component projects (e.g., backend + frontend).
    let metadata = ProjectMetadata {
        source_path: source_path.to_path_buf(),
        language: language.clone(),
        project_type: crate::core::detect::project_type::detect_project_type(
            source_path,
            language,
        )?,
        assets: crate::core::detect::assets::detect_assets(source_path, &crate::ProjectType::Cli)?,
        configs: crate::core::detect::detect_configs(source_path)?,
    };
    Ok(vec![metadata])
}
