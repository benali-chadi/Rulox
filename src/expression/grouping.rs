use std::fmt::Display;

use crate::{expression::utils, rulox_error::RuloxError};

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
    fn interpret(&self) -> Result<Literal, RuloxError> {
        self.expression.interpret()
    }
}
