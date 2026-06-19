use thiserror::Error;

#[derive(Debug, Error)]
pub enum BinfuseError {
    #[error("No project detected in directory: {0}")]
    NoProjectDetected(String),

    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),

    #[error("Detection error: {0}")]
    DetectionError(String),

    #[error("Build error: {0}")]
    BuildError(String),

    #[error("Pack error: {0}")]
    PackError(String),

    #[error("Invalid config file: {0}")]
    ConfigError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("TOML parse error: {0}")]
    TomlParseError(#[from] toml::de::Error),
}
