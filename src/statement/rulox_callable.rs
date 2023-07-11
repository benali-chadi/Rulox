use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{expression::Expr, rulox_error::RuloxResult};

use super::{Environment, VarValue};

pub trait RuloxCallableTrait: Display {
    fn call(&mut self, arguments: &[Expr], env: Rc<RefCell<Environment>>) -> RuloxResult<VarValue>;
    fn arity(&self) -> usize;
}

// pub struct RuloxCallable {
//     pub callable: Box<dyn RuloxCallableTrait>,
// }

// impl RuloxCallable {
//     pub fn new(callable: Box<dyn RuloxCallableTrait>) -> Self {
//         Self { callable }
//     }

//     pub fn call(
//         &mut self,
//         arguments: &[Expr],
//         env: Rc<RefCell<Environment>>,
//     ) -> RuloxResult<VarValue> {
//         self.callable.call(arguments, env)
//     }
//     fn arity(&self) -> usize {
//         self.callable.arity()
//     }
// }

// pub struct FunctionCallable {
//     function: Function,
// }

// impl FunctionCallable {
//     pub fn new(function: &Function) -> Self {
//         Self {
//             function: Function {
//                 name: function.name.clone(),
//                 params: function.params.clone(),
//                 body: FunctionCallable::clone_body(&function.body),
//             },
//         }
//     }

//     fn clone_body(body: &Block) -> Block {
//         let mut statements: Vec<Stmt> = vec![];

//         for stmt in body.statements.iter() {
//             statements.push(stmt);
//         }

//         Block::new(statements)
//     }
// }

// impl Display for FunctionCallable {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "<fun {} >", self.function)
//     }
// }

// impl RuloxCallableTrait for FunctionCallable {
//     fn call(&mut self, arguments: &[Expr], env: Rc<RefCell<Environment>>) -> RuloxResult<VarValue> {
//         let current_env = Rc::new(RefCell::new(Environment::from(Some(Rc::clone(&env)))));

//         for (index, value) in self.function.params.iter().enumerate() {
//             let val = match arguments.get(index) {
//                 Some(expr) => expr.execute(Rc::clone(&current_env))?,
//                 None => {
//                     return Err(RuloxError::RuntimeError {
//                         line: value.line,
//                         message: "Arguments don't match parameters".to_string(),
//                     })
//                 }
//             };

//             current_env
//                 .borrow_mut()
//                 .define(&value.lexeme, VarValue::Literal(val));
//         }

//         self.function.body.execute(Rc::clone(&current_env))?;

//         Ok(VarValue::Literal(Literal::new(Token::new(
//             TokenType::Nil,
//             "nil",
//             1,
//         ))))
//     }

//     fn arity(&self) -> usize {
//         self.function.params.len()
//     }
// }
