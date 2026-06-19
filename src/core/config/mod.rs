pub mod parser;

use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub project: ProjectConfig,

    #[serde(default)]
    pub build: BuildConfig,

    #[serde(default)]
    pub pack: PackConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProjectConfig {
    #[serde(default = "default_name")]
    pub name: String,
    #[serde(default)]
    pub version: Option<String>,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: default_name(),
            version: None,
        }
    }
}

fn default_name() -> String {
    "output".to_string()
}

#[derive(Debug, Clone, Deserialize)]
pub struct BuildConfig {
    #[serde(default = "default_source")]
    pub source: PathBuf,
    #[serde(default = "default_output")]
    pub output: PathBuf,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
    #[serde(default)]
    pub include: Vec<String>,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            source: default_source(),
            output: default_output(),
            target: None,
            exclude: Vec::new(),
            include: Vec::new(),
        }
    }
}

fn default_source() -> PathBuf {
    PathBuf::from(".")
}

fn default_output() -> PathBuf {
    PathBuf::from("./.binfuse/output")
}

#[derive(Debug, Clone, Deserialize)]
pub struct PackConfig {
    #[serde(default = "default_compression")]
    pub compression: String,
    #[serde(default)]
    pub embed_assets: bool,
}

impl Default for PackConfig {
    fn default() -> Self {
        Self {
            compression: default_compression(),
            embed_assets: true,
        }
    }
}

fn default_compression() -> String {
    "zstd".to_string()
}
