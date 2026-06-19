use inkwell::{
    types::StringRadix::Decimal,
    values::{AnyValue, AnyValueEnum},
};
use regex::Regex;

use super::CodeGen;

/** Unescapes \n, \\ and \r in strings */
fn unescape(value: &str) -> String {
    let re_n = Regex::new(r"\\n").unwrap();
    let re_r = Regex::new(r"\\r").unwrap();
    let re_bs = Regex::new(r"\\\\").unwrap();
    
    let value = re_n.replace_all(value, "\n");
    let value = re_r.replace_all(&value, "\r");
    let value = re_bs.replace_all(&value, "\\");
    value.to_string()
}

#[cfg(test)]
mod tests{
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
    pub fn handle_number_literal(&self, value: &str) -> AnyValueEnum<'ctx> {
        self.ir
            .context
            .i64_type()
            .const_int_from_string(value, Decimal)
            .unwrap_or_else(|| panic!("Could not create integer from {}", value))
            .as_any_value_enum()
    }

    pub fn handle_string_literal(&self, value: &str) -> AnyValueEnum<'ctx> {
        let unescaped_value: String = unescape(value);

        self.ir
            .builder
            .build_global_string_ptr(&*&unescaped_value, "string_literal")
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
