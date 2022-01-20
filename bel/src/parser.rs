use crate::Object;
use nom::{IResult, Finish};
use nom::sequence::preceded;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("parse error: {0}")]
    Error(String),
}

pub fn parse(input: &str) -> Result<Object, ParseError> {
    let input = input.trim();
    match char(input) {
        Ok(res) => println!("{:?}", res),
        Err(e) =>  println!("{:?}", e),
    }
    Err(ParseError::Error("not implements".to_string()))
}

fn char(input: &str) -> IResult<&str, &str> {
    Ok(("", ""))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_fails() {
        assert!(parse("x").is_err());
    }

    #[test]
    fn can_parse_char() {
        match parse("\\a") {
            Ok(obj) => {
                if let Object::Char(c) = obj {
                    assert_eq!(c, &[b'a']);
                } else {
                    panic!("unknown object: {:?}", obj)
                }                
            },
            Err(e) => panic!("{:?}", e),
        }
    }
}
