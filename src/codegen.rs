use parser::Expr;
use llvm::{Context, Value, Compile, Builder, Module};
use std::collections::HashMap;

#[derive(Debug)]
pub struct CompilerError(String);

type CompilerResult<'a> = Result<&'a Value, CompilerError>;

fn make_error(desc: &str) -> CompilerResult {
  Err(CompilerError(desc.to_string()))
}

pub fn codegen<'a>(ctx: &'a Context,
  builder: &'a Builder,
  module: &mut Module,
  vals: &mut HashMap<String, &'a Value>,
  expr: Expr) -> CompilerResult<'a> {
  match expr {
    Expr::Number(n) => Ok(n.compile(ctx)),
    Expr::Variable(name) => match vals.get(&name) {
      Some(val) => Ok(val),
      None => make_error("couldn't find variable"),
    },
    Expr::BinaryOp{op, lhs, rhs} => {
      let lhs_code = try!(codegen(ctx, builder, module, vals, *lhs));
      let rhs_code = try!(codegen(ctx, builder, module, vals, *rhs));

      match op {
        '+' => Ok(builder.build_add(lhs_code, rhs_code)),
        '-' => Ok(builder.build_sub(lhs_code, rhs_code)),
        '*' => Ok(builder.build_mul(lhs_code, rhs_code)),
        _ => make_error("don't recognize that operator")
        // '<' => builder.build_cmp(lhs_code, rhs_code),
      }
    },
    Expr::FuncCall{func, args} => unimplemented!(),
  }

}
