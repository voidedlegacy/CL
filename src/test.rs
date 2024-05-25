mod environment;
mod error;
mod file_io;
mod parser;
mod lexer;
mod node;

use parser::{parse_expr, ParsingContext};

fn main() {
    let tests = vec![
        "a : integer = 69",
        "a := 420",
        "b : integer",
        "b := 42",
        "defun foo (a:integer, b:integer):integer { a := a + b }",
    ];

    for (i, test) in tests.iter().enumerate() {
        println!("Test {}: {}", i + 1, test);
        let mut context = ParsingContext::new();
        let mut end = 0;
        match parse_expr(&mut context, test, &mut end) {
            Ok(result) => {
                println!("Parsed result:");
                result.print(0);
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
        println!();
    }
}
