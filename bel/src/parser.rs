use crate::Object;
use nom::{IResult, Finish, };
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("parse error: {0}")]
    Error(String),
}

pub fn parse(input: &str) -> Result<Object, ParseError> {
    object(input)
        .finish()
        .map(|(_, o)| o)
        .map_err(|e| ParseError::Error(format!("{:?}", e)))
}

fn object(input: &str) -> IResult<&str, Object> {
    Err(nom::Err::Incomplete(nom::Needed::new(2)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_fails() {
        assert!(parse("x").is_err());
    }

    #[test]
    fn can_parse_predefined() {
        for &s in &["t", "nil", "o", "apply"] {
            match parse(s) {
                Ok(o) => {
                    match o {
                        Object::Symbol(x) => assert_eq!(x, s),
                        _ => panic!("invalid Object: {:?}", o)
                    }
                },
                Err(e) => panic!("{:?}", e)
            }
        }
    }
}
