use inkwell::values::{AnyValueEnum, FunctionValue};

use crate::{ast_store::ASTStore, codegen::scope::ScopeID, types::SimpleType};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum NodeCodegenStatus {
    NotStarted,
    InProgress,
    Done,
}

impl Default for NodeCodegenStatus {
    fn default() -> Self {
        Self::NotStarted
    }
}

#[derive(Default, Debug, Clone)]
pub struct FunctionData<'ctx> {
    pub status: NodeCodegenStatus,
    pub ir_value: Option<FunctionValue<'ctx>>,
    pub return_type: SimpleType,
    pub scope_id: Option<ScopeID>,
}

#[derive(Default, Debug, Clone)]
pub struct ExpressionData<'ctx> {
    pub status: NodeCodegenStatus,
    pub type_: SimpleType,
    pub ir_value: Option<AnyValueEnum<'ctx>>,
}

#[derive(Default, Debug, Clone)]
pub struct StatementData {
    pub scope_id: Option<ScopeID>,
}

#[derive(Debug)]
pub struct NodeDataStore<'ctx> {
    pub functions: Vec<FunctionData<'ctx>>,
    pub expressions: Vec<ExpressionData<'ctx>>,
    pub statements: Vec<StatementData>,
}

impl<'ctx> NodeDataStore<'ctx> {
    pub fn new(ast_store: &ASTStore) -> Self {
        Self {
            functions: ast_store
                .functions
                .iter()
                .map(|_| FunctionData::default())
                .collect(),

            statements: ast_store
                .statements
                .iter()
                .map(|_| StatementData::default())
                .collect(),

            expressions: ast_store
                .expressions
                .iter()
                .map(|_| ExpressionData::default())
                .collect(),
        }
    }
}
