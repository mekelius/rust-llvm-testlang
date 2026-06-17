use std::error::Error;

use inkwell::{types::BasicMetadataTypeEnum, values::{BasicValue, BasicValueEnum}};

use super::CodeGen;
use crate::{
    ast::Node,
    codegen::identifier::{self, Symbol},
};

impl<'ctx> CodeGen<'ctx> {
    pub fn handle_function(&mut self, function: &Node) -> Result<(), Box<dyn Error>> {
        let Node::Function {
            name,
            formals,
            body,
        } = function
        else {
            unreachable!();
        };

        // Special case for main
        if name == "main" {
            self.handle_main_function(formals, body);
            return Ok(())
        }

        {
            let CodeGen { ir, scopes } = self;

            let void_t = ir.context.void_type();
            let i64_t = ir.context.i64_type();
            // let i64_i64_i64_ft = i64_t.fn_type(&[i64_t.into(), i64_t.into(), i64_t.into()], false);

            // Process formal params
            let formal_types: Vec<BasicMetadataTypeEnum<'ctx>> = formals
                .iter()
                .map(|formal| match formal {
                    Node::UntypedFormal(_) => i64_t.into(),

                    Node::TypedFormal(type_, value) => todo!("typed formals"),
                    _ => unreachable!(),
                })
                .collect();

            let void_ft = void_t.fn_type(&formal_types, false);

            let function = ir.module.add_function(&name, void_ft, None);
            scopes.define_identifier(&name.to_string(), Symbol::Function(function));

            let entry_b = ir.context.append_basic_block(function, "Entry");
            ir.builder.position_at_end(entry_b);

            // Create function scope and add formal parameters to it
            let function_scope = scopes.push_new_scope();

            for (i, formal) in formals.iter().enumerate() {
                let value = function.get_nth_param(i.try_into()?);

                match formal {
                    Node::TypedFormal(_type_, _value) => todo!("typed formals"),
                    Node::UntypedFormal(identifier) => {
                        function_scope.define_formal(identifier, value.unwrap())
                    }
                    _ => unreachable!(),
                };
            }
        }

        self.handle_function_body(body);

        {
            let CodeGen { ir, scopes } = self;

            ir.builder.build_return(None)?;
            scopes.pop_scope();
        }
        Ok(())
    }

    fn handle_main_function(&mut self, formals: &Vec<Node>, body: &Box<Node>) {
        {
            let CodeGen { ir, scopes } = self;

            let i64_t = ir.context.i64_type();
            let i64_ft = i64_t.fn_type(&[], false);

            let main_f = ir.module.add_function("main", i64_ft, None);
            scopes.define_identifier("main", Symbol::Function(main_f));

            let entry_b = ir.context.append_basic_block(main_f, "Entry");
            ir.builder.position_at_end(entry_b);

            // Create function scope and add formal parameters to it
            let function_scope = scopes.push_new_scope();
        }

        self.handle_function_body(body);

        {
            let CodeGen { ir, scopes } = self;

            let exit_code: BasicValueEnum<'ctx> = ir.context.i64_type().const_int(0, false).as_basic_value_enum();
            ir.builder.build_return(Some(&exit_code)).unwrap();
            scopes.pop_scope();
        }
    }

    fn handle_function_body(&mut self, body: &Box<Node>) {
        let Node::FunctionBody(body) = &**body else {
            unreachable!();
        };
        for statement in body {
            self.handle_statement(&statement);
        }
    }
}
