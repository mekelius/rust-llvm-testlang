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
    pub fn from_type_string(type_string: &str) -> Option<SimpleType> {
        match type_string {
            "Boolean" => Some(SimpleType::Boolean),
            "Bool" => Some(SimpleType::Boolean),
            "Int" => Some(SimpleType::Int),
            "Float" => Some(SimpleType::Float),
            "Char" => Some(SimpleType::Char),
            "Byte" => Some(SimpleType::Byte),
            "String" => Some(SimpleType::String),
            "Void" => Some(SimpleType::Void),
            "Unknown" => Some(SimpleType::Unknown),
            _ => None,
        }
    }
}

pub struct TypeInfo {
    pub actual_type: SimpleType,
    pub is_comptime: bool,
    pub inferred_type: SimpleType,
    pub declared_type: Option<SimpleType>,
}
