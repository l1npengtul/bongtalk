use crate::keyed::Keyed;
use rhai::{Dynamic, EvalAltResult, Identifier, Map};

pub trait BongTalkStd {
    fn event(identifier: String, message: Dynamic);

    fn lang() -> String;

    fn set_lang(key: String) -> bool;

    fn say(character: Option<String>, message: Dynamic) -> Result<(), EvalAltResult>;

    fn set(value: Dynamic) -> Result<Dynamic, EvalAltResult>;

    fn get(identifier: String) -> Option<Dynamic>;

    fn traversed(identifier: String) -> i64;

    fn choice(
        identifier: String,
        message: String,
        choices: &[String],
    ) -> Result<String, EvalAltResult>;

    fn sleep(time: i64);

    fn new_character(
        identifier: String,
        special: bool,
        name: Dynamic,
        speech_prefix: Dynamic,
        speech_postfix: Dynamic,
        name_prefix: Dynamic,
        name_postfix: Dynamic,
    ) -> Result<(), EvalAltResult>;

    fn character_exists(identifier: String) -> bool;

    fn delete_character(identifier: String);

    fn key(key: String, alt: Option<String>) -> Keyed;

    fn translation_exists(key: String, language: String) -> bool;
}

pub struct BongTalkStandard<'a> {}
