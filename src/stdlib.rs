use crate::keyed::{Keyed, KeyedRef};
use rhai::{Dynamic, EvalAltResult, ImmutableString, Map};

pub trait BongTalkStd {
    fn event(&self, identifier: ImmutableString, message: Dynamic);

    fn lang(&self) -> &str;

    fn say(
        &self,
        character: Option<ImmutableString>,
        message: Dynamic,
    ) -> Result<(), EvalAltResult>;

    fn set(&self, value: Dynamic) -> Result<Dynamic, EvalAltResult>;

    fn get(&self, identifier: ImmutableString) -> Option<Dynamic>;

    fn traversed(&self, identifier: ImmutableString) -> i64;

    fn choice(
        &self,
        identifier: ImmutableString,
        message: ImmutableString,
        choices: &[ImmutableString],
    ) -> Result<String, EvalAltResult>;

    fn sleep(&self, time: i64);

    fn character_exists(&self, identifier: ImmutableString) -> bool;

    fn key<'a>(&self, key: ImmutableString, alt: Option<ImmutableString>) -> KeyedRef<'a>;

    fn translation_exists(&self, key: &KeyedRef, language: ImmutableString) -> bool;
}
