use crate::character::Character;
use crate::error::{BResult, BongTalkError};
use crate::store::TraversedStore;
use crate::value::Value;
use ahash::RandomState;
use dashmap::DashMap;
use flume::{Receiver, Sender};
use once_cell::sync::OnceCell;
use rhai::{Engine, AST};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
#[cfg(not(target_arch = "wasm"))]
use std::thread::{spawn, JoinHandle};
use tinytemplate::TinyTemplate;
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
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BongTalkContext {
    scripts: HashMap<String, AST, RandomState>,
    script_data: HashMap<String, ScriptData, RandomState>,
    global_data: HashMap<String, Value, RandomState>,
    characters: HashMap<String, Character, RandomState>,
    rhai_engine: Engine,
    template: TinyTemplate<'static>,
    gil: Arc<AtomicBool>,
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

        self.scripts.insert(name.as_ref().to_string(), compiled);
        Ok(())
    }

    pub fn add_script_ast(&mut self, name: impl AsRef<str>, ast: AST) -> BResult<()> {
        if self.scripts.contains_key(name.as_ref()) {
            return Err(BongTalkError::Compile(format!(
                "{} already exists!",
                name.as_ref()
            )));
        }

        self.scripts.insert(name.as_ref().to_string(), ast);
        Ok(())
    }

    pub fn is_locked(&self) -> bool {
        self.gil.load(Ordering::SeqCst)
    }
    
    pub fn read(&self, script: impl AsRef<str>) -> 
}
