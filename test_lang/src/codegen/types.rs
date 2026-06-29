use inkwell::{
    AddressSpace,
    types::{AnyType, AnyTypeEnum},
};

use crate::{codegen::ir::IR, types::SimpleType};

impl<'ctx> IR<'ctx> {
    pub fn simple_type_to_ir_type(&self, type_: SimpleType) -> Option<AnyTypeEnum<'ctx>> {
        match type_ {
            SimpleType::Boolean => Some(self.context.bool_type().as_any_type_enum()),
            SimpleType::Int => Some(self.context.i32_type().as_any_type_enum()),
            SimpleType::Float => Some(self.context.f64_type().as_any_type_enum()),
            SimpleType::Char => Some(self.context.i8_type().as_any_type_enum()),
            SimpleType::Byte => Some(self.context.i8_type().as_any_type_enum()),
            SimpleType::String => Some(
                self.context
                    .ptr_type(AddressSpace::default())
                    .as_any_type_enum(),
            ),
            SimpleType::Void => None,
            SimpleType::Unknown => panic!("Tried to convern Unknown into llvm type"),
        }
    }

    pub fn type_string_to_ir_type(&self, type_string: &str) -> Option<AnyTypeEnum<'ctx>> {
        self.simple_type_to_ir_type(SimpleType::from_type_string(type_string))
    }
}
