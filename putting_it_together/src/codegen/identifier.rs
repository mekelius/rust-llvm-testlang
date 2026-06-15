use dict::DictIface;
use inkwell::values::FunctionValue;

use super::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub fn resolve_function(&self, identifier: &str) -> Option<&FunctionValue<'_>> {
        self.builtins
            .get(identifier)
            .or_else(|| self.function_identifiers.get(identifier))
    }
}
