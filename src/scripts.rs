use crate::character::CharacterRef;
use crate::error::BongTalkError;
use crate::keyed::{KeyedOrRaw, KeyedRef};
use crate::scripts::EventMessage::RhaiError;
use crate::{
    bongtalk::{BongTalkContext, ScriptData},
    character::Character,
    value::Value,
};
use ahash::RandomState;
use dashmap::DashMap;
use flume::{Receiver, RecvError, Sender};
use ramhorns::{Ramhorns, Template};
use rhai::debugger::{BreakPoint, Debugger, DebuggerEvent};
use rhai::plugin::RhaiResult;
use rhai::{
    debugger::DebuggerCommand, ASTNode, Dynamic, Engine, EvalAltResult, Expr, Expression,
    Identifier, ImmutableString, LexError, Map, Module, ModuleResolver, ParseError, ParseErrorType,
    Position, Shared, Stmt, Variant, AST,
};
use serde::{Deserialize, Serialize};
use smartstring::{LazyCompact, SmartString};
use std::str::FromStr;
use std::sync::RwLock;
#[cfg(not(all(feature = "wasm", target_arch = "wasm")))]
use std::thread::{Builder, JoinHandle, Thread};
use std::{collections::HashMap, sync::Arc};
#[cfg(all(feature = "wasm", target_arch = "wasm"))]
use wasm_thread::{Builder, JoinHandle, Thread};

pub struct ScriptReading {
    script: Arc<AST>,
    script_data: Arc<RwLock<ScriptData>>,
    global_data: Arc<DashMap<ImmutableString, Arc<RwLock<Value>>, RandomState>>,
    characters: Arc<DashMap<ImmutableString, Arc<RwLock<Character>>, RandomState>>,
    template_engine: Arc<RwLock<Ramhorns<RandomState>>>,
    current_reading: ImmutableString,
    do_processing: bool,

    rhai_engine: Engine,
    thread: Option<JoinHandle<()>>,
    control_sender: Arc<Sender<ControlMessage>>,
    event_receiver: Arc<Receiver<EventMessage>>,
}

