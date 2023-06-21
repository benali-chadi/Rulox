use std::fmt::Display;

use crate::{
    parser::expression::utils,
    rulox_error::RuloxError,
    scanner::token::{Token, TokenType},
};

use super::{Expr, ExprTrait, Literal};

pub struct Unary {
    pub operator: Token,
    pub right: Expr,
}

impl Unary {
    pub fn new(operator: Token, right: Expr) -> Self {
        Self { operator, right }
    }
}

impl Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            utils::parenthisize(&self.operator.lexeme, &[&self.right])
        )
    }
}

impl ExprTrait for Unary {
    fn interpret(&self) -> Result<Literal, RuloxError> {
        let literal = self.right.interpret()?;

        match self.operator.token_type {
            TokenType::Minus => match literal.value.token_type {
                TokenType::Number(val) => Ok(Literal::new(Token::new(
                    TokenType::Number(-val),
                    (-val).to_string(),
                    literal.value.line,
                ))),
                _ => Err(RuloxError::RuntimeError {
                    line: literal.value.line,
                    message: "Operand must be a number".to_string(),
                }),
            },
            TokenType::Bang => {
                if Literal::is_truthy(literal.value.token_type) {
                    return Ok(Literal::new(Token::new(
                        TokenType::False,
                        "false".to_string(),
                        literal.value.line,
                    )));
                }
                Ok(Literal::new(Token::new(
                    TokenType::True,
                    "true".to_string(),
                    literal.value.line,
                )))
            }
            _ => Err(RuloxError::RuntimeError {
                line: literal.value.line,
                message: "unary operator not supported".to_string(),
            }),
        }
    }
}
