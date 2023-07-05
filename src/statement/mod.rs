use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::rulox_error::RuloxResult;

pub use block::Block;
pub use environment::Environment;
pub use expression::Expression;
pub use if_stmt::If;
pub use print::Print;
pub use var::Var;
pub use while_stmt::While;

mod block;
mod environment;
mod expression;
mod if_stmt;
mod print;
mod var;
mod while_stmt;

pub trait StmtTrait: Display {
    fn execute(&self, env: Rc<RefCell<Environment>>) -> RuloxResult<()>;
}

pub struct Stmt {
    pub statement: Box<dyn StmtTrait>,
}

impl Stmt {
    pub fn new(statement: Box<dyn StmtTrait>) -> Self {
        Self { statement }
    }

    pub fn execute(&self, env: Rc<RefCell<Environment>>) -> RuloxResult<()> {
        self.statement.execute(env)
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.statement)
    }
}
