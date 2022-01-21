pub mod parser;
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BelError {
    #[error("bel error")]
    Error,
}

/// Bel has four fundamental data types:
/// symbols, pairs, characters, and streams.
/// Instances of the four fundamental types are called objects
/// https://sep.yimg.com/ty/cdn/paulgraham/bellanguage.txt
#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Nil,
    Symbol(String),
    List(Vec<Object>),
    Char(String),
    Stream,
}

impl Object {
    pub fn is_nil(&self) -> bool {
        self == &Object::Nil
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Object::*;
        match &self {
            Nil => write!(f, "nil"),
            Symbol(word) => write!(f, "{}", word),
            List(l) => write!(f, "{:?}", l),
            Char(c) => write!(f, "\\{}", c),
            Stream => write!(f, "<stream>"),
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
