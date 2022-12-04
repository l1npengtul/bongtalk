use crate::character::Character;
use crate::value::Value;
use dashmap::DashMap;
use flume::{Receiver, Sender};
use once_cell::sync::OnceCell;
use rhai::Engine;
use std::sync::{Arc, RwLock};
#[cfg(not(target_arch = "wasm"))]
use std::thread::{JoinHandle, spawn};
use upon::Engine as UponEngine;
#[cfg(all(target_arch = "wasm", target_feature = "wasm"))]
use wasm_thread::{JoinHandle, spawn};

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
    thread: JoinHandle<()>,
}

impl BongTalk {
    pub fn new
}
