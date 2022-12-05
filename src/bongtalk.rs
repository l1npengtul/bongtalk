use crate::character::Character;
use crate::value::Value;
use dashmap::DashMap;
use flume::{Receiver, Sender};
use once_cell::sync::OnceCell;
use rhai::Engine;
use std::sync::{Arc, RwLock};
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

struct SendRecv<T> {
    pub sender: Arc<Sender<T>>,
    pub receiver: Arc<Receiver<T>>,
}

impl<T> SendRecv<T> {
    pub fn new() -> Self {
        let (send, recv) = flume::unbounded();
        Self {
            sender: Arc::new(send),
            receiver: Arc::new(recv),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExecContext {
    pub traversed: Arc<DashMap<u64, u64>>,
    pub character: Arc<DashMap<String, Character>>,
    pub state: Arc<DashMap<String, Value>>,
}

pub struct BongTalk {
    rhai_engine: Arc<RwLock<Engine>>,
    template: Arc<RwLock<UponEngine<'static>>>,
    from_thread: SendRecv<ScriptRequest>,
    to_thread: SendRecv<ScriptReply>,
    context: ExecContext,
    thread: Option<JoinHandle<()>>,
}

impl BongTalk {
    pub fn new() -> Self {
        Self::default()
    }
    
    
}

impl Default for BongTalk {
    fn default() -> Self {
        Self {
            rhai_engine: Arc::new(RwLock::new(Engine::new())),
            template: Arc::new(RwLock::new(UponEngine::new())),
            from_thread: SendRecv::new(),
            to_thread: SendRecv::new(),
            context: ExecContext {
                traversed: Arc::new(Default::default()),
                character: Arc::new(Default::default()),
                state: Arc::new(Default::default()),
            },
            thread: None,
        }
    }
}
