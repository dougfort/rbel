use thiserror::Error;

#[derive(Error, Debug)]
pub enum BelError {
    #[error("not implemented: {0}")]
    NotImplemented(String),

    #[error("parse error: {0}")]
    ParseError(String),

    #[error("unbound symbol: {0}")]
    UnboundSymbol(String),

    #[error("invalid object: expected {expected}; found {found}.")]
    InvalidObject { expected: String, found: String },

    #[error("invalid quote: {0}")]
    InvalidQuote(String),

    #[error("invalid def: {0}")]
    InvalidDef(String),
}
