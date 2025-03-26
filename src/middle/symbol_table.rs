use std::collections::HashMap;
use super::ast::{Type, Ident};

pub struct SymbolTable {
    scopes: Vec<HashMap<Ident, Type>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn insert(&mut self, ident: Ident, ty: Type) -> Result<(), String> {
        if self.scopes.last_mut().unwrap().contains_key(&ident) {
            Err(format!("Identifier '{}' already declared in this scope", ident))
        } else {
            self.scopes.last_mut().unwrap().insert(ident, ty);
            Ok(())
        }
    }

    pub fn lookup(&self, ident: &Ident) -> Option<Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(ident) {
                return Some(ty.clone());
            }
        }
        None
    }
}