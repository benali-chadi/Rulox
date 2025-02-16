# Rulox – A Rust Implementation of Lox

Rulox is a Rust implementation of the `lox` language from the amazing book [Crafting Interpreters](https://craftinginterpreters.com/). This project serves as an interpreter for the Lox programming language, following the principles and implementation details outlined in the book.

## Requirements

To run Rulox, ensure you have Rust and Cargo installed. You can install Rust using the official Rust toolchain:

- [Install Rust](https://www.rust-lang.org/tools/install)

## Running Rulox

You can run Rulox in two ways:

### 1. Interactive Mode (REPL)
To launch an interactive session where you can type and execute Lox code in real-time, run:

```sh
cargo run
```
### 2. Running a Lox Script
To execute a Lox source file, specify the filename as an argument:

```sh
cargo run path/to/script.lox
```
Replace `path/to/script.lox` with the actual path to your Lox script.
