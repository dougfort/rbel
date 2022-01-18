use bel::parser;
use anyhow::Error;
use std::io::{stdin, stdout, Write};

fn main() -> Result<(), Error> {
    loop {
        let stdin_line = get_stdin_line(">")?;
        let line = stdin_line.trim();
        match line {
            ":q" => {
                println!("break!");
                break;
            }

            _ => match parser::parse(line) {
                Ok(parse_result) => {
                    println!("parse result = {}", parse_result);
                }
                Err(e) => {
                    eprintln!("parse error: {}", e);
                }
            },
        }
    }
    Ok(())
}

fn get_stdin_line(prompt: &str) -> std::io::Result<String> {
    println!();
    print!("{} ", prompt);
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    Ok(input)
}
