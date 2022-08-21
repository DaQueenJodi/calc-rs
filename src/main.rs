pub mod interpretor;
pub mod tokens;
use crate::interpretor::{Parser, Tokenizer};
use std::io::{stdin, stdout, Write};
fn main() {
    loop {
        let mut text = String::new();
        print!("calc> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut text).unwrap();
        let mut interpretor = Tokenizer::new(text);
        let mut parser = Parser::new(interpretor.tokenize());
        println!("{}", parser.expr());
    }
}
