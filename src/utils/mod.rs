use std::{
    fs,
    io::{self, Write},
};

use crate::{error_reporintg::Report, expression::Expr, rulox::Rulox};

pub fn run_file(filename: String) {
    let content = fs::read_to_string(filename).unwrap();
    let rulox = Rulox::from(content);

    match rulox.run() {
        Ok(_) => {}
        Err(err) => {
            err.report();
        }
    }
}

pub fn run_prompt() -> io::Result<()> {
    let rulox = Rulox::new();
    loop {
        print!("rulox> ");
        io::stdout().flush()?;
        let mut input = String::new();

        let bytes = io::stdin().read_line(&mut input)?;

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

    Ok(())
}

pub fn print_tree(expression: &Expr) {
    println!("{}", expression);
}
