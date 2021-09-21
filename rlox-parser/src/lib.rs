extern crate rlox_contract;
use std::error::Error;
use std::fmt::Display;
use std::collections::VecDeque;
use rlox_contract::{Expr,ExprLiteralValue, TokenContext, Token, LiteralTokenType};
pub mod ast_printer;

pub type Result<B> = std::result::Result<B, ParseError>;
pub fn parse(tokens: Vec<TokenContext>) -> Result<Expr> {
    let mut ts = VecDeque::from(tokens);
    expression(&mut ts)

}

fn expression(tokens: &mut VecDeque<TokenContext>) -> Result<Expr> {
    if tokens.len() != 0 {
 
        println!("expression {:?}", tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());
        equality(tokens)
    } else {
        Err(ParseError::new("Unexpected end of file"))
    }
}

fn equality(tokens: &mut VecDeque<TokenContext>) -> Result<Expr> {
    let l = comparison(tokens)?;
    println!("equality   {:?}", tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());
    if tokens.len() > 0 && (tokens[0].token() == &Token::BangEqual || tokens[0].token() == &Token::EqualEqual) {
        let operator = tokens.pop_front().unwrap();
        if tokens.len() > 0 {
             let r = comparison(tokens)?;
             Ok(Expr::new_binary_expr(l, operator.token().clone(), r))
        }else {
            Err(ParseError::new("Unexpected end of file"))
        }
    } else {
        Ok(l)
    }

    
}

fn comparison(tokens: &mut VecDeque<TokenContext>) -> Result<Expr> {
    let l = term(tokens)?;
    println!("comparison {:?}", tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());
    if let Some(t) = tokens.pop_front() {
        match t.token() {
            Token::Greater | Token::GreaterEqual | Token::Less | Token::LessEqual => {
                let o = t.token();
                let e = term(tokens)?;
                Ok(Expr::new_binary_expr(l, o.clone(), e))
            },
            _ => {
                tokens.push_front(t);
                Ok(l)
            }
        }
    } else {
        Ok(l)
    }
}

fn term(tokens: &mut VecDeque<TokenContext>) -> Result<Expr> {
    let l = factor(tokens)?;
    println!("term       {:?}", tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());
    if let Some(t) = tokens.pop_front() {
        match t.token() {
            Token::Minus | Token::Plus => {
                let o = t.token();
                let r = factor(tokens)?;
                Ok(Expr::new_binary_expr(l, o.clone(), r))
            },
            _ => {
                tokens.push_front(t);
                Ok(l)
            }
        }
    } else {
        Ok(l)
    }
}

fn factor(tokens: &mut VecDeque<TokenContext>) -> Result<Expr> {
    let l = unary(tokens)?;
    println!("factor     {:?}", tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());
    if let Some(t) = tokens.pop_front() {
        match t.token() {
            Token::Slash | Token::Star => {
                let o = t.token();
                let r = unary(tokens)?;
                Ok(Expr::new_binary_expr(l, o.clone(), r))
            },
            _ => {
                tokens.push_front(t);
                Ok(l)
            }
        }
    } else {
        Ok(l)
    }
}

fn unary(tokens: &mut VecDeque<TokenContext>) -> Result<Expr> {
    println!("unary      {:?}", tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());
    if let Some(t) = tokens.pop_front() {
        match t.token() {
            Token::Bang | Token::Minus => {
                let o = t;
                let right = unary(tokens)?;
                Ok(Expr::UnaryExpr { operator: o.token().clone(), right: Box::new(right)})
            },
            _ => { tokens.push_front(t); primary(tokens) }
        }
    } else {
        Err(ParseError::new("Unexpected end of file"))
    }
}

fn primary(tokens: &mut VecDeque<TokenContext>) -> Result<Expr> {
    println!("primary    {:?}", tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());
    
    if let Some(t) = tokens.pop_front() {
        match t.token() {
            Token::Literal(LiteralTokenType::NumberLiteral(n)) => Ok(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(*n))),
            Token::Literal(LiteralTokenType::StringLiteral(s)) => Ok(Expr::LiteralExpr(ExprLiteralValue::StringLiteral(s.clone()))),
            Token::True => Ok(Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(true))),
            Token::False => Ok(Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(false))),
            Token::Nil => Ok(Expr::LiteralExpr(ExprLiteralValue::NilLiteral)),
            Token::LeftParen => {
                let inner = expression(tokens)?;
                if let Some(t) = tokens.pop_front() {
                    if t.token() == &Token::RightParen {
                        Ok(Expr::GroupingExpr(Box::new(inner)))
                    } else {
                        Err(ParseError::new("expected right paren"))

                    }
                } else {
                    Err(ParseError::new("expected right paren"))
                }
            },
            _ => Err(ParseError::new("Unexpected char"))
        }
    } else {
        Err(ParseError::new("Unexpected end of file"))
    }
}

#[derive(Debug)]
pub struct ParseError {
    msg: String
}

impl ParseError {
    pub fn new<B : ToString>(msg:B) -> ParseError {
        ParseError { msg: msg.to_string()}
    }
}
impl Error for ParseError {}
impl Display for ParseError {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        write!(f, "{}", self.msg)?;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::{Token, TokenContext, parse};
    use super::ast_printer::print;

    #[test]
    fn test_parser_basic() {
    let ts = vec![
        TokenContext::new(Token::from_number(3.0), 1, 0, "3.0"),
        TokenContext::new(Token::BangEqual, 1, 4, "!="), 
        TokenContext::new(Token::from_string("\"bye now\""), 1, 6, "\"bye now\"")
        ];
    
    let res = parse(ts).unwrap();

    let r = print(&res);
    assert_eq!("3.00 BangEqual \"bye now\"",r);
}
}
