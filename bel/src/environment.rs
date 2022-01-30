use crate::error::BelError;
use crate::Object;
use std::collections::HashMap;

#[derive(Default)]
pub struct Environment {
    pub global: HashMap<String, Object>,
    primatives: HashMap<String, EnvFunc>,
}

type EnvFunc = fn(&mut Environment, &[Object]) -> Result<Object, BelError>;

impl Environment {
    pub fn new() -> Self {
        let mut env = Environment {
            global: HashMap::new(),
            primatives: HashMap::new(),
        };

        // some Symbols bind to themselves
        for name in vec![
            "nil".to_string(),
            "t".to_string(),
            "o".to_string(),
            "apply".to_string(),
        ] {
            env.global.insert(name.clone(), Object::Symbol(name));
        }

        env.primatives.insert("id".to_string(), id);

        env
    }

    // Return an object that is reduced to its lowest terms
    pub fn evaluate(&mut self, obj: &Object) -> Result<Object, BelError> {
        let output = match obj {
            Object::Symbol(name) => self.get_global(name)?,
            Object::List(list) => {
                if list.is_empty() {
                    return Ok(Object::Symbol("nil".to_string()));
                }
                // if this list starts with a symbol, it may be 'special'
                if let Object::Symbol(name) = list[0].clone() {
                    match name.as_ref() {
                        "set" => {
                            return self.set(&list[1..]);
                        }
                        "def" => {
                            return self.def(&list[1..]);
                        }
                        "quote" => {
                            return self.quote(&list[1..]);
                        }
                        n if self.primatives.contains_key(n) => {
                            let evaluated_list = self.evaluate_list(&list[1..])?;
                            return self.primatives[n](self, &evaluated_list);
                        }
                        _ => {
                            // if the leading symbol refers to a function,
                            // we apply the function
                            let obj = self.get_global(&name)?;
                            if obj.is_function() {
                                let evaluated_list = self.evaluate_list(&list[1..])?;
                                return self.apply(&obj, &evaluated_list);
                            }
                        }
                    }
                }
                let evaluated_list = self.evaluate_list(&list[1..])?;
                Object::List(evaluated_list)
            }
            Object::Char(_c) => {
                return Err(BelError::NotImplemented("Object::Char".to_string()));
            }
            Object::Stream => return Err(BelError::NotImplemented("Object::Stream".to_string())),
        };

        Ok(output)
    }

    fn apply(&mut self, _fn_obj: &Object, _args: &[Object]) -> Result<Object, BelError> {
        Err(BelError::NotImplemented("apply".to_string()))
    }

    fn evaluate_list(&mut self, list: &[Object]) -> Result<Vec<Object>, BelError> {
        let mut evaluated_list = Vec::new();
        for item in list {
            let eval_item = self.evaluate(item)?;
            evaluated_list.push(eval_item);
        }
        Ok(evaluated_list)
    }

    fn get_global(&self, name: &str) -> Result<Object, BelError> {
        match self.global.get(name) {
            Some(obj) => Ok(obj.clone()),
            None => Err(BelError::UnboundSymbol(name.to_string())),
        }
    }

    fn set(&mut self, list: &[Object]) -> Result<Object, BelError> {
        for i in 0..list.len() - 1 {
            if let Object::Symbol(key) = list[i].clone() {
                self.global.insert(key, list[i + 1].clone());
            } else {
                return Err(BelError::InvalidObject {
                    expected: "symbol".to_string(),
                    found: list[i].t(),
                });
            }
        }
        // append nil if the final arg isn't present
        // an odd number of entries
        // means the last value is unspecified
        if list.len() % 2 == 1 {
            let i = list.len() - 1;
            if let Object::Symbol(key) = list[i].clone() {
                self.global.insert(key, Object::Symbol("nil".to_string()));
            } else {
                return Err(BelError::InvalidObject {
                    expected: "symbol".to_string(),
                    found: list[i].t(),
                });
            }
        }
        Ok(Object::Symbol("nil".to_string()))
    }

    // When you see
    //  (def n p e)
    // treat it as an abbreviation for
    //  (set n (lit clo nil p e))
    fn def(&mut self, list: &[Object]) -> Result<Object, BelError> {
        if list.len() == 3 {
            let name = list[0].clone();

            let p = list[1].clone();
            // we want the parameters to be a list
            let parameters = if p.is_nil() {
                Object::List(vec![])
            } else if let Object::List(_) = p {
                p
            } else {
                Object::List(vec![p])
            };

            let e = list[2].clone();
            let body = Object::List(vec![
                Object::Symbol("lit".to_string()),
                Object::Symbol("clo".to_string()),
                Object::Symbol("nil".to_string()),
                parameters,
                e,
            ]);
            self.set(&[name, body])
        } else {
            Err(BelError::InvalidDef(format!("{:?}", list)))
        }
    }

    fn quote(&self, list: &[Object]) -> Result<Object, BelError> {
        if list.len() == 1 {
            // return the inner object without evaluating
            Ok(list[0].clone())
        } else {
            Err(BelError::InvalidQuote(format!("{:?}", list)))
        }
    }
}

