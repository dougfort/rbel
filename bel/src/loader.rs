use crate::environment;
use crate::object;
use crate::parser;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load(env: &mut environment::Environment, filepath: &str) -> Result<()> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut parser = parser::Parser::new();

    let mut accum = String::new();
    'line_loop: for line in reader.lines() {
        let line = line?;
        // https://sep.yimg.com/ty/cdn/paulgraham/bellanguage.txt has some weird
        // unicode byte order mark
        if line.starts_with('\u{feff}') {
            continue 'line_loop;
        }
        if line.starts_with(';') {
            continue 'line_loop;
        }
        if line.is_empty() && !accum.is_empty() {
            let parsed_expr = parser.parse(&accum)?;
            let locals: HashMap<String, object::Object> = HashMap::new();
            env.evaluate(&locals, &parsed_expr)
                .context(format!("\n\n{}\n", accum))?;
            accum.clear();
        }

        accum.push_str(&format!("{}\n", &line));
    }

    Ok(())
}
