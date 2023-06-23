use std::{
    fs,
    io::{self, Write},
    // process,
};

use crate::{
    expression::Expr,
    rulox::Rulox,
    rulox_error::{Report, RuloxError},
};

pub fn run_file(filename: String) -> Result<(), RuloxError> {
    let content = fs::read_to_string(filename).unwrap();
    let rulox = Rulox::from(content);

    // match rulox.run() {
    //     Ok(_) => {}
    //     Err(err) => {
    //         err.report();
    //         match err {
    //             RuloxError::RuntimeError { .. } => {
    //                 process::exit(70);
    //             }
    //             _ => {}
    //         }
    //     }
    // }
    rulox.run()
}

pub fn run_prompt() -> Result<(), RuloxError> {
    let rulox = Rulox::new();
    loop {
        print!("rulox> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();

        let bytes = io::stdin().read_line(&mut input).unwrap();

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
