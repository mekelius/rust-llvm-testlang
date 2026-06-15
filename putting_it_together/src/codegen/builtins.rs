use dict::DictIface;
use inkwell::{AddressSpace, types::FunctionType, values::FunctionValue};

use super::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub fn init_builtins(&mut self) {
        let i32_t = self.context.i32_type();
        let i64_t = self.context.i64_type();
        let void_t = self.context.void_type();
        let ptr_t = self.context.ptr_type(AddressSpace::default());

        ///////////////////////////////////////// printf ////////////////////////////////////////// 
        let printf = self.declare_builtin("printf", &i32_t.fn_type(&[ptr_t.into()], true));

        //////////////////////////////////////// print_int //////////////////////////////////////// 
        let print_int = self.declare_builtin("print_int", &void_t.fn_type(&[i64_t.into()], false));

        let int_format_string = "%d\n";

        let entry_b = self.context.append_basic_block(print_int, "Entry");

        let arg1 = print_int.get_nth_param(0).unwrap();
        self.builder.position_at_end(entry_b);
        let int_format_string_val_ptr = self
            .builder
            .build_global_string_ptr(int_format_string, "printf_int_format")
            .unwrap();
        self.builder
            .build_call(
                printf,
                &[int_format_string_val_ptr.as_pointer_value().into(), arg1.into()],
                "",
            )
            .unwrap();

        self.builder.build_return(None).unwrap();


        ////////////////////////////////////////// print ////////////////////////////////////////// 
        let print = self.declare_builtin("print", &void_t.fn_type(&[ptr_t.into()], false));

        let entry_b = self.context.append_basic_block(print, "Entry");

        let arg1 = print.get_nth_param(0).unwrap();
        self.builder.position_at_end(entry_b);
        self.builder
            .build_call(
                printf,
                &[arg1.into()],
                "",
            )
            .unwrap();

        self.builder.build_return(None).unwrap();
    }

    fn declare_builtin(&mut self, name: &str, signature: &FunctionType<'ctx>) -> FunctionValue<'ctx> {
        let function = self.module.add_function(name, *signature, None);
        self.builtins.add(name.to_string(), function);
        function
    }
}
