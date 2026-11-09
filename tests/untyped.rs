use serde_derive::Deserialize;
use serde_json::{Map, Value};

pub type Node = clang_ast::Node<Clang>;

#[derive(Deserialize)]
pub struct Clang {
    #[serde(default)]
    pub kind: clang_ast::Kind,
    #[serde(default)]
    pub loc: clang_ast::SourceLocation,
    #[serde(default)]
    pub range: clang_ast::SourceRange,
    #[serde(flatten)]
    pub data: Map<String, Value>,
}

#[test]
fn test() {
    let json = clang_ast_test_suite::cxx_ast_json();
    let mut de = serde_json::Deserializer::from_slice(&json);
    de.disable_recursion_limit();
    let _: Node = serde::Deserialize::deserialize(&mut de).unwrap();
}
