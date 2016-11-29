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
      unimplemented!()
    },
    &Token::Symbol(ref chr) => {
      unimplemented!()
    },
    &Token::Number(n) => Expr::Number(n),
  };
  Some((exp, rest))
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
