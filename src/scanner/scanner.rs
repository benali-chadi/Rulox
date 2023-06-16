use crate::error_reporintg;

use super::keywords::TokenKeywords;
use super::token::{Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.add_token(TokenType::Eof);

        self.tokens.clone()
    }

    pub fn is_alpha(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    pub fn is_alpha_numeric(c: char) -> bool {
        Scanner::is_alpha(c) || c.is_digit(10)
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),

            '!' => {
                let matches = self.match_to('=');

                self.add_token(if matches {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                })
            }

            '=' => {
                let matches = self.match_to('=');

                self.add_token(if matches {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                })
            }

            '<' => {
                let matches = self.match_to('=');

                self.add_token(if matches {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                })
            }

            '>' => {
                let matches = self.match_to('=');

                self.add_token(if matches {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                })
            }

            '/' => {
                if self.match_to('/') {
                    // It's a comment, skip untill the end of the line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_to('*') {
                    // Multiline comment, skip untill the next */
                    while self.peek() != '*' && self.peek_next() != '/' && !self.is_at_end() {
                        self.advance();
                    }
                    if self.is_at_end() {
                        error_reporintg::error(self.line, "Unterminated comment.");
                    } else {
                        self.advance();
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }

            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => {}

            // Literals
            '"' => self.string(),

            _ => {
                if c.is_digit(10) {
                    self.number();
                } else if Scanner::is_alpha(c) {
                    self.identifier_or_keyword();
                } else {
                    error_reporintg::error(self.line, "Unexpected character.")
                }
            }
        }
    }

    fn identifier_or_keyword(&mut self) {
        let keywords = TokenKeywords::new();

        while Scanner::is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let token = keywords.matches(self.source[self.start..self.current].to_string());

        self.add_token(token);
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        self.add_token(TokenType::Number(
            self.source[self.start..self.current].parse().unwrap(),
        ))
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            error_reporintg::error(self.line - 1, "Unterminated string.");
            return;
        }

        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];

        self.add_token(TokenType::String(value.to_string()));
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        };
        self.source.as_bytes()[self.current] as char
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.as_bytes()[self.current + 1] as char
    }

    fn match_to(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.as_bytes()[self.current] as char != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn add_token(&mut self, token: TokenType) {
        let text: &str = &self.source[self.start..self.current];

        self.tokens
            .push(Token::new(token, text.to_string(), self.line))
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn advance(&mut self) -> char {
        let c = self.source.as_bytes()[self.current] as char;
        self.current += 1;

        c
    }
}
