use crate::Object;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("parse error: {0}")]
    Error(String),
}

enum State {
    ConsumeWhitespace,
    BuildSymbol,
    BuildChar,
}

pub fn parse(input: &str) -> Result<Vec<Object>, ParseError> {
    let mut level = 0;
    let mut accum = String::new();
    // start with an outer list whether we need it or not
    let mut list_stack: Vec<Vec<Object>> = vec![Vec::new()];
    let mut state = State::ConsumeWhitespace;
    let mut current_list: Vec<Object> = Vec::new();
    for c in input.chars() {
        match c {
            '(' => {
                match state {
                    State::BuildSymbol => {
                        current_list.push(Object::Symbol(accum.clone()));
                    }
                    State::BuildChar => {
                        current_list.push(Object::Char(accum.clone()));
                    }
                    _ => {}
                }
                list_stack.push(current_list);
                current_list = Vec::new();
                state = State::ConsumeWhitespace;
                level += 1;
            }
            ')' => {
                match state {
                    State::BuildSymbol => {
                        current_list.push(Object::Symbol(accum.clone()));
                    }
                    State::BuildChar => {
                        current_list.push(Object::Char(accum.clone()));
                    }
                    _ => {}
                }
                level -= 1;
                list_stack[level].push(Object::List(current_list));
                list_stack.pop();
                current_list = Vec::new();
                state = State::ConsumeWhitespace;
            }
            '\\' => {
                accum.clear();
                state = State::BuildChar;
            }
            _ => match state {
                State::BuildSymbol if c.is_whitespace() => {
                    current_list.push(Object::Symbol(accum.clone()));
                    state = State::ConsumeWhitespace;
                }
                State::BuildChar if c.is_whitespace() => {
                    current_list.push(Object::Char(accum.clone()));
                    state = State::ConsumeWhitespace;
                }
                State::ConsumeWhitespace if c.is_whitespace() => {}
                State::ConsumeWhitespace => {
                    accum.clear();
                    accum.push(c);
                    state = State::BuildSymbol;
                }
                _ => {
                    accum.push(c);
                }
            },
        }
        println!(
            "c = '{}', accum = '{}', current_list = {:?}, (level, len) = ({}, {}), list_stack = {:?}", 
            c, accum, current_list, level, list_stack.len(), list_stack
        );
    }
    if level > 0 {
        Err(ParseError::Error(format!("invalid level: {}", level)))
    } else {
        println!(
            "final, accum = '{}', current_list = {:?}, (level, len) = ({}, {}), list_stack = {:?}",
            accum,
            current_list,
            level,
            list_stack.len(),
            list_stack
        );
        match state {
            State::BuildSymbol => {
                list_stack[0].push(Object::Symbol(accum.clone()));
            }
            State::BuildChar => {
                list_stack[0].push(Object::Char(accum.clone()));
            }
            _ => {}
        }
        Ok(list_stack[0].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_empty_string() {
        assert_eq!(parse("").unwrap(), vec![]);
    }

    #[test]
    fn can_parse_symbol() {
        assert_eq!(parse("a").unwrap(), vec![Object::Symbol("a".to_string())]);
    }

    #[test]
    fn can_parse_empty_list() {
        let mut res = parse("()").unwrap();
        match res.pop() {
            Some(object) => {
                if let Object::List(l) = object {
                    assert!(l.is_empty());
                } else {
                    panic!("unexpected return type: {:?}", res);
                }
            }
            None => {
                panic!("unexpected empty list: {:?}", res);
            }
        }
    }

    #[test]
    fn can_parse_list_of_symbols() {
        let mut res = parse("(a b)").unwrap();
        match res.pop() {
            Some(object) => {
                if let Object::List(l) = object {
                    assert_eq!(
                        l,
                        vec![
                            Object::Symbol("a".to_string()),
                            Object::Symbol("b".to_string())
                        ]
                    );
                } else {
                    panic!("unexpected return type: {:?}", res);
                }
            }
            None => {
                panic!("unexpected empty list: {:?}", res);
            }
        }
    }

    #[test]
    fn can_parse_embedded_list() {
        let mut res = parse("((x))").unwrap();
        match res.pop() {
            Some(object) => {
                if let Object::List(l) = object {
                    assert_eq!(l, vec![Object::List(vec![Object::Symbol("c".to_string())])]);
                } else {
                    panic!("unexpected return type: {:?}", res);
                }
            }
            None => {
                panic!("unexpected empty list: {:?}", res);
            }
        }
    }

    #[test]
    fn can_parse_embedded_list_of_symbols() {
        let mut res = parse("(a b (c))").unwrap();
        match res.pop() {
            Some(object) => {
                if let Object::List(l) = object {
                    assert_eq!(
                        l,
                        vec![
                            Object::Symbol("a".to_string()),
                            Object::Symbol("b".to_string()),
                            Object::List(vec![Object::Symbol("c".to_string())]),
                        ]
                    );
                } else {
                    panic!("unexpected return type: {:?}", res);
                }
            }
            None => {
                panic!("unexpected empty list: {:?}", res);
            }
        }
    }
}
