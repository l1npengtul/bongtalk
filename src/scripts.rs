use crate::character::CharacterRef;
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
use rhai::debugger::{BreakPoint, Debugger, DebuggerEvent};
use rhai::{
    debugger::DebuggerCommand, ASTNode, Dynamic, Engine, EvalAltResult, Expr, ImmutableString,
    LexError, Map, Module, ModuleResolver, ParseError, ParseErrorType, Position, Shared, Stmt,
    Variant, AST,
};
use smartstring::{LazyCompact, SmartString};
use std::str::FromStr;
#[cfg(not(all(feature = "wasm", target_arch = "wasm")))]
use std::thread::*;
use std::{collections::HashMap, sync::Arc};
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

#[allow(deprecated)]
fn reading_fn(
    script: String,
    scripts: Arc<DashMap<SmartString<LazyCompact>, AST, RandomState>>,
    global_data: Arc<DashMap<SmartString<LazyCompact>, Value, RandomState>>,
    character: Arc<DashMap<SmartString<LazyCompact>, Character, RandomState>>,
    script_data: Arc<RwLock<ScriptData>>,
    control: Receiver<ControlMessage>,
    event: Sender<EventMessage>,
) {
    let mut engine = Engine::new();
    let resolver = HashmapResolver {
        data: scripts.clone(),
    };

    // ex)
    // [expression] face <character> says <text> with_extra [metadata]
    // this gets parsed with <character> says <text> => TextSayThing, with modifications on top

    engine
        .register_custom_operator("says", 20)
        .expect("Failed to register custom operator! This is a bug, please report it!");
    engine.register_fn("says", |character: &Dynamic, text: &Dynamic| {
        // resolve character
        // TODO
    });

    engine
        .register_custom_operator("face", 10)
        .expect("Failed to register custom operator! This is a bug, please report it!");
    engine.register_fn("face", |expression: &Dynamic, text_say: &Dynamic| {
        // TODO
    });

    engine
        .register_custom_operator("with_extra", 5)
        .expect("Failed to register custom operator! This is a bug, please report it!");
    engine.register_fn("with_extra", |text_say: &Dynamic, data: &Dynamic| {
        // TODO
    });

    // global/local data stores

    engine.register_fn("get", |key: &str| -> Option<&Dynamic> {
        script_data
            .read()
            .local_kv_store
            .get(key.into())
            .map(|val| val.into())
    });

    engine.register_fn("set", |key: &str, value: Dynamic| -> Option<&Dynamic> {
        script_data
            .write()
            .local_kv_store
            .insert(key.into(), value.into())
            .map(|v| v.into())
    });

    engine.register_fn("remove", |key: &str| {
        script_data.write().local_kv_store.remove(key)
    });

    engine.register_fn("clear_all", || script_data.write().local_kv_store.clear());

    engine.register_fn("get_global", |key: &str| -> Option<&Dynamic> {
        global_data.get(key.into()).map(|val| val.into())
    });

    engine.register_fn(
        "set_global",
        |key: &str, value: Dynamic| -> Option<&Dynamic> {
            global_data
                .insert(key.into(), value.into())
                .map(|v| v.into())
        },
    );

    engine.register_fn("remove_global", |key: &str| global_data.remove(key));

    engine.register_fn("clear_global", || global_data.clear());

    // traversals

    engine.register_fn("traversed", |function: &str| -> i64 {
        script_data.read().traversals.get(function.into())
    });

    engine.register_fn("reset_traversed", |function: &str| {
        script_data.write().traversals.reset(function.into());
    });

    // character
    engine
        .register_type_with_name::<CharacterRef>("Character")
        .register_get_set(
            "name",
            |shelf: &mut CharacterRef| -> Option<&str> {
                character
                    .get(shelf.identifier.as_str())
                    .map(|x| &x.value().name.into())
            },
            |shelf: &mut CharacterRef, value: &str| {
                character
                    .get_mut(shelf.identifier.as_str())
                    .map(|mut x| x.value_mut().set_name(value));
            },
        )
        .register_get_set(
            "special",
            |shelf: &mut CharacterRef| -> Option<bool> {
                character
                    .get(shelf.identifier.as_str())
                    .map(|x| x.value().special)
            },
            |shelf: &mut CharacterRef, value: bool| {
                character
                    .get_mut(shelf.identifier.as_str())
                    .map(|mut x| x.value_mut().set_is_special(value));
            },
        )
        .register_get_set(
            "name_prefix",
            |shelf: &mut CharacterRef| -> Option<&str> {
                character
                    .get(shelf.identifier.as_str())
                    .map(|x| x.value().name_prefix())
                    .flatten()
            },
            |shelf: &mut CharacterRef, value: &str| {
                character
                    .get_mut(shelf.identifier.as_str())
                    .map(|mut x| x.value_mut().set_name_prefix(value.to_string()));
            },
        )
        .register_fn("clear_name_prefix", |shelf: &mut CharacterRef| {
            character
                .get_mut(shelf.identifier.as_str())
                .map(|mut x| x.value_mut().clear_name_prefix());
        })
        .register_get_set(
            "name_postfix",
            |shelf: &mut CharacterRef| -> Option<&str> {
                character
                    .get(shelf.identifier.as_str())
                    .map(|x| x.value().name_postfix())
                    .flatten()
            },
            |shelf: &mut CharacterRef, value: &str| {
                character
                    .get_mut(shelf.identifier.as_str())
                    .map(|mut x| x.value_mut().set_name_postfix(value.to_string()));
            },
        )
        .register_fn("clear_name_postfix", |shelf: &mut CharacterRef| {
            character
                .get_mut(shelf.identifier.as_str())
                .map(|mut x| x.value_mut().clear_name_postfix());
        })
        .register_get_set(
            "speech_prefix",
            |shelf: &mut CharacterRef| -> Option<&str> {
                character
                    .get(shelf.identifier.as_str())
                    .map(|x| x.value().speech_prefix())
                    .flatten()
            },
            |shelf: &mut CharacterRef, value: &str| {
                character
                    .get_mut(shelf.identifier.as_str())
                    .map(|mut x| x.value_mut().set_speech_prefix(value.to_string()));
            },
        )
        .register_fn("clear_speech_prefix", |shelf: &mut CharacterRef| {
            character
                .get_mut(shelf.identifier.as_str())
                .map(|mut x| x.value_mut().clear_speech_prefix());
        })
        .register_get_set(
            "speech_postfix",
            |shelf: &mut CharacterRef| -> Option<&str> {
                character
                    .get(shelf.identifier.as_str())
                    .map(|x| x.value().speech_postfix())
                    .flatten()
            },
            |shelf: &mut CharacterRef, value: &str| {
                character
                    .get_mut(shelf.identifier.as_str())
                    .map(|mut x| x.value_mut().set_speech_postfix(value.to_string()));
            },
        )
        .register_fn("clear_speech_postfix", |shelf: &mut CharacterRef| {
            character
                .get_mut(shelf.identifier.as_str())
                .map(|mut x| x.value_mut().clear_speech_postfix());
        })
        .register_fn(
            "get_value",
            |shelf: &mut CharacterRef, key: &ImmutableString| -> Option<&Dynamic> {
                character
                    .get(shelf.identifier.as_str())
                    .map(|x| x.value().get_dynamic(key.into()))
                    .flatten()
            },
        )
        .register_fn(
            "set_value",
            |shelf: &mut CharacterRef,
             key: &ImmutableString,
             value: &Dynamic|
             -> Option<&Dynamic> {
                character
                    .get_mut(shelf.identifier.as_str())
                    .map(|mut x| x.value_mut().set_dynamic(key.into(), value))
                    .flatten()
            },
        )
        .register_fn("still_exists", |shelf: &mut CharacterRef| -> bool {
            character.contains_key(shelf.identifier.into())
        });

    engine.register_fn("character_exists", |key: &str| -> bool {
        character.contains_key(key)
    });

    engine.register_fn("get_character", |key: &str| -> Option<CharacterRef> {
        if let Some(character) = character.get(key) {
            Some(CharacterRef {
                identifier: character.identifier.clone(),
            })
        } else {
            None
        }
    });

    // choices custom syntax

    engine.register_custom_syntax_with_state_raw(
        "choice",
        |symbols, look_ahead, state| {
            if !state.is_map() {
                let tag = state.tag();
                *state = Dynamic::from_map(Map::new());
                state.set_tag(tag);
            }

            if symbols.len() == 1 {
                state.set_tag(0);
                return Ok(Some("$expr$".into()));
            } else if symbols.len() == 2 {
                return Ok(Some("{".into()));
            } else if symbols.len() == 3 {
                return Ok(Some("$expr$".into()));
            }

            if state.tag() == 0 {
                state.set_tag(1);
                return Ok(Some("=>".into()));
            } else if state.tag() == 1 {
                state.set_tag(2);
                return Ok(Some("$block$".into()));
            } else if state.tag() == 2 {
                let state_map = state.as_any_mut().downcast_mut::<Map>().unwrap();
                let this_len = (symbols.len() / 3) - 2;
                let ilen = symbols.len() - 1 as u32;
                state_map.insert(this_len.into(), vec![ilen - 2, ilen].into());
                return if look_ahead == "}" {
                    let state_map = state.as_any().downcast_ref::<Map>().unwrap().keys().len() - 1;
                    state.set_tag(state_map as i32);
                    Ok(None)
                } else {
                    Ok(Some("$expr$".into()))
                };
            }

            return Err(ParseError(
                Box::from(ParseErrorType::BadInput(LexError::UnexpectedInput(
                    "Unknown state?".into(),
                ))),
                Position::NONE,
            ));
        },
        false,
        |context, inputs, state| {
            let counts = state.tag();

            let ask = context.eval_expression_tree(
                inputs
                    .get(0)
                    .ok_or(Err("Key Expression Missing".to_string()))?,
            )?;
            let mut options =
                HashMap::with_capacity_and_hasher(counts as usize, RandomState::new());
            for (_, value) in state.as_any().downcast_ref::<Map>().unwrap() {
                if let Ok(lenarr) = value.into_typed_array::<u32>() {
                    let keyed = context.eval_expression_tree(&inputs[lenarr[0] as usize])?;
                    let block_run = &inputs[lenarr[1] as usize];
                    options.insert(keyed, block_run);
                }
            }

            // TODO: implement ask
        },
    );

    engine.set_module_resolver(resolver);
    engine.register_debugger(
        |eng, mut debugger| {
            for fn_def in scripts
                .get(&script)
                .map(|s| s.value().iter_fn_def())
                .unwrap()
            {
                debugger
                    .break_points_mut()
                    .push(BreakPoint::AtFunctionName {
                        name: fn_def.name.into(),
                        enabled: true,
                    });
            }
            debugger
        },
        |ctx, event, node, _, _| {
            if let DebuggerEvent::BreakPoint(_) = event {
                let fn_name = match node {
                    ASTNode::Stmt(s) => match s {
                        Stmt::FnCall(f, _) => &f.name,
                        _ => return,
                    },
                    ASTNode::Expr(e) => match e {
                        Expr::FnCall(f, _) => &f.name,
                        _ => return,
                    },
                };
                script_data.write().traversals.add(fn_name.into());
            }

            Ok(DebuggerCommand::Continue)
        },
    );
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
