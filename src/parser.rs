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
pub struct Prototype {
  name: String,
  args: Vec<String>,
}

#[derive(Debug)]
pub struct Function {
  proto: Prototype,
  body: Expr,
}

fn primary_expr(rem: &[Token]) -> Option<(Expr, &[Token])> {
  if rem.len() == 0 {
    return None
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
  Some((exp, rest))
}

fn identifier_expr<'a>(id: &str, rem: &'a [Token]) -> Option<(Expr, &'a [Token])> {
  // peek at next value
  match rem.first() {
    Some(&Token::Symbol('(')) => {
      unimplemented!()
    }
    _ => Some((Expr::Variable(id.to_string()), rem)),
  }
}

fn paren_expr(rem: &[Token]) -> Option<(Expr, &[Token])> {
  unimplemented!()
}

pub fn parse(tokens: &[Token]) -> Option<Expr> {
  primary_expr(tokens).and_then(|(exp, remaining)| {
    if remaining.len() == 0 {
      Some(exp)
    } else {
      None
    }
  })
}
