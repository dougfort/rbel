use std::fmt;

/// Bel has four fundamental data types:
/// symbols, pairs, characters, and streams.
/// Instances of the four fundamental types are called objects
/// https://sep.yimg.com/ty/cdn/paulgraham/bellanguage.txt
#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Symbol(String),
    Pair(Box<(Object, Object)>),
    List(Vec<Object>),
    Char(String),
    Stream,
}

impl Object {
    pub fn t(&self) -> String {
        match &self {
            Object::Symbol(_) => "symbol".to_string(),
            Object::Pair(_) => "pair".to_string(),
            Object::List(_) => "list".to_string(),
            Object::Char(_) => "char".to_string(),
            Object::Stream => "stream".to_string(),
        }
    }

    pub fn is_nil(&self) -> bool {
        if let Object::Symbol(name) = self {
            name == "nil"
        } else {
            false
        }
    }

    pub fn is_true(&self) -> bool {
        if let Object::Symbol(name) = self {
            name == "t"
        } else {
            false
        }
    }

    pub fn is_quote(&self) -> bool {
        if let Object::Symbol(name) = self {
            name == "quote"
        } else {
            false
        }
    }

    fn is_literal(&self) -> bool {
        if let Object::Symbol(name) = self {
            name == "lit"
        } else {
            false
        }
    }

    fn is_closure(&self) -> bool {
        if let Object::Symbol(name) = self {
            name == "clo"
        } else {
            false
        }
    }

    pub fn is_function(&self) -> bool {
        if let Object::List(list) = self {
            //  (set n (lit clo nil p e))
            if list.len() != 5 {
                return false;
            }
            if !list[0].is_literal() {
                return false;
            }
            return list[1].is_closure();
        }

        false
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Object::*;
        match &self {
            Symbol(word) => write!(f, "{}", word),
            Pair(pair) => write!(f, "({} . {})", pair.0, pair.1),
            List(l) => write!(f, "{:?}", l),
            Char(c) => write!(f, "\\{}", c),
            Stream => write!(f, "<stream>"),
        }
    }
}
