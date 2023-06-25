use std::fmt::Display;

use crate::{expression::Expr, rulox_error::RuloxResult, scanner::token::Token};

use super::StmtTrait;

pub struct Var {
    pub name: Token,
    pub initializer: Option<Expr>,
}

impl Var {
    pub fn new(name: Token, initializer: Option<Expr>) -> Self {
        Self { name, initializer }
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {} value: {}",
            self.name,
            match &self.initializer {
                Some(expr) => expr.to_string(),
                None => "nil".to_string(),
            }
        )
    }
}
// TODO: Implement the Stmt Trait
impl StmtTrait for Var {
    fn execute(&self) -> RuloxResult<()> {
        Ok(())
    }
}
