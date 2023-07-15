#[macro_use]
extern crate log;

use std::{env, process, error::Error};

use signal_hook::consts::signal::*;

use signal_hook_tokio::Signals;
use futures::stream::StreamExt;

use rulox::utils;

async fn handle_signals(mut signals: Signals) {
    while let Some(signal) = signals.next().await {
        match signal {
            SIGINT => {
                println!("\nto quit press Ctrl-d or type 'quit'");
            }
            //* SIGQUIT => { }
            _ => {
                unreachable!()
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let signals = Signals::new([SIGINT, /* SIGQUIT */])?;

    let handle = signals.handle();

    let signals_task = tokio::spawn(handle_signals(signals));

    env_logger::init();
    info!("starting up!");

    match env::args().len() {
        len @ 1..=2 => {
            if len == 1 {
                utils::run_prompt();
            } else {
                utils::run_file(env::args().nth(1).unwrap());
            }
        }
        _ => {
            println!("Usage: rulox [script]");
            process::exit(64);
        }
    }

    handle.close();
    signals_task.await?;

    Ok(())
}
