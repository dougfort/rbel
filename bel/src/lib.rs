pub mod parser;
use thiserror::Error;
use std::fmt;

#[derive(Error, Debug)]
pub enum BelError {
    #[error("bel error")]
    Error, 
}

/// Bel has four fundamental data types:
/// symbols, pairs, characters, and streams.
/// Instances of the four fundamental types are called objects
/// https://sep.yimg.com/ty/cdn/paulgraham/bellanguage.txt
#[derive(Debug)]
pub enum Object {
    Symbol(String),
    Pair(Box<(Object, Object)>),
    Char(u8),
    Stream,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Object::*;
        match &self {
            Symbol(word) => write!(f, "{}", word),
            Pair(pair) => {
                let unboxed_pair = &*pair;
                write!(f, "({}, {})", unboxed_pair.0, unboxed_pair.1)
            }
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
