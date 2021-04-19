use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let cxx_dir = PathBuf::from(out_dir).join("cxx");
    if !cxx_dir.exists() {
        let status = Command::new("git")
            .arg("clone")
            .arg("--depth")
            .arg("1")
            .arg("https://github.com/dtolnay/cxx")
            .arg(cxx_dir)
            .status()
            .unwrap();
        assert!(status.success());
    }
}
