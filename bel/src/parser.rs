use super::object::Object;
use failure::{format_err, Error};

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

    pub fn parse(&self, text: &str) -> Result<(Object, &str), Error> {
        let mut depth = 0;
        let mut content: String = String::new();
        for (i, c) in text.chars().enumerate() {
            match c {
                '(' => {
                    if depth > 0 {
                        return Err(format_err!("cannot handle nested ')' yet at {}", i));
                    }
                    depth += 1
                }
                ')' => {
                    if depth < 1 {
                        return Err(format_err!("unmatched ')' at {}", i));
                    }
                    depth -= 1;
                    let pieces = content.split_whitespace().collect::<Vec<_>>();
                    if pieces.len() == 0 {
                        return Ok((Object::Nil, ""));
                    }
                    return Err(format_err!("too much content: '{:?}'", pieces));
                }
                _ => content.push(c),
            }
        }
        Err(format_err!("not implemented"))
    }
}
