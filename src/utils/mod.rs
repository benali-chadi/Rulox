use std::{
    fs,
    io::{self, Write},
    process,
};

use crate::{
    expression::Expr,
    rulox::Rulox,
    rulox_error::{Report, RuloxError},
};

pub fn run_file(filename: String) {
    let content = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            println!("Error {}", err);
            process::exit(64);
        }
    };

    let mut rulox = Rulox::from(content);

    if let Err(err) = rulox.run() {
        if let RuloxError::SyntaxError { .. } | RuloxError::RuntimeError { .. } = err {
            err.report();
        };
        process::exit(64);
    }
}

pub fn run_prompt() {
    let mut rulox = Rulox::new();
    loop {
        print!("rulox> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(bytes) => {
                if bytes == 0 {
                    break;
                }
            }
            Err(err) => {
                println!("{}", err);
                process::exit(64);
            }
        };

        match input.trim() {
            "quit" => {
                break;
            }
            "clear" => {
                if let Err(err) = process::Command::new("clear").spawn() {
                    println!("{err}")
                }
            }
            _ => match rulox.prompt_run(input) {
                Ok(_) => {}
                Err(err) => {
                    err.report();
                }
            },
        }
    }
}

pub fn print_tree(expression: &Expr) {
    println!("{}", expression);
}
