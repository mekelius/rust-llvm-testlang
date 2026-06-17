use inkwell::{AddressSpace, types::FunctionType, values::FunctionValue};

use crate::codegen::IR;

use super::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub fn init_builtins(&mut self) {
        let CodeGen { ir, scopes } = self;

        macro_rules! builtin {
            ($identifier:ident, $signature:expr) => {
                let $identifier = ir.declare_builtin(stringify!($identifier), $signature);
                scopes.define_global_function(stringify!($identifier), $identifier);
            };
        }

        let bool_t = ir.context.bool_type();
        let i32_t = ir.context.i32_type();
        let int_t = ir.context.i64_type();
        let void_t = ir.context.void_type();
        let ptr_t = ir.context.ptr_type(AddressSpace::default());

        ///////////////////////////////////////// printf //////////////////////////////////////////
        builtin!(printf, &i32_t.fn_type(&[ptr_t.into()], true));

        //////////////////////////////////////// print_int ////////////////////////////////////////
        {
            builtin!(print_int, &void_t.fn_type(&[int_t.into()], false));

            let entry_b = ir.context.append_basic_block(print_int, "PrintIntEntry");
            ir.builder.position_at_end(entry_b);

            let int_format_string = ir
                .builder
                .build_global_string_ptr("%d\n", "printf_int_format")
                .unwrap();

            let arg1 = print_int.get_nth_param(0).unwrap();

            ir.builder
                .build_call(
                    printf,
                    &[int_format_string.as_pointer_value().into(), arg1.into()],
                    "",
                )
                .unwrap();

            ir.builder.build_return(None).unwrap();
        }

        //////////////////////////////////////// print_bool ////////////////////////////////////////
        {
            builtin!(print_bool, &void_t.fn_type(&[bool_t.into()], false));

            let entry_b = ir.context.append_basic_block(print_bool, "PrintBoolEntry");
            ir.builder.position_at_end(entry_b);

            let string_format_string = ir
                .builder
                .build_global_string_ptr("%s\n", "printf_string_format")
                .unwrap();

            let true_string = ir.builder.build_global_string_ptr("true", "true").unwrap();

            let false_string = ir
                .builder
                .build_global_string_ptr("false", "false")
                .unwrap();

            let arg1 = print_bool.get_nth_param(0).unwrap().into_int_value();
            let val_as_string = ir
                .builder
                .build_select(arg1, true_string, false_string, "val_as_str")
                .unwrap();
            ir.builder
                .build_call(
                    printf,
                    &[
                        string_format_string.as_pointer_value().into(),
                        val_as_string.into(),
                    ],
                    "",
                )
                .unwrap();

            ir.builder.build_return(None).unwrap();
        }

        ////////////////////////////////////////// print //////////////////////////////////////////
        {
            builtin!(print, &void_t.fn_type(&[ptr_t.into()], false));

            let entry_b = ir.context.append_basic_block(print, "PrintEntry");
            ir.builder.position_at_end(entry_b);

            let arg1 = print.get_nth_param(0).unwrap();
            ir.builder.build_call(printf, &[arg1.into()], "").unwrap();

            ir.builder.build_return(None).unwrap();
        }
    }
}

impl<'ctx> IR<'ctx> {
    fn declare_builtin(&self, name: &str, signature: &FunctionType<'ctx>) -> FunctionValue<'ctx> {
        let function = self.module.add_function(name, *signature, None);
        // scopes.define_global_identifier(name, Symbol::Function(function));
        // .builtins.add(name.to_string(), function);
        function
    }
}
