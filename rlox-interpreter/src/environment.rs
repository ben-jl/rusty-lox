use std::collections::VecDeque;
use crate::InterpreterError;
use rlox_contract::Expr;
use super::{Token};
use std::collections::HashMap;
use log::{debug,trace,error};



#[derive(Debug,Clone)]
pub struct ScopeEnvironment {
    scopes: Vec<Scope>,
    current_idx: usize,
    previous_idx: usize,
}

impl ScopeEnvironment {
    pub fn new_root() -> ScopeEnvironment {
        ScopeEnvironment { scopes: vec![Scope::new(None)], current_idx: 0, previous_idx: 0}
    }

    pub fn new_child(&mut self) -> () {
        self.scopes.push(Scope::new(Some(self.scopes.len()-1)));
        self.previous_idx = self.current_idx;
        self.current_idx = self.scopes.len()-1;
        ()
    }

    pub fn pop_scope(&mut self) -> super::Result<()> {
        let cs = &self.scopes[self.current_idx];
        if let Some(px) = cs.parent_idx {
            let nxt_ix = px.clone();
            for i in self.scopes.iter_mut() {
                let v = match i {
                    Scope { parent_idx: Some(pi), variable_context: _} => pi >= &mut self.current_idx,
                    _ => false
                };
                if v {
                    i.parent_idx = Some(i.parent_idx.unwrap() - 1);
                }
            }
            self.scopes.remove(self.current_idx);
            self.previous_idx = self.current_idx;
            self.current_idx = nxt_ix;
            Ok(())
        } else {
            Err(InterpreterError::new("Tried to pop scope but had no parent index"))
        }
    }

    pub fn get(&self, identifier: &str) -> Option<&Expr> {

        let mut cx = Some(self.current_idx);
        let mut found_expr = None;
        while cx.is_some() {
            let v = cx.unwrap();
            let s = &self.scopes[v];
            if s.variable_context.contains_key(identifier) {
                found_expr = Some(s.variable_context.get(identifier).unwrap().as_ref());
                break;
            } else {
                cx = s.parent_idx;
            }

        }
        found_expr
    }

    pub fn declare(&mut self, identifier: &str, value: Box<Expr>) -> super::Result<()> {
        if let Some(_) = &self.scopes[self.current_idx].variable_context.get(identifier) {
            Err(InterpreterError::new(format!("Variable {} already declared", identifier)))
        } else {
            let s = &mut self.scopes[self.current_idx];
            s.declare(identifier, value)?;
            Ok(())
        }
    }

    pub fn assign(&mut self, identifier: &str, value: Box<Expr>) -> super::Result<()> {
        if let Some(_) = self.get(identifier) {
            let mut cx = Some(self.current_idx);
            while cx.is_some() {
                let v = cx.unwrap();
                let s = &self.scopes[v];
                if s.variable_context.contains_key(identifier) {
                    break;
                } else {
                    cx = s.parent_idx;
                }
            }
            let scope_to_update = cx.unwrap();
            self.scopes[scope_to_update].assign(identifier, value)?;
            Ok(())
        } else {
            Err(InterpreterError::new(format!("Variable {} assigned but never declared", identifier)))
        }
    }

    pub fn set_to_root(&mut self) -> () {
        self.previous_idx = self.current_idx;
        self.current_idx = 0;
        ()
    }

    pub fn set_to_previous(&mut self) -> () {
        let tmp = self.previous_idx;
        self.previous_idx = self.current_idx;
        self.current_idx = tmp;
        ()
    }
}

#[derive(Debug,Clone)]
pub struct Scope {
    variable_context: HashMap<String,Box<Expr>>,
    parent_idx: Option<usize>
}

impl Scope {
    fn new(parent_idx: Option<usize>) -> Scope {
        Scope {variable_context: HashMap::new(), parent_idx} 
    }



    fn declare(&mut self, identifier: &str, value: Box<Expr>) -> super::Result<()> {

        if self.variable_context.contains_key(identifier) {
            Err(InterpreterError::new(format!("Variable {} already defined", identifier)))
        } else {
            self.variable_context.insert(identifier.to_string(), value);
            Ok(())
        }
    }

    fn assign(&mut self, identifier: &str, value: Box<Expr>) -> super::Result<()> {
        if !self.variable_context.contains_key(identifier) {
            Err(InterpreterError::new(format!("Variable {} assigned but never declared", identifier)))
        } else {
            self.variable_context.insert(identifier.to_string(), value);
            Ok(())
        }
    }
}

impl std::fmt::Display for Scope {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        writeln!(f,"Scope~{} [", self.parent_idx.map(|e| format!("{}", e)).unwrap_or("Root".to_string()))?;
        for i in &self.variable_context {
            writeln!(f, "    ({}, {})", i.0, i.1)?;
        }
        write!(f, "]")?;
        Ok(())
     }
}

impl std::fmt::Display for ScopeEnvironment {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        writeln!(f, "ScopeEnv@{} [", &self.current_idx)?;
        for c in &self.scopes {
            writeln!(f, "{}", c)?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use crate::ExprLiteralValue;
    use super::*;

    #[test]
    fn it_puts_and_returns_variable_on_single_scope() {
        let mut s = ScopeEnvironment::new_root();
        s.declare("test", Box::from(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(1.1)))).expect("failed to declare");
        let r = s.get("test");
        assert_eq!(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(1.1)), *r.unwrap());
    }

    #[test]
    fn it_puts_on_root_and_gets_from_child() {
        let mut s = ScopeEnvironment::new_root();
        s.declare("test", Box::from(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(1.1)))).expect("failed to declare");
        s.new_child();
        let r = s.get("test");
        assert_eq!(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(1.1)), *r.unwrap());
    }
}