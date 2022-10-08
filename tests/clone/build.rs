#![allow(
    clippy::let_underscore_drop,
    clippy::redundant_else,
    clippy::uninlined_format_args
)]

use std::env;
use std::fs::{self, File};
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;

// Executable names to try.
static CLANG: &[&str] = &[
    "clang++-14",
    "clang++-13",
    "clang++-12",
    "clang++-11",
    "clang++",
];

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let cxx_dir = Path::new(&out_dir).join("cxx");
    if !cxx_dir.exists() {
        let status = Command::new("git")
            .arg("clone")
            .arg("--depth")
            .arg("1")
            .arg("https://github.com/dtolnay/cxx")
            .arg(&cxx_dir)
            .status()
            .unwrap();
        assert!(status.success());
    }

    let ast_json = Path::new(&out_dir).join("ast.json");
    if !ast_json.exists() {
        let input = cxx_dir.join("src").join("cxx.cc");
        let mut clangs = CLANG.iter();
        while let Some(&clang) = clangs.next() {
            let output = File::create(&ast_json).unwrap();
            match Command::new(clang)
                .arg("-Xclang")
                .arg("-ast-dump=json")
                .arg("-Xclang")
                .arg("-std=c++17")
                .arg("-fsyntax-only")
                .arg(&input)
                .stdout(output)
                .status()
            {
                Ok(status) => {
                    if status.success() {
                        break;
                    } else {
                        let _ = fs::remove_file(&ast_json);
                        assert!(status.success());
                    }
                }
                Err(error) => {
                    let _ = fs::remove_file(&ast_json);
                    if error.kind() == ErrorKind::NotFound && !clangs.as_slice().is_empty() {
                        continue;
                    } else {
                        panic!("{:?}", error);
                    }
                }
            }
        }
    }

    // Disable rerun on changes to Cargo.toml and lib.rs.
    println!("cargo:rerun-if-changed=build.rs");
}
