//! Run every examples/ fixture through the public betula API. Valid fixtures
//! must parse, and serializing then re-parsing must give an equal value (the
//! JSON itself may differ: empty optional arrays are omitted on output).
//! Invalid fixtures must be rejected.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").canonicalize().unwrap()
}

fn schema_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            schema_files(&path, out);
        } else if path.to_string_lossy().ends_with(".schema.json") {
            out.push(path);
        }
    }
}

fn json_files(dir: &Path) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(dir) else { return vec![] };
    let mut files: Vec<PathBuf> = entries
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|ext| ext == "json"))
        .collect();
    files.sort();
    files
}

#[test]
fn fixtures() {
    let repo = repo();
    let schema_dir = repo.join("schema");
    let mut schemas = Vec::new();
    schema_files(&schema_dir, &mut schemas);
    schemas.sort();

    let mut failures = Vec::new();
    let mut count = 0;
    for schema_path in schemas {
        let schema: Value = serde_json::from_str(&fs::read_to_string(&schema_path).unwrap()).unwrap();
        let title = schema["title"].as_str().unwrap();
        let name = schema_path.strip_prefix(&schema_dir).unwrap().to_string_lossy().replace('\\', "/");
        let name = name.trim_end_matches(".schema.json");

        for (kind, should_pass) in [("valid", true), ("invalid", false)] {
            for path in json_files(&repo.join("examples").join(name).join(kind)) {
                count += 1;
                let original: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
                let result = betula::round_trip_by_name(title, original)
                    .unwrap_or_else(|| panic!("{title} is not exported as a betula kind"));
                let label = path.strip_prefix(&repo).unwrap().display();
                match (should_pass, result) {
                    (true, Ok(true)) => {}
                    (true, Ok(false)) => failures.push(format!("{label}: serialize/re-parse changed the value")),
                    (true, Err(err)) => failures.push(format!("{label}: expected valid, got {err}")),
                    (false, Err(betula::Error::Invalid { .. })) => {}
                    (false, other) => failures.push(format!("{label}: expected invalid, got {other:?}")),
                }
            }
        }
    }
    assert!(count > 0, "no fixtures found");
    assert!(failures.is_empty(), "{} failures:\n{}", failures.len(), failures.join("\n"));
}

/// "ACGN" is valid in both the DNA and RNA alphabets, so only the `type`
/// discriminator can route this to the right variant of the untagged enum.
#[test]
fn discriminator_selects_variant() {
    let rna: betula::Sequence =
        betula::parse_str(r#"{"type": "rna-sequence", "identifier": "s", "sequence": "ACGN"}"#).unwrap();
    assert!(matches!(rna, betula::Sequence::RnaSequence(_)), "got {rna:?}");

    let untyped: betula::Sequence = betula::parse_str(r#"{"identifier": "s", "sequence": "ACGN"}"#).unwrap();
    assert!(matches!(untyped, betula::Sequence::UntypedSequence(_)), "got {untyped:?}");
}

/// The docs' Getting started section embeds examples/usage.rs as the API tour,
/// so every public, non-hidden item in lib.rs must appear in it.
#[test]
fn usage_example_covers_public_api() {
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let lib = fs::read_to_string(crate_dir.join("src/lib.rs")).unwrap();
    let example = fs::read_to_string(crate_dir.join("examples/usage.rs")).unwrap();
    let mut previous = "";
    let mut missing = Vec::new();
    for line in lib.lines().map(str::trim) {
        if previous != "#[doc(hidden)]" {
            for prefix in ["pub fn ", "pub enum ", "pub trait ", "pub const "] {
                if let Some(rest) = line.strip_prefix(prefix) {
                    let name: String = rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
                    if !example.contains(&name) {
                        missing.push(name);
                    }
                }
            }
        }
        previous = line;
    }
    assert!(missing.is_empty(), "examples/usage.rs doesn't use: {missing:?}");
}
