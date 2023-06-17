#[macro_use]
extern crate log;

use std::{
    env,
    fmt::Binary,
    fs,
    io::{self, Write},
    process,
};

use rulox::{
    expression::{self, Expr, Grouping, Literal, Unary},
    scanner,
    scanner::token::*,
};

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

    // let expression = expression::Binary::new(
    //     Expr::new(Box::new(Unary::new(
    //         Token::new(TokenType::Minus, "-".to_string(), 1),
    //         Expr::new(Box::new(Literal::new(Token::new(
    //             TokenType::Number(127.0),
    //             "127.0".to_string(),
    //             1,
    //         )))),
    //     ))),
    //     Token::new(TokenType::Star, "*".to_string(), 1),
    //     Expr::new(Box::new(Grouping::new(Expr::new(Box::new(Literal::new(
    //         Token::new(TokenType::Number(45.5), "45.5".to_string(), 1),
    //     )))))),
    // );
    //
    // println!("{:?}", format!("{expression}"));
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
