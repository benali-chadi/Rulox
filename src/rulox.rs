use std::{cell::RefCell, rc::Rc};

use colored::Colorize;

use crate::{
    native_functions::ClockCallable,
    parser::Parser,
    rulox_error::RuloxResult,
    scanner::{token::TokenType, Scanner},
    statement::{Environment, Stmt, VarValue},
};

pub struct Rulox {
    source: String,
    environment: Rc<RefCell<Environment>>,
}

impl Rulox {
    pub fn new() -> Self {
        Self {
            source: String::new(),
            environment: Rc::new(RefCell::new(Rulox::globals())),
        }
    }

    pub fn from(source: String) -> Self {
        Self {
            source,
            environment: Rc::new(RefCell::new(Rulox::globals())),
        }
    }

    pub fn run(&mut self) -> RuloxResult<()> {
        let mut scanner = Scanner::new(&self.source);
        let tokens = scanner.scan_tokens()?;
        //* for token in &tokens {
        //     println!("{token:?}");
        // }

        let mut parser = Parser::new(&tokens);
        let statements = parser.parse()?;

        self.interpret(statements, false)
    }

    pub fn prompt_run(&mut self, input: String) -> RuloxResult<()> {
        let mut scanner = Scanner::new(&input);
        let tokens = scanner.scan_tokens()?;
        //* for token in &tokens {
        //     println!("{token:?}");
        // }

        let mut parser = Parser::new(&tokens);
        let statements = parser.parse()?;

        self.interpret(statements, true)
    }

    fn interpret(&mut self, statements: Vec<Stmt>, is_prompt: bool) -> RuloxResult<()> {
        for statement in &statements {
            statement.execute(Rc::clone(&self.environment))?;
            if is_prompt {
                println!(
                    "{}: {}",
                    "expression result".bright_yellow().dimmed(),
                    statement
                );
            }
        }

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

    fn globals() -> Environment {
        let mut globals = Environment::default();

        globals.define(
            "clock",
            VarValue::Callable(Rc::new(RefCell::new(ClockCallable))),
        );

        globals
    }
}

impl Default for Rulox {
    fn default() -> Self {
        Self::new()
    }
}
