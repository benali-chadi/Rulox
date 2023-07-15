use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    expression::utils,
    rulox_error::RuloxResult,
    scanner::token::Token,
    statement::{Environment, VarValue},
};

use super::{Expr, ExprTrait};

#[derive(Clone)]
pub struct Grouping {
    pub expression: Expr,
}

impl Grouping {
    pub fn new(expression: Expr) -> Self {
        Self { expression }
    }
}

impl Display for Grouping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", utils::parenthisize("group", &[&self.expression]))
    }
}

impl ExprTrait for Grouping {
    fn execute(&self, env: Rc<RefCell<Environment>>) -> RuloxResult<VarValue> {
        self.expression.execute(Rc::clone(&env))
    }

    fn get_token(&self) -> &Token {
        self.expression.expression.get_token()
    }
}
