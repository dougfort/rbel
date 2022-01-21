use anyhow::{anyhow, Error};
use bel::parser;
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() -> Result<(), Error> {
    env_logger::init();

    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let result = parser::parse(&line)?;
                println!("Line: {}; result = {:?}", line, result);
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
