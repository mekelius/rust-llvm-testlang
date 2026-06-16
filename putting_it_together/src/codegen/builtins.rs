use dict::DictIface;
use inkwell::{
    AddressSpace,
    types::FunctionType,
    values::FunctionValue,
};

use super::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub fn init_builtins(&mut self) {
        let bool_t = self.context.bool_type();
        let i32_t = self.context.i32_type();
        let int_t = self.context.i64_type();
        let void_t = self.context.void_type();
        let ptr_t = self.context.ptr_type(AddressSpace::default());

        ///////////////////////////////////////// printf //////////////////////////////////////////
        let printf = self.declare_builtin("printf", &i32_t.fn_type(&[ptr_t.into()], true));

        //////////////////////////////////////// print_int ////////////////////////////////////////
        {
            let print_int =
                self.declare_builtin("print_int", &void_t.fn_type(&[int_t.into()], false));
            let entry_b = self.context.append_basic_block(print_int, "PrintIntEntry");
            self.builder.position_at_end(entry_b);

            let int_format_string = self
                .builder
                .build_global_string_ptr("%d\n", "printf_int_format")
                .unwrap();

            let arg1 = print_int.get_nth_param(0).unwrap();
            
            self.builder
                .build_call(
                    printf,
                    &[int_format_string.as_pointer_value().into(), arg1.into()],
                    "",
                )
                .unwrap();

            self.builder.build_return(None).unwrap();
        }

        //////////////////////////////////////// print_bool ////////////////////////////////////////
        {
            let print_bool =
                self.declare_builtin("print_bool", &void_t.fn_type(&[bool_t.into()], false));
            let entry_b = self.context.append_basic_block(print_bool, "PrintBoolEntry");
            self.builder.position_at_end(entry_b);

            let string_format_string = self
                .builder
                .build_global_string_ptr("%s\n", "printf_string_format")
                .unwrap();

            let true_string = self
                .builder
                .build_global_string_ptr("true", "true")
                .unwrap();

            let false_string = self
                .builder
                .build_global_string_ptr("false", "false")
                .unwrap();

            let arg1 = print_bool.get_nth_param(0).unwrap().into_int_value();
            let val_as_string = self
                .builder
                .build_select(arg1, true_string, false_string, "val_as_str")
                .unwrap();
            self.builder
                .build_call(
                    printf,
                    &[
                        string_format_string.as_pointer_value().into(),
                        val_as_string.into(),
                    ],
                    "",
                )
                .unwrap();

            self.builder.build_return(None).unwrap();
        }

        ////////////////////////////////////////// print //////////////////////////////////////////
        {
            let print = self.declare_builtin("print", &void_t.fn_type(&[ptr_t.into()], false));
            let entry_b = self.context.append_basic_block(print, "PrintEntry");
            self.builder.position_at_end(entry_b);

            let arg1 = print.get_nth_param(0).unwrap();
            self.builder.build_call(printf, &[arg1.into()], "").unwrap();

            self.builder.build_return(None).unwrap();
        }
    }

    fn declare_builtin(
        &mut self,
        name: &str,
        signature: &FunctionType<'ctx>,
    ) -> FunctionValue<'ctx> {
        let function = self.module.add_function(name, *signature, None);
        self.builtins.add(name.to_string(), function);
        function
    }
}
