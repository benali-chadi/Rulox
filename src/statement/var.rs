use std::fmt::Display;

use crate::{
    expression::{Expr, Literal},
    rulox_error::RuloxResult,
    scanner::token::{Token, TokenType},
};

use super::{environment::Environment, StmtTrait};

pub struct Var {
    pub name: Token,
    pub initializer: Option<Expr>,
}

impl Var {
    pub fn new(name: Token, initializer: Option<Expr>) -> Self {
        Self { name, initializer }
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self.initializer {
                Some(expr) => expr.to_string(),
                None => "nil".to_string(),
            }
        )
    }
}

impl StmtTrait for Var {
    fn execute(&self, env: &mut Environment) -> RuloxResult<()> {
        match &self.initializer {
            Some(expr) => {
                let value = expr.execute(env)?;
                env.define(&self.name.lexeme, value);
                Ok(())
            }
            None => {
                env.define(
                    &self.name.lexeme,
                    Literal::new(Token::new(TokenType::Nil, "nil", 1)),
                );
                Ok(())
            }
        }
    }
}
