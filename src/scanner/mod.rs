pub use self::scanner::Scanner;
use self::token::Token;
pub(self) mod keywords;
mod scanner;
pub(crate) mod token;

pub fn run(source: String) -> Vec<Token> {
    let mut scanner = Scanner::new(&source);
    let tokens = scanner.scan_tokens();

    // let tokens = source.split(' ').collect::<Vec<&str>>();

    for token in &tokens {
        println!("{:?}", token);
    }

    tokens
}
