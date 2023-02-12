use crate::bongtalk::{BongTalkContext, ScriptData};
use crate::character::Character;
use crate::value::Value;
use ahash::RandomState;
use flume::Receiver;
use rhai::debugger::DebuggerCommand;
use rhai::{Engine, AST};
use std::collections::HashMap;
use tinytemplate::TinyTemplate;

pub enum ScriptEvent<'a> {}

pub enum InternalEvent<'a> {}

pub struct ScriptReading<'a> {
    global_data: &'a mut HashMap<String, Value, RandomState>,
    characters: &'a mut HashMap<String, Character, RandomState>,
    rhai_engine: &'a mut Engine,
    template: &'a mut TinyTemplate<'a>,

    assigned_reading: &'a AST,
    data: &'a mut ScriptData,
    reading_name: &'a str,
    do_processing: bool,
    message_recv: Receiver<InternalEvent<'a>>,
}

impl<'a> ScriptReading<'a> {
    #[allow(clippy::too_many_arguments)]
    #[allow(deprecated)]
    pub fn new(
        assigned_reading: &AST,
        data: &ScriptData,
        reading_name: &str,
        do_processing: bool,
        global_data: &'a mut HashMap<String, Value, RandomState>,
        characters: &'a mut HashMap<String, Character, RandomState>,
        rhai_engine: &'a mut Engine,
        template: &'a mut TinyTemplate<'a>,
    ) -> Self {
        // register debugger interface

        let (send, recv) = flume::unbounded();

        rhai_engine.register_debugger(
            |eng, dbg| dbg,
            |context, event, node, source, pos| DebuggerCommand::Continue,
        )
    }
}

impl<'a> Iterator for ScriptReading<'a> {
    type Item = ScriptEvent<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
