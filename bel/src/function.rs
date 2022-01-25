use crate::error::BelError;
use crate::Object;
use std::collections::HashMap;

pub trait Function {
    fn apply(
        &self,
        locals: HashMap<String, Object>,
        params: HashMap<String, Object>,
    ) -> Result<Object, BelError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn can_create_function() -> Result<(), BelError> {
        let f = parser::parse("(def f nil (()))");
        assert_eq!(parser::parse("")?, vec![]);

        Ok(())
    }
}
