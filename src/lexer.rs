#[derive(Debug, PartialEq)]
pub enum Token {
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

named!(lex_internal< &str, Vec<Token> >, many0!(delimited!(
  many0!(white_space),
  token,
  many0!(white_space)
)));

// TODO pass errors down properly instead of using an option
pub fn lex(code: &str) -> Option<Vec<Token>> {
  // TODO this may sometimes skip lexing invalid stuff at end but still say it's okay, which is bad
  lex_internal(code).to_full_result().ok()
}
