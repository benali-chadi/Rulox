use std::fmt::Display;

use crate::{rulox_error::RuloxError, scanner::token::Token};

use super::StmtTrait;

pub struct Continue {
    value: Token,
}
impl Continue {
    pub fn new(value: Token) -> Self {
        Self { value }
    }
}

impl Display for Continue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "continue")
    }
}

impl StmtTrait for Continue {
    fn execute(
        &self,
        _env: std::rc::Rc<std::cell::RefCell<super::Environment>>,
    ) -> crate::rulox_error::RuloxResult<()> {
        Err(RuloxError::ContinueError {
            line: self.value.line,
        })
    }
}
