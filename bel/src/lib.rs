pub mod object;
pub mod parser;
use thiserror::Error;
use object::*;

#[derive(Error, Debug)]
pub enum BelError {
    #[error("bel error")]
    Error, 
}

#[derive(Default)]
pub struct Bel {}

impl Bel {
    pub fn new() -> Self {
        Bel {}
    }

    pub fn eval(&mut self, _obj: &Object) -> Result<Object, BelError> {
        Err(BelError::Error)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
