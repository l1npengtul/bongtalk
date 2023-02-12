use crate::keyed::{Keyed, KeyedRef};
use rhai::{Dynamic, EvalAltResult, Map};

pub trait BongTalkStd {
    fn event(&self, identifier: &str, message: Dynamic);

    fn lang(&self) -> &str;

    fn say(&self, character: Option<&str>, message: Dynamic) -> Result<(), EvalAltResult>;

    fn set(&self, value: Dynamic) -> Result<Dynamic, EvalAltResult>;

    fn get(&self, identifier: &str) -> Option<Dynamic>;

    fn traversed(&self, identifier: &str) -> i64;

    fn choice(
        &self,
        identifier: &str,
        message: &str,
        choices: &[&str],
    ) -> Result<String, EvalAltResult>;

    fn sleep(&self, time: i64);

    fn character_exists(&self, identifier: &str) -> bool;

    fn key<'a>(&self, key: &str, alt: Option<&str>) -> KeyedRef<'a>;

    fn translation_exists(&self, key: &KeyedRef, language: &str) -> bool;
}
