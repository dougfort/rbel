use std::collections::HashMap;

use anyhow::{anyhow, Error};
use bel::{environment, parser, Object};
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() -> Result<(), Error> {
    env_logger::init();

    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    };

    let mut parser = parser::Parser::new();
    let mut env = environment::Environment::new();

    'repl_loop: loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if line == ":global" {
                    println!("global");
                    for (key, value) in &env.globals {
                        println!("({}, {:?}", key, value);
                    }
                    continue 'repl_loop;
                }
                rl.add_history_entry(line.as_str());
                let object = match parser.parse(&line) {
                    Ok(object) => object,
                    Err(err) => {
                        eprintln!("error: {:?}", err);
                        continue 'repl_loop;
                    }
                };
                let locals: HashMap<String, Object> = HashMap::new();
                match env.evaluate(&locals, &object) {
                    Ok(evaluated_object) => println!("evaluated: {:?}", evaluated_object),
                    Err(err) => eprintln!("error: {:?}", err),
                };
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                return Err(anyhow!("Error from readline: {:?}", err));
            }
        }
    }
    rl.save_history("history.txt").unwrap();

    Ok(())
}
