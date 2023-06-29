use std::collections::HashMap;

use crate::{
    expression::Literal,
    rulox_error::{RuloxError, RuloxResult},
    scanner::token::Token,
};

#[derive(Default)]
pub struct Environment {
    values: HashMap<String, Literal>,
}

impl Environment {
    pub fn from(values: HashMap<String, Literal>) -> Self {
        Self { values }
    }

    pub fn define(&mut self, name: &str, value: Literal) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, name: Token) -> RuloxResult<&Literal> {
        match self.values.get(&name.lexeme) {
            Some(value) => Ok(value),
            None => Err(RuloxError::RuntimeError {
                line: name.line,
                message: format!("Undefined variable '{}'.", name.lexeme),
            }),
        }
    }

    pub fn assign(&mut self, name: Token, value: Literal) -> RuloxResult<()> {
        // if self.values.contains_key(&name.lexeme) {
        //     self.values.insert(name.lexeme, value);
        //     return Ok(());
        // }
        if let std::collections::hash_map::Entry::Occupied(mut e) =
            self.values.entry(name.lexeme.to_string())
        {
            e.insert(value);
            return Ok(());
        }

        Err(RuloxError::RuntimeError {
            line: name.line,
            message: format!("Undefined variable '{}'.", name.lexeme),
        })
    }
}
