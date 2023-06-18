use crate::{
    expression::{self, Expr, Grouping, Literal, Unary},
    scanner::token::{Token, TokenType},
};

pub fn print_tree(expression: &Expr) {
    println!("{}", expression);
}

pub fn sample_tree() {
    let expression = expression::Binary::new(
        Expr::new(Box::new(Unary::new(
            Token::new(TokenType::Minus, "-".to_string(), 1),
            Expr::new(Box::new(Literal::new(Token::new(
                TokenType::Number(127.0),
                "127.0".to_string(),
                1,
            )))),
        ))),
        Token::new(TokenType::Star, "*".to_string(), 1),
        // Expr::new(Box::new(Grouping::new(Expr::new(Box::new(Literal::new(
        //     Token::new(TokenType::Number(45.5), "45.5".to_string(), 1),
        // )))))),
        Expr::new(Box::new(Grouping::new(Expr::new(Box::new(Unary::new(
            Token::new(TokenType::Minus, "-".to_string(), 1),
            Expr::new(Box::new(Literal::new(Token::new(
                TokenType::Number(42.0),
                "42.0".to_string(),
                1,
            )))),
        )))))),
    );

    let expr = Expr::new(Box::new(expression));

    print_tree(&expr);
}
