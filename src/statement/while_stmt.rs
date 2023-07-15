use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::expression::{Expr, Literal};

use super::{Environment, Stmt, StmtTrait};

#[derive(Clone)]
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
        while Literal::is_truthy(match &self.condition.execute(Rc::clone(&env))? {
            super::VarValue::Literal(val) => &val.value.token_type,
            super::VarValue::Callable(_) => unreachable!(),
        }) {
            self.body.execute(Rc::clone(&env))?;
        }

        Ok(())
    }
}
