#[macro_use]
extern crate log;

use std::{env, process};

use rulox::utils;

fn main() {
    env_logger::init();
    info!("starting up!");

    match env::args().len() {
        len @ 1..=2 => {
            if len == 1 {
                let _ = utils::run_prompt();
            } else {
                utils::run_file(env::args().nth(1).unwrap());
            }
        }
        _ => {
            println!("Usage: rulox [script]");
            process::exit(64);
        }
    }
}
