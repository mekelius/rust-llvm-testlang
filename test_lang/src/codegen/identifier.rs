use inkwell::values::{AnyValue, AnyValueEnum, FunctionValue};

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
    pub fn handle_identifier(&self, identifier: &str) -> Symbol<'ctx> {
        (self
            .scopes
            .resolve_identifier(identifier)
            .unwrap_or_else(|| panic!("Attempt to resolve undefined identifier {}", identifier)))
        .clone()
    }
}
