use std::fmt::Display;

use super::Stmt;

pub struct Block {
    pub statements: Vec<Stmt>,
}

impl Block {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Self { statements }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "This is a Block")
    }
}

// TODO: Implement the Stmt Trait
