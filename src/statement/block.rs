use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::rulox_error::RuloxResult;

use super::{Environment, Stmt, StmtTrait};

pub struct Block {
    pub statements: Vec<Stmt>,
}

impl Block {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Self { statements }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "This is a Block")?;
        for stmt in &self.statements {
            write!(f, "{}\n", stmt)?;
        }
        Ok(())
    }
}

impl StmtTrait for Block {
    fn execute(&self, env: Rc<RefCell<Environment>>) -> RuloxResult<()> {
        let current_env = Rc::new(RefCell::new(Environment::from(Some(Rc::clone(&env)))));

        for stmt in &self.statements {
            stmt.execute(Rc::clone(&current_env))?;
        }

        Ok(())
    }
}
