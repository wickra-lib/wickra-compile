//! The output metadata: `GeneratedFile`, `Manifest`, `Artifact` — the golden
//! contract. The manifest is byte-identical across languages and across runs; it
//! contains no timestamp, no absolute path, and no artifact bytes.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::canonical::sha256_hex;
use crate::spec::{OptLevel, Target};

/// One generated file: its project-relative path, content hash and length.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GeneratedFile {
    /// Path relative to the generated project root (forward slashes).
    pub path: String,
    /// Lowercase hex SHA-256 of the file's bytes.
    pub sha256: String,
    /// The file's length in bytes.
    pub bytes_len: usize,
}

/// The deterministic project fingerprint.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Manifest {
    /// The `wickra-compile` version that produced this manifest.
    pub compiler_version: String,
    /// The target artifact.
    pub target: Target,
    /// The optimisation level.
    pub opt_level: OptLevel,
    /// The resolved crate name.
    pub crate_name: String,
    /// SHA-256 (hex) of the canonical `CompileSpec`.
    pub spec_hash: String,
    /// The exact `wickra-backtest` version written into the generated Cargo.toml.
    pub backtest_dep: String,
    /// The generated files, stably sorted by path.
    pub files: Vec<GeneratedFile>,
    /// SHA-256 (hex) over the ordered `(path, sha256)` list — the project hash.
    pub project_hash: String,
}

/// A build result: the manifest plus, after a real build, the artifact path.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Artifact {
    /// The target artifact kind.
    pub target: Target,
    /// The machine-local artifact path after a real build; `None` on dry-run.
    /// Not part of the golden comparison.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// The deterministic manifest.
    pub manifest: Manifest,
    /// `true` if `cargo` / `wasm-pack` actually ran; `false` for dry-run.
    pub built: bool,
}

/// Compute the project hash from the generated files.
///
/// For each file in path order, the path, a newline, its hex SHA-256 and another
/// newline are appended to an accumulator; the project hash is the SHA-256 of
/// that accumulator. The files map is a [`BTreeMap`], so iteration is already in
/// path order — the reduction is serial and OS-independent.
#[must_use]
pub fn project_hash(files: &BTreeMap<String, Vec<u8>>) -> String {
    let mut acc = String::new();
    for (path, bytes) in files {
        acc.push_str(path);
        acc.push('\n');
        acc.push_str(&sha256_hex(bytes));
        acc.push('\n');
    }
    sha256_hex(acc.as_bytes())
}

/// Build the sorted [`GeneratedFile`] list from a files map.
#[must_use]
pub fn generated_files(files: &BTreeMap<String, Vec<u8>>) -> Vec<GeneratedFile> {
    files
        .iter()
        .map(|(path, bytes)| GeneratedFile {
            path: path.clone(),
            sha256: sha256_hex(bytes),
            bytes_len: bytes.len(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_hash_is_order_independent_of_insertion() {
        let mut a = BTreeMap::new();
        a.insert("b.rs".to_owned(), b"two".to_vec());
        a.insert("a.rs".to_owned(), b"one".to_vec());
        let mut b = BTreeMap::new();
        b.insert("a.rs".to_owned(), b"one".to_vec());
        b.insert("b.rs".to_owned(), b"two".to_vec());
        assert_eq!(project_hash(&a), project_hash(&b));
    }

    #[test]
    fn generated_files_are_sorted_by_path() {
        let mut files = BTreeMap::new();
        files.insert("src/lib.rs".to_owned(), b"x".to_vec());
        files.insert("Cargo.toml".to_owned(), b"y".to_vec());
        let list = generated_files(&files);
        assert_eq!(list[0].path, "Cargo.toml");
        assert_eq!(list[1].path, "src/lib.rs");
    }
}
