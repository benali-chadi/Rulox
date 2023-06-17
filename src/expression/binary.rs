use std::fmt::Display;

use crate::{expression::utils, scanner::token::Token};

use super::{Expr, ExprTrait};

pub struct Binary {
    pub left: Expr,
    pub operator: Token,
    pub right: Expr,
}

impl Binary {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            utils::parenthisize(&self.operator.lexeme, &[&self.left, &self.right])
        )
    }
}

impl ExprTrait for Binary {
    fn evaluate(&self) -> bool {
        true
    }
}
