pub mod object;

pub struct Bel {

}

impl Bel {
    pub fn new() -> Self {
        Bel{}
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
