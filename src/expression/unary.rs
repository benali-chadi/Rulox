use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    expression::utils,
    rulox_error::{RuloxError, RuloxResult},
    scanner::token::{Token, TokenType},
    statement::{Environment, VarValue},
};

use super::{Expr, ExprTrait, Literal};

#[derive(Clone)]
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
    fn execute(&self, env: Rc<RefCell<Environment>>) -> RuloxResult<VarValue> {
        let literal = self.right.execute(Rc::clone(&env))?;

        match literal {
            VarValue::Literal(literal) => match self.operator.token_type {
                TokenType::Minus => match literal.value.token_type {
                    TokenType::Number(val) => Ok(VarValue::Literal(Literal::new(Token::new(
                        TokenType::Number(-val),
                        &(-val).to_string(),
                        literal.value.line,
                    )))),
                    _ => Err(RuloxError::RuntimeError {
                        line: literal.value.line,
                        message: "Operand must be a number".to_string(),
                    }),
                },
                TokenType::Bang => {
                    if Literal::is_truthy(&literal.value.token_type) {
                        return Ok(VarValue::Literal(Literal::new(Token::new(
                            TokenType::False,
                            "false",
                            literal.value.line,
                        ))));
                    }
                    Ok(VarValue::Literal(Literal::new(Token::new(
                        TokenType::True,
                        "true",
                        literal.value.line,
                    ))))
                }
                _ => Err(RuloxError::RuntimeError {
                    line: literal.value.line,
                    message: "unary operator not supported".to_string(),
                }),
            },

            _ => unreachable!(),
        }
    }

    fn get_token(&self) -> &Token {
        &self.operator
    }
}