fn id(_env: &mut Environment, params: &[Object]) -> Result<Object, BelError> {
    // id is true if
    // * there are two arguments
    // * they are both symbols
    // they have the same name
    let mut result = Object::Symbol("nil".to_string());
    if params.len() == 2 {
        if let Object::Symbol(lhs) = &params[0] {
            if let Object::Symbol(rhs) = &params[1] {
                if lhs == rhs {
                    result = Object::Symbol("t".to_string());
                }
            }
        }
    }

    Ok(result)
}
#[cfg(test)]
mod tests {

    use super::*;
    use crate::parser;

    #[test]
    fn some_objects_evaluate_to_themselves() -> Result<(), BelError> {
        let mut env = Environment::new();
        for obj in vec![
            Object::Symbol("nil".to_string()),
            Object::Symbol("t".to_string()),
            Object::Symbol("o".to_string()),
            Object::Symbol("apply".to_string()),
        ] {
            let res = env.evaluate(&obj)?;
            assert_eq!(res, obj);
        }
        assert_eq!(2 + 2, 4);

        Ok(())
    }

    #[test]
    fn can_set_global() -> Result<(), BelError> {
        let mut parser = parser::Parser::new();
        let mut env = Environment::new();

        let parse_obj = parser.parse("(set a b)")?;
        let obj = env.evaluate(&parse_obj)?;
        assert!(obj.is_nil());

        let parse_obj = parser.parse("a")?;
        let obj = env.evaluate(&parse_obj)?;
        if let Object::Symbol(s) = obj {
            assert_eq!(s, "b");
        } else {
            panic!("unexpected object {:?}", obj);
        }

        Ok(())
    }

    #[test]
    fn can_set_multiple() -> Result<(), BelError> {
        let mut parser = parser::Parser::new();
        let mut env = Environment::new();

        let parse_obj = parser.parse("(set a b c d e f)")?;
        let obj = env.evaluate(&parse_obj)?;
        assert!(obj.is_nil());

        for (key, val) in &[
            ("a", "b".to_string()),
            ("c", "d".to_string()),
            ("e", "f".to_string()),
        ] {
            let parse_obj = parser.parse(key)?;
            let obj = env.evaluate(&parse_obj)?;
            if let Object::Symbol(s) = obj {
                assert_eq!(&s, val);
            } else {
                panic!("unexpected object {:?}", obj);
            }
        }

        Ok(())
    }

    #[test]
    fn can_set_multiple_with_default() -> Result<(), BelError> {
        let mut parser = parser::Parser::new();
        let mut env = Environment::new();

        let parse_obj = parser.parse("(set a b c d e)")?;
        let obj = env.evaluate(&parse_obj)?;
        assert!(obj.is_nil());

        for (key, val) in &[
            ("a", "b".to_string()),
            ("c", "d".to_string()),
            ("e", "nil".to_string()),
        ] {
            let parse_obj = parser.parse(key)?;
            let obj = env.evaluate(&parse_obj)?;
            if let Object::Symbol(s) = obj {
                assert_eq!(&s, val);
            } else {
                panic!("unexpected object {:?}", obj);
            }
        }

        Ok(())
    }

    #[test]
    fn can_quote_symbol() -> Result<(), BelError> {
        let mut parser = parser::Parser::new();
        let mut env = Environment::new();

        let parse_obj = parser.parse("(set a b)")?;
        let obj = env.evaluate(&parse_obj)?;
        assert!(obj.is_nil());

        let parse_obj = parser.parse("(quote a)")?;
        let obj = env.evaluate(&parse_obj)?;
        if let Object::Symbol(s) = obj {
            assert_eq!(s, "a");
        } else {
            panic!("unexpected object {:?}", obj);
        }

        Ok(())
    }

    #[test]
    fn id_checks_for_identity() -> Result<(), BelError> {
        let mut parser = parser::Parser::new();
        let mut env = Environment::new();

        let parse_obj = parser.parse("(id 'a 'a)")?;
        let obj = env.evaluate(&parse_obj)?;
        assert!(obj.is_true());

        let parse_obj = parser.parse("(id 'a 'b)")?;
        let obj = env.evaluate(&parse_obj)?;
        assert!(obj.is_nil());

        Ok(())
    }

    #[test]
    fn can_def_a_function() -> Result<(), BelError> {
        let mut parser = parser::Parser::new();
        let mut env = Environment::new();

        let parse_obj = parser.parse(
            r#"(def no (x)
                (id x nil))
          "#,
        )?;
        let obj = env.evaluate(&parse_obj)?;
        assert!(obj.is_nil());

        let parse_obj = parser.parse("(no nil)")?;
        let obj = env.evaluate(&parse_obj)?;
        assert!(obj.is_true());

        let parse_obj = parser.parse("(no 'a)")?;
        let obj = env.evaluate(&parse_obj)?;
        assert!(obj.is_nil());

        Ok(())
    }
}
