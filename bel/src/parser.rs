use crate::error::BelError;
use crate::Object;

enum State {
    ConsumeWhitespace,
    BuildSymbol,
    BuildChar,
}

pub fn parse(input: &str) -> Result<Vec<Object>, BelError> {
    let mut level = 0;
    let mut accum = String::new();
    // start with an outer list whether we need it or not
    let mut list_stack: Vec<Vec<Object>> = vec![Vec::new()];
    let mut state = State::ConsumeWhitespace;
    for c in input.chars() {
        match c {
            '(' => {
                match state {
                    State::BuildSymbol => {
                        list_stack[level].push(Object::Symbol(accum.clone()));
                    }
                    State::BuildChar => {
                        list_stack[level].push(Object::Char(accum.clone()));
                    }
                    _ => {}
                }
                state = State::ConsumeWhitespace;
                list_stack.push(Vec::<Object>::new());
                level += 1;
            }
            ')' => {
                match state {
                    State::BuildSymbol => {
                        list_stack[level].push(Object::Symbol(accum.clone()));
                    }
                    State::BuildChar => {
                        list_stack[level].push(Object::Char(accum.clone()));
                    }
                    _ => {}
                }
                let list = list_stack.pop().unwrap();
                level -= 1;
                list_stack[level].push(Object::List(list));
                state = State::ConsumeWhitespace;
            }
            '\\' => {
                accum.clear();
                state = State::BuildChar;
            }
            _ => match state {
                State::BuildSymbol if c.is_whitespace() => {
                    list_stack[level].push(Object::Symbol(accum.clone()));
                    state = State::ConsumeWhitespace;
                }
                State::BuildChar if c.is_whitespace() => {
                    list_stack[level].push(Object::Char(accum.clone()));
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
        // println!(
        //     "c = '{}', accum = '{}', (level, len) = ({}, {}), list_stack = {:?}",
        //     c,
        //     accum,
        //     level,
        //     list_stack.len(),
        //     list_stack
        // );
    }
    if level > 0 {
        Err(BelError::ParseError(format!("invalid level: {}", level)))
    } else {
        // println!(
        //     "final, accum = '{}', (level, len) = ({}, {}), list_stack = {:?}",
        //     accum,
        //     level,
        //     list_stack.len(),
        //     list_stack
        // );
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
    fn can_parse_empty_string() -> Result<(), BelError> {
        assert_eq!(parse("")?, vec![]);

        Ok(())
    }

    #[test]
    fn can_parse_symbol() -> Result<(), BelError> {
        assert_eq!(parse("a")?, vec![Object::Symbol("a".to_string())]);

        Ok(())
    }

    #[test]
    fn can_parse_empty_list() -> Result<(), BelError> {
        let mut res = parse("()")?;
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

        Ok(())
    }

    #[test]
    fn can_parse_list_of_symbols() -> Result<(), BelError> {
        let mut res = parse("(a b)")?;
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

        Ok(())
    }

    #[test]
    fn can_parse_embedded_list() -> Result<(), BelError> {
        let mut res = parse("((x))")?;
        match res.pop() {
            Some(object) => {
                if let Object::List(l) = object {
                    assert_eq!(l, vec![Object::List(vec![Object::Symbol("x".to_string())])]);
                } else {
                    panic!("unexpected return type: {:?}", res);
                }
            }
            None => {
                panic!("unexpected empty list: {:?}", res);
            }
        }

        Ok(())
    }

    #[test]
    fn can_parse_embedded_list_of_symbols() -> Result<(), BelError> {
        let mut res = parse("(a b (c))")?;
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

        Ok(())
    }
}
