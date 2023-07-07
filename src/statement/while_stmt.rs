use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    expression::{Expr, Literal},
    rulox_error::RuloxError,
};

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
        while Literal::is_truthy(&self.condition.execute(Rc::clone(&env))?.value.token_type) {
            match self.body.execute(Rc::clone(&env)) {
                Ok(_) => {}
                Err(err) => match err {
                    RuloxError::BreakError { .. } => {
                        break;
                    }
                    RuloxError::ContinueError { .. } => {
                        continue;
                    }
                    _ => {
                        return Err(err);
                    }
                },
            }
        }

        Ok(())
    }
}
