#[macro_use]
extern crate log;

use std::{
    env, fs,
    io::{self, Write},
    process,
};

use Rulox::scanner;

fn main() {
    env_logger::init();
    info!("starting up!");

    if env::args().len() > 2 {
        println!("Usage: rulox [script]");
        process::exit(64);
    } else if env::args().len() == 2 {
        run_file(env::args().nth(1).unwrap());
    } else {
        let _ = run_prompt();
    }
}

fn run_file(filename: String) {
    let content = fs::read(filename).unwrap();

    scanner::run(String::from_utf8(content).unwrap());
}

fn run_prompt() -> io::Result<()> {
    loop {
        print!("rulox> ");
        io::stdout().flush()?;
        let mut input = String::new();

        let bytes = io::stdin().read_line(&mut input)?;

        if bytes == 0 || input.trim() == "quit" {
            break;
        }

        scanner::run(input);
    }

    Ok(())
}
