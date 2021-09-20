use crate::rlox_core::shared_models::{Expr, ExprType, Token, TokenType};
use std::rc::Rc;
#[derive(Debug)]
pub enum ExecutionResult {
    Number(f64),
    String(String),
    Error(String),
    Bool(bool),
    Nil
}

pub fn run(expr: Expr) -> std::io::Result<()> {
    let e = std::rc::Rc::from(expr);
    let final_result = execute(e);
    println!("{:?}", final_result);
    Ok(())
}

fn execute(expr: std::rc::Rc<Expr>) -> ExecutionResult {
    match expr.clone().expr_type() {
        ExprType::LiteralExpr(_, tok) => match tok.token_type() {
            TokenType::Number(n) => ExecutionResult::Number(n.clone()),
            TokenType::String(s) => ExecutionResult::String(s.clone()),
            TokenType::Nil => ExecutionResult::Nil,
            e => ExecutionResult::Error(format!("UNEXPECTED LITERAL {:?}", e))
        },
        ExprType::BinaryExpr(l, o, r) => {
            let lval = execute(l.clone());
            let rval = execute(r.clone());
            let o_i = &*o.clone();
            println!("{:?} {:?} {:?}", lval,o, rval);
            match (lval, o_i, rval) {
                (ExecutionResult::Error(m1), _, ExecutionResult::Error(m2)) => ExecutionResult::Error(format!("{} {}", m1, m2)),
                (ExecutionResult::Error(m1), _, _) => ExecutionResult::Error(m1),
                (_, _, ExecutionResult::Error(m2)) => ExecutionResult::Error(m2),
                (ExecutionResult::String(s1), TokenType::EqualEqual, ExecutionResult::String(s2)) => if s1 == s2 { ExecutionResult::Bool(true) } else {ExecutionResult::Bool(false)},
                (ExecutionResult::String(s1), TokenType::BangEqual, ExecutionResult::String(s2)) => if s1 == s2 { ExecutionResult::Bool(false) } else {ExecutionResult::Bool(true)},

                _ => unimplemented!()
            }
        },
        ExprType::GroupingExpr(g) => {
            let gval = execute(g.clone());
            println!("{:?}", gval);
            gval
        },
        ExprType::UnaryExpr(o, e) => {
            let eval = execute(e.clone());
            println!("{:?} {:?}", e, o);
            match eval {
                ExecutionResult::Nil => ExecutionResult::Nil,
                 ExecutionResult::Number(n) => {
                     println!("{:?}", n);
                    unimplemented!()
                 },
                ExecutionResult::String(s) => ExecutionResult::String(s),
                ExecutionResult::Error(m) => ExecutionResult::Error(m),
                ExecutionResult::Bool(b) => ExecutionResult::Bool(b)
            }
        }
    }

}

