use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    rulox_error::RuloxError,
    scanner::token::{Token, TokenType},
    statement::{Environment, VarValue},
};

use super::{Expr, ExprTrait, Literal};

#[derive(Clone)]
pub struct Call {
    callee: Expr,
    paren: Token,
    arguments: Vec<Expr>,
}

impl Call {
    pub fn new(callee: Expr, paren: Token, arguments: Vec<Expr>) -> Self {
        Self {
            callee,
            paren,
            arguments,
        }
    }
}

impl Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.callee)
    }
}

impl ExprTrait for Call {
    fn execute(
        &self,
        env: Rc<RefCell<Environment>>,
    ) -> crate::rulox_error::RuloxResult<super::Literal> {
        // TODO: Modify this later
        match self.callee.expression.get_token().token_type {
            TokenType::Identifier(_) => {
                let callee = self.callee.expression.get_token();
                let mut args: Vec<_> = Vec::new();

                for arg in &self.arguments {
                    args.push(arg.execute(Rc::clone(&env))?);
                }

                let function = env.borrow().get(callee)?;

                match function {
                    VarValue::Callable(fun) => {
                        fun.borrow_mut().call(&self.arguments, Rc::clone(&env))?;
                        Ok(Literal::new(Token::new(TokenType::Nil, "nil", 1)))
                    }
                    VarValue::Literal(_) => {
                        return Err(RuloxError::RuntimeError {
                            line: callee.line,
                            message: "Can only call functions and classes.".to_string(),
                        });
                    }
                }
            }
            _ => {
                return Err(RuloxError::RuntimeError {
                    line: self.paren.line,
                    message: "Can only call functions and classes.".to_string(),
                })
            }
        }
        // let callee = self.callee.execute(Rc::clone(&env))?;
        // println!("Calling {}", callee);
        //
        // let mut args: Vec<_> = Vec::new();
        //
        // for arg in &self.arguments {
        //     args.push(arg.execute(Rc::clone(&env))?);
        // }
        //
        // let function = env.borrow().get(&callee.value)?;
        //
        // match function {
        //     VarValue::Callable(fun) => {
        //         fun.borrow_mut().call(&self.arguments, Rc::clone(&env))?;
        //         Ok(Literal::new(Token::new(TokenType::Nil, "nil", 1)))
        //     }
        //     VarValue::Literal(_) => {
        //         return Err(RuloxError::RuntimeError {
        //             line: callee.value.line,
        //             message: "Can only call functions and classes.".to_string(),
        //         });
        //     }
        // }
    }

    fn get_token(&self) -> &Token {
        &self.paren
    }
}
