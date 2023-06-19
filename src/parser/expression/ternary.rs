use std::fmt::Display;

use crate::parser::expression::utils;

use super::{Expr, ExprTrait};

pub struct Ternary {
    pub cond: Expr,
    pub first: Expr,
    pub second: Expr,
}

impl Ternary {
    pub fn new(cond: Expr, first: Expr, second: Expr) -> Self {
        Self {
            cond,
            first,
            second,
        }
    }
}

impl Display for Ternary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            utils::parenthisize("Ternary", &[&self.cond, &self.first, &self.second])
        )
    }
}

impl ExprTrait for Ternary {
    fn evaluate(&self) -> bool {
        true
    }
}
