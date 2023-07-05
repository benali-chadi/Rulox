use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    expression::utils,
    rulox_error::RuloxResult,
    scanner::token::{Token, TokenType},
    statement::Environment,
};

use super::{Expr, ExprTrait, Literal};

pub struct Logical {
    left: Expr,
    operator: Token,
    right: Expr,
}

impl Logical {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl Display for Logical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            utils::parenthisize(&self.operator.lexeme, &[&self.left, &self.right])
        )
    }
}

impl ExprTrait for Logical {
    fn execute(&self, env: Rc<RefCell<Environment>>) -> RuloxResult<super::Literal> {
        let left = self.left.execute(Rc::clone(&env))?;

        match self.operator.token_type {
            TokenType::Or => {
                if Literal::is_truthy(&left.value.token_type) {
                    return Ok(left);
                }
            }
            // Then its And
            _ => {
                if !Literal::is_truthy(&left.value.token_type) {
                    return Ok(left);
                }
            }
        }

        self.right.execute(Rc::clone(&env))
    }

    fn get_token(&self) -> &Token {
        &self.operator
    }
}
