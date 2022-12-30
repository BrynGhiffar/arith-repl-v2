#![allow(dead_code)]
pub mod lexer;
use lexer::*;


pub fn run() {
    let input = 
br"(11 + 12) 
* False - 123 {} || && ===";
    let mut lexer = Lexer::from_cstream(input);
    lexer.debug();

    hello();
}