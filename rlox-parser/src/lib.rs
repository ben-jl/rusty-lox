extern crate rlox_contract;
extern crate log;
use log::{debug, error};
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
        debug!("expression {:?}", tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());
        let r = equality(tokens)?;
        if tokens.len() >= 1 && tokens[0].token() != &Token::Eof {
            error!("Unable to match tokens {:?}", tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());
            Err(ParseError::new("Unexpected tokens at end of source"))
        } else {
            Ok(r)
        }
    } else {
        error!("Unexpected end of file");
        Err(ParseError::new("Unexpected end of file"))
    }
}

fn equality(tokens: &mut VecDeque<TokenContext>) -> Result<Expr> {
    let mut l = comparison(tokens)?;
    debug!("equality   {:?}", tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());
    while let Some(t) = tokens.pop_front() {
        match t.token() {
            Token::BangEqual | Token::EqualEqual => {
                let o = t.token();
                let e = comparison(tokens)?;
                l = Expr::new_binary_expr(l, o.clone(), e);
            }, 
            _ => {
                tokens.push_front(t);
                break;
            }
        }
    }
    Ok(l)
}

fn comparison(tokens: &mut VecDeque<TokenContext>) -> Result<Expr> {
    let mut l = term(tokens)?;
    debug!("comparison {:?}", tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());
    while let Some(t) = tokens.pop_front() {
        match t.token() {
            Token::Greater | Token::GreaterEqual | Token::Less | Token::LessEqual => {
                let o = t.token();
                let e = term(tokens)?;
                l = Expr::new_binary_expr(l, o.clone(), e);
            },
            _ => {
                tokens.push_front(t);
                break;
            }
        }
    } 
    Ok(l)
}

fn term(tokens: &mut VecDeque<TokenContext>) -> Result<Expr> {
    let mut l = factor(tokens)?;
    debug!("term       {:?}", tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());
    while let Some(t) = tokens.pop_front() {
        match t.token() {
            Token::Minus | Token::Plus => {
                let o = t.token();
                let r = factor(tokens)?;
                l = Expr::new_binary_expr(l, o.clone(), r);
            },
            _ => {
                tokens.push_front(t);
                break;
            }
        }
    } 
    Ok(l)
}

fn factor(tokens: &mut VecDeque<TokenContext>) -> Result<Expr> {
    let mut l = unary(tokens)?;
    debug!("factor     {:?}", tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());
    while let Some(t) = tokens.pop_front() {
        match t.token() {
            Token::Slash | Token::Star => {
                let o = t.token();
                let r = unary(tokens)?;
                l = Expr::new_binary_expr(l, o.clone(), r)
            },
            _ => {
                tokens.push_front(t);
                break;
            }
        }
    } 
    Ok(l)
}

fn unary(tokens: &mut VecDeque<TokenContext>) -> Result<Expr> {
    debug!("unary      {:?}", tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());
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
        error!("Unexpected end of file, expected unary");
        Err(ParseError::new("Unexpected end of file"))
    }
}

fn primary(tokens: &mut VecDeque<TokenContext>) -> Result<Expr> {
    debug!("primary    {:?}", tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());
    
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
                        error!("Expected right paren, found {}", t.token());
                        Err(ParseError::new("expected right paren"))

                    }
                } else {
                    error!("Expected right paren, found {}", t.token());
                    Err(ParseError::new("expected right paren"))
                }
            },
            _ => {
                error!("Expected right paren, found {}", t.token());
                Err(ParseError::new("Unexpected char"))
            }
        }
    } else {
        error!("Unexpected end of file, expected primary");
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

    #[test]
    fn test_parses_flat_series_of_terms_to_end() {
        let ts = vec![
            TokenContext::new(Token::from_number(3.0), 1, 0, "3.0"),
            TokenContext::new(Token::Plus, 1, 3, "+"),
            TokenContext::new(Token::from_number(10.4),1, 5, "10.4"),
            TokenContext::new(Token::Minus, 1, 9, "-"),
            TokenContext::new(Token::from_number(1.2), 1, 11, "1.2")
        ];

        let res = parse(ts).unwrap();
        let r = print(&res);
        
        assert_eq!("3.00 Plus 10.40 Minus 1.20", r);
    }

    #[test]
    fn test_parses_flat_series_of_factors_correctly() {
        let ts = vec![
            TokenContext::new(Token::from_number(3.0), 1, 0, "3.0"),
            TokenContext::new(Token::Star, 1, 3, "*"),
            TokenContext::new(Token::from_number(10.4),1, 5, "10.4"),
            TokenContext::new(Token::Slash, 1, 9, "/"),
            TokenContext::new(Token::from_number(1.2), 1, 11, "1.2")
        ];

        let res = parse(ts).unwrap();
        let r = print(&res);
        
        assert_eq!("3.00 Star 10.40 Slash 1.20", r);
    }
}
