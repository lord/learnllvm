extern crate llvm;
#[macro_use]
extern crate nom;

#[derive(Debug)]
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

named!(token<&str, Token>, chain!(
  tok: take_while_s!(token_char) ,
  || {
    if tok == "def" {
      Token::Def
    } else if tok == "extern" {
      Token::Extern
    } else if let Ok(n) = tok.parse::<f64>() {
      Token::Number(n)
    } else {
      Token::Identifier(tok.to_string())
    }
  }
));

// take_while_s!(token_char));

// named!(comment<&str, &str>, alt!())

named!(lex< &str, Vec<Token> >, many0!(delimited!(
  many0!(white_space),
  token,
  many0!(white_space)
)));

fn main() {
    println!("{:?}", lex(&"hello    \ntesting def extern  129382 1.198 198.2e2 blah"));
}
