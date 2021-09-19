use crate::rlox_core::shared_models::{Expr, ExprType, TokenType, Token};
use std::rc::Rc;

pub fn parse_expr(tokens: &[Token]) -> Result<Expr, ParserError> {
    for t in tokens {

    }
    unimplemented!();
}

fn expression(tokens: &[Token]) -> Result<(Expr, usize), ParserError> {
    equality(tokens)
    
}

fn equality(tokens: &[Token]) -> Result<(Expr, usize), ParserError> {
    let (mut comp_expr, consumed) = comparison(tokens)?;

    let mut cidx = consumed;
    while *tokens[cidx].token_type() == TokenType::BangEqual || *tokens[cidx].token_type() == TokenType::EqualEqual {
        let operator = Rc::from(tokens[cidx].clone());
        let (rexpr, nxtconsumed) = comparison(&tokens[cidx+1..])?;
        cidx = nxtconsumed;
        comp_expr = Expr::new_binary_expr(Rc::from(comp_expr), operator, Rc::from(rexpr));
    }

    Ok((comp_expr, cidx))
}

fn comparison(tokens: &[Token]) -> Result<(Expr, usize), ParserError> {
    let (mut ce, consumed) = term(tokens)?;

    let mut should_loop = match *tokens[consumed].token_type() {
        TokenType::GreaterEqual|TokenType::Greater|TokenType::Less|TokenType::LessEqual => true,
        _ => false
    };
    if !should_loop {
        Err(ParserError { message: "not a comparison".to_string()})
    } else {
        let mut cidx = consumed;
        while should_loop {
            let (t, nxtconsumed) = term(&tokens[cidx+1..])?;
            let o = Rc::from(tokens[cidx].clone());
            cidx = nxtconsumed;
            ce = Expr::new_binary_expr(Rc::from(ce), o, Rc::from(t));

            should_loop = match *tokens[cidx].token_type() {
                TokenType::GreaterEqual|TokenType::Greater|TokenType::Less|TokenType::LessEqual => true,
            _ => false
            };
        }
        Ok((ce, cidx))
    }
    
}

fn term(tokens: &[Token]) -> Result<(Expr, usize), ParserError> {
    unimplemented!();
}

fn factor(tokens: &[Token]) -> Result<(Expr, usize), ParserError> {
    unimplemented!();
}

fn unary(tokens: &[Token]) -> Result<(Expr, usize), ParserError> {
    unimplemented!();
}

fn primary(tokens: &[Token]) -> Result<(Expr, usize), ParserError> {
    unimplemented!();
}


#[derive(Debug)]
pub struct ParserError {
    message: String
}

impl std::error::Error for ParserError {}
impl std::fmt::Display for ParserError {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        write!(f, "ERROR {}", self.message)?;
        Ok(())
     }
}