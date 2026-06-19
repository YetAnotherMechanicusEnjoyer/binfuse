use super::Config;
use crate::Result;
use std::path::PathBuf;

pub fn parse_config(config_path: Option<&str>, args: &crate::cli::Commands) -> Result<Config> {
    let mut config: Config = if let Some(path) = config_path {
        let toml_str = std::fs::read_to_string(path)?;
        toml::from_str(&toml_str)?
    } else {
        Config::default()
    };

    match args {
        crate::cli::Commands::Build {
            source,
            output,
            no_compress,
            ..
        } => {
            config.build.source = PathBuf::from(source);
            config.build.output = PathBuf::from(output);
            if *no_compress {
                config.pack.compression = "none".to_string();
            }
        }
        crate::cli::Commands::Detect { source, .. } => {
            config.build.source = PathBuf::from(source);
        }
        _ => {}
    }

    Ok(config)
}
