use crate::{Language, Result};
use std::path::Path;

const LANGUAGE_SIGNATURES: &[LanguageSignature] = &[
    LanguageSignature {
        language: Language::Rust,
        signatures: &["Cargo.toml"],
    },
    LanguageSignature {
        language: Language::Zig,
        signatures: &["build.zig"],
    },
    LanguageSignature {
        language: Language::NodeJs,
        signatures: &["package.json"],
    },
    LanguageSignature {
        language: Language::Go,
        signatures: &["go.mod"],
    },
    LanguageSignature {
        language: Language::C,
        signatures: &["Makefile", "CMakeLists.txt"],
    },
    LanguageSignature {
        language: Language::Python,
        signatures: &["requirements.txt", "pyproject.toml", "setup.py"],
    },
    LanguageSignature {
        language: Language::Static,
        signatures: &["index.html"],
    },
];

struct LanguageSignature<'a> {
    language: Language,
    signatures: &'a [&'a str],
}

pub fn detect_language(source_path: &Path) -> Option<Language> {
    for signature in LANGUAGE_SIGNATURES {
        for file in signature.signatures {
            if source_path.join(file).exists() {
                return Some(signature.language.clone());
            }
        }
    }
    None
}

pub fn validate_language(language: &Language) -> Result<()> {
    match language {
        Language::Rust
        | Language::Zig
        | Language::NodeJs
        | Language::Go
        | Language::C
        | Language::Python
        | Language::Static => Ok(()),
    }
}
