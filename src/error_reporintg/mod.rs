use crate::scanner::token::{Token, TokenType};
use colored::*;

pub trait Report {
    fn report(&self);
}

pub enum MyError {
    ParseError {
        token: Option<Token>,
        line: usize,
        message: String,
    },
    SyntaxError {
        token: Option<Token>,
        line: usize,
        message: String,
    },
}

impl Report for MyError {
    fn report(&self) {
        match self {
            MyError::ParseError {
                token,
                line,
                message,
            } => {
                if let Some(token) = token {
                    match token.token_type {
                        TokenType::Eof => {
                            let msg = "at end: ".to_string() + &message;
                            error(*line, msg, "Parse".to_string());
                        }
                        _ => {
                            let msg = "at '".to_string() + &token.lexeme + "' " + &message;
                            error(*line, msg, "Parse".to_string());
                        }
                    }
                }
            }
            MyError::SyntaxError { line, message, .. } => {
                error(*line, message.to_string(), "Syntax".to_string())
            }
        }
    }
}

fn error(line: usize, message: String, err_type: String) {
    report(line, message, err_type, 0);
}

// fn warn(line: usize, message: String) {
//     report(line, message, 1);
// }
//
// fn info(message: String) {
//     report(0, message, 2);
// }

fn report(line: usize, message: String, rep_type: String, level: i32) {
    if level == 0 {
        eprintln!(
            "[line {}] {} {}: {}",
            line.to_string().bold(),
            rep_type.red().bold(),
            "Error".red().bold(),
            message.bold()
        );
    }

    if level == 1 {
        eprintln!(
            "[line {}] {}: {}",
            line.to_string().bold(),
            "Warning".yellow().bold(),
            message.bold()
        );
    }

    if level == 2 {
        eprintln!("{}: {}", "Info".green().bold(), message.bold());
    }
}
