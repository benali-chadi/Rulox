use std::fmt::Display;

pub use binary::Binary;
pub use grouping::Grouping;
pub use literal::Literal;
pub use ternary::Ternary;
pub use unary::Unary;
mod binary;
mod grouping;
mod literal;
mod ternary;
mod unary;
mod utils;

pub trait ExprTrait: Display {
    fn evaluate(&self) -> bool;
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
    pub fn eval(&self) -> bool {
        self.expression.evaluate()
    }
}
