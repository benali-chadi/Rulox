use std::fmt::Display;

use crate::rulox_error::RuloxResult;

use super::{environment::Environment, Stmt, StmtTrait};

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
        write!(f, "This is a Block")
    }
}

impl StmtTrait for Block {
    fn execute(&self, env: &mut Environment) -> RuloxResult<()> {
        let mut current_env = Environment::from(Some(Box::new(env.clone())));

        for stmt in &self.statements {
            stmt.execute(&mut current_env)?;
        }

        Ok(())
    }
}
