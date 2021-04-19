use std::process::Command;

fn main() {
    let scratch = scratch::path("clang-ast-test-suite");
    let out = scratch.join("cxx");
    if !out.exists() {
        let status = Command::new("git")
            .arg("clone")
            .arg("--depth")
            .arg("1")
            .arg("https://github.com/dtolnay/cxx")
            .arg(out)
            .status()
            .unwrap();
        assert!(status.success());
    }
}
