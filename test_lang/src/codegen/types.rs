use inkwell::{
    AddressSpace,
    types::{AnyType, AnyTypeEnum},
};

use crate::{
    codegen::{CodegenError, ir::IR},
    types::SimpleType,
};

impl<'ctx> IR<'ctx> {
    pub fn simple_type_to_ir_type(
        &self,
        type_: SimpleType,
    ) -> Result<Option<AnyTypeEnum<'ctx>>, CodegenError> {
        match type_ {
            SimpleType::Boolean => Ok(Some(self.context.bool_type().as_any_type_enum())),
            SimpleType::Int => Ok(Some(self.context.i32_type().as_any_type_enum())),
            SimpleType::Float => Ok(Some(self.context.f64_type().as_any_type_enum())),
            SimpleType::Char => Ok(Some(self.context.i8_type().as_any_type_enum())),
            SimpleType::Byte => Ok(Some(self.context.i8_type().as_any_type_enum())),
            SimpleType::String => Ok(Some(
                self.context
                    .ptr_type(AddressSpace::default())
                    .as_any_type_enum(),
            )),
            SimpleType::Void => Ok(None),
            SimpleType::Unknown => Err("Tried to convern Unknown into llvm type".into()),
        }
    }

    pub fn type_string_to_ir_type(
        &self,
        type_string: &str,
    ) -> Result<Option<AnyTypeEnum<'ctx>>, CodegenError> {
        self.simple_type_to_ir_type(
            SimpleType::from_type_string(type_string)
                .ok_or::<CodegenError>("invalid_type_string".into())?,
        )
    }
}
