pub use self::scanner_type::Scanner;
use self::token::Token;
pub(self) mod keywords;
mod scanner_type;
pub mod token;

pub fn run(source: String) -> Vec<Token> {
    let mut scanner = Scanner::new(&source);
    let tokens = scanner.scan_tokens();

    for token in &tokens {
        println!("{:?}", token);
    }

    tokens
}
