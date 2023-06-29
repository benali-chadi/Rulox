pub(self) mod keywords;
pub mod token;

use crate::rulox_error::{RuloxError, RuloxResult};

use token::{Token, TokenType};

use self::keywords::TokenKeywords;

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

    pub fn scan_tokens(&mut self) -> RuloxResult<Vec<Token>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.add_token(TokenType::Eof);

        Ok(self.tokens.clone())
    }

    pub fn is_alpha(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    pub fn is_alpha_numeric(c: char) -> bool {
        Scanner::is_alpha(c) || c.is_ascii_digit()
    }

    fn scan_token(&mut self) -> RuloxResult<()> {
        let c: char = self.advance();

        match c {
            '(' => {
                self.add_token(TokenType::LeftParen);
                Ok(())
            }
            ')' => {
                self.add_token(TokenType::RightParen);
                Ok(())
            }
            '{' => {
                self.add_token(TokenType::LeftBrace);
                Ok(())
            }
            '}' => {
                self.add_token(TokenType::RightBrace);
                Ok(())
            }
            ',' => {
                self.add_token(TokenType::Comma);
                Ok(())
            }
            '.' => {
                self.add_token(TokenType::Dot);
                Ok(())
            }
            '-' => {
                self.add_token(TokenType::Minus);
                Ok(())
            }
            '+' => {
                self.add_token(TokenType::Plus);
                Ok(())
            }
            ';' => {
                self.add_token(TokenType::Semicolon);
                Ok(())
            }
            '*' => {
                self.add_token(TokenType::Star);
                Ok(())
            }

            '!' => {
                let matches = self.match_to('=');

                self.add_token(if matches {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                });
                Ok(())
            }

            '=' => {
                let matches = self.match_to('=');
                self.add_token(if matches {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                });
                Ok(())
            }

            '<' => {
                let matches = self.match_to('=');
                self.add_token(if matches {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                });
                Ok(())
            }

            '>' => {
                let matches = self.match_to('=');
                self.add_token(if matches {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                });
                Ok(())
            }

            '/' => {
                if self.match_to('/') {
                    // It's a comment, skip untill the end of the line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    Ok(())
                } else if self.match_to('*') {
                    // Multiline comment, skip untill the next */
                    while self.peek() != '*' && self.peek_next() != '/' && !self.is_at_end() {
                        self.advance();
                    }
                    if self.is_at_end() {
                        Err(RuloxError::SyntaxError {
                            token: None,
                            line: self.line,
                            message: "Unterminated comment.".to_string(),
                        })
                    } else {
                        self.advance();
                        self.advance();
                        Ok(())
                    }
                } else {
                    self.add_token(TokenType::Slash);
                    Ok(())
                }
            }

            '\n' => {
                self.line += 1;
                Ok(())
            }
            ' ' | '\r' | '\t' => Ok(()),

            // Literals
            '"' => self.string(),

            _ => {
                if c.is_ascii_digit() {
                    self.number();
                    Ok(())
                } else if Scanner::is_alpha(c) {
                    self.identifier_or_keyword();
                    Ok(())
                } else {
                    Err(RuloxError::SyntaxError {
                        token: None,
                        line: self.line,
                        message: "Unexpected character.".to_string(),
                    })
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
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.add_token(TokenType::Number(
            self.source[self.start..self.current].parse().unwrap(),
        ))
    }

    fn string(&mut self) -> RuloxResult<()> {
        let initial_line = self.line - 1;

        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(RuloxError::SyntaxError {
                token: None,
                line: initial_line,
                message: "Unterminated string.".to_string(),
            });
        }

        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];

        self.add_token(TokenType::String(value.to_string()));

        Ok(())
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

        self.tokens.push(Token::new(token, text, self.line))
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source.as_bytes()[self.current] as char;
        self.current += 1;

        c
    }
}
