use crate::ast_visitor::ASTVisitor;

pub struct SymbolTable {

}

// impl ASTVisitor<()> for SymbolTable {
//     fn visit_program(&mut self, _program: &crate::span::SourceIDSpanned<crate::ast::Program>) -> Option<()> {
//         None
//     }

//     fn visit_function(&mut self, function: &crate::span::SourceIDSpanned<crate::ast::Function>) -> Option<()> {
        
//     }

//     fn visit_statement(&mut self, statement: &crate::span::SourceIDSpanned<crate::ast::Statement>) -> Option<()> {
        
//     }

//     fn visit_expression(&mut self, _expression: &crate::span::SourceIDSpanned<crate::ast::Expression>) -> Option<()> {
//         None        
//     }

//     fn visit_case(&mut self, _case: &crate::span::SourceIDSpanned<crate::ast::Case>) -> Option<()> {
//         None
//     }
// }