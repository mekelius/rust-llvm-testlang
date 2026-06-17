use inkwell::{
    types::StringRadix::Decimal,
    values::{AnyValue, AnyValueEnum},
};

use super::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub fn handle_number_literal(&self, value: &str) -> AnyValueEnum<'ctx> {
        self.ir
            .context
            .i64_type()
            .const_int_from_string(value, Decimal)
            .unwrap_or_else(|| panic!("Could not create integer from {}", value))
            .as_any_value_enum()
    }

    pub fn handle_string_literal(&self, value: &str) -> AnyValueEnum<'ctx> {
        self.ir
            .builder
            .build_global_string_ptr(value, "string_literal")
            .unwrap_or_else(|_| panic!("Creating global string from {} failed", value))
            .as_any_value_enum()
    }

    pub fn handle_boolean_literal(&self, value: &bool) -> AnyValueEnum<'ctx> {
        self.ir
            .context
            .bool_type()
            .const_int(*value as u64, false)
            .as_any_value_enum()
    }
}
