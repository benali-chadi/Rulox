use std::fmt::Display;

use crate::expression::Expr;

use super::{environment::Environment, StmtTrait};

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
    fn execute(&self, env: &mut Environment) -> Result<(), crate::rulox_error::RuloxError> {
        self.expression.execute(env)?;
        Ok(())
    }
}
