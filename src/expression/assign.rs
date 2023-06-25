use crate::scanner::token::Token;

use super::Expr;

pub struct Assign {
    name: Token,
    value: Expr,
}

impl Assign {
    pub fn new(name: Token, value: Expr) -> Self {
        Self { name, value }
    }
}

// TODO: Implement the Expr Trait
