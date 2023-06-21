use std::fmt::Display;

pub use binary::Binary;
pub use grouping::Grouping;
pub use literal::Literal;
pub use unary::Unary;

use crate::rulox_error::RuloxError;
mod binary;
mod grouping;
mod literal;
mod unary;
mod utils;

pub trait ExprTrait: Display {
    fn interpret(&self) -> Result<Literal, RuloxError>;
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
    pub fn interpret(&self) -> Result<Literal, RuloxError> {
        self.expression.interpret()
    }
}
