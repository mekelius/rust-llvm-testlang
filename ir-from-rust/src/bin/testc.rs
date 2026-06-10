use std::error::Error;
use inkwell::module::Module;
use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::support::LLVMString;

struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let i64_t = self.context.i64_type();
        // let i64_i64_i64_ft = i64_t.fn_type(&[i64_t.into(), i64_t.into(), i64_t.into()], false);
        let i64_ft = i64_t.fn_type(&[i64_t.into()], false);

        let c1 = i64_t.const_int(34, false);
        let test_f = self.module.add_function("test", i64_ft, None);

        let entry_b = self.context.append_basic_block(test_f, "entry");
        self.builder.position_at_end(entry_b);
        self.builder.build_return(Some(&c1))?;

        Ok(())
    }

    fn print(&self, output_file: &str) -> Result<(), LLVMString>  {
        self.module.verify()?;
        self.module.print_to_file(output_file)?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();

    let codegen = CodeGen {
        context: &context,
        module,
        builder,
    };

    codegen.run()?;
    codegen.print("./out.ll")?;

    Ok(())
}
