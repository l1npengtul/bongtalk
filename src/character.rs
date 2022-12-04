use crate::value::Value;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Character {
    pub name: String,
    pub prefix: Modifier,
    pub suffix: Modifier,
    pub assets: BTreeMap<Cow<'static, str>, Cow<'static, str>>,
    pub data: BTreeMap<String, Value>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Modifier {
    pub speech: Option<String>,
    pub name: Option<String>,
}
