use std::fmt::Display;

use crate::{expression::Expr, rulox_error::RuloxResult};

use super::{environment::Environment, StmtTrait};

pub struct Print {
    pub expression: Expr,
}

impl Print {
    pub fn new(expression: Expr) -> Self {
        Self { expression }
    }
}

impl Display for Print {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expression)
    }
}

// TODO: Implement the Stmt Trait

impl StmtTrait for Print {
    fn execute(&self, env: &mut Environment) -> RuloxResult<()> {
        let literal = self.expression.execute(env)?;
        println!("{}", literal);
        Ok(())
    }
}
