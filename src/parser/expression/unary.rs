use std::fmt::Display;

use crate::{parser::expression::utils, scanner::token::Token};

use super::{Expr, ExprTrait};

pub struct Unary {
    pub operator: Token,
    pub right: Expr,
}

impl Unary {
    pub fn new(operator: Token, right: Expr) -> Self {
        Self { operator, right }
    }
}

impl Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            utils::parenthisize(&self.operator.lexeme, &[&self.right])
        )
    }
}

impl ExprTrait for Unary {
    fn evaluate(&self) -> bool {
        true
    }
}
