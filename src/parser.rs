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
  let (cur, rest) = match rem.split_first() {
    Some(result) => result,
    None => return make_error("tried to parse primary expression but no tokens found"),
  };
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

fn parse_single_expr(rem: &[Token]) -> ParseResult {
  let (lhs, rest) = try!(primary_expr(rem));
  parse_binop_rhs(0, lhs, rest)
}

fn get_token_precedence(tok: &Token) -> i64 {
  match tok {
    &Token::Symbol('*') => 40,
    &Token::Symbol('-') => 20,
    &Token::Symbol('+') => 20,
    &Token::Symbol('<') => 10,
    _ => -1,
  }
}

fn parse_binop_rhs(expr_prec: i64, lhs: Expr, rem: &[Token]) -> ParseResult {
  let (op, rest) = match rem.split_first() {
    None => return Ok((lhs, rem)),
    Some(result) => result,
  };
  let op_char = match op {
    &Token::Symbol(sym) => sym,
    _ => 'x',
  };
  if get_token_precedence(op) < expr_prec {
    return Ok((lhs, rem));
  }
  let (mut rhs, mut rest) = try!(primary_expr(rest));

  if let Some((next_op, _)) = rest.split_first() {
    if get_token_precedence(op) < get_token_precedence(next_op) {
      let (new_rhs, new_rest) = try!(parse_binop_rhs(get_token_precedence(op) + 1, rhs, rest));
      rhs = new_rhs;
      rest = new_rest;
    }
  }
  let binop = Expr::BinaryOp{
    op: op_char,
    lhs: Box::new(lhs),
    rhs: Box::new(rhs),
  };
  Ok((binop, rest))
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
  while let Some((next, leftover)) = rest.split_first() {
    // next symbol is ), so this is the end, return
    if next == &Token::Symbol(')') {
      let call = Expr::FuncCall{
        func: id.to_string(),
        args: args,
      };
      return Ok((call, leftover));
    }

    let (expr, rest_from_parse) = try!(parse_single_expr(rest));
    rest = rest_from_parse;
    args.push(expr);
  }
  // reached end of token stream without hitting ).
  make_error("Mismatched (, reached end of stream without )")
}

fn paren_expr(rem: &[Token]) -> ParseResult {
  let (expr, rest) = try!(parse_single_expr(rem));
  match rest.split_first() {
    Some((&Token::Symbol(')'), rest)) => Ok((expr, rest)),
    Some(_) => make_error("Expected ), got another token instead"),
    None => make_error("Expected ), but reached end of file instead"),
  }
}

pub fn parse(tokens: &[Token]) -> Result<Expr, ParseError> {
  parse_single_expr(tokens).and_then(|(exp, remaining)| {
    if remaining.len() == 0 {
      Ok(exp)
    } else {
      Err(ParseError(format!("didn't reach end of stream, {:?} remains.", remaining)))
    }
  })
}
