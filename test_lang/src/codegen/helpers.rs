use inkwell::{
    types::{AnyTypeEnum, BasicTypeEnum},
    values::{AnyValueEnum, BasicValueEnum},
};

use crate::codegen::CodegenError;

pub trait TryIntoOverride<T> {
    fn try_into_override(self) -> Result<T, CodegenError>;
}

impl<'ctx> TryIntoOverride<BasicValueEnum<'ctx>> for AnyValueEnum<'ctx> {
    fn try_into_override(self) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        self.try_into()
            .map_err(|_| "converting AnyValueEnum to BasicValueEnum failed".into())
    }
}

impl<'ctx> TryIntoOverride<BasicTypeEnum<'ctx>> for AnyTypeEnum<'ctx> {
    fn try_into_override(self) -> Result<BasicTypeEnum<'ctx>, CodegenError> {
        self.try_into()
            .map_err(|_| "converting AnyTypeEnum to BasicTypeEnum failed".into())
    }
}
