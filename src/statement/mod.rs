use std::fmt::Display;

use crate::rulox_error::RuloxResult;

pub mod block;
pub mod environment;
pub mod expression;
pub mod print;
pub mod var;

pub trait StmtTrait: Display {
    fn execute(&self) -> RuloxResult<()>;
}

pub struct Stmt {
    statement: Box<dyn StmtTrait>,
}

impl Stmt {
    pub fn new(statement: Box<dyn StmtTrait>) -> Self {
        Self { statement }
    }

    pub fn execute(&self) -> RuloxResult<()> {
        self.statement.execute()
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.statement)
    }
}
