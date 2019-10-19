use std::io::{stdin, stdout, Write};
use bel::object::Object;
use bel::parser::Parser;
use bel::Bel;
use failure::Error;

fn main() -> Result<(), Error> {
    let parser = Parser::new();
    let _bel = Bel::new();
    loop {
        let stdin_line = get_stdin_line(">")?;
        match parser.parse(&stdin_line) {
            Ok((result, _)) => {
                println!("parse result = {}", result);
            }
            Err(e) => {
                eprintln!("parse error: {:?}", e);
            }
        }
    }
    Ok(())
}

fn get_stdin_line(prompt: &str) -> std::io::Result<String> {
    println!("");
    print!("{} ", prompt);
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    Ok(input)
} 
