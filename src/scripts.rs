use crate::error::BongTalkError;
use crate::keyed::KeyedOrRaw;
use crate::{
    bongtalk::{BongTalkContext, ScriptData},
    character::Character,
    value::Value,
};
use ahash::RandomState;
use dashmap::DashMap;
use flume::{Receiver, Sender};
use parking_lot::RwLock;
use rhai::{
    debugger::DebuggerCommand, Engine, EvalAltResult, ImmutableString, Module, ModuleResolver,
    Position, Shared, AST,
};
#[cfg(not(all(feature = "wasm", target_arch = "wasm")))]
use std::thread::*;
use std::{collections::HashMap, sync::Arc};
use rhai::debugger::Debugger;
use tinytemplate::TinyTemplate;
#[cfg(all(feature = "wasm", target_arch = "wasm"))]
use wasm_thread::*;
pub enum ControlMessage {}

pub enum EventMessage {
    RhaiError(EvalAltResult),
    BongTalkError(BongTalkError),
    Exit,
    Say(KeyedOrRaw),
    Sleep(u64),
    Choice(Vec<Choice>),
    Event { ident: ImmutableString, data: Value },
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Choice {
    display: KeyedOrRaw,
    value: Value,
}

pub struct ScriptReading {
    scripts: Arc<AST>,
    script_data: Arc<RwLock<ScriptData>>,
    global_data: Arc<DashMap<String, Value, RandomState>>,
    characters: Arc<DashMap<String, Arc<RwLock<Character>>, RandomState>>,
    rhai_engine: Arc<RwLock<Engine>>,
    template: Arc<RwLock<TinyTemplate<'static>>>,

    current_reading: String,
    do_processing: bool,
    thread: Option<JoinHandle<()>>,
    control_sender: Sender<ControlMessage>,
    event_receiver: Receiver<EventMessage>,
}

impl ScriptReading {
    #[allow(clippy::too_many_arguments)]
    #[allow(deprecated)]
    pub fn new(
        scripts: Arc<AST>,
        script_data: Arc<RwLock<ScriptData>>,
        global_data: Arc<DashMap<String, Value, RandomState>>,
        characters: Arc<RwLock<Character>>,
        rhai_engine: Arc<RwLock<Engine>>,
        template: Arc<RwLock<TinyTemplate<'static>>>,
        current_reading: String,
        do_processing: bool,
    ) -> Result<Self, BongTalkError> {
        // register debugger interface

        let (send_ctrl, recv_ctrl) = flume::unbounded();
        let (send_event, recv_event) = flume::unbounded();

        let thread = Builder::new()
            .spawn(move || {})
            .map_err(|why| BongTalkError::ReaderInit(why.to_string()))?;
    }
}

impl Iterator for ScriptReading {
    type Item = EventMessage;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

fn reading_fn(
    scripts: Arc<DashMap<String, Arc<RwLock<AST>>, RandomState>>,
    global_data: Arc<DashMap<String, Value, RandomState>>,
    character: Arc<DashMap<String, Arc<RwLock<Character>>, RandomState>>,
    script_data: Arc<RwLock<ScriptData>>,
    control: Receiver<ControlMessage>,
    event: Sender<EventMessage>,
) {
    let mut engine = Engine::new();
    let resolver = HashmapResolver { data: scripts.clone() };
    engine.set_module_resolver(resolver);
    engine.register_debugger(
        |eng, debugger| {
            Debugger::
        }
    )


}

struct HashmapResolver {
    data: Arc<DashMap<String, Arc<RwLock<AST>>, RandomState>>,
}

impl ModuleResolver for HashmapResolver {
    fn resolve(
        &self,
        engine: &Engine,
        _source: Option<&str>,
        path: &str,
        pos: Position,
    ) -> Result<Shared<Module>, Box<EvalAltResult>> {
        let ast = self
            .data
            .get(path)
            .ok_or(Box::new(EvalAltResult::ErrorModuleNotFound(
                path.to_string(),
                pos,
            )))?
            .value()
            .clone()
            .read();

        Ok(Shared::new(Module::from(Module::eval_ast_as_new(
            rhai::Scope::new(),
            &ast,
            engine,
        ))))
    }
}
