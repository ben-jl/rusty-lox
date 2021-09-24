use super::{Expr, ExprLiteralValue, ComputedValue};

trait Callable {
    fn arity(&self) -> super::Result<usize>;
    fn call(&self, interpreter: &mut super::Interpreter, args: Vec<ComputedValue>) -> super::Result<ComputedValue>;
}

impl Callable for Expr {

    fn arity(&self) -> super::Result<usize> { 
        match self {
            Expr::FunctionExpr { name: _, params, body: _} => Ok(params.len()),
            _ => Err(super::InterpreterError::new("expected callable expression"))
        }
     }
    fn call(&self, i: &mut super::Interpreter, args: std::vec::Vec<ComputedValue>) -> std::result::Result<ComputedValue, super::InterpreterError> { 
        match self {
            Expr::FunctionExpr { name, params, body } => {
                let globals = i.environment.clone().get_global_env();
                let mut fn_env = super::Environment::new_environment(Box::from(globals.clone()));
                for (i, p) in params.iter().enumerate() {
                    let a = match args.get(i) {
                        Some(ab) => ab,
                        _ => {
                            return Err(super::InterpreterError::new("args length and param length mismatch"));
                        }
                    };

                    if let super::Token::Literal(super::LiteralTokenType::IdentifierLiteral(s)) = p {
                        fn_env.put(&s, a.clone());
                    } else {
                        return Err(super::InterpreterError::new("expected literal as function param"));
                    }
                }

                let r = i.interpret_with_env(body.clone(), fn_env.clone())?;
                Ok(r)
            },
            _ => Err(super::InterpreterError::new("expected callable expression"))
        }
     }
}