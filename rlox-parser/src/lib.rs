extern crate rlox_contract;
extern crate log;
use log::{debug, error, trace};
use std::error::Error;
use std::fmt::Display;
use std::collections::VecDeque;
use rlox_contract::{Expr,ExprLiteralValue, TokenContext, Token, LiteralTokenType};
pub mod ast_printer;

pub type Result<B> = std::result::Result<B, ParseError>;

pub struct Parser {
    tokens: VecDeque<TokenContext>,
}

impl Parser {
    pub fn new() -> Parser {
        let tokens = VecDeque::new();
        Parser { tokens }
    }

    pub fn add_tokens(&mut self, tokens: Vec<TokenContext>) {
        self.tokens.extend(tokens);
    }

    pub fn parse(&mut self) -> Result<Vec<Expr>> {

        let mut stmts = Vec::new();
        let mut errors = Vec::new();
        while !self.eof() {
            match self.decl() {
                Ok(smt) => { stmts.push(smt); },
                Err(e) => {               
                    errors.push(e);                   
                }
            }
            
        }
        self.tokens.clear();
        if errors.len() != 0 {
            let mut msg = String::new();
            for e in errors.iter() {
                msg.push_str(&format!("{:?}\n", e.msg));
            }

            Err(ParseError::new(msg))
        } else {
            for s in stmts.iter() {
                debug!("{:?}", s);
            }
            Ok(stmts)
        }
    }

    fn decl(&mut self) -> Result<Expr> {
        let parse_result = if let Some(Token::Var) = self.peek().map(|e| e.token()) {
            self.var_decl()
        } else {
            self.stmt()
        };

        if let Err(e) = parse_result {
            error!("Parse Error {}", e.msg);
            self.synchronize()?;
            Err(e)
        } else {
            parse_result
        }
    }

    fn var_decl(&mut self) -> Result<Expr> {
        debug!("[ini] var_decl     {:?}", self.tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());
        self.consume(&Token::Var)?;
        if let Some(tc) = self.tokens.pop_front() {
            if let Token::Literal(LiteralTokenType::IdentifierLiteral(_)) = &tc.token() {
                
                if let Some(Token::Equal) = self.peek().map(|e| e.token()) {
                    self.consume(&Token::Equal)?;
                    let initializer = self.expression()?;
                    self.consume(&Token::Semicolon)?;
                    Ok(Expr::VarDecl { name: tc.token().clone(), initializer: Box::from(initializer) })    
                } else {
                    Ok(Expr::VarDecl { name: tc.token().clone(), initializer: Box::from(Expr::LiteralExpr(ExprLiteralValue::NilLiteral))})
                }

            } else {
                return Err(ParseError::new("Expected identifier"));
            }
        } else {
            return Err(ParseError::new("Unexpected EOF"));
        }
    }

    fn stmt(&mut self) -> Result<Expr> {
        debug!("[ini] stmt        {:?}", self.tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());
        if self.eof() {
            error!("Unexpected EOF, expected [stmt]");
            Err(ParseError::new(format!("Unexpected EOF, expected [stmt]"))) 
        } else if let Some(tc) = self.peek() {
            match tc.token() {
                Token::Print => self.print_stmt(),
                Token::LeftBrace => self.block(),
                _ => self.expression_stmt()
            }
        } else {
            error!("Unexpected EOF, expected [stmt]");
            Err(ParseError::new(format!("Unexpected EOF, expected [stmt]"))) 
        }
    }

    fn expression_stmt(&mut self) -> Result<Expr> {
        debug!("[ini] exprStmt    {:?}", self.tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());
        let e = self.expression()?;
        self.consume(&Token::Semicolon)?;
        Ok(e)
    }

    fn print_stmt(&mut self) -> Result<Expr> {
        debug!("[ini] printStmt   {:?}", self.tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());
        self.consume(&Token::Print)?;
        let e = self.expression()?;
        self.consume(&Token::Semicolon)?;
        Ok(Expr::PrintStmt(Box::from(e)))
    }

    fn block(&mut self) -> Result<Expr> {
        let mut es = Vec::new();
        self.consume(&Token::LeftBrace)?;
        while !self.eof() && self.peek().map(|e| e.token()) != Some(&Token::RightBrace) {
            let dec = self.decl()?;
            es.push(Box::from(dec));
        }

        self.consume(&Token::RightBrace)?;
        Ok(Expr::BlockStmt(es))
    }

    fn expression(&mut self) -> Result<Expr> {
        debug!("[ini] expression  {:?}", self.tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());
        if self.eof() { 
            error!("Unexpected EOF, expected [equality]");
            Err(ParseError::new(format!("Unexpected EOF, expected [equality]"))) 
        }
        else {
            let r = self.assignment();
            debug!("[ret] expression  {:?}", self.tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());
            r
        }
    }

