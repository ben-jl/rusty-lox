use super::{Expr, ExprLiteralValue};
use super::environment::{ScopeEnvironment, Scope};
pub trait Callable {
    fn arity(&self) -> super::Result<usize>;
    fn call(&self, global_scope: &mut super::ScopeEnvironment, args: Vec<Expr>) -> super::Result<Expr>;
}

impl Callable for Expr {

    fn arity(&self) -> super::Result<usize> { 
        match self {
            Expr::FunctionExpr { name: _, params, body: _} => Ok(params.len()),
            _ => Err(super::InterpreterError::new("expected callable expression"))
        }
     }
    fn call(&self, global_scope: &mut super::ScopeEnvironment, args: std::vec::Vec<Expr>) -> std::result::Result<Expr, super::InterpreterError> { 
        if let Expr::FunctionExpr {name: _, params, body } = &self {
            let arity = self.arity()?;
            dbg!(&params, &body);
            if arity != args.len() {
                Err(super::InterpreterError::new("Arity didn't match arg length"))
            } else {
                global_scope.new_child();
                for (i,a) in args.iter().enumerate() {
                    dbg!(&i, &a);
                    if let super::Token::Literal(super::LiteralTokenType::IdentifierLiteral(s)) = &params[i] {
                        global_scope.declare(&s, Box::from(a.clone()))?;
                    } else {
                        return Err(super::InterpreterError::new("Invalid param type, must be identifier"));
                    }
                }
                dbg!(&global_scope);
                let mut i = super::Interpreter::with_env(global_scope.clone());
                let e = i.interpret(body.clone())?; 
                dbg!(&e);
                Ok(e)
            }
            
            
         
        } else {
            Err(super::InterpreterError::new(format!("Tried to call an uncallable expression {:?}", &self)))
        }
     }
}