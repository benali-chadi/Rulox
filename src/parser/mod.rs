use crate::{
    error_reporintg::MyError,
    scanner::token::{Token, TokenType},
};

pub mod expression;
pub use self::expression::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: &[Token]) -> Self {
        Self {
            tokens: tokens.to_vec(),
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Expr, MyError> {
        self.comma()
    }

    fn comma(&mut self) -> Result<Expr, MyError> {
        let mut expr = self.ternary()?;

        while self.matches(&[TokenType::Comma]) {
            expr = self.ternary()?;
        }

        Ok(expr)
    }

    fn ternary(&mut self) -> Result<Expr, MyError> {
        let mut expr = self.expression()?;

        while self.matches(&[TokenType::QuestionMark]) && !self.is_at_end() {
            let first = self.expression()?;
            if !self.matches(&[TokenType::Colon]) {
                return Err(MyError::ParseError {
                    token: Some(Token::new(TokenType::QuestionMark, "?".to_string(), 1)),
                    line: 1,
                    message: "Expected a ':'".to_string(),
                });
            }
            let second = self.expression()?;
            expr = Expr::new(Box::new(Ternary::new(expr, first, second)));
        }

        Ok(expr)
    }

    fn expression(&mut self) -> Result<Expr, MyError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, MyError> {
        let mut expr = self.comparison()?;

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::new(Box::new(Binary::new(expr, operator, right)));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, MyError> {
        let mut expr = self.term()?;

        while self.matches(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::new(Box::new(Binary::new(expr, operator, right)));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, MyError> {
        let mut expr = self.factor()?;

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::new(Box::new(Binary::new(expr, operator, right)));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, MyError> {
        let mut expr = self.unary()?;

        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::new(Box::new(Binary::new(expr, operator, right)));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, MyError> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::new(Box::new(Unary::new(operator, right))));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, MyError> {
        match self.peek().token_type {
            TokenType::False
            | TokenType::True
            | TokenType::Number(_)
            | TokenType::String(_)
            | TokenType::Nil => {
                self.advance();

                Ok(Expr::new(Box::new(Literal::new(self.previous().clone()))))
            }

            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression()?;

                match self.consume(TokenType::RightParen, "Expect ')' after expression.") {
                    Ok(_) => Ok(Expr::new(Box::new(Grouping::new(expr)))),
                    Err(err) => Err(err),
                }
            }

            _ => Err(MyError::ParseError {
                token: Some(self.peek().clone()),
                line: self.peek().line,
                message: "Expect expression".to_string(),
            }),
        }
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<Token, MyError> {
        if self.check(&token_type) {
            return Ok(self.advance());
        }

        Err(MyError::ParseError {
            token: Some(self.peek().clone()),
            line: self.peek().line,
            message: msg.to_string(),
        })
    }

    pub fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class => {}
                TokenType::Fun => {}
                TokenType::Var => {}
                TokenType::For => {}
                TokenType::If => {}
                TokenType::While => {}
                TokenType::Print => {}
                TokenType::Return => {}
                _ => {}
            }

            self.advance();
        }
    }

    fn matches(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(&token_type)
    }

    fn is_at_end(&self) -> bool {
        match self.peek().token_type {
            TokenType::Eof => true,
            _ => false,
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous().clone()
    }
}
