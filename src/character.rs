use crate::value::Value;
use rhai::Dynamic;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Character {
    pub identifier: String,
    pub name: String,
    pub special: bool,
    pub name_modifier: Modifier,
    pub speech_modifier: Modifier,
    pub data: BTreeMap<String, Value>,
}

impl Character {
    pub fn name_prefix(&self) -> Option<&str> {
        self.name_modifier.prefix.as_ref().into()
    }

    pub fn name_postfix(&self) -> Option<&str> {
        self.name_modifier.postfix.as_ref().into()
    }

    pub fn speech_prefix(&self) -> Option<&str> {
        self.speech_modifier.prefix.as_ref().into()
    }

    pub fn speech_postfix(&self) -> Option<&str> {
        self.speech_modifier.postfix.as_ref().into()
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_special(&self) -> bool {
        self.special
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }

    pub fn set(&mut self, key: String, value: Value) -> Option<Value> {
        self.data.insert(key, value)
    }

    pub fn get_dynamic(&self, key: &str) -> Dynamic {
        self.data.get(key).map(|x| {})
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Modifier {
    pub prefix: Option<String>,
    pub postfix: Option<String>,
}
