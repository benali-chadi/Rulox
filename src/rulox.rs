use crate::{
    parser::Parser,
    rulox_error::{Report, RuloxResult},
    scanner::{token::TokenType, Scanner},
    statement::Stmt,
};

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

    pub fn run(&self) -> RuloxResult<()> {
        let mut scanner = Scanner::new(&self.source);
        let tokens = scanner.scan_tokens()?;

        let mut parser = Parser::new(&tokens);
        let statements = parser.parse()?;

        Rulox::interpret(statements);

        Ok(())
    }

    pub fn prompt_run(&self, input: String) -> RuloxResult<()> {
        let mut scanner = Scanner::new(&input);
        let tokens = scanner.scan_tokens()?;
        for token in &tokens {
            println!("{token:?}");
        }

        let mut parser = Parser::new(&tokens);
        let statements = parser.parse()?;

        Rulox::interpret(statements);

        Ok(())
    }

    pub fn interpret(statements: Vec<Stmt>) {
        for statement in statements {
            match statement.execute() {
                Ok(_) => {}
                Err(err) => {
                    err.report();
                }
            }
        }
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

impl Default for Rulox {
    fn default() -> Self {
        Self::new()
    }
}
