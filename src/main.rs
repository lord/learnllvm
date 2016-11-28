extern crate llvm;
#[macro_use]
extern crate nom;

enum Token {
  Eof,
  Def,
  Extern,
  Identifier(String),
  Number(f64)
}

fn token_char(chr: char) -> bool {
  chr.is_alphanumeric() || chr == '.'
}

named!(white_space<&str, &str>, alt!(tag_s!(" ") | tag_s!("\t") | tag_s!("\n")));

named!(token<&str, &str>, take_while_s!(token_char));

// named!(comment<&str, &str>, alt!())

named!(lex< &str, Vec<&str> >, many0!(delimited!(
  many0!(white_space),
  token,
  many0!(white_space)
)));

fn main() {
    println!("{:?}", lex(&"hello    \ntesting blah"));
}
