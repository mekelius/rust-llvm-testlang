use inkwell::{
    types::BasicTypeEnum,
    values::{AnyValue, AnyValueEnum, BasicValueEnum, FunctionValue, PointerValue},
};

use super::CodeGen;
use crate::{ast::Expression, codegen::CodegenError, types::SimpleType};

#[derive(Debug, Clone)]
pub enum Symbol<'ctx> {
    Function(FunctionValue<'ctx>),
    Variable {
        pointer: PointerValue<'ctx>,
        type_: SimpleType,
    },
    Formal(BasicValueEnum<'ctx>),
    Value(AnyValueEnum<'ctx>),
}

impl<'ctx> Symbol<'ctx> {
    /*
     * NOTE: Variables cannot be converted to enum, because they are initially created as stack variables
     */
    pub fn as_any_value_enum(&self) -> Option<AnyValueEnum<'ctx>> {
        match self {
            Symbol::Function(value) => Some(value.as_any_value_enum()),
            Symbol::Variable {
                pointer: _,
                type_: _,
            } => None,
            Symbol::Formal(value) => Some(*&value.as_any_value_enum()),
            Symbol::Value(value) => Some(*value),
        }
    }
}

impl<'ctx> CodeGen<'ctx> {
    pub fn handle_lvalue(&self, lvalue: &Expression) -> Result<AnyValueEnum<'ctx>, CodegenError> {
        match lvalue {
            Expression::Identifier(value) => self.handle_identifier(value),
            _ => todo!("Non identifier lvalues"),
        }
    }

    pub fn handle_identifier(&self, identifier: &str) -> Result<AnyValueEnum<'ctx>, CodegenError> {
        let symbol = self
            .scopes
            .resolve_identifier(identifier)
            .unwrap_or_else(|| panic!("Attempt to resolve undefined identifier {}", identifier));

        Ok(match symbol {
            Symbol::Variable { pointer, type_ } => {
                let ir_type: BasicTypeEnum<'ctx> = self
                    .ir
                    .simple_type_to_ir_type(*type_)
                    .unwrap()
                    .try_into()
                    .unwrap();
                let value: BasicValueEnum<'ctx> =
                    self.ir.builder.build_load(ir_type, *pointer, "")?;
                Some(value.as_any_value_enum())
            }
            _ => None,
        }
        .or_else(|| symbol.as_any_value_enum())
        .unwrap())
    }

    /** unwraps the symbol, and if it is a variable, loads it from the stack */
    pub fn symbol_to_value(
        &self,
        symbol: &Symbol<'ctx>,
    ) -> Result<AnyValueEnum<'ctx>, CodegenError> {
        Ok(symbol
            .as_any_value_enum()
            .or_else(|| match symbol {
                Symbol::Variable { pointer, type_ } => {
                    Some(self.load_variable(*pointer, type_).unwrap())
                }
                _ => unreachable!(),
            })
            .unwrap())
    }
}
