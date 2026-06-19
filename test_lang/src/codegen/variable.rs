use super::CodeGen;
use crate::{ast::Node, codegen::identifier::Symbol};

impl<'ctx> CodeGen<'ctx> {
    pub fn handle_const(&mut self, identifier: &str, expression: &Node) {
        let value = self.handle_expression(expression);
        let symbol = Symbol::Value(value);
        let current_scope = self.scopes.get_current_scope_mut();

        match current_scope.identifiers.get(identifier) {
            Some(_) => panic!("Attempting to redefine identifier \"{}\"", identifier),
            // NOTE: Only values defined with let atm
            None => current_scope.identifiers.insert(identifier.to_string(), symbol),
        };
    }
}
