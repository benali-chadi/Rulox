use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    expression::Literal,
    rulox_error::{RuloxError, RuloxResult},
    scanner::token::Token,
};

use super::RuloxCallableTrait;

// use super::RuloxCallable;

pub enum VarValue {
    Literal(Literal),
    Callable(Rc<RefCell<dyn RuloxCallableTrait>>),
}

#[derive(Default)]
pub struct Environment {
    values: HashMap<String, VarValue>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn from(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            values: HashMap::default(),
            enclosing,
        }
    }
    pub fn define(&mut self, name: &str, value: VarValue) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &Token) -> RuloxResult<VarValue> {
        if let Some(value) = self.values.get(&name.lexeme) {
            // return Ok(value);
            match value {
                VarValue::Literal(val) => return Ok(VarValue::Literal(val.clone())),
                VarValue::Callable(val) => return Ok(VarValue::Callable(Rc::clone(val))),
            }
        }

        if let Some(env) = &self.enclosing {
            match env.borrow().get(name) {
                Ok(value) => return Ok(value),
                Err(err) => {
                    return Err(err);
                }
            }
        }

        Err(RuloxError::RuntimeError {
            line: name.line,
            message: format!("Undefined variable '{}'.", name.lexeme),
        })
    }

    pub fn assign(&mut self, name: &Token, value: &VarValue) -> RuloxResult<()> {
        if let std::collections::hash_map::Entry::Occupied(mut e) =
            self.values.entry(name.lexeme.to_string())
        {
            let val = match value {
                VarValue::Literal(v) => VarValue::Literal(v.clone()),
                VarValue::Callable(v) => VarValue::Callable(Rc::clone(v)),
            };

            e.insert(val);
            return Ok(());
        }

        if let Some(env) = &mut self.enclosing {
            if env.borrow_mut().assign(name, value).is_ok() {
                return Ok(());
            }
        }

        Err(RuloxError::RuntimeError {
            line: name.line,
            message: format!("Undefined variable '{}'.", name.lexeme),
        })
    }
}
