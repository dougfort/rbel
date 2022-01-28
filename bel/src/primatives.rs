use std::collections::HashMap;

use crate::error::BelError;
use crate::environment::Environment;
use crate::Object;

pub fn id(env: &Environment, _locals: HashMap<String, Object>, params: Vec<Object>) -> Result<Object, BelError> {
    Ok(params[0].clone())
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;
    use super::*;

    #[test]
    fn id_checks_for_identity() -> Result<(), BelError> {
        let mut parser = Parser::new();
        let mut env = Environment::new();

        let parse_obj = parser.parse("(id 'a 'a)")?;
        let obj = env.evaluate(&parse_obj)?;
        assert!(obj.is_true());

        let parse_obj = parser.parse("(id 'a 'b)")?;
        let obj = env.evaluate(&parse_obj)?;
        assert!(obj.is_nil());

        Ok(())
    }
}