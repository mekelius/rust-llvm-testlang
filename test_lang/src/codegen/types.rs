use inkwell::{
    AddressSpace,
    types::{AnyType, AnyTypeEnum},
};

use crate::codegen::ir::IR;

#[derive(Copy, PartialEq, Clone, Debug)]
pub enum SimpleType {
    Boolean,
    Int,
    Float,
    Char,
    String,
    Void,
    Unknown,
}

// pub struct FunctionType {
//     return_type: Box<dyn Type>,
//     formal_types: Vec<Box<dyn Type>>,
// }

// pub trait Type {}

// impl Type for SimpleType {}
// impl Type for FunctionType {}

impl SimpleType {
    pub fn from_type_string(type_string: &str) -> SimpleType {
        match type_string {
            "Boolean" => SimpleType::Boolean,
            "Int" => SimpleType::Int,
            "Float" => SimpleType::Float,
            "Char" => SimpleType::Char,
            "String" => SimpleType::String,
            "Void" => SimpleType::Void,
            "Unknown" => SimpleType::Unknown,
            _ => panic!("Unknown type string"),
        }
    }
}

impl<'ctx> IR<'ctx> {
    pub fn simple_type_to_ir_type(&self, type_: SimpleType) -> Option<AnyTypeEnum<'ctx>> {
        match type_ {
            SimpleType::Boolean => Some(self.context.bool_type().as_any_type_enum()),
            SimpleType::Int => Some(self.context.i32_type().as_any_type_enum()),
            SimpleType::Float => Some(self.context.f64_type().as_any_type_enum()),
            SimpleType::Char => Some(self.context.i8_type().as_any_type_enum()),
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
