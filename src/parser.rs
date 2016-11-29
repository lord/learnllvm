use lexer::Token;


#[derive(Debug)]
pub enum Expr {
  Number(f64),
  Variable(String),
  BinaryOp{
    op: char,
    lhs: Box<Expr>,
    rhs: Box<Expr>,
  },
  FuncCall{
    func: String,
    args: Vec<Expr>,
  }
}

#[derive(Debug)]
pub struct ParseError(String);

type ParseResult<'a> = Result<(Expr, &'a [Token]), ParseError>;

fn make_error(desc: &str) -> ParseResult {
  Err(ParseError(desc.to_string()))
}

#[derive(Debug)]
pub struct Prototype {
  name: String,
  args: Vec<String>,
}

#[derive(Debug)]
pub struct Function {
  proto: Prototype,
  body: Expr,
}

fn primary_expr(rem: &[Token]) -> ParseResult {
  if rem.len() == 0 {
    return make_error("tried to parse primary expression but no tokens found");
  }
  let (cur, rest) = rem.split_first().unwrap();
  let exp = match cur {
    &Token::Def => {
      unimplemented!()
    },
    &Token::Extern => {
      unimplemented!()
    },
    &Token::Identifier(ref id) => {
      return identifier_expr(id, rest)
    },
    &Token::Symbol(ref chr) => match chr {
      &'(' => return paren_expr(rest),
      _ => unimplemented!(),
    },
    &Token::Number(n) => Expr::Number(n),
  };
  Ok((exp, rest))
}

fn parse_single_expr<'a>(rem: &'a [Token]) -> ParseResult<'a> {
  unimplemented!()
}

fn identifier_expr<'a>(id: &str, rem: &'a [Token]) -> ParseResult<'a> {
  let mut rest = match rem.split_first() {
    Some((&Token::Symbol('('), rest)) => rest,

    // either at end of token stream, or next symbol isn't '(',
    // so just a variable id, not func call
    _ => {
      return Ok((Expr::Variable(id.to_string()), rem));
    }
  };

  let mut args = Vec::new();
  while let Some(next) = rest.first() {
    // next symbol is ), so this is the end, return
    if next == &Token::Symbol(')') {
      let call = Expr::FuncCall{
        func: id.to_string(),
        args: args,
      };
      return Ok((call, rest));
    }

    let (expr, rest_from_parse) = try!(parse_single_expr(rest));
    rest = rest_from_parse;
    args.push(expr);
  }
  // reached end of token stream without hitting ). TODO ADD ERROR
  make_error("Mismatched (, reached end of stream without )")
}

fn paren_expr(rem: &[Token]) -> ParseResult {
  unimplemented!()
}

pub fn parse(tokens: &[Token]) -> Result<Expr, ParseError> {
  primary_expr(tokens).and_then(|(exp, remaining)| {
    if remaining.len() == 0 {
      Ok(exp)
    } else {
      Err(ParseError("didn't reach end of stream".to_string()))
    }
  })
}
