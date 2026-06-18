use crate::codegen::CodeGen;

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
