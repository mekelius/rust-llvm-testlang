use chumsky::container::Seq;
use inkwell::{
    types::StringRadix::Decimal,
    values::{AnyValue, AnyValueEnum},
};

use super::CodeGen;

/** Unescapes \n, \\ and \r in strings */
fn unescape(value: &str) -> String {
    let mut escaping = false;
    let mut out = "".to_string();

    for char_ in value.seq_iter() {
        if escaping {
            match char_ {
                '\\' => out.push('\\'),
                'n' => out.push('\n'),
                'r' => out.push('\r'),
                _ => panic!("Unsupported character escape \\{} in string literal", char_),
            }
            escaping = false;
            continue;
        }

        if char_ == '\\' {
            escaping = true;
            continue;
        }

        out.push(char_);
    }

    out
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn unescapes_newline() {
        let escaped = unescape("\\n");
        assert_eq!(escaped, "\n");
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