    fn assignment(&mut self) -> Result<Expr> {
        debug!("[ini] assignment  {:?}", self.tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());
        let expr = self.equality()?;
        if let Some(Token::Equal) = self.peek().map(|e| e.token()) {
            self.consume(&Token::Equal)?;
            let value = self.assignment()?;

            if let Expr::VariableExpr(name) = expr {
                Ok(Expr::AssigmentExpr { name, value: Box::from(value)})
            } else {
                return Err(ParseError::new("Invalid assignment target"));
            }
        }  else {
            Ok(expr)
        }
        
    }

    fn equality(&mut self) -> Result<Expr> {
        debug!("[ini] equality    {:?}", self.tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());

        let mut l = self.comparison()?;
        debug!("[ret] equality    {:?}", self.tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());

        while self.token_match(&Token::BangEqual) || self.token_match(&Token::EqualEqual) {
            let o = self.tokens.pop_front().unwrap();
            let r = self.comparison()?;
            l = Expr::new_binary_expr(l, o.token().clone(), r);
        }

        Ok(l)
    }

    fn comparison(&mut self) -> Result<Expr> {
        debug!("[ini] comparison  {:?}", self.tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());

        let mut l = self.term()?;
        debug!("[ret] comparison  {:?}", self.tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());

        while self.token_match(&Token::Less) || self.token_match(&Token::LessEqual) || self.token_match(&Token::Greater) || self.token_match(&Token::GreaterEqual) {
            let o = self.tokens.pop_front().unwrap();
            let r = self.term()?;
            l = Expr::new_binary_expr(l, o.token().clone(), r);
        }
        Ok(l)
    }

    fn term(&mut self) -> Result<Expr> {
        debug!("[ini] term        {:?}", self.tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());
        let mut l = self.factor()?;
        debug!("[ret] term        {:?}", self.tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());

        while self.token_match(&Token::Minus) || self.token_match(&Token::Plus) {
            let o = self.tokens.pop_front().unwrap();
            let r = self.factor()?;
            l = Expr::new_binary_expr(l, o.token().clone(), r);
        }
        Ok(l)
    }

    fn factor(&mut self) -> Result<Expr> {
        debug!("[ini] factor      {:?}", self.tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());
        let mut l = self.unary()?;
        debug!("[ret] factor      {:?}", self.tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());
        while self.token_match(&Token::Star) || self.token_match(&Token::Slash) {
            let o = self.tokens.pop_front().unwrap();
            let r = self.unary()?;
            l = Expr::new_binary_expr(l, o.token().clone(), r);
        }
        Ok(l)
    }

    fn unary(&mut self) -> Result<Expr> {
        debug!("[ini] unary       {:?}", self.tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());

        if self.token_match(&Token::Bang) || self.token_match(&Token::Minus) {
            let o = self.tokens.pop_front().unwrap();
            let r = self.unary()?;
            debug!("[ret] unary       {:?}", self.tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());
            Ok(Expr::UnaryExpr { operator: o.token().clone(), right: Box::from(r) })
        } else {
            let p = self.primary()?;
            debug!("[ret] unary       {:?}", self.tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());
            Ok(p)
        }
    }

