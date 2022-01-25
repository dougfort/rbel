use crate::error::BelError;
use crate::Object;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Environment {
    pub global: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        let mut env = Environment {
            global: HashMap::new(),
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

        env
    }

    // Return an object that is reduced to its lowest terms
    pub fn evaluate(&mut self, obj: &Object) -> Result<Object, BelError> {
        let output = match obj {
            Object::Symbol(name) => match self.global.get(name) {
                Some(obj) => obj.clone(),
                None => {
                    return Err(BelError::UnboundSymbol(name.clone()));
                }
            },
            Object::List(list) => {
                if list.is_empty() {
                    return Ok(Object::Symbol("nil".to_string()));
                }
                if let Object::Symbol(name) = list[0].clone() {
                    match name.as_ref() {
                        "set" => {
                            for i in 1..list.len()-1 {
                                if let Object::Symbol(key) = list[i].clone() {
                                    self.global.insert(key, list[i+1].clone());
                                } else {
                                    return Err(BelError::InvalidObject{expected: "list".to_string(), found: list[i+1].t()})
                                }                                
                            }
                            return Ok(Object::Symbol("nil".to_string()));
                        }
                        "xxx" => {},
                        _ => {}
                    }
                }
                let mut evaluated_list = Vec::new();
                for item in list {
                    let eval_item = self.evaluate(item)?;
                    evaluated_list.push(eval_item);
                }
                Object::List(evaluated_list)
            }
            Object::Char(_c) => {
                return Err(BelError::NotImplemented("Object::Char".to_string()));
            }
            Object::Stream => return Err(BelError::NotImplemented("Object::Stream".to_string())),
        };

        Ok(output)
    }
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
        let mut env = Environment::new();

        let stmt = parser::parse("(set a b)")?;
        assert_eq!(stmt.len(), 1);
        let stmt_obj = &stmt[0];

        let obj =  env.evaluate(stmt_obj)?;
        if let Object::Symbol(s) = obj {
            assert_eq!(s, "nil");
        } else {
            panic!("unexpected object {:?}", obj);
        }

        let stmt = parser::parse("a")?;
        assert_eq!(stmt.len(), 1);        
        let stmt_obj = &stmt[0];

        let obj =  env.evaluate(stmt_obj)?;
        if let Object::Symbol(s) = obj {
            assert_eq!(s, "b");
        } else {
            panic!("unexpected object {:?}", obj);
        }

        Ok(())
    }
}
