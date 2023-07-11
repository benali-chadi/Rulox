use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::expression::Expr;

use super::{Environment, StmtTrait};

#[derive(Clone)]
pub struct Expression {
    pub expression: Expr,
}

impl Expression {
    pub fn new(expression: Expr) -> Self {
        Self { expression }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expression)
    }
}

impl StmtTrait for Expression {
    fn execute(&self, env: Rc<RefCell<Environment>>) -> Result<(), crate::rulox_error::RuloxError> {
        self.expression.execute(Rc::clone(&env))?;
        Ok(())
    }
}
