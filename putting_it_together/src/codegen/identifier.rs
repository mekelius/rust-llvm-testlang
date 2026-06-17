use inkwell::values::{AnyValueEnum, FunctionValue};

use crate::codegen::scope::Scopes;

use super::CodeGen;

#[derive(Debug)]
pub enum Symbol<'ctx> {
    Function(FunctionValue<'ctx>),
    Variable(AnyValueEnum<'ctx>),
    Formal(AnyValueEnum<'ctx>),
    Value(AnyValueEnum<'ctx>),
    Empty,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn resolve_function(&self, identifier: &str, scopes: &Scopes<'ctx>) -> Option<FunctionValue<'ctx>> {
        match scopes.resolve_identifier(identifier) {
            Some(Symbol::Function(function)) => Some(function.clone()),
            _ => None,
        }
    }
}
