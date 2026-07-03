use std::error::Error;

use inkwell::{
    AddressSpace,
    types::BasicMetadataTypeEnum,
    values::{AnyValueEnum, BasicValue, BasicValueEnum},
};

use super::CodeGen;
use crate::{
    ast::{Expression, Function, Literal, Parameter}, ast_store::{ExpressionID, FunctionID}, ast_visitor::ASTVisitor, codegen::identifier::Symbol, span::SourceIDSpanned, types::SimpleType,
};

impl<'ctx> CodeGen<'ctx> {
    fn enter_function(
        &mut self,
        (function, _): (&SourceIDSpanned<Function>, FunctionID),
    ) -> Option<Box<dyn Error>> {
        let Function {
            name,
            return_type_string,
            formals,
            body: _,
        } = &function.inner;

        let return_type = match return_type_string {
            Some(string) => SimpleType::from_type_string(&string.inner),
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
            self.enter_main_function(&formals);
            return None;
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
                let value = function.get_nth_param(i.try_into().unwrap());

                match &formal.inner {
                    Parameter::Typed(_, identifier) => {
                        function_scope.define_formal(&identifier, value.unwrap())
                    }
                    Parameter::Untyped(identifier) => {
                        function_scope.define_formal(&identifier, value.unwrap())
                    }
                };
            }
            None
        }
    }

    fn exit_function(
        &mut self,
        (function, _): (&SourceIDSpanned<Function>, FunctionID),
    ) -> Option<Box<dyn Error>> {
        {
            let Function {
                name,
                return_type_string,
                formals: _,
                body: _,
            } = &function.inner;

            // Special case for main
            if name.inner == "main" {
                self.exit_main_function();
                return None;
            }
            let CodeGen { ir, scopes } = self;

            let return_type = match return_type_string {
                Some(string) => SimpleType::from_type_string(&string.inner),
                None => SimpleType::Void,
            };
            // Insert return if return type is Void there isn't one
            if !ir.at_terminator() && return_type == SimpleType::Void {
                ir.builder.build_return(None).unwrap();
            }
            scopes.pop_scope();
        }
        None
    }
}

impl<'ctx> CodeGen<'ctx> {
    fn enter_main_function(&mut self, _formals: &Vec<SourceIDSpanned<Parameter>>) {
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

    fn exit_main_function(&mut self) {
        let CodeGen { ir, scopes } = self;

        let exit_code: BasicValueEnum<'ctx> = ir
            .context
            .i32_type()
            .const_int(0, false)
            .as_basic_value_enum();

        // Insert return if there isn't one

        if !ir.at_terminator() {
            ir.builder.build_return(Some(&exit_code)).unwrap();
        }
        scopes.pop_scope();
    }

    pub fn exit_return(&mut self) -> Option<Box<dyn Error>> {
        let return_value = self.ir.expression_stack.pop().unwrap();

        // let return_type = self.get_current_function().get_return_type();
        let value: Option<&dyn BasicValue> = match return_value {
            Some(AnyValueEnum::IntValue(value)) => Some(&value.clone()),
            Some(AnyValueEnum::FloatValue(value)) => Some(&value.clone()),
            Some(AnyValueEnum::PointerValue(value)) => Some(&value.clone()),
            Some(AnyValueEnum::VectorValue(value)) => Some(&value.clone()),
            None => None,
            _ => todo!("Other return types"),
        };

        self.ir.builder.build_return(value).unwrap();
        // match *return_type {
        //     SimpleType::Boolean => self.ir.builder.build_return(Some(&value.into_int_value())).unwrap(),
        //     SimpleType::Int => self.ir.builder.build_return(Some(&value.into_int_value())).unwrap(),
        //     SimpleType::Float => self.ir.builder.build_return(Some(&value.into_float_value())).unwrap(),
        //     SimpleType::Char => self.ir.builder.build_return(Some(&value.into_int_value())).unwrap(),
        //     SimpleType::String => self.ir.builder.build_return(Some(&value.into_pointer_value())).unwrap(),
        //     SimpleType::Void => self.ir.builder.build_return(None).unwrap(),
        //     SimpleType::Unknown => panic!("Unknown type as return type"),
        // };

        None
    }

    // fn update_return_type(&self, function: FunctionValue<'ctx>, return_type: BasicTypeEnum<'ctx>) -> FunctionValue<'ctx> {
    //     let name = function.get_name().to_str().unwrap();
    //     function.nam
    //     let formal_types = function.get_type().get_param_types();
    //     let new_signature = return_type.fn_type(&formal_types, false);
    //     let new_function = self.ir.module.add_function(name, new_signature, None);

    //     new_function
    // }
}
