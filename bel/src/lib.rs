pub mod environment;
pub mod error;
mod function;
pub mod parser;

use std::fmt;

/// Bel has four fundamental data types:
/// symbols, pairs, characters, and streams.
/// Instances of the four fundamental types are called objects
/// https://sep.yimg.com/ty/cdn/paulgraham/bellanguage.txt
#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Symbol(String),
    List(Vec<Object>),
    Char(String),
    Stream,
}

impl Object {}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Object::*;
        match &self {
            Symbol(word) => write!(f, "{}", word),
            List(l) => write!(f, "{:?}", l),
            Char(c) => write!(f, "\\{}", c),
            Stream => write!(f, "<stream>"),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use error::BelError;

    #[test]
    fn some_objects_evaluate_to_themselves() -> Result<(), BelError> {
        Ok(())
    }
}
