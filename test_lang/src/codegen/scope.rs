use inkwell::values::{BasicValueEnum, FunctionValue};

use crate::codegen::identifier::Symbol;
use std::{collections::HashMap, error::Error};
pub type ScopeID = usize;

#[derive(Debug)]
pub struct Scope<'ctx> {
    pub identifiers: HashMap<String, Symbol<'ctx>>,
    pub parent_id: Option<ScopeID>,
}

#[derive(Debug)]
pub struct Scopes<'ctx> {
    pub scopes: Vec<Scope<'ctx>>,
    pub current_scope_id: ScopeID,
}

impl<'ctx> Scope<'ctx> {
    pub fn new(parent_id: Option<ScopeID>) -> Self {
        Scope {
            identifiers: HashMap::new(),
            parent_id,
        }
    }

    pub fn define_param(
        &mut self,
        identifier: &str,
        value: BasicValueEnum<'ctx>,
    ) -> Result<(), Box<dyn Error>> {
        if self.identifiers.contains_key(identifier) {
            return Err(format!(
                "Attempt to redefine identifier {} as a formal parameter",
                identifier
            )
            .into());
        }
        self.identifiers
            .insert(identifier.to_string(), Symbol::Formal(value));

        Ok(())
    }
}

impl<'ctx> Scopes<'ctx> {
    pub fn new() -> Self {
        Scopes {
            scopes: vec![Scope::new(None)],
            current_scope_id: 0,
        }
    }

    pub fn get_global_scope(&self) -> &Scope<'ctx> {
        self.scopes.get(0).expect("global scope should exist")
    }

    pub fn get_scope(&self, scope_id: &ScopeID) -> &Scope<'ctx> {
        self.scopes
            .get(*scope_id)
            .expect("called should be sure that scope exists")
    }

    pub fn get_current_scope(&self) -> &Scope<'ctx> {
        self.scopes
            .get(self.current_scope_id)
            .expect("there should always be a current scope")
    }

    pub fn get_global_scope_mut(&mut self) -> &mut Scope<'ctx> {
        self.scopes.get_mut(0).expect("global scope should exist")
    }

    pub fn get_scope_mut(&mut self, scope_id: &ScopeID) -> &mut Scope<'ctx> {
        self.scopes
            .get_mut(*scope_id)
            .expect("called should be sure that scope exists")
    }

    pub fn get_current_scope_mut(&mut self) -> &mut Scope<'ctx> {
        self.scopes
            .get_mut(self.current_scope_id)
            .expect("there should always be a current scope")
    }

    pub fn push_new_scope(&mut self) -> &mut Scope<'ctx> {
        self.scopes.push(Scope::new(Some(self.current_scope_id)));
        self.current_scope_id = self.scopes.len() - 1;

        self.get_current_scope_mut()
    }

    pub fn pop_scope(&mut self) {
        self.current_scope_id = self
            .get_current_scope_mut()
            .parent_id
            .expect("Tried to pop_scope the global scope");
    }

    pub fn resolve_identifier_in_scope(
        &self,
        identifier: &str,
        scope_id: &ScopeID,
    ) -> Option<&Symbol<'ctx>> {
        let scope = self.get_scope(scope_id);
        scope.identifiers.get(identifier).or_else(|| {
            let parent_id = scope.parent_id?;
            self.resolve_identifier_in_scope(identifier, &parent_id)
        })
    }

    /* Resolves an identifier in the current scope */
    pub fn resolve_identifier(&self, identifier: &str) -> Option<&Symbol<'ctx>> {
        self.resolve_identifier_in_scope(identifier, &self.current_scope_id)
    }

    pub fn resolve_function(&self, identifier: &str) -> Option<FunctionValue<'ctx>> {
        match self.resolve_identifier(identifier) {
            Some(Symbol::Function(function)) => Some(function.clone()),
            _ => None,
        }
    }

    /* Defines an identifier in the current scope */
    pub fn define_identifier(&mut self, identifier: &str, rvalue: Symbol<'ctx>) {
        self.get_current_scope_mut()
            .identifiers
            .insert(identifier.to_string(), rvalue);
    }

    /* Defines an identifier in the current scope */
    pub fn define_global_identifier(&mut self, identifier: &str, rvalue: Symbol<'ctx>) {
        self.get_global_scope_mut()
            .identifiers
            .insert(identifier.to_string(), rvalue);
    }

    pub fn define_global_function(&mut self, identifier: &str, function: FunctionValue<'ctx>) {
        self.get_global_scope_mut()
            .identifiers
            .insert(identifier.to_string(), Symbol::Function(function));
    }
}
