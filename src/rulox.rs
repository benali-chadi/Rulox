use crate::{error_reporintg::MyError, parser::Parser, scanner::Scanner, utils};

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
    pub fn run(&self) -> Result<(), MyError> {
        let mut scanner = Scanner::new(&self.source);
        let tokens = scanner.scan_tokens()?;
        let mut parser = Parser::new(tokens);
        let expr = parser.parse()?;

        utils::print_tree(&expr);

        Ok(())
    }

    pub fn prompt_run(&self, input: String) -> Result<(), MyError> {
        let mut scanner = Scanner::new(&input);
        let tokens = scanner.scan_tokens()?;
        let mut parser = Parser::new(tokens);
        let expr = parser.parse()?;

        utils::print_tree(&expr);

        Ok(())
    }
}
