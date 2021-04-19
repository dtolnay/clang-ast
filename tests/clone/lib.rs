use std::path::Path;
use std::process::Command;

pub fn cxx_ast_json() -> String {
    let out_dir = env!("OUT_DIR");
    let input = Path::new(out_dir).join("cxx").join("src").join("cxx.cc");

    let output = Command::new("clang++")
        .arg("-Xclang")
        .arg("-ast-dump=json")
        .arg("-fsyntax-only")
        .arg(input)
        .output()
        .unwrap();

    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap()
}
