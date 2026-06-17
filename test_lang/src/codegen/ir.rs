use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::support::LLVMString;
use inkwell::builder::Builder;

pub struct IR<'ctx> {
        pub context: &'ctx Context,
        pub module: Module<'ctx>,
        pub builder: Builder<'ctx>,
    }

impl<'ctx> IR<'ctx>{
    pub fn print(&self) -> Result<(), LLVMString> {
        let passed = self.module.verify();

        let dump = self.module.print_to_string().to_string();
        print!("{}", &dump);

        match passed {
            Err(err) => {
                print!("{:?}", err);
                panic!("Module verification failed");
            }
            Ok(_) => return Ok(()),
        };
    }

    pub fn print_to_file(&self, output_file: &str) -> Result<(), LLVMString> {
        self.module.verify()?;
        self.module.print_to_file(output_file)?;
        Ok(())
    }    
}