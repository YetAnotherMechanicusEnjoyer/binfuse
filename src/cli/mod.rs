use crate::Result;
use clap::{Parser, Subcommand};

/// CLI tool to embed any project in a single static binary
#[derive(Parser, Debug)]
#[command(name = "bindfuse")]
#[command(author = "YetAnotherMechanicusEnjoyer")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Embeds any project (code + assets + configs) in a single static binary.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Detects the language and project type in a directory.
    Detect {
        /// Path to the source directory (default: .).
        #[arg(short, long, default_value = ".")]
        source: String,
        /// Show verbose output (e.g., detected files).
        #[arg(short, long)]
        verbose: bool,
    },
    /// Builds the project and generates a static binary.
    Build {
        /// Path to the source directory (default: .).
        #[arg(short, long, default_value = ".")]
        source: String,
        /// Output path for the binary (default: ./.binfuse/output).
        #[arg(short, long, default_value = "./.binfuse/output")]
        output: String,
        /// Optional config file (default: binfuse.toml).
        #[arg(short, long)]
        config: Option<String>,
        /// Disable asset compression.
        #[arg(long)]
        no_compress: bool,
    },
    /// Lists supported targets (languages, OS, etc.).
    Targets,
    /// Cleans generated files (target/, zig-out/, etc.).
    Clean {
        /// Path to clean (default: .).
        #[arg(short, long, default_value = ".")]
        path: String,
    },
}

impl Cli {
    pub fn run(self) -> Result<()> {
        match self.command {
            Commands::Detect { source, verbose } => {
                let metadata = crate::core::detect::run_detect(&source, verbose)?;
                log::info!(
                    "Detected project: language={:?}, type={:?}, assets={}, configs={}",
                    metadata.language,
                    metadata.project_type,
                    metadata.assets.len(),
                    metadata.configs.len()
                );
                Ok(())
            }
            Commands::Build {
                source,
                output,
                config,
                no_compress,
            } => crate::core::build::run_build(source, output, config, !no_compress),
            Commands::Targets => todo!(), //crate::core::targets::run_targets(),
            Commands::Clean { path } => todo!(), //crate::core::clean::run_clean(&path),
        }
    }
}
