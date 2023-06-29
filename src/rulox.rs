use crate::{
    parser::Parser,
    rulox_error::{Report, RuloxResult},
    scanner::{token::TokenType, Scanner},
    statement::{environment::Environment, Stmt},
};

pub struct Rulox {
    source: String,
    environment: Environment,
}

impl Rulox {
    pub fn new() -> Self {
        Self {
            source: String::new(),
            environment: Environment::default(),
        }
    }

    pub fn from(source: String, environment: Environment) -> Self {
        Self {
            source,
            environment,
        }
    }

    pub fn run(&mut self) -> RuloxResult<()> {
        let mut scanner = Scanner::new(&self.source);
        let tokens = scanner.scan_tokens()?;
        // for token in &tokens {
        //     println!("{token:?}");
        // }

        let mut parser = Parser::new(&tokens);
        let statements = parser.parse()?;

        self.interpret(statements)

        // Ok(())
    }

    pub fn prompt_run(&mut self, input: String) -> RuloxResult<()> {
        let mut scanner = Scanner::new(&input);
        let tokens = scanner.scan_tokens()?;
        // for token in &tokens {
        //     println!("{token:?}");
        // }

        let mut parser = Parser::new(&tokens);
        let statements = parser.parse()?;

        self.interpret(statements)

        // Ok(())
    }

    fn interpret(&mut self, statements: Vec<Stmt>) -> RuloxResult<()> {
        for statement in &statements {
            // match statement.execute(&mut self.environment) {
            //     Ok(_) => {}
            //     Err(err) => {
            //         err.report();
            //     }
            // }
            statement.execute(&mut self.environment)?;
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
}

impl Default for Rulox {
    fn default() -> Self {
        Self::new()
    }
}
