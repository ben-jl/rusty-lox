use crate::rlox_core::shared_models::{Expr, ExprType, TokenType, Token};
use std::rc::Rc;

pub fn parse_expr(tokens: &[Token]) -> Result<Expr, ParserError> {
    let (r, e) = expression(tokens)?;
    Ok(r)
}

fn expression(tokens: &[Token]) -> Result<(Expr, usize), ParserError> {
    equality(tokens)
    
}

fn equality(tokens: &[Token]) -> Result<(Expr, usize), ParserError> {
    let (mut comp_expr, consumed) = comparison(tokens)?;
    let mut cidx = consumed;
    while tokens.len() > cidx && (*tokens[cidx].token_type() == TokenType::BangEqual || *tokens[cidx].token_type() == TokenType::EqualEqual) {
        let operator = Rc::from(tokens[cidx].clone());
        let (rexpr, nxtconsumed) = comparison(&tokens[cidx+1..])?;
        cidx += nxtconsumed;
        comp_expr = Expr::new_binary_expr(Rc::from(comp_expr), operator, Rc::from(rexpr));
    }

    Ok((comp_expr, cidx))
}

fn comparison(tokens: &[Token]) -> Result<(Expr, usize), ParserError> {
    let (mut ce, consumed) = term(tokens)?;
    let mut should_loop = tokens.len() > consumed && match *tokens[consumed].token_type() {
        TokenType::GreaterEqual|TokenType::Greater|TokenType::Less|TokenType::LessEqual => true,
        _ => false
    };
    if !should_loop {
        Ok((ce, consumed))
    } else {
        let mut cidx = consumed+1;
        while tokens.len() > cidx && should_loop {
            let (t, nxtconsumed) = term(&tokens[cidx..])?;
            let o = Rc::from(tokens[cidx].clone());
            cidx += nxtconsumed;
            ce = Expr::new_binary_expr(Rc::from(ce), o, Rc::from(t));

            should_loop = match *tokens[cidx+1].token_type() {
                TokenType::GreaterEqual|TokenType::Greater|TokenType::Less|TokenType::LessEqual => {cidx+=1; true},
            _ => false
            };
        }
        Ok((ce, cidx))
    }
    
}

fn term(tokens: &[Token]) -> Result<(Expr, usize), ParserError> {
    let (mut lfactor, consumed) = factor(tokens)?;

    let mut cidx = consumed;
    while tokens.len() > cidx && (*tokens[cidx].token_type() == TokenType::Plus || *tokens[cidx].token_type() == TokenType::Minus) {
        let (rfactor, nxtconsumed) = factor(&tokens[cidx+1..])?;
        let o = Rc::from(tokens[cidx].clone());
        cidx += nxtconsumed + 1;
        lfactor = Expr::new_binary_expr(Rc::from(lfactor), o, Rc::from(rfactor));
    }
    
    Ok((lfactor, cidx))
}

fn factor(tokens: &[Token]) -> Result<(Expr, usize), ParserError> {
    let (mut lu, consumed) = unary(tokens)?;

    let mut cidx = consumed;
    while tokens.len() > cidx && (*tokens[cidx].token_type() == TokenType::Slash || *tokens[cidx].token_type() == TokenType::Star) {
        let (ru, nxtconsumed) = unary(&tokens[cidx+1..])?;
        let o = Rc::from(tokens[cidx].clone());
        cidx += nxtconsumed;
        lu = Expr::new_binary_expr(Rc::from(lu), o, Rc::from(ru));
    }

    Ok((lu, cidx))
}

fn unary(tokens: &[Token]) -> Result<(Expr, usize), ParserError> {
    
    if *tokens[0].token_type() == TokenType::Bang || *tokens[0].token_type() == TokenType::Minus {
        unary(&tokens[1..])
    } else {
        let (r, c) = primary(&tokens[..])?;
        Ok((r, c))
    }
}

fn primary(tokens: &[Token]) -> Result<(Expr, usize), ParserError> {
    match *tokens[0].token_type() {
        TokenType::Number(_) | TokenType::String(_) | TokenType::True | TokenType::False | TokenType::Nil => Ok((Expr::new_literal_expr(Rc::from(tokens[0].clone())), 1)),
        TokenType::LeftParen => {
            let (interior, consumed) = expression(&tokens[1..])?;
            if *tokens[consumed+1].token_type() == TokenType::RightParen {
                Ok((Expr::new_grouping_expr(Rc::from(interior), Rc::from(tokens[consumed].clone())), consumed))
            } else {
                Err(ParserError { message: "Expected close paren".to_string() })
            }
            
        },
        _ => Err(ParserError { message: "Expected primary token".to_string() })
    }
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

mod test {

    #[test]
    fn it_parses_simple_binary_token_expr_correctly() {
        let toks = vec![super::Token::new(super::TokenType::Number(1.2), "1.2".to_string(), 1), super::Token::new(super::TokenType::EqualEqual, "==".to_string(), 1), super::Token::new(super::TokenType::Number(2.1), "2.1".to_string(), 1), super::Token::new(super::TokenType::Eof, "".to_string(), 1)];

        let pres = super::parse_expr(&toks).unwrap();
        println!("{:?}", pres);
        assert_eq!(false,true);
    }
}