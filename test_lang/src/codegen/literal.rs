use inkwell::{
    types::StringRadix::Decimal,
    values::{AnyValue, AnyValueEnum},
};
use regex::Regex;

use crate::codegen::CodegenError;

use super::CodeGen;

/** Unescapes \n, \\ and \r in strings */
fn unescape(value: &str) -> String {
    let re_n = Regex::new(r"\\n").expect("regex should be valid");
    let re_r = Regex::new(r"\\r").expect("regex should be valid");
    let re_bs = Regex::new(r"\\\\").expect("regex should be valid");

    let value = re_n.replace_all(value, "\n");
    let value = re_r.replace_all(&value, "\r");
    let value = re_bs.replace_all(&value, "\\");
    value.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unescapes_newline() {
        let escaped = unescape("\\n");
        assert_eq!(escaped, "\n");
    }

    #[test]
    fn unescapes_cr() {
        let escaped = unescape("\\n");
        assert_eq!(escaped, "\n");
    }

    #[test]
    fn unescapes_backslash() {
        let escaped = unescape("\\\\");
        assert_eq!(escaped, "\\");
    }
}

impl<'ctx> CodeGen<'ctx> {
    pub fn handle_number_literal(&self, value: &str) -> Result<AnyValueEnum<'ctx>, CodegenError> {
        Ok(self
            .ir
            .context
            .i32_type()
            .const_int_from_string(value, Decimal)
            .ok_or_else(|| format!("could not create integer from {}", value))?
            .as_any_value_enum())
    }

    pub fn handle_string_literal(&self, value: &str) -> Result<AnyValueEnum<'ctx>, CodegenError> {
        let unescaped_value: String = unescape(value);

        Ok(self
            .ir
            .builder
            .build_global_string_ptr(&*&unescaped_value, "string_literal")?
            .as_any_value_enum())
    }

    pub fn handle_boolean_literal(&self, value: &bool) -> Result<AnyValueEnum<'ctx>, CodegenError> {
        Ok(self
            .ir
            .context
            .bool_type()
            .const_int(*value as u64, false)
            .as_any_value_enum())
    }
}
