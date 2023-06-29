use std::fmt::Display;

use crate::{rulox_error::RuloxResult, scanner::token::Token, statement::environment::Environment};

use super::{ExprTrait, Literal};

#[derive(Debug)]
pub struct Variable {
    name: Token,
}

impl Variable {
    pub fn new(name: &Token) -> Self {
        Self { name: name.clone() }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.lexeme)
    }
}

impl ExprTrait for Variable {
    fn execute(&self, env: &mut Environment) -> RuloxResult<Literal> {
        // let token = Token::new(
        //     TokenType::String(self.name.lexeme.to_string()),
        //     &self.name.lexeme,
        //     self.name.line,
        // );
        // Ok(Literal::new(token))
        env.get(self.name.clone()).cloned()
    }

    fn get_token(&self) -> &Token {
        &self.name
    }
}
