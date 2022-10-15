pub enum BILFAst {
    Identifier(String),
    SubaccessIdentifier(Vec<String>),
    NamespacedIdentifier(Vec<String>),
    Integer(i32),
    Float(f32),
    String(String),
}
