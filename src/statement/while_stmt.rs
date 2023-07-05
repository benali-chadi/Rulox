use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::expression::{Expr, Literal};

use super::{Environment, Stmt, StmtTrait};

pub struct While {
    condition: Expr,
    body: Stmt,
}

impl While {
    pub fn new(condition: Expr, body: Stmt) -> Self {
        Self { condition, body }
    }
}

impl Display for While {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.condition)
    }
}

impl StmtTrait for While {
    fn execute(&self, env: Rc<RefCell<Environment>>) -> crate::rulox_error::RuloxResult<()> {
        let mut cond =
            Literal::is_truthy(&self.condition.execute(Rc::clone(&env))?.value.token_type);
        while cond {
            cond = Literal::is_truthy(&self.condition.execute(Rc::clone(&env))?.value.token_type);
            self.body.execute(Rc::clone(&env))?;
        }

        Ok(())
    }
}
