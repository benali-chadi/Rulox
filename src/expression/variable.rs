use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{rulox_error::RuloxResult, scanner::token::Token, statement::Environment};

use super::{ExprTrait, Literal};

#[derive(Debug)]
pub struct Variable {
    name: Token,
}

impl Variable {
    pub fn new(name: &Token) -> Self {
        Self { name: name.clone() }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.lexeme)
    }
}

impl ExprTrait for Variable {
    fn execute(&self, env: Rc<RefCell<Environment>>) -> RuloxResult<Literal> {
        env.borrow().get(&self.name)
    }

    fn get_token(&self) -> &Token {
        &self.name
    }
}
