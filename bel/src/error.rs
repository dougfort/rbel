use thiserror::Error;

#[derive(Error, Debug)]
pub enum BelError {
    #[error("not implemented: {0}")]
    NotImplemented(String),

    #[error("parse error: {0}")]
    ParseError(String),

    #[error("unbound symbol: {0}")]
    UnboundSymbol(String),
}
