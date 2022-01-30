use std::collections::HashMap;

use crate::object::Object;
use crate::error::BelError;

pub type PrimFunc = fn(&[Object]) -> Result<Object, BelError>;

pub fn load_primatives() -> HashMap<String, PrimFunc> {
    let mut prim: HashMap<String, PrimFunc>  = HashMap::new();

    prim.insert("id".to_string(), id);

    prim
}

fn id(params: &[Object]) -> Result<Object, BelError> {
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
