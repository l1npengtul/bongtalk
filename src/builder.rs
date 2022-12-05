use crate::bongtalk::{BongTalk, ExecContext};
use crate::character::{Character, Modifier};
use crate::value::Value;

#[derive(Clone, Default, Debug)]
pub struct BongTalkBuilder {
    script: Option<String>,
    context: ExecContext,
}

impl BongTalkBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_script(mut self, script: String) -> Self {
        self.script = Some(script);
        self
    }

    pub fn with_world_character(mut self) -> Self {
        self.context.character.insert(
            "World".to_string(),
            Character {
                name: "World".to_string(),
                display_name: "[]".to_string(),
                special: true,
                name_modifier: Modifier {
                    prefix: None,
                    postfix: None,
                },
                speech_modifier: Modifier {
                    prefix: None,
                    postfix: None,
                },
                assets: Default::default(),
                data: Default::default(),
            },
        );
        self
    }

    pub fn with_anonymous_character(mut self) -> Self {
        self.context.character.insert(
            "Anonymous".to_string(),
            Character {
                name: "Anonymous".to_string(),
                display_name: "?".to_string(),
                special: true,
                name_modifier: Modifier {
                    prefix: None,
                    postfix: None,
                },
                speech_modifier: Modifier {
                    prefix: None,
                    postfix: None,
                },
                assets: Default::default(),
                data: Default::default(),
            },
        );
        self
    }

    pub fn with_character(mut self, character: Character) -> Self {
        self.context
            .character
            .insert(character.name.clone(), character);
        self
    }

    pub fn with_variable(mut self, name: impl AsRef<str>, value: impl Into<Value>) -> Self {
        self.context
            .state
            .insert(name.as_ref().to_string(), value.into());
        self
    }

    pub fn with_traverse(mut self, fn_hash: u64, count: u64) -> Self {
        self.context.traversed.insert(fn_hash, count);
        self
    }

    pub fn with_context(mut self, context: ExecContext) -> Self {
        self.context = context;
        self
    }

    pub fn build(self) -> BongTalk {
        let mut bongtalk = BongTalk::new();
        bongtalk
    }
}
