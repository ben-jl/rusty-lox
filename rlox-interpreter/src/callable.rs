use super::{Expr, ExprLiteralValue};
pub trait Callable {
    fn arity(&self) -> super::Result<usize>;
    fn call(&self, global_scope: &super::ScopeEnvironment, args: Vec<Expr>) -> super::Result<Expr>;
}

impl Callable for Expr {

    fn arity(&self) -> super::Result<usize> { 
        match self {
            Expr::FunctionExpr { name: _, params, body: _} => Ok(params.len()),
            _ => Err(super::InterpreterError::new("expected callable expression"))
        }
     }
    fn call(&self, global_scope: &super::ScopeEnvironment, args: std::vec::Vec<Expr>) -> std::result::Result<Expr, super::InterpreterError> { 
        if let Expr::FunctionExpr {name: _, params, body } = &self {
            
            let arity = self.arity()?;
            if arity != args.len() {
                Err(super::InterpreterError::new("Arity didn't match arg length"))
            } else {

                let mut fun_scope = global_scope.clone();

                for (i,a) in args.iter().enumerate() {
                    
                    if let super::Token::Literal(super::LiteralTokenType::IdentifierLiteral(s)) = &params[i] {
                        fun_scope.declare(&s, Box::from(a.clone()))?;
                    } else {
                        return Err(super::InterpreterError::new("Invalid param type, must be identifier"));
                    }
                }
                
                let r = fun_scope.clone();
                let mut i = super::Interpreter::with_env(r);
                let e = match i.interpret(body.clone()) {
                    Ok(ev) => ev,
                    Err(super::InterpreterError { msg:_, returned}) => if returned.is_some() { returned.unwrap() } else { Expr::LiteralExpr(ExprLiteralValue::NilLiteral) }
                };
                Ok(e)
            }
            
            
         
        } else {
            Err(super::InterpreterError::new(format!("Tried to call an uncallable expression {:?}", &self)))
        }
     }
}