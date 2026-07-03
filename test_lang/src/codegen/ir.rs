use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::support::LLVMString;
use inkwell::values::AnyValueEnum;

pub struct IR<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub expression_stack: Vec<Option<AnyValueEnum<'ctx>>>,
    pub statement_stack: Vec<BasicBlock<'ctx>>,
}

impl<'ctx> IR<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &'ctx str) -> Self {
        IR {
            context,
            module: context.create_module(module_name),
            builder: context.create_builder(),
            expression_stack: vec![],
            statement_stack: vec![],
        }
    }

    /** Helper that returns true if the last instruction in the current block is a terminator 
     *  (and the block is not empty) */
    pub fn at_terminator(&self) -> bool {
        let last_instruction = self
            .builder
            .get_insert_block()
            .unwrap()
            .get_last_instruction();

        last_instruction.is_some_and(|instruction| instruction.is_terminator())
    }

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
