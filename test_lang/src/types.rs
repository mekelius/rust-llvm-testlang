
#[derive(Copy, PartialEq, Clone, Debug)]
pub enum SimpleType {
    Boolean,
    Int,
    Float,
    Char,
    Byte,
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
            "Bool" => SimpleType::Boolean,
            "Int" => SimpleType::Int,
            "Float" => SimpleType::Float,
            "Char" => SimpleType::Char,
            "Byte" => SimpleType::Byte,
            "String" => SimpleType::String,
            "Void" => SimpleType::Void,
            "Unknown" => SimpleType::Unknown,
            _ => panic!("Unknown type string"),
        }
    }
}

pub struct TypeInfo {
    pub actual_type: SimpleType,
    pub is_comptime: bool,
    pub inferred_type: SimpleType,
    pub declared_type: Option<SimpleType>,
}