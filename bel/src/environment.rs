use crate::Object;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvalError {
    #[error("unbound symbol: {0}")]
    UnboundSymbol(String),
    #[error("not implemented: {0}")]
    NotImplemented(String),
}

#[derive(Default, Debug)]
pub struct Environment {
    global: HashMap<String, Object>,
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
    pub fn evaluate(&mut self, obj: &Object) -> Result<Object, EvalError> {
        let output = match obj {
            Object::Symbol(name) => match self.global.get(name) {
                Some(obj) => obj.clone(),
                None => {
                    return Err(EvalError::UnboundSymbol(name.clone()));
                }
            },
            Object::List(list) => {
                let mut evaluated_list = Vec::new();
                for item in list {
                    let eval_item = self.evaluate(item)?;
                    evaluated_list.push(eval_item);
                }
                Object::List(evaluated_list)
            }
            Object::Char(_c) => {
                return Err(EvalError::NotImplemented("Object::Char".to_string()));
            }
            Object::Stream => return Err(EvalError::NotImplemented("Object::Stream".to_string())),
        };

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_objects_evaluate_to_themselves() {
        let mut env = Environment::new();
        for obj in vec![
            Object::Symbol("nil".to_string()),
            Object::Symbol("t".to_string()),
            Object::Symbol("o".to_string()),
            Object::Symbol("apply".to_string()),
        ] {
            let res = env.evaluate(&obj).unwrap();
            assert_eq!(res, obj);
        }
        assert_eq!(2 + 2, 4);
    }
}
