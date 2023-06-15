use self::scanner::Scanner;

pub fn run(source: String) {
    let mut scanner = Scanner::new(&source);
    let tokens = scanner.scan_tokens();

    // let tokens = source.split(' ').collect::<Vec<&str>>();

    for token in tokens {
        println!("{:?}", token);
    }
}

pub(self) mod keywords;
pub mod scanner;
pub mod token;
