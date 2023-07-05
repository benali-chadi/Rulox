use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{expression::Expr, rulox_error::RuloxResult};

use super::{Environment, StmtTrait};

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

impl StmtTrait for Print {
    fn execute(&self, env: Rc<RefCell<Environment>>) -> RuloxResult<()> {
        let literal = self.expression.execute(Rc::clone(&env))?;
        println!("{}", literal);
        Ok(())
    }
}
