use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{expression::Expr, rulox_error::RuloxResult};

use super::{Environment, VarValue};

pub trait RuloxCallableTrait: Display {
    fn call(&mut self, arguments: &[Expr], env: Rc<RefCell<Environment>>) -> RuloxResult<VarValue>;
    fn arity(&self) -> usize;
}
