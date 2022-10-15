use std::collections::HashMap;

pub struct TaggedString {
    main: String,
    tag: Option<String>,
}

pub struct CharacterString {
    character: String,
    content: TaggedString,
}

pub enum Value {
    Null,
    Boolean(bool),
    Float(f64),
    Integer(i64),
    String(String),
    TaggedString(TaggedString),
    CharacterString(CharacterString),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
}
