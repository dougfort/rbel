pub mod object;
pub mod parser;
use failure::{format_err, Error};
use object::*;

pub struct Bel {}

impl Bel {
    pub fn new() -> Self {
        Bel {}
    }

    pub fn eval(&mut self, _obj: &Object) -> Result<Object, Error> {
        Err(format_err!("eval not implemented"))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
