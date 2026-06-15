use inkwell::{
    AddressSpace, builder::Builder, context::Context, module::Module, values::FunctionValue,
};
use dict::{ Dict, DictIface };

use super::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub fn init_builtins(&mut self) {
        self.printf_decl();
        self.print_def();
    }

    fn printf_decl(&mut self) {
        let i32_t = self.context.i32_type();
        let ptr_t = self.context.ptr_type(AddressSpace::default());

        let printf_ft = i32_t.fn_type(&[ptr_t.into()], true);

        let printf = self.module.add_function("printf", printf_ft, None);
        self.builtins.add("printf".to_string(), printf);
    }

    fn print_def(&mut self) {
        let printf = self.builtins.get("printf").unwrap();
        let i64_t = self.context.i64_type();
        let void_t = self.context.void_type();

        let format_string = "%d\n";

        let print_ft = void_t.fn_type(&[i64_t.into()], false);

        let print_f = self.module.add_function("print", print_ft, None);
        let entry_b = self.context.append_basic_block(print_f, "Entry");

        let arg1 = print_f.get_nth_param(0).unwrap();
        self.builder.position_at_end(entry_b);
        let format_string_val_ptr = self
            .builder
            .build_global_string_ptr(format_string, "int_printf_format")
            .unwrap();
        self.builder.build_call(
            *printf,
            &[format_string_val_ptr.as_pointer_value().into(), arg1.into()],
            "",
        );
        self.builder.build_return(None);
    }
}
