use std::fmt::Display;

use crate::{
    error_reporintg::MyError,
    parser::expression::utils,
    scanner::token::{Token, TokenType},
};

use super::{Expr, ExprTrait, Literal};

pub struct Binary {
    pub left: Expr,
    pub operator: Token,
    pub right: Expr,
}

impl Binary {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            utils::parenthisize(&self.operator.lexeme, &[&self.left, &self.right])
        )
    }
}

impl ExprTrait for Binary {
    fn evaluate(&self) -> Result<Literal, MyError> {
        let left = self.left.evaluate()?;
        let right = self.right.evaluate()?;

        match self.operator.token_type {
            TokenType::Minus => match (left.value.token_type, right.value.token_type) {
                (TokenType::Number(lval), TokenType::Number(rval)) => {
                    let value = lval - rval;
                    return Ok(Literal::new(Token::new(
                        TokenType::Number(value),
                        value.to_string(),
                        left.value.line,
                    )));
                }
                _ => {
                    return Err(MyError::RuntimeError { line: left.value.line, message: "for the '-' binary oprator, the left and right operands must be of type Number".to_string() });
                }
            },

            TokenType::Slash => match (left.value.token_type, right.value.token_type) {
                (TokenType::Number(lval), TokenType::Number(rval)) => {
                    let value = lval / rval;
                    return Ok(Literal::new(Token::new(
                        TokenType::Number(value),
                        value.to_string(),
                        left.value.line,
                    )));
                }
                _ => {
                    return Err(MyError::RuntimeError { line: left.value.line, message: "for the '/' binary oprator, the left and right operands must be of type Number".to_string() });
                }
            },

            TokenType::Star => match (left.value.token_type, right.value.token_type) {
                (TokenType::Number(lval), TokenType::Number(rval)) => {
                    let value = lval * rval;
                    return Ok(Literal::new(Token::new(
                        TokenType::Number(value),
                        value.to_string(),
                        left.value.line,
                    )));
                }
                _ => {
                    return Err(MyError::RuntimeError { line: left.value.line, message: "for the '*' binary oprator, the left and right operands must be of type Number".to_string() });
                }
            },
            TokenType::Plus => match (left.value.token_type, right.value.token_type) {
                (TokenType::Number(lval), TokenType::Number(rval)) => {
                    let value = lval + rval;
                    return Ok(Literal::new(Token::new(
                        TokenType::Number(value),
                        value.to_string(),
                        left.value.line,
                    )));
                }
                (TokenType::String(lval), TokenType::String(rval)) => {
                    let value = lval + &rval;
                    return Ok(Literal::new(Token::new(
                        TokenType::String((&value).to_string()),
                        (&value).to_string(),
                        left.value.line,
                    )));
                }
                _ => {
                    return Err(MyError::RuntimeError { line: left.value.line, message: "for the '+' binary oprator, the left and right operands must be of type Number or String".to_string() });
                }
            },
            // Comparison Operators
            TokenType::Greater => match (left.value.token_type, right.value.token_type) {
                (TokenType::Number(lval), TokenType::Number(rval)) => {
                    let val = lval > rval;
                    let value = if val {
                        TokenType::True
                    } else {
                        TokenType::False
                    };
                    return Ok(Literal::new(Token::new(
                        value,
                        val.to_string(),
                        left.value.line,
                    )));
                }
                _ => {
                    return Err(MyError::RuntimeError { line: left.value.line, message: "for the '>' binary oprator, the left and right operands must be of type Number".to_string() });
                }
            },
            TokenType::GreaterEqual => match (left.value.token_type, right.value.token_type) {
                (TokenType::Number(lval), TokenType::Number(rval)) => {
                    let val = lval >= rval;
                    let value = if val {
                        TokenType::True
                    } else {
                        TokenType::False
                    };
                    return Ok(Literal::new(Token::new(
                        value,
                        val.to_string(),
                        left.value.line,
                    )));
                }
                _ => {
                    return Err(MyError::RuntimeError { line: left.value.line, message: "for the '>=' binary oprator, the left and right operands must be of type Number".to_string() });
                }
            },
            TokenType::Less => match (left.value.token_type, right.value.token_type) {
                (TokenType::Number(lval), TokenType::Number(rval)) => {
                    let val = lval < rval;
                    let value = if val {
                        TokenType::True
                    } else {
                        TokenType::False
                    };
                    return Ok(Literal::new(Token::new(
                        value,
                        val.to_string(),
                        left.value.line,
                    )));
                }
                _ => {
                    return Err(MyError::RuntimeError { line: left.value.line, message: "for the '<' binary oprator, the left and right operands must be of type Number".to_string() });
                }
            },
            TokenType::LessEqual => match (left.value.token_type, right.value.token_type) {
                (TokenType::Number(lval), TokenType::Number(rval)) => {
                    let val = lval <= rval;
                    let value = if val {
                        TokenType::True
                    } else {
                        TokenType::False
                    };
                    return Ok(Literal::new(Token::new(
                        value,
                        val.to_string(),
                        left.value.line,
                    )));
                }
                _ => {
                    return Err(MyError::RuntimeError { line: left.value.line, message: "for the '<=' binary oprator, the left and right operands must be of type Number".to_string() });
                }
            },
            TokenType::EqualEqual => match (left.value.token_type, right.value.token_type) {
                (TokenType::Nil, TokenType::Nil) => {
                    return Ok(Literal::new(Token::new(
                        TokenType::True,
                        "true".to_string(),
                        left.value.line,
                    )))
                }
                (TokenType::Nil, ..) | (.., TokenType::Nil) => {
                    return Ok(Literal::new(Token::new(
                        TokenType::False,
                        "false".to_string(),
                        left.value.line,
                    )))
                }
                (TokenType::True, token_type) | (token_type, TokenType::True) => {
                    if Literal::is_truthy(token_type) {
                        return Ok(Literal::new(Token::new(
                            TokenType::True,
                            "true".to_string(),
                            left.value.line,
                        )));
                    } else {
                        return Ok(Literal::new(Token::new(
                            TokenType::False,
                            "false".to_string(),
                            left.value.line,
                        )));
                    }
                }
                (TokenType::False, token_type) | (token_type, TokenType::False) => {
                    if !Literal::is_truthy(token_type) {
                        return Ok(Literal::new(Token::new(
                            TokenType::True,
                            "true".to_string(),
                            left.value.line,
                        )));
                    } else {
                        return Ok(Literal::new(Token::new(
                            TokenType::False,
                            "false".to_string(),
                            left.value.line,
                        )));
                    }
                }
                // match cases:
                // number -> number, number -> string
                (TokenType::Number(lval), TokenType::Number(rval)) => {
                    let val = lval == rval;
                    let value = if val {
                        TokenType::True
                    } else {
                        TokenType::False
                    };

                    return Ok(Literal::new(Token::new(
                        value,
                        val.to_string(),
                        left.value.line,
                    )));
                }

                (TokenType::String(lval), TokenType::String(rval)) => {
                    let val = lval == rval;
                    let value = if val {
                        TokenType::True
                    } else {
                        TokenType::False
                    };

                    return Ok(Literal::new(Token::new(
                        value,
                        val.to_string(),
                        left.value.line,
                    )));
                }

                (TokenType::Number(nval), TokenType::String(sval))
                | (TokenType::String(sval), TokenType::Number(nval)) => {
                    let val = nval.to_string() == sval;
                    let value = if val {
                        TokenType::True
                    } else {
                        TokenType::False
                    };

                    return Ok(Literal::new(Token::new(
                        value,
                        val.to_string(),
                        left.value.line,
                    )));
                }
                // string -> string, string -> number
                _ => {
                    return Err(MyError::RuntimeError { line: left.value.line, message: "for the '<=' binary oprator, the left and right operands must be of type Number".to_string() });
                }
            },
            TokenType::BangEqual => match (left.value.token_type, right.value.token_type) {
                (TokenType::Nil, TokenType::Nil) => {
                    return Ok(Literal::new(Token::new(
                        TokenType::False,
                        "false".to_string(),
                        left.value.line,
                    )))
                }
                (TokenType::Nil, ..) | (.., TokenType::Nil) => {
                    return Ok(Literal::new(Token::new(
                        TokenType::True,
                        "true".to_string(),
                        left.value.line,
                    )))
                }
                (TokenType::True, token_type) | (token_type, TokenType::True) => {
                    if !Literal::is_truthy(token_type) {
                        return Ok(Literal::new(Token::new(
                            TokenType::True,
                            "true".to_string(),
                            left.value.line,
                        )));
                    } else {
                        return Ok(Literal::new(Token::new(
                            TokenType::False,
                            "false".to_string(),
                            left.value.line,
                        )));
                    }
                }
                (TokenType::False, token_type) | (token_type, TokenType::False) => {
                    if Literal::is_truthy(token_type) {
                        return Ok(Literal::new(Token::new(
                            TokenType::True,
                            "true".to_string(),
                            left.value.line,
                        )));
                    } else {
                        return Ok(Literal::new(Token::new(
                            TokenType::False,
                            "false".to_string(),
                            left.value.line,
                        )));
                    }
                }
                // match cases:
                // number -> number, number -> string
                (TokenType::Number(lval), TokenType::Number(rval)) => {
                    let val = lval != rval;
                    let value = if val {
                        TokenType::True
                    } else {
                        TokenType::False
                    };

                    return Ok(Literal::new(Token::new(
                        value,
                        val.to_string(),
                        left.value.line,
                    )));
                }

                (TokenType::String(lval), TokenType::String(rval)) => {
                    let val = lval != rval;
                    let value = if val {
                        TokenType::True
                    } else {
                        TokenType::False
                    };

                    return Ok(Literal::new(Token::new(
                        value,
                        val.to_string(),
                        left.value.line,
                    )));
                }

                (TokenType::Number(nval), TokenType::String(sval))
                | (TokenType::String(sval), TokenType::Number(nval)) => {
                    let val = nval.to_string() != sval;
                    let value = if val {
                        TokenType::True
                    } else {
                        TokenType::False
                    };

                    return Ok(Literal::new(Token::new(
                        value,
                        val.to_string(),
                        left.value.line,
                    )));
                }
                // string -> string, string -> number
                _ => {
                    return Err(MyError::RuntimeError { line: left.value.line, message: "for the '<=' binary oprator, the left and right operands must be of type Number".to_string() });
                }
            },
            _ => Err(MyError::RuntimeError {
                line: left.value.line,
                message: "Binary Operator not supported".to_string(),
            }),
        }
    }
}
