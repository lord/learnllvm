use lexer;

enum Expr {
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

struct Prototype {
  name: String,
  args: Vec<String>,
}

struct Function {
  proto: Prototype,
  body: Expr,
}

fn parse(tokens: Vec<lexer::Token>) -> Expr {
  unimplemented!()
}
