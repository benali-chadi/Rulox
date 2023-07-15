use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    expression::{Expr, Literal},
    rulox_error::{RuloxError, RuloxResult},
    scanner::token::{Token, TokenType},
};

use super::{Block, Environment, RuloxCallableTrait, StmtTrait, VarValue};

#[derive(Clone)]
pub struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Block,
}

impl Function {
    pub fn new(name: Token, params: Vec<Token>, body: Block) -> Self {
        Self { name, params, body }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fun {} >", self.name)
    }
}

impl StmtTrait for Function {
    fn execute(&self, env: Rc<RefCell<Environment>>) -> RuloxResult<()> {
        env.borrow_mut().define(
            &self.name.lexeme,
            super::VarValue::Callable(Rc::new(RefCell::new(self.clone()))),
        );
        Ok(())
    }
}

impl RuloxCallableTrait for Function {
    fn call(&mut self, arguments: &[Expr], env: Rc<RefCell<Environment>>) -> RuloxResult<VarValue> {
        let current_env = Rc::new(RefCell::new(Environment::from(Some(Rc::clone(&env)))));

        for (index, value) in self.params.iter().enumerate() {
            let val = match arguments.get(index) {
                Some(expr) => expr.execute(Rc::clone(&current_env))?,
                None => {
                    return Err(RuloxError::RuntimeError {
                        line: value.line,
                        message: "Arguments don't match parameters".to_string(),
                    })
                }
            };

            current_env.borrow_mut().define(&value.lexeme, val);
        }

        self.body.execute(Rc::clone(&current_env))?;

        Ok(VarValue::Literal(Literal::new(Token::new(
            TokenType::Nil,
            "nil",
            1,
        ))))
    }

    fn arity(&self) -> usize {
        self.params.len()
    }
}
