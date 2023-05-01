use crate::{
    character::Character,
    error::{BResult, BongTalkError},
    store::TraversedStore,
    value::Value,
};
use ahash::RandomState;
use dashmap::DashMap;
use flume::{Receiver, Sender};
use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use ramhorns::Ramhorns;
use rhai::{Engine, AST};
use serde::{Deserialize, Serialize};
use smartstring::{Compact, LazyCompact, SmartString, SmartStringMode};
use std::{
    collections::{BTreeMap, HashMap},
    sync::atomic::{AtomicBool, AtomicU32, Ordering},
    sync::Arc,
    thread::{spawn, JoinHandle},
};
#[cfg(all(target_arch = "wasm", target_feature = "wasm"))]
use wasm_thread::{spawn, JoinHandle};

enum ScriptReply {
    Continue,
    Stop,
    Picked,
    ContextChange,
    Track,
    Event,
}

enum ScriptRequest {
    Say,
    Monologue,
    Choice,
    ContextChange,
    TrackChange,
    Event,
}

pub struct ScriptData {
    pub traversals: TraversedStore,
    pub local_kv_store: BTreeMap<SmartString<Compact>, Value>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BongTalkContext {
    scripts: Arc<DashMap<SmartString<LazyCompact>, AST, RandomState>>,
    script_data: HashMap<SmartString<LazyCompact>, ScriptData, RandomState>,
    global_data: Arc<DashMap<SmartString<LazyCompact>, Value, RandomState>>,
    characters: Arc<DashMap<SmartString<LazyCompact>, Character, RandomState>>,
    rhai_engine: Arc<RwLock<Engine>>,
    template_engine: Arc<RwLock<Ramhorns<RandomState>>>,
    run_counter: Arc<AtomicU32>,
}

impl BongTalkContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_script(&mut self, name: impl AsRef<str>, script: impl AsRef<str>) -> BResult<()> {
        if self.scripts.contains_key(name.as_ref()) {
            return Err(BongTalkError::Compile(format!(
                "{} already exists!",
                name.as_ref()
            )));
        }

        let compiled = match self.rhai_engine.compile(script) {
            Ok(ast) => ast,
            Err(why) => return Err(BongTalkError::Compile(why.to_string())),
        };

        self.scripts.insert(name.as_ref().into(), compiled);
        Ok(())
    }

    pub fn add_script_ast(&mut self, name: impl AsRef<str>, ast: AST) -> BResult<()> {
        if self.scripts.contains_key(name.as_ref()) {
            return Err(BongTalkError::Compile(format!(
                "{} already exists!",
                name.as_ref()
            )));
        }

        self.scripts.insert(name.as_ref().into(), ast);
        Ok(())
    }

    pub fn is_locked(&self) -> bool {
        self.gil.load(Ordering::SeqCst)
    }

    pub fn read(&self, script: impl AsRef<str>) {}
}
