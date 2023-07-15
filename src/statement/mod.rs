use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::rulox_error::RuloxResult;

pub use block::Block;
pub use environment::*;
pub use expression::Expression;
pub use function_stmt::Function;
pub use if_stmt::If;
pub use print::Print;
pub use return_stmt::Return;
pub use rulox_callable::*;
pub use var::Var;
pub use while_stmt::While;

mod block;
mod environment;
mod expression;
mod function_stmt;
mod if_stmt;
mod print;
mod return_stmt;
mod rulox_callable;
mod var;
mod while_stmt;

pub trait StmtTrait: Display + StmtClone {
    fn execute(&self, env: Rc<RefCell<Environment>>) -> RuloxResult<()>;
}

pub trait StmtClone {
    fn clone_box(&self) -> Box<dyn StmtTrait>;
}

impl<T> StmtClone for T
where
    T: 'static + StmtTrait + Clone,
{
    fn clone_box(&self) -> Box<dyn StmtTrait> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn StmtTrait> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct Stmt {
    pub statement: Box<dyn StmtTrait>,
}

impl Stmt {
    pub fn new(statement: Box<dyn StmtTrait>) -> Self {
        Self { statement }
    }

    pub fn execute(&self, env: Rc<RefCell<Environment>>) -> RuloxResult<()> {
        self.statement.execute(Rc::clone(&env))
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.statement)
    }
}
