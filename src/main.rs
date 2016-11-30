extern crate llvm;
#[macro_use]
extern crate nom;

mod lexer;
mod parser;
mod codegen;

use std::io;
use std::io::Write;
use std::io::stdout;

fn main() {
  loop {
    print!("ks> ");
    let _ = stdout().flush();
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    if let Some(tokens) = lexer::lex(&input) {
      let ast = parser::parse(&tokens);
      println!("lex: {:?}", &tokens);
      println!("parse: {:?}", &ast);
    } else {
      println!("failed to lex!")
    }
  }
}
