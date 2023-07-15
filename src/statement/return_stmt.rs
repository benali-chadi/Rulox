use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    expression::Expr,
    rulox_error::{RuloxError, RuloxResult},
    scanner::token::Token,
};

use super::{Environment, StmtTrait};

#[derive(Clone)]
pub struct Return {
    keyword: Token,
    value: Option<Expr>,
}

impl Return {
    pub fn new(keyword: &Token, value: Option<Expr>) -> Self {
        Self {
            keyword: keyword.clone(),
            value,
        }
    }
}

impl Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.keyword)
    }
}

impl StmtTrait for Return {
    fn execute(&self, env: Rc<RefCell<Environment>>) -> RuloxResult<()> {
        match &self.value {
            Some(val) => Err(RuloxError::ReturnError {
                line: self.keyword.line,
                value: Some(val.execute(Rc::clone(&env))?),
            }),
            None => Err(RuloxError::ReturnError {
                line: self.keyword.line,
                value: None,
            }),
        }
    }
}
