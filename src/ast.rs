use crate::values::{CharacterString, Value};

pub struct Identifier {
    name: Vec<String>,
}

pub struct Import {
    import: Option<Vec<Identifier>>,
    from: String,
}

pub struct ChoiceOption {
    value: Value,
    rhai_expression: Expression,
}

pub struct FunctionDecleration {
    public: bool,
    recorded: bool,
    name: Identifier,
    body: Block,
}

pub enum Expression {
    Literal(Value),
    Rhai(String),
}

pub enum Statement {
    Import(Import),
    Comment(String),
    FunctionDecl(FunctionDecleration),
    ChoiceDecl {
        choice: CharacterString,
        options: Vec<ChoiceOption>,
    },
    RaiseEvent(Value),
}

pub type Block = Vec<Statement>;
