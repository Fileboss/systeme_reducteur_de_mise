//! Embarque toutes les tables `tables/{n}_{t}.txt` dans la bibliothèque.

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

fn main() {
    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("tables");
    println!("cargo:rerun-if-changed={}", dir.display());

    let mut entries: Vec<(usize, u8, PathBuf)> = Vec::new();
    if let Ok(read) = fs::read_dir(&dir) {
        for entry in read.flatten() {
            let path = entry.path();
            println!("cargo:rerun-if-changed={}", path.display());
            let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            if path.extension().and_then(|e| e.to_str()) != Some("txt") {
                continue;
            }
            let Some((n, t)) = stem.split_once('_') else {
                continue;
            };
            if let (Ok(n), Ok(t)) = (n.parse(), t.parse()) {
                entries.push((n, t, path));
            }
        }
    }
    entries.sort();

    let mut out = String::from("pub static EMBEDDED: &[(usize, u8, &str)] = &[\n");
    for (n, t, path) in entries {
        writeln!(
            out,
            "    ({n}, {t}, include_str!({:?})),",
            path.display().to_string()
        )
        .unwrap();
    }
    out.push_str("];\n");
    fs::write(
        PathBuf::from(env::var("OUT_DIR").unwrap()).join("tables.rs"),
        out,
    )
    .unwrap();
}
