use binfuse::Result;
use clap::Parser;

fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    /*
    log::debug!(
        "Starting {} v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    */

    binfuse::cli::Cli::parse().run()
}
