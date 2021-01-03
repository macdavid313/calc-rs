mod lexer;
mod parser;

use crate::parser::eval;
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let info = "calc-rs, version 0.1";
    let prompt = "calc-rs> ";
    let history_path = match dirs::home_dir() {
        Some(mut path) => {
            path.push(".calc-rs-history");
            path.to_str().unwrap().to_owned()
        }
        _ => ".calc-rs-history".to_string(),
    };
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history(&history_path).is_err() {
        println!("No previous history of calc-rs.");
    }
    println!("{}", info);
    loop {
        match rl.readline(prompt) {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                rl.save_history(&history_path).unwrap();
                if !line.is_empty() {
                    match eval(line) {
                        Ok(n) => println!("{}", n),
                        Err(e) => println!("{}", e),
                    }
                }
            }
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {}", err.to_string());
                break;
            }
        }
    }
}
