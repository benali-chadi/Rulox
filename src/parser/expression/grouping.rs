use std::fmt::Display;

use crate::{error_reporintg::MyError, parser::expression::utils};

use super::{Expr, ExprTrait, Literal};

pub struct Grouping {
    pub expression: Expr,
}

impl Grouping {
    pub fn new(expression: Expr) -> Self {
        Self { expression }
    }
}

impl Display for Grouping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", utils::parenthisize("group", &[&self.expression]))
    }
}

impl ExprTrait for Grouping {
    fn evaluate(&self) -> Result<Literal, MyError> {
        self.expression.evaluate()
    }
}
