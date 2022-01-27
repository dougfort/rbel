use std::default;

use crate::error::BelError;
use crate::Object;

enum State {
    ConsumeWhitespace,
    BuildSymbol,
    BuildChar,
}

pub struct Parser {
    level: usize,
    accum: String,
    list_stack: Vec<Vec<Object>>,
    state: State,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            level: 0,
            accum: String::new(),
            list_stack: Vec::new(),
            state: State::ConsumeWhitespace,
        }
    }

    pub fn parse(&mut self, input: &str) -> Result<Vec<Object>, BelError> {
        self.level = 0;
        self.accum = String::new();
        // start with an outer list whether we need it or not
        self.list_stack = vec![Vec::new()];
        self.state = State::ConsumeWhitespace;

        for c in input.chars() {
            match c {
                '(' => {
                    self.finish_build();
                    self.start_level();
                }
                ')' => {
                    self.finish_build();
                    self.finish_level();
                }
                '\\' => {
                    self.accum.clear();
                    self.state = State::BuildChar;
                }
                _ => match self.state {
                    State::BuildSymbol if c.is_whitespace() => {
                        self.list_stack[self.level].push(Object::Symbol(self.accum.clone()));
                        self.state = State::ConsumeWhitespace;
                    }
                    State::BuildChar if c.is_whitespace() => {
                        self.list_stack[self.level].push(Object::Char(self.accum.clone()));
                        self.state = State::ConsumeWhitespace;
                    }
                    State::ConsumeWhitespace if c.is_whitespace() => {}
                    State::ConsumeWhitespace => {
                        self.accum.clear();
                        self.accum.push(c);
                        self.state = State::BuildSymbol;
                    }
                    _ => {
                        self.accum.push(c);
                    }
                },
            }
        }
        if self.level > 0 {
            Err(BelError::ParseError(format!(
                "invalid level: {}",
                self.level
            )))
        } else {
            self.finish_build();
            Ok(self.list_stack[0].clone())
        }
    }

    fn finish_build(&mut self) {
        match self.state {
            State::BuildSymbol => {
                self.list_stack[self.level].push(Object::Symbol(self.accum.clone()));
            }
            State::BuildChar => {
                self.list_stack[self.level].push(Object::Char(self.accum.clone()));
            }
            _ => {}
        }
    }

    fn start_level(&mut self) {
        self.list_stack.push(Vec::<Object>::new());
        self.level += 1;
        self.state = State::ConsumeWhitespace;
    }

    fn finish_level(&mut self) {
        let list = self.list_stack.pop().unwrap();
        self.level -= 1;
        self.list_stack[self.level].push(Object::List(list));
        self.state = State::ConsumeWhitespace;
    }
}

impl default::Default for Parser {
    fn default() -> Self {
        Parser::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_empty_string() -> Result<(), BelError> {
        let mut parser = Parser::new();
        assert_eq!(parser.parse("")?, vec![]);

        Ok(())
    }

    #[test]
    fn can_parse_symbol() -> Result<(), BelError> {
        let mut parser = Parser::new();
        assert_eq!(parser.parse("a")?, vec![Object::Symbol("a".to_string())]);

        Ok(())
    }

    #[test]
    fn can_parse_empty_list() -> Result<(), BelError> {
        let mut parser = Parser::new();
        let mut res = parser.parse("()")?;
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
        let mut parser = Parser::new();
        let mut res = parser.parse("(a b)")?;
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
        let mut parser = Parser::new();
        let mut res = parser.parse("((x))")?;
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
        let mut parser = Parser::new();
        let mut res = parser.parse("(a b (c))")?;
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
