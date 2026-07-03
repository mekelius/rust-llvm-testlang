// use inkwell::values::{AnyValue, AnyValueEnum, IntValue};

// use crate::{
//     ast::{Expression, UnaryOperator, UnopExpression},
//     codegen::CodeGen,
// };

// impl<'ctx> CodeGen<'ctx> {
//     pub fn handle_unop(&self, UnopExpression { op, term }: &UnopExpression) -> AnyValueEnum<'ctx> {
//         match op {
//             UnaryOperator::UnaryMinus => self.handle_unary_minus(term),
//             UnaryOperator::UnaryNot => self.handle_unary_not(term),
//         }
//     }

//     pub fn handle_unary_minus(&self, rhs: &Expression) -> AnyValueEnum<'ctx> {
//         self.negate_int(self.handle_expression(rhs).into_int_value())
//     }

//     pub fn negate_int(&self, value: IntValue<'ctx>) -> AnyValueEnum<'ctx> {
//         self.ir
//             .builder
//             .build_int_nsw_sub(self.ir.context.i32_type().const_int(0, false), value, "")
//             .unwrap()
//             .as_any_value_enum()
//     }

//     pub fn handle_unary_not(&self, rhs: &Expression) -> AnyValueEnum<'ctx> {
//         self.ir
//             .builder
//             .build_not(self.handle_expression(rhs).into_int_value(), "")
//             .unwrap()
//             .as_any_value_enum()
//     }
// }
