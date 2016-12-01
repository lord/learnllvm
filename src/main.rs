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
  let mut module = Module::new("mod_name", &ctx);
  let mut vars = HashMap::new();
  loop {
    print!("ks> ");
    let _ = stdout().flush();
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    if let Some(tokens) = lexer::lex(&input) {
      let ast = parser::parse(&tokens);
      println!("lex: {:?}", &tokens);
      println!("parse: {:?}", &ast);
      match ast {
        Ok(parser::AST::Expr(e)) => {
          let val = codegen::codegen(&ctx, &builder, &module, &mut vars, e);
          println!("codegen value: {:?}", &val);
          // unsafe {
          //   let vref: LLVMValueRef = mem::transmute(value.unwrap());
          //   println!("foo")
          //   LLVMDumpValue(vref);
          // }
        }
        Ok(parser::AST::Prototype(p)) => {
          codegen::codegen_proto(&ctx, &builder, &module, &mut vars, &p);
        },
        Ok(parser::AST::Function(f)) => {
          let val = codegen::codegen_func(&ctx, &builder, &module, &mut vars, f);
          if let Err(msg) = val {
            println!("codegen error: {:?}", &msg);
          }
          println!("module so far:");
          unsafe {
              let mref: llvm_sys::prelude::LLVMModuleRef = module.as_ptr();
              llvm_sys::core::LLVMDumpModule(mref);
          }
        }
        _ => {
          println!("Failed to parse!");
        }
      };
    } else {
      println!("failed to lex!")
    }
  }
}
