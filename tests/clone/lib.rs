use memmap::Mmap;
use std::fs::File;
use std::ops::Deref;
use std::path::Path;

pub fn cxx_ast_json() -> impl Deref<Target = [u8]> {
    let out_dir = env!("OUT_DIR");
    let ast_json = Path::new(out_dir).join("ast.json");
    let file = File::open(ast_json).unwrap();
    unsafe { Mmap::map(&file) }.unwrap()
}
