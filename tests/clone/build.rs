#![allow(
    clippy::let_underscore_untyped,
    clippy::redundant_else,
    clippy::uninlined_format_args
)]

use std::env;
use std::fs::{self, File};
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::Command;

// Executable names to try.
static CLANG: &[&str] = &[
    "clang++-15",
    "clang++-14",
    "clang++-13",
    "clang++-12",
    "clang++-11",
    "clang++",
];

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let out_ast_json = out_dir.join("ast.json");

    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let override_ast_json = manifest_dir.join("ast.json");
    if override_ast_json.exists() {
        println!("cargo:rerun-if-changed=ast.json");
        fs::copy(&override_ast_json, &out_ast_json).unwrap();
        return;
    }

    let override_ast_cc = manifest_dir.join("ast.cc");
    let input = if override_ast_cc.exists() {
        println!("cargo:rerun-if-changed=ast.cc");
        override_ast_cc
    } else {
        let cxx_dir = out_dir.join("cxx");
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
        cxx_dir.join("src").join("cxx.cc")
    };

    let mut clangs = CLANG.iter();
    while let Some(&clang) = clangs.next() {
        let output = File::create(&out_ast_json).unwrap();
        match Command::new(clang)
            .arg("-Xclang")
            .arg("-ast-dump=json")
            .arg("-Xclang")
            .arg("-std=c++20")
            .arg("-fsyntax-only")
            .arg(&input)
            .stdout(output)
            .status()
        {
            Ok(status) => {
                if status.success() {
                    break;
                } else {
                    let _ = fs::remove_file(&out_ast_json);
                    assert!(status.success());
                }
            }
            Err(error) => {
                let _ = fs::remove_file(&out_ast_json);
                if error.kind() != ErrorKind::NotFound || clangs.as_slice().is_empty() {
                    panic!("{:?}", error);
                }
            }
        }
    }
}
