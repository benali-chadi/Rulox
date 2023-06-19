use std::fmt::Display;

use crate::scanner::token::{Token, TokenType};

use super::ExprTrait;

pub struct Literal {
    value: Token,
}

// Add true and false
impl Literal {
    pub fn new(value: Token) -> Self {
        let value = match value.token_type {
            TokenType::Number(_) => value,
            TokenType::String(_) => value,
            TokenType::True => value,
            TokenType::False => value,
            _ => Token::new(TokenType::Nil, "nil".to_string(), value.line),
        };
        Self { value }
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
    fn evaluate(&self) -> bool {
        true
    }
}
