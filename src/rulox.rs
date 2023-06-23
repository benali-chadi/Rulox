use crate::{
    parser::Parser,
    rulox_error::RuloxError,
    scanner::{token::TokenType, Scanner},
    utils,
};
use colored::*;

pub struct Rulox {
    source: String,
}

impl Rulox {
    pub fn new() -> Self {
        Self {
            source: String::new(),
        }
    }

    pub fn from(source: String) -> Self {
        Self { source }
    }

    pub fn run(&self) -> Result<(), RuloxError> {
        let mut scanner = Scanner::new(&self.source);
        let tokens = scanner.scan_tokens()?;

        println!("{}", "Tokens".bold().blue());
        for token in &tokens {
            println!("{token}");
        }

        let mut parser = Parser::new(&tokens);
        let expr = parser.parse()?;

        println!("{}", "ASTree".bold().green());
        utils::print_tree(&expr);

        println!("{}", "Result".bold().yellow());
        let literal = expr.interpret()?;

        println!(
            "{}",
            Rulox::literal_token_type_to_string(literal.value.token_type)
        );

        Ok(())
    }

    pub fn prompt_run(&self, input: String) -> Result<(), RuloxError> {
        let mut scanner = Scanner::new(&input);
        let tokens = scanner.scan_tokens()?;
        println!("{}", "Tokens".bold().blue());
        for token in &tokens {
            println!("{token}");
        }
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse()?;

        println!("{}", "ASTree".bold().green());
        utils::print_tree(&expr);

        println!("{}", "Result".bold().yellow());
        let literal = expr.interpret()?;

        println!(
            "{}",
            Rulox::literal_token_type_to_string(literal.value.token_type)
        );

        Ok(())
    }

    pub fn literal_token_type_to_string(token_type: TokenType) -> String {
        match token_type {
            TokenType::Number(val) => val.to_string(),

            TokenType::String(val) => val,

            TokenType::True => "true".to_string(),

            TokenType::False => "false".to_string(),

            _ => "nil".to_string(),
        }
    }
}
