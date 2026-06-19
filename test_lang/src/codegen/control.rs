use inkwell::{
    basic_block::BasicBlock,
    types::StringRadix::Decimal,
    values::{AnyValueEnum, IntValue},
};

use super::CodeGen;
use crate::ast::Node;

struct HandleCasesReturn<'a, 'ctx> {
    cases: Vec<(String, BasicBlock<'ctx>)>,
    default_block: &'a BasicBlock<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn handle_if(&mut self, condition: &Node, body: &Node) {
        let condition_value = self.handle_expression(condition);

        let current_function = self
            .ir
            .builder
            .get_insert_block()
            .unwrap()
            .get_parent()
            .unwrap();

        // Create basic blocks
        let then_block = self
            .ir
            .context
            .append_basic_block(current_function, "if_then");
        let after_block = self
            .ir
            .context
            .append_basic_block(current_function, "if_after");

        // Jump
        self.ir
            .builder
            .build_conditional_branch(condition_value.into_int_value(), then_block, after_block)
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to build conditional branch from {:?}, {}",
                    condition, e
                )
            });

        self.ir.builder.position_at_end(then_block);
        self.handle_statement(body);

        if !self.ir.at_terminator() {
            self.ir
                .builder
                .build_unconditional_branch(after_block)
                .unwrap();
        }
        self.ir.builder.position_at_end(after_block);
    }

    pub fn handle_if_else(&mut self, if_branch: &Node, else_branch: &Node) {
        let Node::IfStatement {
            condition,
            body: if_branch_body,
        } = if_branch
        else {
            unreachable!("Encountered if-else-statement with non if-statement first branch");
        };

        let condition_value = self.handle_expression(condition);

        let current_function = self
            .ir
            .builder
            .get_insert_block()
            .unwrap()
            .get_parent()
            .unwrap();

        // Create basic blocks
        let if_block = self
            .ir
            .context
            .append_basic_block(current_function, "if_branch_body");
        let else_block = self
            .ir
            .context
            .append_basic_block(current_function, "else_branch_body");
        let after_block = self
            .ir
            .context
            .append_basic_block(current_function, "after");

        // Jump
        self.ir
            .builder
            .build_conditional_branch(condition_value.into_int_value(), if_block, else_block)
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to build conditional branch from {:?}, {}",
                    condition, e
                )
            });

        self.ir.builder.position_at_end(if_block);
        self.handle_statement(if_branch_body);

        self.ir
            .builder
            .build_unconditional_branch(after_block)
            .unwrap();

        self.ir.builder.position_at_end(else_block);

        self.handle_statement(else_branch);

        if !self.ir.at_terminator() {
            self.ir
                .builder
                .build_unconditional_branch(after_block)
                .unwrap();
        }
        self.ir.builder.position_at_end(after_block);
    }

    pub fn handle_while(&mut self, condition: &Node, body: &Node) {
        let from_block = self.ir.builder.get_insert_block().unwrap();
        let current_function = from_block.get_parent().unwrap();

        let loop_header = self
            .ir
            .context
            .append_basic_block(current_function, "loop_header");
        let loop_body = self
            .ir
            .context
            .append_basic_block(current_function, "loop_body");
        let after_block = self
            .ir
            .context
            .append_basic_block(current_function, "loop_after");

        self.ir
            .builder
            .build_unconditional_branch(loop_header)
            .unwrap();

        self.ir.builder.position_at_end(loop_header);
        let condition_value = self.handle_expression(condition);
        self.ir
            .builder
            .build_conditional_branch(condition_value.into_int_value(), loop_body, after_block)
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to build conditional branch from {:?}, {}",
                    condition, e
                )
            });

        self.ir.builder.position_at_end(loop_body);
        self.handle_statement(body);
        self.ir
            .builder
            .build_unconditional_branch(loop_header)
            .unwrap();

        self.ir.builder.position_at_end(after_block);
    }

    pub fn handle_for(&self, _init: &Node, _condition: &Node, _step: &Node, _body: &Node) {
        todo!("For loops");
    }

    /** Returns true if the block all_cases_returned */
    pub fn handle_switch(&mut self, matched_value_expression: &Node, body: &Vec<Node>) {
        let matched_value = self.handle_expression(matched_value_expression);

        self.scopes.push_new_scope();

        let entry_block = self.ir.builder.get_insert_block().unwrap();
        let current_function = entry_block.get_parent().unwrap();

        // TODO: Check the type of the matched value

        // Create basic blocks
        let after_block = self
            .ir
            .context
            .append_basic_block(current_function, "after");

        let HandleCasesReturn {
            cases,
            default_block,
        } = self.handle_cases(body, &after_block);

        self.ir.builder.position_at_end(entry_block);

        let cases: Vec<(IntValue<'ctx>, BasicBlock<'ctx>)> = cases
            .iter()
            .map(|(case_string, case_block)| {
                (
                    self.ir
                        .context
                        .i64_type()
                        .const_int_from_string(&case_string, Decimal)
                        .unwrap(),
                    *case_block,
                )
            })
            .collect();

        let matched_value = matched_value.into_int_value();

        self.ir
            .builder
            .build_switch(matched_value, *default_block, &cases)
            .unwrap();

        self.ir.builder.position_at_end(after_block);
        self.scopes.pop_scope();
    }

    /** Returned default block will be the after_block if no default case was encountered */
    fn handle_cases<'a>(
        &mut self,
        cases_body: &Vec<Node>,
        after_block: &'a BasicBlock<'ctx>,
    ) -> HandleCasesReturn<'a, 'ctx> {
        let entry_block = self.ir.builder.get_insert_block().unwrap();
        let current_function = entry_block.get_parent().unwrap();
        let mut next_block = self.ir.context.append_basic_block(current_function, "case");
        let mut default_block = after_block;
        let mut cases = Vec::<(String, BasicBlock)>::new();

        for case in cases_body {
            let case_block = next_block;
            next_block = self.ir.context.append_basic_block(current_function, "case");

            match case {
                Node::Case {
                    value,
                    body: case_body,
                } => {
                    cases.push((value.clone(), case_block));
                    self.ir.builder.position_at_end(case_block);
                    self.handle_case(case_body, &next_block, &after_block);
                }
                Node::DefaultCase(_body) => todo!("Default case"),
                _ => unreachable!(
                    "Switch statement body contained something other than a Case or DefaultCase"
                ),
            };
        }

        // Slightly hacky way to implement fallthrough from cases
        next_block.replace_all_uses_with(after_block);
        unsafe {
            next_block.delete().unwrap();
        }

        HandleCasesReturn {
            cases,
            default_block,
        }
    }

    fn handle_case(&mut self, body: &Vec<Node>, next_block: &BasicBlock, after_block: &BasicBlock) {
        for statement in body {
            match statement {
                Node::BreakStatement => {
                    self.ir
                        .builder
                        .build_unconditional_branch(*after_block)
                        .unwrap();
                    return;
                }
                _ => self.handle_statement(&statement),
            }
        }

        self.ir
            .builder
            .build_unconditional_branch(*next_block)
            .unwrap();
    }
}
