use testl::codegen::CodeGen;
use inkwell::context::Context;
use std::error::Error;
use std::io;
use testl::parser;

fn main() -> Result<(), Box<dyn Error>> {
    let src = io::read_to_string(io::stdin())?;
    let ast = parser::run(&src)?;

    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "test");

    codegen.run(&ast)?;
    codegen.ir.print()?;

    Ok(())
}
