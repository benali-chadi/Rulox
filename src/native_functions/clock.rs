use std::{
    cell::RefCell,
    fmt::Display,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    expression::{Expr, Literal},
    scanner::token::{Token, TokenType},
    statement::{Environment, RuloxCallableTrait, VarValue},
};

pub struct ClockCallable;

impl Display for ClockCallable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn>")
    }
}

impl RuloxCallableTrait for ClockCallable {
    fn call(
        &self,
        _arguments: &[Expr],
        _env: Rc<RefCell<Environment>>,
    ) -> crate::rulox_error::RuloxResult<VarValue> {
        let start = SystemTime::now();
        let now = start.duration_since(UNIX_EPOCH).unwrap().as_secs_f64();

        println!("[this is temporary] {}", now);

        Ok(VarValue::Literal(Literal::new(Token::new(
            TokenType::Number(now),
            "",
            1,
        ))))
    }

    fn arity(&self) -> usize {
        0
    }
}
