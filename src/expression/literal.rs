use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    rulox_error::RuloxResult,
    scanner::token::{Token, TokenType},
    statement::Environment,
};

use super::ExprTrait;

#[derive(Clone)]
pub struct Literal {
    pub value: Token,
}

// Add true and false
impl Literal {
    pub fn new(value: Token) -> Self {
        let value = match value.token_type {
            TokenType::Number(_) => value,
            TokenType::String(_) => value,
            TokenType::True => value,
            TokenType::False => value,
            _ => Token::new(TokenType::Nil, "nil", value.line),
        };
        Self { value }
    }

    pub fn is_truthy(token_type: &TokenType) -> bool {
        !matches!(token_type, TokenType::False | TokenType::Nil)
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value.token_type {
            TokenType::Number(val) => write!(f, "{}", val),
            TokenType::String(val) => write!(f, "\"{}\"", val),
            TokenType::True => write!(f, "true"),
            TokenType::False => write!(f, "false"),
            _ => write!(f, "nil"),
        }
    }
}

impl ExprTrait for Literal {
    fn execute(&self, _env: Rc<RefCell<Environment>>) -> RuloxResult<Literal> {
        Ok(Literal::new(self.value.clone()))
    }

    fn get_token(&self) -> &Token {
        &self.value
    }
}
