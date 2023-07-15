use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    expression::{Expr, Literal},
    rulox_error::RuloxResult,
    scanner::token::{Token, TokenType},
};

use super::{Environment, StmtTrait, VarValue};

#[derive(Clone)]
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
    fn execute(&self, env: Rc<RefCell<Environment>>) -> RuloxResult<()> {
        match &self.initializer {
            Some(expr) => {
                let value = expr.execute(Rc::clone(&env))?;
                env.borrow_mut().define(&self.name.lexeme, value);
                Ok(())
            }
            None => {
                env.borrow_mut().define(
                    &self.name.lexeme,
                    VarValue::Literal(Literal::new(Token::new(TokenType::Nil, "nil", 1))),
                );
                Ok(())
            }
        }
    }
}
