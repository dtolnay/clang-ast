use serde_derive::Deserialize;

pub type Node = clang_ast::Node<Clang>;

#[derive(Deserialize)]
pub enum Clang {
    NamespaceDecl(NamespaceDecl),
    EnumDecl(EnumDecl),
    EnumConstantDecl(EnumConstantDecl),
    Unknown,
}

#[derive(Deserialize, Debug)]
pub struct NamespaceDecl {
    pub name: Option<String>,
    #[serde(rename = "isInline", default)]
    pub is_inline: bool,
}

#[derive(Deserialize, Debug)]
pub struct EnumDecl {
    pub name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct EnumConstantDecl {
    pub name: String,
}

#[test]
fn test() {
    let json = clang_ast_test_suite::cxx_ast_json();
    let _: Node = serde_json::from_slice(&json).unwrap();
}
