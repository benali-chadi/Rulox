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
    // pub fn new() -> Self {
    //     Self {
    //         values: HashMap::new(),
    //     }
    // }
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
}
