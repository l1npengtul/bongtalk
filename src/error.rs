use crate::character::CharacterRef;
use csscolorparser::ParseColorError;
use roxmltree::Error;
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
    #[error("XML Document Parsing Error: {0}.")]
    XmlParsingError(quick_xml::Error),
    #[error("XML Document: Not UTF-8")]
    XmlEncodingError,
    #[error("XML Document Error: {0}")]
    XmlError(String),
}

impl From<quick_xml::Error> for BongTalkError {
    fn from(value: quick_xml::Error) -> Self {
        BongTalkError::XmlParsingError(value)
    }
}

impl From<ParseColorError> for BongTalkError {
    fn from(value: ParseColorError) -> Self {
        BongTalkError::XmlError(format!("Bad Color: {}", value.to_string()))
    }
}

pub type BResult<T> = Result<T, BongTalkError>;
