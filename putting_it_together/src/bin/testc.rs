use testl::codegen::CodeGen;
use inkwell::context::Context;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let source = io::read_to_string(io::stdin())?;

    let context = Context::create();
    // let module = context.create_module("test");
    // let builder = context.create_builder();

    let codegen = CodeGen {
        context: &context,
        module: context.create_module("test"),
        builder: context.create_builder(),
    };

    codegen.run()?;
    codegen.print()?;

    Ok(())
}
