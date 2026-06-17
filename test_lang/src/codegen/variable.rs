use super::CodeGen;
use crate::{ast::Node, codegen::{identifier::Symbol, scope::Scopes}};

impl<'ctx> CodeGen<'ctx> {
    pub fn handle_let(&self, identifier: &str, expression: &Node, scopes: &mut Scopes<'ctx>) {
        let value = self.handle_expression(expression, scopes);
        let symbol = Symbol::Value(value);
        let current_scope = scopes.get_current_scope_mut();

        match current_scope.identifiers.get(identifier) {
            Some(_) => panic!("Attempting to redefine identifier \"{}\"", identifier),
            // NOTE: Only values defined with let atm
            None => current_scope.identifiers.insert(identifier.to_string(), symbol),
        };
    }
}
