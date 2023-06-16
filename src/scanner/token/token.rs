use super::TokenType;

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

    pub fn to_string(&self) -> String {
        let literal = match &self.token_type {
            TokenType::Identifier(val) => val.clone(),
            TokenType::String(val) => val.clone(),
            TokenType::Number(val) => val.to_string(),
            _ => "".to_string(),
        };

        format!("{:?} {} {}", self.token_type, self.lexeme, literal)
    }
}