impl ScriptReading {
    #[allow(clippy::too_many_arguments)]
    #[allow(deprecated)]
    pub fn new(
        script: Arc<AST>,
        script_data: Arc<RwLock<ScriptData>>,
        global_data: Arc<DashMap<ImmutableString, Arc<RwLock<Value>>, RandomState>>,
        characters: Arc<DashMap<ImmutableString, Arc<RwLock<Character>>, RandomState>>,
        rhai_engine: &Engine,
        current_reading: ImmutableString,
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

#[derive(Clone, Debug, PartialEq)]
pub enum ControlMessage {
    Continue,
    ChoicePicked(i32),
    Abort,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EventMessage {
    RhaiError(EvalAltResult),
    BongTalkError(BongTalkError),
    Exit,
    Say(SpokenAction),
    Sleep(u64),
    Choice(Question),
    Event(ScriptEvent),
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Question {
    pub(crate) text: KeyedOrRaw,
    pub(crate) character: Option<CharacterRef>,
    pub(crate) emotion: Option<ImmutableString>,
    pub(crate) extra: Option<Value>,
    pub(crate) choices: Vec<Choice>,
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Choice {
    display: KeyedOrRaw,
    value: Value,
}

#[derive(Clone, Debug, Default, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct SpokenAction {
    pub(crate) character: CharacterRef,
    pub(crate) text: KeyedOrRaw,
    pub(crate) emotion: Option<ImmutableString>,
    pub(crate) extra: Option<Value>,
}

impl SpokenAction {
    pub fn character(&self) -> &CharacterRef {
        &self.character
    }

    pub fn text(&self) -> &KeyedOrRaw {
        &self.text
    }

    pub fn emotion(&self) -> &Option<ImmutableString> {
        &self.emotion
    }

    pub fn extra(&self) -> &Option<Value> {
        &self.extra
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct ScriptEvent {
    pub name: SmartString<LazyCompact>,
    pub data: Option<Value>,
}

#[allow(deprecated)]
fn engine_setup(
    script: Arc<AST>,
    script_data: Arc<RwLock<ScriptData>>,
    global_data: Arc<DashMap<ImmutableString, Arc<RwLock<Value>>, RandomState>>,
    characters: Arc<DashMap<ImmutableString, Arc<RwLock<Character>>, RandomState>>,
    base_engine: Engine,
    control_receiver: Arc<Receiver<ControlMessage>>,
    event_sender: Arc<Sender<EventMessage>>,
) -> Result<Engine, BongTalkError> {
    let mut engine = base_engine; // muhahahahaahahahahaha ive repossessed your engine
                                  // event [event] | {custom data}
    engine.register_custom_syntax_with_state_raw(
        "event",
        |symbols, look_ahead, state| match symbols.len() {
            1 => Ok(Some("$expr$".into())),
            2 => match look_ahead {
                "" => Ok(Some("$$none$$".into())),
                "|" => Ok(Some("|".into())),
                i => Err(ParseError(
                    Box::from(ParseErrorType::BadInput(LexError::UnexpectedInput(
                        i.to_string(),
                    ))),
                    Position::NONE,
                )),
            },
            3 => Ok(Some("$expr$".into())),
            4 => Ok(Some("$$some$$".into())),
            _ => Err(ParseError(
                Box::from(ParseErrorType::BadInput(LexError::UnexpectedInput(
                    look_ahead.to_string(),
                ))),
                Position::NONE,
            )),
        },
        false,
        |context, inputs, state| {
            let key = inputs.last().unwrap().get_string_value().unwrap();

            let event_name = match inputs.get(2) {
                Some(s) => context.eval_expression_tree(s)?.into_immutable_string()?,
                None => {
                    return Err(RhaiError::from(ParseError(
                        Box::new(ParseErrorType::BadInput(LexError::UnexpectedInput(
                            "No Input".into(),
                        ))),
                        Position::NONE,
                    )))
                }
            };

            let event = match key {
                "$$none$$" => {
                    ScriptEvent {
                        name: event_name.into_owned().into(), // todo: find more better way to do this, maybe a PR
                        data: None,
                    }
                }
                "$$some$$" => {
                    let metadata = inputs.get(4).unwrap().eval_with_context(context)?.into();
                    ScriptEvent {
                        name: event_name.into_owned().into(), // todo: find more better way to do this, maybe a PR
                        data: Some(metadata),
                    }
                }
                i => {
                    return Err(RhaiError::from(ParseError(
                        Box::new(ParseErrorType::BadInput(LexError::UnexpectedInput(
                            format!("Expected end token, got {}", i),
                        ))),
                        Position::NONE,
                    )))
                }
            };

            event_sender.send(EventMessage::Event(event))?;

            Ok(Dynamic::UNIT)
        },
    );

    // say [character] @ sad | {custom data}: "text"
    engine.register_custom_syntax_with_state_raw(
        "say",
        |symbols, look_ahead, state| {
            //
            if !state.is_map() {
                let tag = state.tag();
                *state = Dynamic::from_map(Map::new());
                state.set_tag(tag);
            }

            match symbols.len() {
                1 => {
                    return Ok(Some("[".into()));
                }
                2 => {
                    return Ok(Some("$expr$".into()));
                }
                3 => {
                    return Ok(Some("]".into()));
                }
                n => {
                    if let Some(c) = state
                        .read_lock::<Map>()
                        .map(|x| x.get("current".into()))
                        .flatten()
                    {
                        let current = c.into_immutable_string().unwrap();
                        if current == "@" {
                            let mut state_map = state.write_lock::<Map>().unwrap();

                            if state_map.contains_key("emotion_len".into()) {
                                state_map.insert("finished_emotion".into(), true.into());
                            } else {
                                state_map.insert("emotion_len".into(), (n as i64).into());
                                return Ok(Some("$expr$".into()));
                            }
                        } else if current == "|" {
                            let mut state_map = state.write_lock::<Map>().unwrap();

                            if state_map.contains_key("metadata_len".into()) {
                                state_map.insert("finished_metadata".into(), true.into());
                            } else {
                                state_map.insert("metadata_len".into(), (n as i64).into());
                                return Ok(Some("$expr$".into()));
                            }
                        } else if current == ":" {
                            let mut state_map = state.write_lock::<Map>().unwrap();

                            return if state_map.contains_key("text_len".into()) {
                                state_map.insert("finished_text".into(), true.into());
                                Ok(None)
                            } else {
                                state_map.insert("text_len".into(), (n as i64).into());
                                Ok(Some("$expr$".into()))
                            };
                        }
                    }

                    if look_ahead == "@" {
                        let mut state_map = state.write_lock::<Map>().unwrap();

                        if state_map.contains_key("finished_emotion") {
                            return Err(ParseError(
                                Box::from(ParseErrorType::DuplicatedProperty("Emotion (@)".into())),
                                Position::NONE,
                            ));
                        }

                        state_map.insert("current".into(), "emotion".into());
                        return Ok(Some("@".into()));
                    } else if look_ahead == "|" {
                        let mut state_map = state.write_lock::<Map>().unwrap();

                        if state_map.contains_key("finished_metadata") {
                            return Err(ParseError(
                                Box::from(ParseErrorType::DuplicatedProperty(
                                    "Metadata (|)".into(),
                                )),
                                Position::NONE,
                            ));
                        }

                        state_map.insert("current".into(), "extras".into());
                        return Ok(Some("|".into()));
                    } else if look_ahead == ":" {
                        let mut state_map = state.write_lock::<Map>().unwrap();

                        if state_map.contains_key("finished_text") {
                            return Err(ParseError(
                                Box::from(ParseErrorType::DuplicatedProperty("Text (:)".into())),
                                Position::NONE,
                            ));
                        }

                        state_map.insert("current".into(), "text".into());
                        return Ok(Some(":".into()));
                    }
                }
            }
            Err(ParseError(
                Box::new(ParseErrorType::BadInput(LexError::UnexpectedInput(
                    "No Input".into(),
                ))),
                Position::NONE,
            ))
        },
        false,
        |context, inputs, state| -> RhaiResult {
            let state_map = state.read_lock::<Map>().unwrap();

            let mut spoken = SpokenAction::default();

            let finished_text = state_map.contains_key("finished_text");
            let finished_metadata = state_map.contains_key("finished_metadata");
            let finished_emotion = state_map.contains_key("finished_emotion");

            let character_expr = inputs.get(2).unwrap();
            let character_eval = context.eval_expression_tree(character_expr)?;
            let character = if character_eval.is_string() {
                CharacterRef::from(character_eval.into_immutable_string()?)
            } else {
                character_eval.try_cast::<CharacterRef>()
            };

            if !character.contains_key(character_ref.as_str()) {
                return Err(Box::new(EvalAltResult::ErrorRuntime(
                    Dynamic::from(
                        BongTalkError::InvalidCharacterReference(character_ref)
                            .to_string()
                            .into(),
                    ),
                    character_expr.position(),
                )));
            }

            spoken.character = character;

            if finished_text {
                let text_len = state_map.get("text_len")?.as_int()?;
                let eval = context.eval_expression_tree(inputs.get(text_len as usize).unwrap())?;

                let text = if eval.is_string() {
                    KeyedOrRaw::Raw(eval.into_immutable_string()?)
                } else {
                    // turn into keyed
                    KeyedOrRaw::Keyed(eval.try_cast::<KeyedRef>().ok_or(|why| {}).into())
                };

                spoken.text = text;
            }

            if finished_metadata {
                let metadata_len = state_map.get("metadata_len")?.as_int()?;
                let eval =
                    context.eval_expression_tree(inputs.get(metadata_len as usize).unwrap())?;

                let metadata = Value::from(eval);

                spoken.extra = Some(metadata);
            }

            if finished_emotion {
                let text_len = state_map.get("text_len")?.as_int()?;
                let eval = context.eval_expression_tree(inputs.get(text_len as usize).unwrap())?;

                let emotion = eval.into_immutable_string()?;

                spoken.emotion = Some(emotion);
            }

            event_sender.clone().send(EventMessage::Say(spoken))?;

            match control_receiver.clone().recv() {
                Ok(ctrl) => match ctrl {
                    ControlMessage::Abort => return Ok(Dynamic::UNIT),
                    _ => {}
                },
                Err(why) => {
                    return Err(why.to_string().into());
                }
            }

            Ok(Dynamic::UNIT)
        },
    );

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
                identifier: ImmutableString::from(character.identifier.clone()),
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
                let mut state_map = state.write_lock::<Map>().unwrap();
                let this_len = (symbols.len() / 3) - 2;
                let ilen = symbols.len() - 1 as u32;
                state_map.insert(this_len.into(), vec![ilen - 2, ilen].into());
                return if look_ahead == "}" {
                    let state_map = state.read_lock::<Map>().unwrap().keys().len() - 1;
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
            let mut question = Question {
                text: "".into(),
                character: None,
                emotion: None,
                extra: None,
                choices: vec![],
            };

            let ask = {
                let q = context.eval_expression_tree(
                    inputs
                        .get(0)
                        .ok_or(Err("Key Expression Missing".to_string()))?,
                )?;

                // see if array
                // 1 => text
                // 2 => character, text
                // 3 => character, text, emotion
                // 4 => character, text, emotion, additional info
                if let Ok(question_arr) = q.into_array() {
                    if question_arr.len() == 0 || question_arr.len() > 4 {
                        Err("Invalid Question Array: Len > 0!".into())
                    }
                    if question_arr.len() >= 1 {
                        question.text = match question_arr
                            .get(0)
                            .map(|x| x.into_immutable_string().ok())
                            .flatten()
                        {
                            Some(t) => t.into(),
                            None => return Err("Invalid Text!".into()),
                        }
                    } else if question_arr.len() >= 2 {
                        question.character = match question_arr
                            .get(1)
                            .map(|x| x.into_immutable_string().ok())
                            .flatten()
                        {
                            Some(t) => {
                                if !characters.contains_key(&t) {
                                    // TODO: Add default non-existant characters
                                    return Err("Invalid Character!".into());
                                }
                                Some(t.into())
                            }
                            None => return Err("Invalid Character!".into()),
                        }
                    } else if question_arr.len() >= 3 {
                        question.emotion = match question_arr
                            .get(2)
                            .map(|x| x.into_immutable_string().ok())
                            .flatten()
                        {
                            Some(t) => Some(t.into()),
                            None => return Err("Invalid Emotion!".into()),
                        }
                    } else if question_arr.len() >= 4 {
                        question.extra = match question_arr.get(3) {
                            Some(t) => Some(Value::from(t)),
                            None => return Err("Invalid Text!".into()),
                        }
                    }
                } else if let Some(map) = q.read_lock::<Map>() {
                    if map.is_empty() {
                        Err("Invalid Question Object: Len > 0!".into())
                    }

                    question.text =
                        match map.get("text").unwrap_or_default().into_immutable_string() {
                            Ok(txt) => txt,
                            Err(why) => {}
                        };
                }
            };

            // see if it

            let mut options =
                HashMap::with_capacity_and_hasher(counts as usize, RandomState::new());
            for (_, value) in state.read_lock::<Map>().unwrap() {
                if let Ok(lenarr) = value.into_typed_array::<u32>() {
                    let keyed = context.eval_expression_tree(&inputs[lenarr[0] as usize])?;
                    let block_run = &inputs[lenarr[1] as usize];
                    options.insert(keyed, block_run);
                }
            }

            event_sender.clone().send(EventMessage::Choice());

            Ok(Dynamic::UNIT)
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
                    _ => return,
                };
                script_data.write().traversals.add(fn_name.into());
            }

            Ok(DebuggerCommand::Continue)
        },
    );

    // engine
    //     .run_ast(
    //         scripts
    //             .get(&script)
    //             .ok_or(BongTalkError::ReaderInit(format!(
    //                 "Script {script} doesn't exist."
    //             )))?
    //             .value(),
    //     )
    //     .map_err(|why| BongTalkError::Script(script, why.to_string()))?;

    Ok(engine)
}
