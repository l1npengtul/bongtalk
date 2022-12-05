use crate::value::Value;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Character {
    pub name: String,
    pub display_name: String,
    pub special: bool,
    pub name_modifier: Modifier,
    pub speech_modifier: Modifier,
    pub assets: BTreeMap<Cow<'static, str>, Cow<'static, str>>,
    pub data: BTreeMap<String, Value>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Modifier {
    pub prefix: Option<String>,
    pub postfix: Option<String>,
}