    fn primary(&mut self) -> Result<Expr> {
        debug!("[ini] primary     {:?}", &self.tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());
        if self.peek().is_none() {
            Err(ParseError::new("Unexpected EOF, expected [primary]"))
        } else {
            if let Some(e) = self.tokens.pop_front() {
                let res = match e.token() {
                    Token::Literal(LiteralTokenType::NumberLiteral(n)) => {
                        Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(*n))
                    },
                    Token::Literal(LiteralTokenType::StringLiteral(s)) => Expr::LiteralExpr(ExprLiteralValue::StringLiteral(s.to_string())),
                    Token::Literal(LiteralTokenType::IdentifierLiteral(s)) => Expr::VariableExpr(Token::Literal(LiteralTokenType::IdentifierLiteral(s.clone()))),
                    Token::Nil => Expr::LiteralExpr(ExprLiteralValue::NilLiteral),
                    Token::True => Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(true)),
                    Token::False => Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(false)),
                    Token::LeftParen => {
                        let ex = self.expression()?;
                        self.consume(&Token::RightParen)?;
                        Expr::GroupingExpr(Box::from(ex))
                    },
                    _ => {
                        let t = &e.token().clone();
                        self.tokens.push_front(e);
                        return Err(ParseError::new(format!("Unexpected {:?}, expected [primary]", t)));
                    }
                };
                debug!("[ret] primary     {:?}", &self.tokens.iter().map(|t| format!("{}", t)).collect::<Vec<String>>());
                Ok(res)
            } else {
                return Err(ParseError::new("Unexpected EOF, expected [primary]"));
            }
            

           
        }
    }

    fn peek(&self) -> Option<&TokenContext> {
        if self.tokens.len() == 0 {
            None
        } else {
            Some(&self.tokens[0])
        }
    }

    fn token_match(&self, token: &Token) -> bool {
        trace!("Searching for {:?} in {:?}", token, self.tokens.iter().map(|e| e.token()).collect::<Vec<&Token>>());
        if !self.eof() && self.peek().unwrap().token() == token {
            true
        } else {
            false
        }
    }

    fn eof(&self) -> bool {
        if let Some(&Token::Eof) = self.peek().map(|e| e.token()) {

            true
        } else { false }
    }

    fn consume(&mut self, token: &Token) -> Result<()> {
        if self.eof() {
            Err(ParseError::new(format!("Unexpected EOF, expected {:?}", token)))
        } else {
            match self.peek() {
                Some(t) => {
                    if *(*t).token() == *token { 
                        self.tokens.pop_front(); Ok(()) 
                    } else {
                        Err(ParseError::new(format!("Unexpected char {:?}, expected {:?}", t.token(), token)))
                    }
                },
                _ => {
                    panic!("UNKNOWN STATE - SHOULD BE CAUGHT BY EOF BEFORE THIS");
                }
            }
        }
    }

    fn synchronize(&mut self) -> Result<()> {
        debug!("Syncronizing parser");
        if self.eof() {
            return Ok(())
        }
        let mut previous = self.tokens.pop_front().unwrap();
        while !self.eof() {
            if previous.token() == &Token::Semicolon { return Ok(()); }
            match self.peek().map(|e| e.token()) {
                Some(t) => match t {
                    Token::Class|Token::Fun|Token::Var|Token::For|Token::If|Token::While|Token::Print|Token::Return => { 
                        debug!("Found synchroization target {:?}", t);
                        return Ok(())
                    },
                    _ => {
                        debug!("Skipping token {:?}", previous);
                        previous = self.tokens.pop_front().unwrap();
                    }
                },
                None => {return Ok(());}
            }
        }
        Ok(())
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
    use super::{Token, TokenContext, Parser};
    use super::ast_printer::print;

    #[test]
    fn test_parser_basic() {
        let ts = vec![
            TokenContext::new(Token::from_number(3.0), 1, 0, "3.0"),
            TokenContext::new(Token::BangEqual, 1, 4, "!="), 
            TokenContext::new(Token::from_string(r#""bye now""#), 1, 6, "\"bye now\""),
            TokenContext::new(Token::Semicolon, 1, 7, ";"),
            TokenContext::new(Token::Eof, 1, 11, "")
            ];
        let mut parser = Parser::new();
        parser.add_tokens(ts);
        let res = parser.parse().unwrap();

        let r = print(&res[0]);
        assert_eq!("3.00 BangEqual \"bye now\"",r);
    }

    #[test]
    fn test_parses_flat_series_of_terms_to_end() {
        let ts = vec![
            TokenContext::new(Token::from_number(3.0), 1, 0, "3.0"),
            TokenContext::new(Token::Plus, 1, 3, "+"),
            TokenContext::new(Token::from_number(10.4),1, 5, "10.4"),
            TokenContext::new(Token::Minus, 1, 9, "-"),
            TokenContext::new(Token::from_number(1.2), 1, 11, "1.2"),
            TokenContext::new(Token::Semicolon, 1, 7, ";"),
            TokenContext::new(Token::Eof, 1, 11, "")
        ];
        let mut parser = Parser::new();
        parser.add_tokens(ts);
        let res = parser.parse().unwrap();
        let r = print(&res[0]);
        
        assert_eq!("3.00 Plus 10.40 Minus 1.20", r);
    }

    #[test]
    fn test_parses_flat_series_of_factors_correctly() {
        let ts = vec![
            TokenContext::new(Token::from_number(3.0), 1, 0, "3.0"),
            TokenContext::new(Token::Star, 1, 3, "*"),
            TokenContext::new(Token::from_number(10.4),1, 5, "10.4"),
            TokenContext::new(Token::Slash, 1, 9, "/"),
            TokenContext::new(Token::from_number(1.2), 1, 11, "1.2"),
            TokenContext::new(Token::Semicolon, 1, 7, ";"),
            TokenContext::new(Token::Eof, 1, 11, "")
        ];
        let mut parser = Parser::new();
        parser.add_tokens(ts);
        let res = parser.parse().unwrap();
        let r = print(&res[0]);
        
        assert_eq!("3.00 Star 10.40 Slash 1.20", r);
    }

    #[test]
    fn test_parses_ending_right_paren_correctly() {
        let ts = vec![
            
            TokenContext::new(Token::from_number(3.0), 1, 0, "3.0"),
            TokenContext::new(Token::Star, 1, 3, "*"),
            TokenContext::new(Token::LeftParen, 1, 0 , "("),
            TokenContext::new(Token::from_number(10.4),1, 5, "10.4"),
            TokenContext::new(Token::Slash, 1, 9, "/"),
            TokenContext::new(Token::from_number(1.2), 1, 11, "1.2"),
            TokenContext::new(Token::RightParen, 1, 0 , ")"),
            TokenContext::new(Token::Semicolon, 1, 7, ";"),
            TokenContext::new(Token::Eof, 1, 11, "")
        ];
        let mut parser = Parser::new();
        parser.add_tokens(ts);
        let res = parser.parse().unwrap();
        let r = print(&res[0]);
    }
}
