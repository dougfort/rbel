use crate::error::BelError;
use crate::Object;
use std::collections::HashMap;

#[derive(Default)]
pub struct Environment {
    pub globals: HashMap<String, Object>,
    primatives: HashMap<String, EnvFunc>,
}

type EnvFunc = fn(&mut Environment, &[Object]) -> Result<Object, BelError>;

impl Environment {
    pub fn new() -> Self {
        let mut env = Environment {
            globals: HashMap::new(),
            primatives: HashMap::new(),
        };

        // some Symbols bind to themselves
        for name in vec![
            "nil".to_string(),
            "t".to_string(),
            "o".to_string(),
            "apply".to_string(),
        ] {
            env.globals.insert(name.clone(), Object::Symbol(name));
        }

        env.primatives.insert("id".to_string(), id);

        env
    }

    // Return an object that is reduced to its lowest terms
    pub fn evaluate(
        &mut self,
        locals: &HashMap<String, Object>,
        obj: &Object,
    ) -> Result<Object, BelError> {
        let output = match obj {
            Object::Symbol(name) => self.get_bound_object(locals, name)?,
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
                            let evaluated_list = self.evaluate_list(locals, &list[1..])?;
                            return self.primatives[n](self, &evaluated_list);
                        }
                        _ => {
                            // if the leading symbol refers to a function,
                            // we apply the function
                            let obj = self.get_bound_object(locals, &name)?;
                            if obj.is_function() {
                                let evaluated_list = self.evaluate_list(locals, &list[1..])?;
                                return self.apply(&obj, &evaluated_list);
                            }
                        }
                    }
                }
                let locals: HashMap<String, Object> = HashMap::new();
                let evaluated_list = self.evaluate_list(&locals, &list[1..])?;
                Object::List(evaluated_list)
            }
            Object::Char(_c) => {
                return Err(BelError::NotImplemented("Object::Char".to_string()));
            }
            Object::Stream => return Err(BelError::NotImplemented("Object::Stream".to_string())),
        };

        Ok(output)
    }

    fn apply(&mut self, fn_obj: &Object, args: &[Object]) -> Result<Object, BelError> {
        // we expect the function list to be the 5th of 5 objects in the function
        // (lit clo nil p e)
        let locals = merge_args(fn_obj, args)?;
        let fn_list = get_function_list(fn_obj)?;
        if fn_list.is_empty() {
            return Err(BelError::InvalidFn("empty function list".to_string()));
        }
        if let Object::Symbol(name) = fn_list[0].clone() {
            let evaluated_list = self.evaluate_list(&locals, &fn_list[1..])?;
            if self.primatives.contains_key(&name) {
                return self.primatives[&name](self, &evaluated_list);
            }
        } else {
            return Err(BelError::InvalidFn(
                "function list does not begin wiht a symbol".to_string(),
            ));
        }

        Ok(Object::Symbol("nil".to_string()))
    }

    fn evaluate_list(
        &mut self,
        locals: &HashMap<String, Object>,
        list: &[Object],
    ) -> Result<Vec<Object>, BelError> {
        let mut evaluated_list = Vec::new();
        for item in list {
            let eval_item = self.evaluate(locals, item)?;
            evaluated_list.push(eval_item);
        }
        Ok(evaluated_list)
    }

    fn get_bound_object(
        &self,
        locals: &HashMap<String, Object>,
        name: &str,
    ) -> Result<Object, BelError> {
        // look first in locals, then in globals
        match locals.get(name) {
            Some(obj) => Ok(obj.clone()),
            None => match self.globals.get(name) {
                Some(obj) => Ok(obj.clone()),
                None => Err(BelError::UnboundSymbol(name.to_string())),
            },
        }
    }

    fn set(&mut self, list: &[Object]) -> Result<Object, BelError> {
        for i in 0..list.len() - 1 {
            if let Object::Symbol(key) = list[i].clone() {
                self.globals.insert(key, list[i + 1].clone());
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
                self.globals.insert(key, Object::Symbol("nil".to_string()));
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

fn merge_args(fn_obj: &Object, args: &[Object]) -> Result<HashMap<String, Object>, BelError> {
    // merge the arguments with the function object's parameters
    // we expect the parameters list to be the 4th of 5 objects in the function
    // (lit clo nil p e)
    if let Object::List(fn_list) = fn_obj {
        if fn_list.len() != 5 {
            return Err(BelError::InvalidFn(format!(
                "expected 5 items found {}",
                fn_list.len()
            )));
        }
        if let Object::List(params) = &fn_list[3] {
            if args.len() > params.len() {
                return Err(BelError::InvalidFn(format!(
                    "expected {} or fewer args found {}",
                    params.len(),
                    args.len()
                )));
            }
            let mut merged: HashMap<String, Object> = HashMap::new();

            for i in 0..args.len() {
                if let Object::Symbol(param_str) = &params[i] {
                    merged.insert(param_str.to_string(), args[i].clone());
                } else {
                    return Err(BelError::InvalidObject {
                        expected: "symbol".to_string(),
                        found: params[i].t(),
                    });
                }
            }

            // if we have unmatched params, fill with nil
            if args.len() < params.len() {
                for param in &params[args.len()..] {
                    if let Object::Symbol(param_str) = param {
                        merged.insert(param_str.to_string(), Object::Symbol("nil".to_string()));
                    } else {
                        return Err(BelError::InvalidObject {
                            expected: "symbol".to_string(),
                            found: param.t(),
                        });
                    }
                }
            }

            Ok(merged)
        } else {
            Err(BelError::InvalidObject {
                expected: "list".to_string(),
                found: fn_obj.t(),
            })
        }
    } else {
        Err(BelError::InvalidObject {
            expected: "list".to_string(),
            found: fn_obj.t(),
        })
    }
}

fn get_function_list(fn_obj: &Object) -> Result<Vec<Object>, BelError> {
    if let Object::List(fn_list) = fn_obj {
        if fn_list.len() != 5 {
            return Err(BelError::InvalidFn(format!(
                "expected 5 items found {}",
                fn_list.len()
            )));
        }

        if let Object::List(list) = &fn_list[4] {
            Ok(list.to_vec())
        } else {
            Err(BelError::InvalidObject {
                expected: "list".to_string(),
                found: fn_list[4].t(),
            })
        }
    } else {
        Err(BelError::InvalidObject {
            expected: "list".to_string(),
            found: fn_obj.t(),
        })
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
            let locals: HashMap<String, Object> = HashMap::new();
            let res = env.evaluate(&locals, &obj)?;
            assert_eq!(res, obj);
        }
        assert_eq!(2 + 2, 4);

        Ok(())
    }

    #[test]
    fn can_set_globals() -> Result<(), BelError> {
        let mut parser = parser::Parser::new();
        let mut env = Environment::new();
        let locals: HashMap<String, Object> = HashMap::new();

        let parse_obj = parser.parse("(set a b)")?;
        let obj = env.evaluate(&locals, &parse_obj)?;
        assert!(obj.is_nil());

        let parse_obj = parser.parse("a")?;
        let obj = env.evaluate(&locals, &parse_obj)?;
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
        let locals: HashMap<String, Object> = HashMap::new();

        let parse_obj = parser.parse("(set a b c d e f)")?;
        let obj = env.evaluate(&locals, &parse_obj)?;
        assert!(obj.is_nil());

        for (key, val) in &[
            ("a", "b".to_string()),
            ("c", "d".to_string()),
            ("e", "f".to_string()),
        ] {
            let parse_obj = parser.parse(key)?;
            let obj = env.evaluate(&locals, &parse_obj)?;
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
        let locals: HashMap<String, Object> = HashMap::new();

        let parse_obj = parser.parse("(set a b c d e)")?;
        let obj = env.evaluate(&locals, &parse_obj)?;
        assert!(obj.is_nil());

        for (key, val) in &[
            ("a", "b".to_string()),
            ("c", "d".to_string()),
            ("e", "nil".to_string()),
        ] {
            let parse_obj = parser.parse(key)?;
            let obj = env.evaluate(&locals, &parse_obj)?;
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
        let locals: HashMap<String, Object> = HashMap::new();

        let parse_obj = parser.parse("(set a b)")?;
        let obj = env.evaluate(&locals, &parse_obj)?;
        assert!(obj.is_nil());

        let parse_obj = parser.parse("(quote a)")?;
        let obj = env.evaluate(&locals, &parse_obj)?;
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
        let locals: HashMap<String, Object> = HashMap::new();

        let parse_obj = parser.parse("(id 'a 'a)")?;
        let obj = env.evaluate(&locals, &parse_obj)?;
        assert!(obj.is_true());

        let parse_obj = parser.parse("(id 'a 'b)")?;
        let obj = env.evaluate(&locals, &parse_obj)?;
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
        let locals: HashMap<String, Object> = HashMap::new();
        let obj = env.evaluate(&locals, &parse_obj)?;
        assert!(obj.is_nil());

        let parse_obj = parser.parse("(no nil)")?;
        let locals: HashMap<String, Object> = HashMap::new();
        let obj = env.evaluate(&locals, &parse_obj)?;
        assert!(obj.is_true());

        let parse_obj = parser.parse("(no 'a)")?;
        let locals: HashMap<String, Object> = HashMap::new();
        let obj = env.evaluate(&locals, &parse_obj)?;
        assert!(obj.is_nil(), "{:?}", obj);

        Ok(())
    }
}
