use std::collections::VecDeque;
use crate::InterpreterError;
use rlox_contract::Expr;
use super::{Token};
use std::collections::HashMap;
use log::{debug,trace,error};

#[derive(Debug,Clone)]
pub struct Scope {
    environment_stack: VecDeque<HashMap<String, Box<Expr>>>
}

impl Scope {
    pub fn new() -> Scope {
        Scope { environment_stack: VecDeque::from(vec![HashMap::new()]) }
    }

    pub fn create_child(&mut self) -> () {
        self.environment_stack.push_front(HashMap::new())
        
    }

    pub fn declare(&mut self, identifier: &str, expr: Box<Expr>) -> super::Result<&mut Scope> {
        if self.environment_stack[0].contains_key(identifier) {
            Err(InterpreterError::new("Variable already declared"))
        } else {
            self.environment_stack[0].insert(identifier.to_string(), expr);
            Ok(self)
        }
    }

    pub fn get(&self, identifier: &str) -> Option<Box<Expr>> {
        for hm in &self.environment_stack {
            if hm.contains_key(identifier) {
                return hm.get(identifier).map(|e| e.clone());
            }
        }
        None
    }

    pub fn assign(&mut self, identifier: &str, value: Box<Expr>) -> super::Result<()> {
        for hm in self.environment_stack.iter_mut() {
            if hm.contains_key(identifier) {
                hm.insert(identifier.to_string(), value);
                return Ok(())
            }
        }
        Err(InterpreterError::new(format!("{:?} not declared", identifier)))
    }

    pub fn pop_scope(&mut self) -> super::Result<()> {
        self.environment_stack.pop_front();
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::ExprLiteralValue;
    use super::*;

    #[test]
    fn it_puts_and_returns_variable_on_single_scope() {
        let mut s = Scope::new();
        s.declare("test", Box::from(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(1.1)))).expect("failed to declare");
        let r = s.get("test");
        assert_eq!(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(1.1)), *r.unwrap());
    }

    #[test]
    fn it_puts_on_root_and_gets_from_child() {
        let mut s = Scope::new();
        s.declare("test", Box::from(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(1.1)))).expect("failed to declare");
        s.create_child();
        let r = s.get("test");
        assert_eq!(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(1.1)), *r.unwrap());
    }
}