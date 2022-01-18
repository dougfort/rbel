use super::object::Object;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("parse error")]
    Error,
}

#[derive(Default)]
pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

    pub fn parse(&self, text: &str) -> Result<Object, ParseError> {
        let mut depth = 0;
        let mut content: String = String::new();
        for (i, c) in text.chars().enumerate() {
            match c {
                '(' => {
                    if depth > 0 {
                        return Err(ParseError::Error);
                    }
                    depth += 1
                }
                ')' => {
                    if depth < 1 {
                        return Err(ParseError::Error);
                    }
                    depth -= 1;
                    let pieces = content.split_whitespace().collect::<Vec<_>>();
                    if pieces.is_empty() {
                        return Ok(Object::Nil);
                    }
                    return Err(ParseError::Error);
                }
                _ => content.push(c),
            }
        }
        Err(ParseError::Error)
    }
}
