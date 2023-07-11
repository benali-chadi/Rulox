use crate::{
    expression::{Assign, Logical, Variable, *},
    rulox_error::{Report, RuloxError, RuloxResult},
    scanner::token::{Token, TokenType},
    statement::{Block, Expression, Function, If, Print, Stmt, Var, While},
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
        // if self.matches(&[TokenType::Var]) {
        // }

        match self.peek().token_type {
            TokenType::Fun => {
                self.advance();

                return self.function("function");
            }
            TokenType::Var => {
                self.advance();

                return self.var_declaration();
            }

            _ => self.statement(),
        }
    }

    fn function(&mut self, kind: &str) -> RuloxResult<Stmt> {
        let name = self.consume(
            TokenType::Identifier(String::new()),
            &format!("Expect {} name.", kind),
        )?;

        self.consume(
            TokenType::LeftParen,
            &format!("Expect '(' after {} name", kind),
        )?;

        let mut params: Vec<Token> = Vec::new();

        if !self.check(&TokenType::RightParen) {
            loop {
                if params.len() >= 255 {
                    return Err(RuloxError::ParseError {
                        token: Some(self.peek().clone()),
                        line: self.peek().line,
                        message: "Can't have more than 255 arguments.".to_string(),
                    });
                }
                params.push(self.consume(
                    TokenType::Identifier(String::new()),
                    "Expect parameter name.",
                )?);

                if !self.matches(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        // self.advance();
        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;

        self.consume(
            TokenType::LeftBrace,
            &format!("Expect '{{' before {} body.", kind),
        )?;

        let body = self.block()?;

        Ok(Stmt::new(Box::new(Function::new(
            name,
            params,
            Block::new(&body),
        ))))
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
        match self.peek().token_type {
            TokenType::For => {
                self.advance();
                self.for_statement()
            }
            TokenType::If => {
                self.advance();
                self.if_statement()
            }
            TokenType::Print => {
                self.advance();
                self.print_statement()
            }
            TokenType::While => {
                self.advance();
                self.while_statement()
            }
            TokenType::LeftBrace => {
                self.advance();
                Ok(Stmt::new(Box::new(Block::new(&self.block()?))))
            }
            _ => self.expression_statement(),
        }
    }

    fn for_statement(&mut self) -> RuloxResult<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let mut initializer: Option<Stmt> = None;

        // if self.matches(&[TokenType::Semicolon]) {}
        if self.matches(&[TokenType::Var]) {
            initializer = Some(self.var_declaration()?);
        } else {
            initializer = Some(self.expression_statement()?);
        }

        let mut condition: Option<Expr> = None;
        if !self.check(&TokenType::Semicolon) {
            condition = Some(self.expression()?);
        }
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition")?;

        let mut increment: Option<Expr> = None;

        if !self.check(&TokenType::RightParen) {
            increment = Some(self.expression()?);
        }
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;

        if let Some(inc) = increment {
            body = Stmt::new(Box::new(Block::new(&vec![
                body,
                Stmt::new(Box::new(Expression::new(inc))),
            ])))
        }

        let mut cond = Expr::new(Box::new(Literal::new(Token::new(
            TokenType::True,
            "true",
            1,
        ))));
        if let Some(c) = condition {
            cond = c;
        }

        body = Stmt::new(Box::new(While::new(cond, body)));

        if let Some(init) = initializer {
            body = Stmt::new(Box::new(Block::new(&vec![init, body])));
        }

        Ok(body)
    }

    fn while_statement(&mut self) -> RuloxResult<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let body = self.statement()?;

        Ok(Stmt::new(Box::new(While::new(condition, body))))
    }

    fn if_statement(&mut self) -> RuloxResult<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = self.statement()?;
        let mut else_branch: Option<Stmt> = None;

        if self.matches(&[TokenType::Else]) {
            else_branch = Some(self.statement()?);
        }

        Ok(Stmt::new(Box::new(If::new(
            condition,
            then_branch,
            else_branch,
        ))))
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
        self.assignment()
    }

    fn assignment(&mut self) -> RuloxResult<Expr> {
        // let expr = self.equality()?;
        let expr = self.or()?;

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

    fn or(&mut self) -> RuloxResult<Expr> {
        let mut expr = self.and()?;

        while self.matches(&[TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::new(Box::new(Logical::new(expr, operator, right)));
        }

        Ok(expr)
    }

    fn and(&mut self) -> RuloxResult<Expr> {
        let mut expr = self.equality()?;

        while self.matches(&[TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::new(Box::new(Logical::new(expr, operator, right)));
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

        // self.primary()
        self.call()
    }

    fn call(&mut self) -> RuloxResult<Expr> {
        let mut expr = self.primary()?;

        loop {
            if self.matches(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> RuloxResult<Expr> {
        let mut arguments: Vec<Expr> = Vec::new();

        if !self.check(&TokenType::RightParen) {
            // arguments.push(self.expression()?);

            // while self.matches(&[TokenType::Comma]) {
            //     if arguments.len() >= 255 {
            //         return Err(RuloxError::ParseError {
            //             token: Some(self.peek().clone()),
            //             line: self.peek().line,
            //             message: "Can't have more than 255 arguments.".to_string(),
            //         });
            //     }
            //     arguments.push(self.expression()?);
            // }
            loop {
                if arguments.len() >= 255 {
                    return Err(RuloxError::ParseError {
                        token: Some(self.peek().clone()),
                        line: self.peek().line,
                        message: "Can't have more than 255 arguments.".to_string(),
                    });
                }
                arguments.push(self.expression()?);
                if !self.matches(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;

        Ok(Expr::new(Box::new(Call::new(callee, paren, arguments))))
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

            _ => {
                self.advance();
                Err(RuloxError::ParseError {
                    token: Some(self.previous().clone()),
                    line: self.previous().line,
                    message: "Expect expression".to_string(),
                })
            }
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

    fn _synchronize(&mut self) {
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
