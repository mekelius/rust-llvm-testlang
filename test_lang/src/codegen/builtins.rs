use inkwell::{AddressSpace, types::FunctionType, values::FunctionValue};

use crate::codegen::{CodegenError, IR};

use super::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub fn init_builtins(&mut self) -> Result<(), CodegenError> {
        let CodeGen { ir, scopes } = self;

        macro_rules! builtin {
            ($identifier:ident, $signature:expr) => {
                let $identifier = ir.declare_builtin(stringify!($identifier), $signature);
                scopes.define_global_function(stringify!($identifier), $identifier);
            };
        }

        let bool_t = ir.context.bool_type();
        let i32_t = ir.context.i32_type();
        let int_t = ir.context.i32_type();
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
                .build_global_string_ptr("%d", "printf_int_format")?;

            let arg1 = print_int.get_nth_param(0).expect("parameter should exist");

            ir.builder.build_call(
                printf,
                &[int_format_string.as_pointer_value().into(), arg1.into()],
                "",
            )?;

            ir.builder.build_return(None)?;
        }

        //////////////////////////////////////// print_bool ////////////////////////////////////////
        {
            builtin!(print_bool, &void_t.fn_type(&[bool_t.into()], false));

            let entry_b = ir.context.append_basic_block(print_bool, "PrintBoolEntry");
            ir.builder.position_at_end(entry_b);

            let string_format_string = ir
                .builder
                .build_global_string_ptr("%s", "printf_string_format")?;

            let true_string = ir.builder.build_global_string_ptr("true", "true")?;

            let false_string = ir.builder.build_global_string_ptr("false", "false")?;

            let arg1 = print_bool
                .get_nth_param(0)
                .expect("parameter should exist")
                .into_int_value();
            let val_as_string =
                ir.builder
                    .build_select(arg1, true_string, false_string, "val_as_str")?;
            ir.builder.build_call(
                printf,
                &[
                    string_format_string.as_pointer_value().into(),
                    val_as_string.into(),
                ],
                "",
            )?;

            ir.builder.build_return(None)?;

            //////////////////////////////////////// println_bool ////////////////////////////////////////
            {
                builtin!(println_bool, &void_t.fn_type(&[bool_t.into()], false));

                let entry_b = ir
                    .context
                    .append_basic_block(println_bool, "PrintlnBoolEntry");
                ir.builder.position_at_end(entry_b);

                let arg1 = println_bool
                    .get_nth_param(0)
                    .expect("parameter should exist")
                    .into_int_value();
                ir.builder.build_call(print_bool, &[arg1.into()], "")?;

                let newline = ir.builder.build_global_string_ptr("\n", "newline")?;

                ir.builder
                    .build_call(printf, &[newline.as_pointer_value().into()], "")?;

                ir.builder.build_return(None)?;
            }
        }

        ////////////////////////////////////////// print //////////////////////////////////////////
        {
            builtin!(print, &void_t.fn_type(&[ptr_t.into()], false));

            let entry_b = ir.context.append_basic_block(print, "PrintEntry");
            ir.builder.position_at_end(entry_b);

            let arg1 = print.get_nth_param(0).expect("parameter should exist");
            ir.builder.build_call(printf, &[arg1.into()], "")?;

            ir.builder.build_return(None)?;
        }

        //////////////////////////////////////// println_int ////////////////////////////////////////
        {
            builtin!(println_int, &void_t.fn_type(&[int_t.into()], false));

            let entry_b = ir
                .context
                .append_basic_block(println_int, "PrintlnIntEntry");
            ir.builder.position_at_end(entry_b);

            let lnint_format_string = ir
                .builder
                .build_global_string_ptr("%d\n", "printf_lnint_format")?;

            let arg1 = println_int
                .get_nth_param(0)
                .expect("parameter should exist");

            ir.builder.build_call(
                printf,
                &[lnint_format_string.as_pointer_value().into(), arg1.into()],
                "",
            )?;

            ir.builder.build_return(None)?;
        }

        Ok(())
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
