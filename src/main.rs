#[macro_use]
extern crate log;

use std::{
    env, fs,
    io::{self, Write},
    process,
};

use rulox::{error_reporintg::Report, rulox::Rulox};

fn main() {
    env_logger::init();
    info!("starting up!");

    match env::args().len() {
        1 => {
            let _ = run_prompt();
        }
        2 => {
            run_file(env::args().nth(1).unwrap());
        }
        _ => {
            println!("Usage: rulox [script]");
            process::exit(64);
        }
    }
}

fn run_file(filename: String) {
    let content = fs::read_to_string(filename).unwrap();
    let rulox = Rulox::from(content);

    match rulox.run() {
        Ok(_) => {}
        Err(err) => {
            err.report();
            // process::exit(1);
        }
    }
}

fn run_prompt() -> io::Result<()> {
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
                // process::exit(1);
            }
        }
    }

    Ok(())
}
