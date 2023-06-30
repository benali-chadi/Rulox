use crate::{
    expression::{assign::Assign, variable::Variable, *},
    rulox_error::{Report, RuloxError, RuloxResult},
    scanner::token::{Token, TokenType},
    statement::{block::Block, expression::Expression, print::Print, var::Var, Stmt},
};

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

    pub fn parse(&mut self) -> RuloxResult<Vec<Stmt>> {
        let mut statements = Vec::new();
        let mut stop = false;

        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(err) => {
                    stop = true;
                    err.report()
                }
            }
        }

        if !stop {
            Ok(statements)
        } else {
            Err(RuloxError::ParseError {
                token: None,
                line: 0,
                message: String::new(),
            })
        }
    }

    fn declaration(&mut self) -> RuloxResult<Stmt> {
        if self.matches(&[TokenType::Var]) {
            return self.var_declaration();
        }

        self.statement()
    }

    fn var_declaration(&mut self) -> RuloxResult<Stmt> {
        let name = self.consume(
            TokenType::Identifier(String::new()),
            "Expect variable name.",
        )?;

        let mut initializer: Option<Expr> = None;
        if self.matches(&[TokenType::Equal]) {
            initializer = Some(self.expression()?);
        }

        self.consume(TokenType::Semicolon, "Expect ';' after variable declration")?;

        Ok(Stmt::new(Box::new(Var::new(name, initializer))))
    }

    fn statement(&mut self) -> RuloxResult<Stmt> {
        if self.matches(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.matches(&[TokenType::LeftBrace]) {
            return Ok(Stmt::new(Box::new(Block::new(self.block()?))));
        }

        self.expression_statement()
    }

    fn block(&mut self) -> RuloxResult<Vec<Stmt>> {
        let mut statements = vec![];

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;

        Ok(statements)
    }

    fn expression_statement(&mut self) -> RuloxResult<Stmt> {
        let expression = self.expression()?;

        self.consume(TokenType::Semicolon, "Expected ';' after expression")?;

        Ok(Stmt::new(Box::new(Expression::new(expression))))
    }

    fn print_statement(&mut self) -> RuloxResult<Stmt> {
        let expression = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after expression")?;

        Ok(Stmt::new(Box::new(Print::new(expression))))
    }

    fn expression(&mut self) -> RuloxResult<Expr> {
        // self.equality()
        self.assignment()
    }

    fn assignment(&mut self) -> RuloxResult<Expr> {
        let expr = self.equality()?;

        if self.matches(&[TokenType::Equal]) {
            // let equals = self.previous();
            let value = self.assignment()?;

            // Check if the expression is a Variable, by checking if its token_type is an identifier
            match &expr.expression.get_token().token_type {
                TokenType::Identifier(_) => {
                    return Ok(Expr::new(Box::new(Assign::new(
                        expr.expression.get_token().clone(),
                        value,
                    ))))
                }

                _ => {
                    //error method here
                    return Err(RuloxError::RuntimeError {
                        line: 1,
                        message: "Invalid assignment target".to_string(),
                    });
                }
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> RuloxResult<Expr> {
        let mut expr = self.comparison()?;

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::new(Box::new(Binary::new(expr, operator, right)));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> RuloxResult<Expr> {
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

    fn term(&mut self) -> RuloxResult<Expr> {
        let mut expr = self.factor()?;

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::new(Box::new(Binary::new(expr, operator, right)));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> RuloxResult<Expr> {
        let mut expr = self.unary()?;

        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::new(Box::new(Binary::new(expr, operator, right)));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> RuloxResult<Expr> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::new(Box::new(Unary::new(operator, right))));
        }

        self.primary()
    }

    fn primary(&mut self) -> RuloxResult<Expr> {
        match self.peek().token_type {
            TokenType::False
            | TokenType::True
            | TokenType::Nil
            | TokenType::Number(_)
            | TokenType::String(_) => {
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

            TokenType::Identifier(_) => {
                self.advance();
                return Ok(Expr::new(Box::new(Variable::new(self.previous()))));
            }

            _ => Err(RuloxError::ParseError {
                token: Some(self.peek().clone()),
                line: self.peek().line,
                message: "Expect expression".to_string(),
            }),
        }
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> RuloxResult<Token> {
        if self.check(&token_type) {
            return Ok(self.advance());
        }

        Err(RuloxError::ParseError {
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

        std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(token_type)
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Eof)
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
