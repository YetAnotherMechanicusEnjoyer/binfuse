pub mod cli;
pub mod core;

pub type Result<T> = anyhow::Result<T>;
pub type Error = anyhow::Error;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Language {
    Rust,
    Zig,
    NodeJs,
    Go,
    C,
    Python,
    Static,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ProjectType {
    Cli,
    WebBackend,
    WebFrontend,
    Fullstack,
    Tui,
    StaticSite,
}

#[derive(Debug, Clone)]
pub struct ProjectMetadata {
    pub source_path: std::path::PathBuf,
    pub language: Language,
    pub project_type: ProjectType,
    pub assets: Vec<std::path::PathBuf>,
    pub configs: Vec<std::path::PathBuf>,
}
