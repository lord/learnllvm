extern crate llvm;
#[macro_use]
extern crate nom;

mod lexer;
mod parser;

fn main() {
    if let Some(tokens) = lexer::lex(&"unetohan + ath") {
      let ast = parser::parse(&tokens);
      println!("lex: {:?}", &tokens);
      println!("parse: {:?}", &ast);
    } else {
      println!("failed to lex!")
    }
}
