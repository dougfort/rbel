use anyhow::{anyhow, Error};
use bel::{environment, parser};
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() -> Result<(), Error> {
    env_logger::init();

    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    };

    let mut env = environment::Environment::new();

    'repl_loop: loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let object = match parser::parse(&line) {
                    Ok(result) => {
                        if result.is_empty() {
                            continue 'repl_loop;
                        }
                        if result.len() == 1 {
                            result[0].clone()
                        } else {
                            bel::Object::List(result)
                        }
                    }
                    Err(err) => {
                        eprintln!("error: {:?}", err);
                        continue 'repl_loop;
                    }
                };
                match env.evaluate(&object) {
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
