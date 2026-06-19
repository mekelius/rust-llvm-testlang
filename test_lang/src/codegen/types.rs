use inkwell::{AddressSpace, AtomicRMWBinOp::Add, types::{AnyType, AnyTypeEnum}};

use crate::codegen::{CodeGen, ir::IR};

#[derive(PartialEq)]
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

impl<'ctx> CodeGen<'ctx> {
    pub fn handle_type_string(&self, type_string: &str) -> SimpleType {
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
    pub fn type_string_to_ir_type(&self, type_string: &str) -> Option<AnyTypeEnum<'ctx>> {
        match type_string {
            "Boolean" => Some(self.context.bool_type().as_any_type_enum()),
            "Int" => Some(self.context.i64_type().as_any_type_enum()),
            "Float" => Some(self.context.f64_type().as_any_type_enum()),
            "Char" => Some(self.context.i8_type().as_any_type_enum()),
            "String" => Some(self.context.ptr_type(AddressSpace::default()).as_any_type_enum()),
            "Void" => None,
            "Unknown" => panic!("Tried to convern Unknown into llvm type"),
            _ => panic!("Unknown type string"),
        }
    }
}