use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum BongTalkError {
    #[error("Error initializing Rhai Engine")]
    EngineInit(#[from] dyn std::error::Error),
}

pub type BResult<T> = Result<T, BongTalkError>;
