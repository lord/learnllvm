extern crate llvm;
#[macro_use]
extern crate nom;
extern crate llvm_sys;

use llvm::{Context, Builder, Module, Value};
use std::collections::HashMap;
use std::mem;

mod lexer;
mod parser;
mod codegen;

use std::io;
use std::io::Write;
use std::io::stdout;
use llvm_sys::core::LLVMDumpValue;
use llvm_sys::prelude::LLVMValueRef;

fn main() {
  let ctx = Context::new();
  let ctx = ctx.as_semi();
  let builder = Builder::new(&ctx);
  let mut vars = HashMap::new();
  let mut module = Module::new("mod_name", &ctx);
  loop {
    print!("ks> ");
    let _ = stdout().flush();
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    if let Some(tokens) = lexer::lex(&input) {
      let ast = parser::parse(&tokens);
      println!("lex: {:?}", &tokens);
      println!("parse: {:?}", &ast);
      if let Ok(parser::AST::Expr(expr)) = ast {
        let val = codegen::codegen(&ctx, &builder, &mut module, &mut vars, expr);
        println!("codegen: {:?}", &val);
        if let Ok(value) = val {
          // unsafe {
          //   let vref: LLVMValueRef = mem::transmute(value);
          //   println!("foo")
          //   LLVMDumpValue(vref);
          // }
        } else {
          println!("failed, so not dumping");
        }
      } else {
        println!("Didn't codegen");
      }
    } else {
      println!("failed to lex!")
    }
  }
}
