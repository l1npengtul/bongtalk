use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum BongTalkError {


}

pub type BResult<T> = Result<T, BongTalkError>; 