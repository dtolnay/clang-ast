use std::fs;
use std::path::Path;

pub fn cxx_ast_json() -> String {
    let out_dir = env!("OUT_DIR");
    let ast_json = Path::new(out_dir).join("ast.json");
    fs::read_to_string(ast_json).unwrap()
}
