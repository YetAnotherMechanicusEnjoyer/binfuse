use binfuse::core::{
    build::{Builder, rust::RustBuilder},
    detect::run_detect,
};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_rust_builder() {
    // Create a temporary Rust project.
    let dir = tempdir().unwrap();
    let project_dir = dir.path();

    // Create Cargo.toml.
    let cargo_toml = project_dir.join("Cargo.toml");
    fs::write(
        &cargo_toml,
        r#"
[package]
name = "test_project"
version = "0.1.0"
edition = "2024"

[dependencies]
"#,
    )
    .unwrap();

    let src_dir = project_dir.join("src");
    fs::create_dir(&src_dir).unwrap();
    let main_rs = src_dir.join("main.rs");
    fs::write(&main_rs, "fn main() {}").unwrap();

    let builder = RustBuilder;
    let config = binfuse::core::config::Config::default();
    let metadata = run_detect(project_dir.to_str().unwrap(), false).unwrap();

    let binary_path = builder.build(&config, &metadata).unwrap();
    assert!(binary_path.exists());
}
