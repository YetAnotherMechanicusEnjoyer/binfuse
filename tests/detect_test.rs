use binfuse::{
    Language, ProjectType,
    core::detect::{language::detect_language, project_type::detect_project_type, run_detect},
};
use tempfile::tempdir;

#[test]
fn test_detect_rust_language() {
    let dir = tempdir().unwrap();
    let cargo_toml = dir.path().join("Cargo.toml");
    std::fs::write(&cargo_toml, "[package]\nname = \"test\"").unwrap();

    let language = detect_language(dir.path()).unwrap();
    assert_eq!(language, Language::Rust);
}

#[test]
fn test_detect_rust_web_backend() {
    let dir = tempdir().unwrap();
    let cargo_toml = dir.path().join("Cargo.toml");
    std::fs::write(&cargo_toml, "[dependencies]\naxum = \"0.8\"").unwrap();

    let language = detect_language(dir.path()).unwrap();
    let project_type = detect_project_type(dir.path(), &language).unwrap();
    assert_eq!(project_type, ProjectType::WebBackend);
}

#[test]
fn test_detect_static_site() {
    let dir = tempdir().unwrap();
    let index_html = dir.path().join("index.html");
    std::fs::write(&index_html, "<html></html>").unwrap();

    let language = detect_language(dir.path()).unwrap();
    assert_eq!(language, Language::Static);

    let project_type = detect_project_type(dir.path(), &language).unwrap();
    assert_eq!(project_type, ProjectType::StaticSite);
}

#[test]
fn test_run_detect() {
    let dir = tempdir().unwrap();
    let cargo_toml = dir.path().join("Cargo.toml");
    std::fs::write(&cargo_toml, "[package]\nname = \"test\"").unwrap();

    let metadata = run_detect(dir.path().to_str().unwrap(), false).unwrap();
    assert_eq!(metadata.language, Language::Rust);
    assert_eq!(metadata.project_type, ProjectType::Cli);
}
