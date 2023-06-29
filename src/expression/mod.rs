use std::fmt::Display;

pub use binary::Binary;
pub use grouping::Grouping;
pub use literal::Literal;
pub use unary::Unary;

use crate::{rulox_error::RuloxResult, scanner::token::Token, statement::environment::Environment};
pub mod assign;
mod binary;
mod grouping;
mod literal;
mod unary;
mod utils;
pub mod variable;

pub trait ExprTrait: Display {
    fn execute(&self, env: &mut Environment) -> RuloxResult<Literal>;
    fn get_token(&self) -> &Token;
}

pub struct Expr {
    pub expression: Box<dyn ExprTrait>,
}

impl Expr {
    pub fn new(expression: Box<dyn ExprTrait>) -> Self {
        Self { expression }
    }
    pub fn execute(&self, env: &mut Environment) -> RuloxResult<Literal> {
        self.expression.execute(env)
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expression)
    }
}
