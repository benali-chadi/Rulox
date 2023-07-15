use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    rulox_error::{RuloxError, RuloxResult},
    scanner::token::{Token, TokenType},
    statement::{Environment, VarValue},
};

use super::{Expr, ExprTrait, Literal};

#[derive(Clone)]
pub struct Call {
    callee: Expr,
    paren: Token,
    arguments: Vec<Expr>,
}

impl Call {
    pub fn new(callee: Expr, paren: Token, arguments: Vec<Expr>) -> Self {
        Self {
            callee,
            paren,
            arguments,
        }
    }
}

impl Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.callee)
    }
}

impl ExprTrait for Call {
    fn execute(&self, env: Rc<RefCell<Environment>>) -> RuloxResult<VarValue> {
        let callee = self.callee.execute(Rc::clone(&env))?;

        match &callee {
            VarValue::Callable(fun) => match fun.borrow().call(&self.arguments, Rc::clone(&env)) {
                Ok(val) => Ok(val),
                Err(err) => match err {
                    RuloxError::ReturnError { value, .. } => match &value {
                        Some(val) => Ok(val.clone()),
                        None => Ok(VarValue::Literal(Literal::new(Token::new(
                            TokenType::Nil,
                            "nil",
                            1,
                        )))),
                    },
                    _ => Err(err),
                },
            },
            VarValue::Literal(callee) => Err(RuloxError::RuntimeError {
                line: callee.value.line,
                message: "Can only call functions and classes.".to_string(),
            }),
        }
    }

    fn get_token(&self) -> &Token {
        &self.paren
    }
}
