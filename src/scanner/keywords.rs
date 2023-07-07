use std::collections::HashMap;

use super::token::TokenType;

pub struct TokenKeywords {
    keywords: HashMap<String, TokenType>,
}

impl TokenKeywords {
    pub fn new() -> Self {
        let mut keywords: HashMap<String, TokenType> = HashMap::new();

        keywords.insert("and".to_string(), TokenType::And);
        keywords.insert("class".to_string(), TokenType::Class);
        keywords.insert("else".to_string(), TokenType::Else);
        keywords.insert("for".to_string(), TokenType::For);
        keywords.insert("fun".to_string(), TokenType::Fun);
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("nil".to_string(), TokenType::Nil);
        keywords.insert("or".to_string(), TokenType::Or);
        keywords.insert("print".to_string(), TokenType::Print);
        keywords.insert("return".to_string(), TokenType::Return);
        keywords.insert("super".to_string(), TokenType::Super);
        keywords.insert("this".to_string(), TokenType::This);
        keywords.insert("true".to_string(), TokenType::True);
        keywords.insert("false".to_string(), TokenType::False);
        keywords.insert("var".to_string(), TokenType::Var);
        keywords.insert("while".to_string(), TokenType::While);

        keywords.insert("break".to_string(), TokenType::Break);
        keywords.insert("continue".to_string(), TokenType::Continue);

        TokenKeywords { keywords }
    }

    pub fn matches(&self, text: String) -> TokenType {
        if let Some(val) = self.keywords.get(&text) {
            val.clone()
        } else {
            TokenType::Identifier(text)
        }
    }
}
