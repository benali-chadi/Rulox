use std::fmt::Display;

use crate::{rulox_error::RuloxError, scanner::token::Token};

use super::StmtTrait;

pub struct Break {
    value: Token,
}

impl Break {
    pub fn new(value: Token) -> Self {
        Self { value }
    }
}

impl Display for Break {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "break")
    }
}

impl StmtTrait for Break {
    fn execute(
        &self,
        _env: std::rc::Rc<std::cell::RefCell<super::Environment>>,
    ) -> crate::rulox_error::RuloxResult<()> {
        Err(RuloxError::BreakError {
            line: self.value.line,
        })
    }
}
