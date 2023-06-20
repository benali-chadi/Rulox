use std::fmt::Display;

pub use binary::Binary;
pub use grouping::Grouping;
pub use literal::Literal;
pub use unary::Unary;

use crate::error_reporintg::MyError;
mod binary;
mod grouping;
mod literal;
mod unary;
mod utils;

pub trait ExprTrait: Display {
    fn evaluate(&self) -> Result<Literal, MyError>;
}

pub struct Expr {
    expression: Box<dyn ExprTrait>,
}

impl Expr {
    pub fn new(expression: Box<dyn ExprTrait>) -> Self {
        Self { expression }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expression)
    }
}

impl Expr {
    pub fn evaluate(&self) -> Result<Literal, MyError> {
        self.expression.evaluate()
    }
}
