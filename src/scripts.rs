use crate::error::BongTalkError;
use crate::keyed::KeyedOrRaw;
use crate::{
    bongtalk::{BongTalkContext, ScriptData},
    character::Character,
    value::Value,
};
use ahash::RandomState;
use dashmap::DashMap;
use flume::Receiver;
use parking_lot::RwLock;
use rhai::{debugger::DebuggerCommand, Engine, EvalAltResult, AST, ImmutableString};
use std::{collections::HashMap, sync::Arc};
use tinytemplate::TinyTemplate;

pub enum ControlMessage {}

pub enum ResultMessage {
    RhaiError(EvalAltResult),
    BongTalkError(BongTalkError),
    Exit,
    Say(KeyedOrRaw),
    Sleep(u64),
    Choice(Vec<Choice>),
    Event {
        ident: ImmutableString,
        data: Value,
    },
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Choice {
    display: KeyedOrRaw,
    value: Value,
}

pub struct ScriptReading {
    scripts: Arc<AST>,
    script_data: Arc<RwLock<ScriptData>>,
    global_data: Arc<RwLock<Value>>,
    characters: Arc<RwLock<Character>>,
    rhai_engine: Arc<RwLock<Engine>>,
    template: Arc<RwLock<TinyTemplate<'static>>>,

    current_reading: String,
    do_processing: bool,
}

impl ScriptReading {
    #[allow(clippy::too_many_arguments)]
    #[allow(deprecated)]
    pub fn new(
        scripts: Arc<AST>,
        script_data: Arc<RwLock<ScriptData>>,
        global_data: Arc<RwLock<Value>>,
        characters: Arc<RwLock<Character>>,
        rhai_engine: Arc<RwLock<Engine>>,
        template: Arc<RwLock<TinyTemplate<'static>>>,
        current_reading: String,
        do_processing: bool,
    ) -> Self {
        // register debugger interface

        let (send, recv) = flume::unbounded();
    }
}

impl Iterator for ScriptReading {
    type Item = ScriptEvent;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
