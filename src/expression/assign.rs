use std::fmt::Display;

use crate::{
    expression::utils::parenthisize, rulox_error::RuloxResult, scanner::token::Token,
    statement::environment::Environment,
};

use super::{Expr, ExprTrait};

pub struct Assign {
    name: Token,
    value: Expr,
}

impl Assign {
    pub fn new(name: Token, value: Expr) -> Self {
        Self { name, value }
    }
}

impl Display for Assign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", parenthisize(&self.name.lexeme, &[&self.value]))
    }
}

impl ExprTrait for Assign {
    fn execute(&self, env: &mut Environment) -> RuloxResult<super::Literal> {
        let value = self.value.execute(env)?;

        env.assign(self.name.clone(), value.clone())?;

        Ok(value)
    }

    fn get_token(&self) -> &Token {
        &self.name
    }
}
