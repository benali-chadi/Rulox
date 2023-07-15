use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    expression::{Expr, Literal},
    rulox_error::RuloxResult,
};

use super::{Environment, Stmt, StmtTrait};

#[derive(Clone)]
pub struct If {
    condition: Expr,
    then_branch: Stmt,
    else_branch: Option<Stmt>,
}

impl If {
    pub fn new(condition: Expr, then_branch: Stmt, else_brance: Option<Stmt>) -> Self {
        Self {
            condition,
            then_branch,
            else_branch: else_brance,
        }
    }
}

impl Display for If {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.condition)
    }
}

impl StmtTrait for If {
    fn execute(&self, env: Rc<RefCell<Environment>>) -> RuloxResult<()> {
        match self.condition.execute(Rc::clone(&env))? {
            super::VarValue::Literal(val) => {
                if Literal::is_truthy(&val.value.token_type) {
                    return self.then_branch.execute(Rc::clone(&env));
                } else if let Some(statement) = &self.else_branch {
                    return statement.execute(Rc::clone(&env));
                }
            }
            super::VarValue::Callable(_) => todo!(),
        }

        Ok(())
    }
}
