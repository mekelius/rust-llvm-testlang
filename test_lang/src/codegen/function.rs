use std::error::Error;

use super::CodeGen;
use crate::{ast::Node, codegen::identifier::Symbol};

impl<'ctx> CodeGen<'ctx> {
    pub fn handle_function(&mut self, function: &Node) -> Result<(), Box<dyn Error>> {
        let Node::Function {
            name,
            formals: _,
            body,
        } = function
        else {
            unreachable!();
        };

        {
            let CodeGen { ir, scopes } = self;

            let void_t = ir.context.void_type();
            // let i64_i64_i64_ft = i64_t.fn_type(&[i64_t.into(), i64_t.into(), i64_t.into()], false);
            let void_ft = void_t.fn_type(&[], false);

            let test_f = ir.module.add_function(&name, void_ft, None);
            scopes.define_identifier(&name.to_string(), Symbol::Function(test_f));

            let entry_b = ir.context.append_basic_block(test_f, "Entry");
            ir.builder.position_at_end(entry_b);

            // Prepare function scope -----------------------------------------------------

            let function_scope = scopes.push_new_scope();

            // ----------------------------------------------------------------------------------------
        }

        {
            let Node::FunctionBody(body) = &**body else {
                unreachable!();
            };
            for statement in body {
                self.handle_statement(&statement);
            }
        }

        {
            let CodeGen { ir, scopes } = self;

            ir.builder.build_return(None)?;
            scopes.pop_scope();
        }
        Ok(())
    }
}
