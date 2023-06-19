mod token_type;

use std::fmt::Display;
pub use token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let literal = match &self.token_type {
            TokenType::Identifier(val) => val.clone(),
            TokenType::String(val) => val.clone(),
            TokenType::Number(val) => val.to_string(),
            _ => "".to_string(),
        };
        write!(f, "[{:?} '{}' '{}']", self.token_type, self.lexeme, literal)
    }
}
