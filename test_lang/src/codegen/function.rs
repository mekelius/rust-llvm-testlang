use inkwell::{
    AddressSpace,
    types::BasicMetadataTypeEnum,
    values::{AnyValueEnum, BasicValue, BasicValueEnum},
};

use super::CodeGen;
use crate::{
    ast::{Expression, Function, Literal, Parameter},
    ast_store::{ASTStore, ExpressionID, FunctionID, StatementID, Store},
    codegen::{CodegenError, identifier::Symbol},
    span::SourceIDSpanned,
    types::SimpleType,
};

impl<'ctx> CodeGen<'ctx> {
    pub fn handle_function(
        &mut self,
        ast_store: &ASTStore,
        function_id: FunctionID,
    ) -> Result<(), CodegenError> {
        let Function {
            name,
            return_type_string,
            formals,
            body,
        } = &ast_store.functions.get_node(function_id).inner;

        let return_type = match return_type_string {
            Some(string) => SimpleType::from_type_string(&string),
            None => SimpleType::Void,
        };

        // Special case for main
        if name.inner == "main" {
            if return_type_string
                .as_ref()
                .is_some_and(|string| string.inner != "Int")
            {
                panic!("main function is only allowed return type Int (if specified)");
            }
            self.handle_main_function(ast_store, &formals, &body)?;
            return Ok(());
        }

        {
            let CodeGen { ir, scopes } = self;

            let i32_t = ir.context.i32_type();

            // Process formal params
            let formal_types: Vec<BasicMetadataTypeEnum<'ctx>> = formals
                .iter()
                .map(|formal| match &formal.inner {
                    Parameter::Untyped(_) => i32_t.into(),
                    Parameter::Typed(type_string, _) => ir
                        .type_string_to_ir_type(&type_string)
                        .unwrap_or_else(|| panic!("Void is not allowed as a parameter type"))
                        .try_into()
                        .unwrap(),
                })
                .collect();

            let signature = match return_type {
                SimpleType::Boolean => ir.context.bool_type().fn_type(&formal_types, false),
                SimpleType::Int => ir.context.i32_type().fn_type(&formal_types, false),
                SimpleType::Float => ir.context.f64_type().fn_type(&formal_types, false),
                SimpleType::Char => ir.context.i8_type().fn_type(&formal_types, false),
                SimpleType::Byte => ir.context.i8_type().fn_type(&formal_types, false),
                SimpleType::String => ir
                    .context
                    .ptr_type(AddressSpace::default())
                    .fn_type(&formal_types, false),
                SimpleType::Void => ir.context.void_type().fn_type(&formal_types, false),
                SimpleType::Unknown => panic!("Unknown type as return type"),
            };

            let function = ir.module.add_function(&name, signature, None);
            scopes.define_identifier(&name.to_string(), Symbol::Function(function));

            let entry_b = ir.context.append_basic_block(function, "Entry");
            ir.builder.position_at_end(entry_b);

            // Create function scope and add formal parameters to it
            let function_scope = scopes.push_new_scope();

            for (i, formal) in formals.iter().enumerate() {
                let value = function.get_nth_param(i.try_into()?);

                match &formal.inner {
                    Parameter::Typed(_, identifier) => {
                        function_scope.define_formal(&identifier, value.unwrap())
                    }
                    Parameter::Untyped(identifier) => {
                        function_scope.define_formal(&identifier, value.unwrap())
                    }
                };
            }
        }

        self.handle_function_body(ast_store, &body)?;

        {
            let CodeGen { ir, scopes } = self;

            // Insert return if return type is Void there isn't one
            if !ir.at_terminator() && return_type == SimpleType::Void {
                ir.builder.build_return(None)?;
            }
            scopes.pop_scope();
        }
        Ok(())
    }

    fn handle_main_function(
        &mut self,
        ast_store: &ASTStore,
        _formals: &Vec<SourceIDSpanned<Parameter>>,
        body: &Vec<StatementID>,
    ) -> Result<(), CodegenError> {
        {
            let CodeGen { ir, scopes } = self;

            let i32_t = ir.context.i32_type();
            let i32_ft = i32_t.fn_type(&[], false);

            let main_f = ir.module.add_function("main", i32_ft, None);
            scopes.define_identifier("main", Symbol::Function(main_f));

            let entry_b = ir.context.append_basic_block(main_f, "Entry");
            ir.builder.position_at_end(entry_b);

            // Create function scope and add formal parameters to it
            scopes.push_new_scope();
        }

        self.handle_function_body(ast_store, &body)?;

        {
            let CodeGen { ir, scopes } = self;

            let exit_code: BasicValueEnum<'ctx> = ir
                .context
                .i32_type()
                .const_int(0, false)
                .as_basic_value_enum();

            // Insert return if there isn't one

            if !ir.at_terminator() {
                ir.builder.build_return(Some(&exit_code))?;
            }
            scopes.pop_scope();
        }

        Ok(())
    }

    // Returns true if the body ends with a return statement
    fn handle_function_body(
        &mut self,
        ast_store: &ASTStore,
        body: &Vec<StatementID>,
    ) -> Result<(), CodegenError> {
        for statement in body {
            self.handle_statement(ast_store, *statement)?;
        }

        Ok(())
    }

    pub fn handle_return(
        &mut self,
        ast_store: &ASTStore,
        expression_id: ExpressionID,
    ) -> Result<(), CodegenError> {
        let expression = ast_store.get_expression(expression_id);

        if *expression == Expression::Literal(Literal::Unit) {
            self.ir.builder.build_return(None)?;
            return Ok(());
        }

        // let return_type = self.get_current_function().get_return_type();
        let value: Option<&dyn BasicValue> = match self.handle_expression(ast_store, expression_id)
        {
            Ok(AnyValueEnum::IntValue(value)) => Some(&value.clone()),
            Ok(AnyValueEnum::FloatValue(value)) => Some(&value.clone()),
            Ok(AnyValueEnum::PointerValue(value)) => Some(&value.clone()),
            Ok(AnyValueEnum::VectorValue(value)) => Some(&value.clone()),
            _ => unreachable!("Encountered unsupported return value type"),
        };

        self.ir.builder.build_return(value)?;
        Ok(())
    }
}
