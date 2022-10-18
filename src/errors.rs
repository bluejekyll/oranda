use miette::Diagnostic;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, OrandaError>;

#[derive(Debug, Error, Diagnostic)]
pub enum OrandaError {
    #[error("{0}")]
    Config(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Toml(#[from] toml::de::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Syntect(#[from] syntect::Error),

    #[error(transparent)]
    Grass(#[from] Box<grass::Error>),

    #[error("{0}")]
    Other(String),
    // TODO: at some context fields / miette stuff
}