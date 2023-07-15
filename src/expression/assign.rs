use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    rulox_error::RuloxResult,
    scanner::token::Token,
    statement::{Environment, VarValue},
};

use super::{Expr, ExprTrait};

#[derive(Clone)]
pub struct Assign {
    name: Token,
    value: Expr,
}

impl Assign {
    pub fn new(name: Token, value: Expr) -> Self {
        Self { name, value }
    }
}

impl Display for Assign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //* write!(f, "{}", parenthisize(&self.name.lexeme, &[&self.value]))
        write!(f, "{}", self.value)
    }
}

impl ExprTrait for Assign {
    fn execute(&self, env: Rc<RefCell<Environment>>) -> RuloxResult<VarValue> {
        let value = self.value.execute(Rc::clone(&env))?;

        env.borrow_mut().assign(&self.name, &value)?;

        Ok(value)
    }

    fn get_token(&self) -> &Token {
        &self.name
    }
}
