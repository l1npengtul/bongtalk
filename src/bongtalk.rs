use crate::character::Character;
use crate::value::Value;
use dashmap::DashMap;
use flume::{Receiver, Sender};
use once_cell::sync::OnceCell;
use rhai::{Engine, AST};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, RwLock};
#[cfg(not(target_arch = "wasm"))]
use std::thread::{spawn, JoinHandle};
use upon::{Engine as UponEngine, Template};
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
    pub
}

pub struct BongTalkContext {
    scripts: BTreeMap<String, Arc<AST>>,
    data: Arc<Mutex<ScriptData>>
}
