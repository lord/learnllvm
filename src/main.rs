extern crate llvm;
#[macro_use]
extern crate nom;

mod lexer;
mod parser;

fn main() {
    println!("{:?}", lexer::lex(&"hello    \ntest+ing #!(*@)#(*) def \nextern  129382 1.198 198.2e2 #blah"));
}
