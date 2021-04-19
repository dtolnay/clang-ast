use std::env;
use std::fs::File;
use std::path::Path;
use std::process::Command;

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
        let output = File::create(ast_json).unwrap();
        let status = Command::new("clang++")
            .arg("-Xclang")
            .arg("-ast-dump=json")
            .arg("-fsyntax-only")
            .arg(input)
            .stdout(output)
            .status()
            .unwrap();
        assert!(status.success());
    }
}
