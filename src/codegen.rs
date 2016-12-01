use parser;
use llvm::{Context, Value, Compile, Builder, Module, Type, FunctionType, Function, BasicBlock};
use std::collections::HashMap;

#[derive(Debug)]
pub struct CompilerError(String);

type CompilerResult<'a> = Result<&'a Value, CompilerError>;

fn make_error(desc: &str) -> CompilerResult {
  Err(CompilerError(desc.to_string()))
}

pub fn codegen_proto<'a>(ctx: &'a Context,
  builder: &'a Builder,
  module: &'a Module,
  vals: &mut HashMap<String, &'a Value>,
  proto_ast: &parser::Prototype) -> &'a Function {
  let typ = Type::get::<f64>(ctx);
  let func_type = FunctionType::new(typ, &vec![typ; proto_ast.args.len()]);
  module.add_function(&proto_ast.name, func_type)
  // TODO Set names for all arguments.
  // unsigned Idx = 0;
  // for (auto &Arg : F->args())
  //   Arg.setName(Args[Idx++]);
}

pub fn codegen_func<'a>(ctx: &'a Context,
  builder: &'a Builder,
  module: &'a Module,
  vals: &mut HashMap<String, &'a Value>,
  func_ast: parser::Function) -> Result<&'a Function, CompilerError> {

  if let Some(_) = module.get_function(&func_ast.proto.name) {
    return Err(CompilerError("function already defined".to_string()));
  }

  let func = codegen_proto(ctx, builder, module, vals, &func_ast.proto);
  let bb = func.append("entry");
  builder.position_at_end(bb);
  vals.clear();
  for i in 0..func_ast.proto.args.len() {
    vals.insert(func_ast.proto.args[i].to_string(), &func[i]);
  };
  let ret_val = try!(codegen(ctx, builder, module, vals, func_ast.body));
  builder.build_ret(ret_val);
  Ok(func)
}

pub fn codegen<'a>(ctx: &'a Context,
  builder: &'a Builder,
  module: &Module,
  vals: &mut HashMap<String, &'a Value>,
  expr: parser::Expr) -> CompilerResult<'a> {
  match expr {
    parser::Expr::Number(n) => Ok(n.compile(ctx)),
    parser::Expr::Variable(name) => match vals.get(&name) {
      Some(val) => Ok(val),
      None => make_error("couldn't find variable"),
    },
    parser::Expr::BinaryOp{op, lhs, rhs} => {
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
    parser::Expr::FuncCall{func, args} => unimplemented!(),
  }

}
