use crate::character::CharacterRef;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum BongTalkError {
    #[error("Error initializing Rhai Engine")]
    EngineInit(#[from] dyn std::error::Error),
    #[error("Failed to compile script: {0}")]
    Compile(String),
    #[error("Failed to start reader: {0}")]
    ReaderInit(String),
    #[error("Failed to run script {0}, {1}")]
    Script(String, String),
    #[error("Character Reference {0} is invalid.")]
    InvalidCharacterReference(CharacterRef),
}

pub type BResult<T> = Result<T, BongTalkError>;
