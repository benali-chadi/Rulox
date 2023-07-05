use std::{cell::RefCell, fmt::Display, rc::Rc};

pub use assign::Assign;
pub use binary::Binary;
pub use grouping::Grouping;
pub use literal::Literal;
pub use logical::Logical;
pub use unary::Unary;
pub use variable::Variable;

use crate::{rulox_error::RuloxResult, scanner::token::Token, statement::Environment};
mod assign;
mod binary;
mod grouping;
mod literal;
mod logical;
mod unary;
mod utils;
mod variable;

pub trait ExprTrait: Display {
    fn execute(&self, env: Rc<RefCell<Environment>>) -> RuloxResult<Literal>;
    fn get_token(&self) -> &Token;
}

pub struct Expr {
    pub expression: Box<dyn ExprTrait>,
}

impl Expr {
    pub fn new(expression: Box<dyn ExprTrait>) -> Self {
        Self { expression }
    }
    pub fn execute(&self, env: Rc<RefCell<Environment>>) -> RuloxResult<Literal> {
        self.expression.execute(env)
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expression)
    }
}
