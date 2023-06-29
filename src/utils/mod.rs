use std::{
    fs,
    io::{self, Write},
    process,
};

use crate::{
    expression::Expr,
    rulox::Rulox,
    rulox_error::{Report, RuloxError},
    statement::environment::Environment,
};

pub fn run_file(filename: String) {
    let content = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            println!("Error {}", err);
            process::exit(64);
        }
    };

    let mut rulox = Rulox::from(content, Environment::default());

    match rulox.run() {
        Err(err) => {
            if let RuloxError::SyntaxError { .. } | RuloxError::RuntimeError { .. } = err {
                err.report();
            };
            process::exit(64);
        }
        _ => {}
    }
}

pub fn run_prompt() {
    let mut rulox = Rulox::new();
    loop {
        print!("rulox> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();

        let bytes = match io::stdin().read_line(&mut input) {
            Ok(bytes) => bytes,
            Err(err) => {
                println!("{}", err);
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
    }
}

pub fn print_tree(expression: &Expr) {
    println!("{}", expression);
}
