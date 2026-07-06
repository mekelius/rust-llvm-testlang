// use crate::ast_visitor::ASTVisitorMut;

// pub struct TypeError {
//     message: String
// }

// pub struct SimpleTypeChecker {

// }

// impl ASTVisitorMut<TypeError> for SimpleTypeChecker {
//     fn visit_function(&mut self, function: &mut crate::span::SourceIDSpanned<crate::ast::Function>) -> Option<TypeError> {
        
//     }
    
//     fn visit_program(&mut self, program: &mut crate::span::SourceIDSpanned<crate::ast::Program>) -> Option<TypeError> {
//         None
//     }

//     fn visit_statement(&mut self, statement: &mut crate::span::SourceIDSpanned<crate::ast::Statement>) -> Option<TypeError> {
//         todo!("statement");
//     }
    
//     fn visit_expression(&mut self, expression: &mut crate::span::SourceIDSpanned<crate::ast::Expression>) -> Option<TypeError> {
        
//     }
    
//     fn visit_case(&mut self, case: &mut crate::span::SourceIDSpanned<crate::ast::Case>) -> Option<TypeError> {
//         None
//     }
// }