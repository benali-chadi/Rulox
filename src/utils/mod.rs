use std::{
    fs,
    io::{self, Write},
    process,
    // process,
};

use crate::{
    expression::Expr,
    rulox::Rulox,
    rulox_error::{Report, RuloxResult},
};

pub fn run_file(filename: String) -> RuloxResult<()> {
    let content = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            println!("Error: {}", err);
            process::exit(64);
        }
    };

    let rulox = Rulox::from(content);

    rulox.run()
}

pub fn run_prompt() -> RuloxResult<()> {
    let rulox = Rulox::new();
    loop {
        print!("rulox> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();

        let bytes = match io::stdin().read_line(&mut input) {
            Ok(bytes) => bytes,
            Err(err) => {
                println!("Error: {}", err);
                process::exit(64);
            }
        };

        if bytes == 0 || input.trim() == "quit" {
            break;
        }

        match rulox.prompt_run(input) {
            Ok(_) => {}
            Err(err) => {
                err.report();
            }
        }
        // rulox.prompt_run(input)?
    }

    Ok(())
}

pub fn print_tree(expression: &Expr) {
    println!("{}", expression);
}
