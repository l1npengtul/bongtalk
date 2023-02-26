use crate::value::Value;
use rhai::Dynamic;
use smartstring::{LazyCompact, SmartString};
use std::{
    borrow::Cow,
    collections::BTreeMap,
    hash::{Hash, Hasher},
    process::id,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Character {
    pub identifier: SmartString<LazyCompact>,
    pub name: String,
    pub special: bool,
    pub name_modifier: Modifier,
    pub speech_modifier: Modifier,
    pub data: BTreeMap<SmartString<LazyCompact>, Value>,
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

    pub fn set_name_prefix(&mut self, prefix: String) {
        self.name_modifier.prefix = Some(prefix);
    }

    pub fn set_name_postfix(&mut self, postfix: String) {
        self.name_modifier.postfix = Some(postfix);
    }

    pub fn set_speech_prefix(&mut self, prefix: String) {
        self.speech_modifier.prefix = Some(prefix);
    }

    pub fn set_speech_postfix(&mut self, postfix: String) {
        self.speech_modifier.postfix = Some(postfix);
    }

    pub fn clear_name_prefix(&mut self) {
        self.name_modifier.prefix = None;
    }

    pub fn clear_name_postfix(&mut self) {
        self.name_modifier.postfix = None;
    }

    pub fn clear_speech_prefix(&mut self) {
        self.speech_modifier.prefix = None;
    }

    pub fn clear_speech_postfix(&mut self) {
        self.speech_modifier.postfix = None;
    }

    pub fn set_identifier(&mut self, identifier: SmartString<LazyCompact>) {
        self.identifier = identifier;
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_is_special(&mut self, special: bool) {
        self.special = special;
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }

    pub fn set(&mut self, key: SmartString<LazyCompact>, value: Value) -> Option<Value> {
        self.data.insert(key, value)
    }

    pub fn get_dynamic(&self, key: &SmartString<LazyCompact>) -> Option<&Dynamic> {
        self.data.get(key).map(|x| x.into())
    }

    pub fn set_dynamic(
        &mut self,
        key: &SmartString<LazyCompact>,
        value: &Dynamic,
    ) -> Option<&Dynamic> {
        self.data.insert(key.into(), value.into()).map(|x| x.into())
    }
}

impl Hash for Character {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.as_str().hash(state)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Modifier {
    pub prefix: Option<String>,
    pub postfix: Option<String>,
}

#[derive(Copy, Clone, Debug)]
pub struct CharacterRef {
    pub identifier: SmartString<LazyCompact>,
}
