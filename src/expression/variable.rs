use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    rulox_error::RuloxResult,
    scanner::token::{Token, TokenType},
    statement::{Environment, VarValue},
};

use super::{ExprTrait, Literal};

#[derive(Debug, Clone)]
pub struct Variable {
    name: Token,
}

impl Variable {
    pub fn new(name: &Token) -> Self {
        Self { name: name.clone() }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.lexeme)
    }
}

impl ExprTrait for Variable {
    fn execute(&self, env: Rc<RefCell<Environment>>) -> RuloxResult<Literal> {
        match env.borrow().get(&self.name) {
            Ok(value) => match value {
                VarValue::Literal(val) => Ok(val.clone()),

                VarValue::Callable(_) => Ok(Literal {
                    value: Token::new(
                        TokenType::String(self.name.lexeme.to_string()),
                        &self.name.lexeme,
                        1,
                    ),
                }),
            },
            Err(err) => Err(err),
        }
    }

    fn get_token(&self) -> &Token {
        &self.name
    }
}
