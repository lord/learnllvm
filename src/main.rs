extern crate llvm;
#[macro_use]
extern crate nom;

#[derive(Debug)]
enum Token {
  Def,
  Extern,
  Identifier(String),
  Symbol(char),
  Number(f64)
}

fn token_char(chr: char) -> bool {
  chr.is_alphanumeric() || chr == '.'
}

fn comment_char(chr: char) -> bool {
  chr != '\n'
}

named!(comment<&str, &str>, preceded!(
  tag_s!("#"),
  take_while_s!(comment_char)
));

named!(white_space<&str, &str>, alt!(comment | tag_s!(" ") | tag_s!("\t") | tag_s!("\n")));

named!(token<&str, Token>, chain!(
  tok: alt!(take_while1_s!(token_char) | take_s!(1)) ,
  || {
    println!("{:?}", tok);
    if tok == "def" {
      Token::Def
    } else if tok == "extern" {
      Token::Extern
    } else if let Ok(n) = tok.parse::<f64>() {
      Token::Number(n)
    } else {
      let first_char = tok.chars().next().unwrap();
      if token_char(first_char) {
        Token::Identifier(tok.to_string())
      } else {
        Token::Symbol(first_char)
      }
    }
  }
));

named!(lex< &str, Vec<Token> >, many0!(delimited!(
  many0!(white_space),
  token,
  many0!(white_space)
)));

fn main() {
    println!("{:?}", lex(&"hello    \ntest+ing #!(*@)#(*) def \nextern  129382 1.198 198.2e2 #blah"));
}
