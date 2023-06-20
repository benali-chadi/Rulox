use std::fmt::Display;

use crate::{
    error_reporintg::MyError,
    parser::expression::utils,
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
    fn evaluate(&self) -> Result<Literal, MyError> {
        let literal = self.right.evaluate()?;

        match self.operator.token_type {
            TokenType::Minus => match literal.value.token_type {
                TokenType::Number(val) => Ok(Literal::new(Token::new(
                    TokenType::Number(-val),
                    (-val).to_string(),
                    literal.value.line,
                ))),
                _ => Err(MyError::RuntimeError {
                    line: literal.value.line,
                    message: "Literal is not a number".to_string(),
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
            _ => Err(MyError::RuntimeError {
                line: literal.value.line,
                message: "unary operator not supported".to_string(),
            }),
        }
    }
}
