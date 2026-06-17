use inkwell::values::{AnyValue, AnyValueEnum, FunctionValue};

use crate::codegen::scope::Scopes;

use super::CodeGen;

#[derive(Debug, Clone)]
pub enum Symbol<'ctx> {
    Function(FunctionValue<'ctx>),
    Variable(AnyValueEnum<'ctx>),
    Formal(AnyValueEnum<'ctx>),
    Value(AnyValueEnum<'ctx>),
}

impl<'ctx> Symbol<'ctx> {
    pub fn as_any_value_enum(&self) -> AnyValueEnum<'ctx> {
        match self {
            Symbol::Function(value) => value.as_any_value_enum(),
            Symbol::Variable(value) => *value,
            Symbol::Formal(value) => *value,
            Symbol::Value(value) => *value,
        }
    }
}

impl<'ctx> CodeGen<'ctx> {
    pub fn resolve_function(
        identifier: &str,
        scopes: &Scopes<'ctx>,
    ) -> Option<FunctionValue<'ctx>> {
        match scopes.resolve_identifier(identifier) {
            Some(Symbol::Function(function)) => Some(function.clone()),
            _ => None,
        }
    }

    pub fn handle_identifier(&self, identifier: &str, scopes: &mut Scopes<'ctx>) -> Symbol<'ctx> {
        (*scopes
            .resolve_identifier(identifier)
            .unwrap_or_else(|| panic!("Attempt to resolve undefined identifier {}", identifier)))
        .clone()
    }
}
