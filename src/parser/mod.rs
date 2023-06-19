use crate::{
    error_reporintg::MyError,
    expression::{Binary, Expr, Grouping, Literal, Unary},
    scanner::token::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Expr::new(Box::new(Binary::new(expr, operator, right)));
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.matches(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Expr::new(Box::new(Binary::new(expr, operator, right)));
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::new(Box::new(Binary::new(expr, operator, right)));
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::new(Box::new(Binary::new(expr, operator, right)));
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Expr::new(Box::new(Unary::new(operator, right)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        // match self.peek().token_type {
        //     TokenType::False => {
        //         self.advance();
        //         return Expr::new(Box::new(Literal::new(self.previous().clone())));
        //     }
        //     TokenType::True => {
        //         self.advance();
        //         return Expr::new(Box::new(Literal::new(self.previous().clone())));
        //     }
        //     Toke
        // }
        if self.matches(&[TokenType::LeftParen]) {
            // let expr = self.expression();
            let expr = self.unary();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Expr::new(Box::new(Grouping::new(expr)));
        }
        self.advance();
        Expr::new(Box::new(Literal::new(self.previous().clone())))
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
