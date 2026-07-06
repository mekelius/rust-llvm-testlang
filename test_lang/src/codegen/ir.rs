use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::support::LLVMString;

pub struct IR<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
}

impl<'ctx> IR<'ctx> {
    /** Helper that returns true if the last instruction in the current block is a terminator
     *  (and the block is not empty) */
    pub fn at_terminator(&self) -> bool {
        let last_instruction = self
            .builder
            .get_insert_block()
            .expect("at_terminator should not be called unless inside a block")
            .get_last_instruction();

        last_instruction.is_some_and(|instruction| instruction.is_terminator())
    }

    pub fn print(&self) -> Result<(), LLVMString> {
        self.module.verify()?;
        let dump = self.module.print_to_string().to_string();
        print!("{}", &dump);

        Ok(())
    }

    pub fn print_to_file(&self, output_file: &str) -> Result<(), LLVMString> {
        self.module.verify()?;
        self.module.print_to_file(output_file)?;
        Ok(())
    }
}
