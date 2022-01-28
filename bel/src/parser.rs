use std::default;

use crate::error::BelError;
use crate::Object;

enum State {
    ConsumeWhitespace,
    BuildSymbol,
    BuildChar,
}

enum QuotingState {
    None,
    Starting,
    Atom,
    List,
}

pub struct Parser {
    level: usize,
    accum: String,
    list_stack: Vec<Vec<Object>>,
    state: State,
    quoting_state: QuotingState,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            level: 0,
            accum: String::new(),
            list_stack: Vec::new(),
            state: State::ConsumeWhitespace,
            quoting_state: QuotingState::None,
        }
    }

    pub fn parse(&mut self, input: &str) -> Result<Object, BelError> {
        self.level = 0;
        self.accum = String::new();
        // start with an outer list whether we need it or not
        self.list_stack = vec![Vec::new()];
        self.state = State::ConsumeWhitespace;
        self.quoting_state = QuotingState::None;

        for c in input.chars() {
            match c {
                '(' => {
                    self.finish_build();
                    self.start_level();
                }
                ')' => {
                    self.finish_build();
                    if let QuotingState::List = self.quoting_state {
                        self.finish_level();
                        self.quoting_state = QuotingState::None;
                    }; 
                    self.finish_level();
                }
                '\'' => {
                    self.finish_build();
                    self.start_level();
                    self.list_stack[self.level].push(Object::Symbol("quote".to_string()));
                    self.quoting_state = QuotingState::Starting; 
                }
                '\\' => {
                    self.finish_build();
                    self.accum.clear();
                    self.state = State::BuildChar;
                    if let QuotingState::Starting = self.quoting_state {
                        self.quoting_state = QuotingState::Atom;
                    };
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
                        if let QuotingState::Starting = self.quoting_state {
                            self.quoting_state = QuotingState::Atom;
                        };
                    },
                    _ => {
                        self.accum.push(c);
                    }
                },
            }
        }

        self.finish_build();


        if self.level > 0 {
            Err(BelError::ParseError(format!(
                "invalid level: {}",
                self.level
            )))
        } else {
            let parse_list = self.list_stack[0].clone();

            // if the parser result is a single Object return that
            // otherwise return an Object::List of the result
            let obj = match parse_list.len() {
                0 => Object::Symbol("nil".to_string()), 
                1 => parse_list[0].clone(),
                _ => Object::List(parse_list),
            };
            Ok(obj)
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
        };
        if let QuotingState::Atom = self.quoting_state {
            self.finish_level();
            self.quoting_state = QuotingState::None;
        }; 
    }

    fn start_level(&mut self) {
        self.list_stack.push(Vec::<Object>::new());
        self.level += 1;
        self.state = State::ConsumeWhitespace;
        if let QuotingState::Starting = self.quoting_state {
            self.quoting_state = QuotingState::List;
        };
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
        assert!(parser.parse("")?.is_nil());

        Ok(())
    }

    #[test]
    fn can_parse_symbol() -> Result<(), BelError> {
        let mut parser = Parser::new();
        assert_eq!(parser.parse("a")?, Object::Symbol("a".to_string()));

        Ok(())
    }

    #[test]
    fn can_parse_empty_list() -> Result<(), BelError> {
        let mut parser = Parser::new();
        let parse_obj = parser.parse("()")?;
        if let Object::List(l) = parse_obj {
            assert!(l.is_empty());
        } else {
            panic!("unexpected return type: {:?}", parse_obj);
        }

        Ok(())
    }

    #[test]
    fn can_parse_list_of_symbols() -> Result<(), BelError> {
        let mut parser = Parser::new();
        let parse_obj = parser.parse("(a b)")?;
        if let Object::List(l) = parse_obj {
            assert_eq!(
                l,
                vec![
                    Object::Symbol("a".to_string()),
                    Object::Symbol("b".to_string())
                ]
            );
        } else {
            panic!("unexpected return type: {:?}", parse_obj);
        }

        Ok(())
    }

    #[test]
    fn can_parse_embedded_list() -> Result<(), BelError> {
        let mut parser = Parser::new();
        let parse_obj = parser.parse("((x))")?;
        if let Object::List(l) = parse_obj {
            assert_eq!(l, vec![Object::List(vec![Object::Symbol("x".to_string())])]);
        } else {
            panic!("unexpected return type: {:?}", parse_obj);
        }

        Ok(())
    }

    #[test]
    fn can_parse_embedded_list_of_symbols() -> Result<(), BelError> {
        let mut parser = Parser::new();
        let parse_obj = parser.parse("(a b (c))")?;
        if let Object::List(l) = parse_obj {
            assert_eq!(
                l,
                vec![
                    Object::Symbol("a".to_string()),
                    Object::Symbol("b".to_string()),
                    Object::List(vec![Object::Symbol("c".to_string())]),
                ]
            );
        } else {
            panic!("unexpected return type: {:?}", parse_obj);
        }

        Ok(())
    }

    #[test]
    fn can_parse_quoted_symbol() -> Result<(), BelError> {
        let mut parser = Parser::new();
        let parse_obj = parser.parse("'a")?;
        if let Object::List(l) = parse_obj {
            assert_eq!(
                l,
                vec![
                    Object::Symbol("quote".to_string()),
                    Object::Symbol("a".to_string()),
                ]
            );
        } else {
            panic!("unexpected return type: {:?}", parse_obj);
        }

        Ok(())
    }
    #[test]
    fn can_parse_quoted_list() -> Result<(), BelError> {
        let mut parser = Parser::new();
        let parse_obj = parser.parse("'(a)")?;
        if let Object::List(l) = parse_obj {
            assert_eq!(
                l,
                vec![
                    Object::Symbol("quote".to_string()),
                    Object::List(vec![Object::Symbol("a".to_string())]),
                ]
            );
        } else {
            panic!("unexpected return type: {:?}", parse_obj);
        }

        Ok(())
    }
}
