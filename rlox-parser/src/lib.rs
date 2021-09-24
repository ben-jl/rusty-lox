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
    stack_depth: i32
}

impl Parser {
    pub fn new() -> Parser {
        let tokens = VecDeque::new();
        Parser { tokens, stack_depth: 0 }
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
    //        for s in stmts.iter() {
//                // debug!("{:?}", s);
  //          }
            Ok(stmts)
        }
    }

    fn decl(&mut self) -> Result<Expr> {
        self.add_stack("decl", 1);
        let parse_result = match self.peek().map(|e| e.token()) {
            Some(Token::Var) => self.var_decl(),
            Some(Token::Fun) => self.function(),
            _ => self.stmt()
        };

        self.add_stack("decl", -1);
        if let Err(e) = parse_result {
            error!("Parse Error {}", e.msg);
            self.synchronize()?;
            Err(e)
        } else {
            parse_result
        }
    }

    fn function(&mut self) -> Result<Expr> {
        self.consume(&Token::Fun)?;
        let fn_name = match &self.peek().map(|e| e.token()) {
            Some(Token::Literal(LiteralTokenType::IdentifierLiteral(s))) => Token::Literal(LiteralTokenType::IdentifierLiteral(s.clone())),
            _ => return Err(ParseError::new("Expected identifier"))
        };
        self.consume(&Token::LeftParen)?;
        let mut params = Vec::new();
        loop {
            if  Some(&Token::RightParen) != self.peek().map(|e| e.token()) {
                self.consume(&Token::RightParen)?;
                let _ = match &self.peek().map(|e| e.token()) {
                    Some(Token::Literal(LiteralTokenType::IdentifierLiteral(s))) => {
                        params.push(Token::Literal(LiteralTokenType::IdentifierLiteral(s.clone())));
                    },
                    _ => return Err(ParseError::new("Expected identifier"))
                };

                if !self.token_match(&Token::Comma) {
                    break;
                } else {
                    self.consume(&Token::Comma)?;
                }
            }
        }
        self.consume(&Token::RightParen)?;
        let body = self.block()?;
        Ok(Expr::FunctionExpr { name: fn_name, params, body: Box::from(body)})
        
    }

    fn var_decl(&mut self) -> Result<Expr> {
        self.add_stack("var_decl", 1);
        self.consume(&Token::Var)?;
        if let Some(tc) = self.tokens.pop_front() {
            if let Token::Literal(LiteralTokenType::IdentifierLiteral(_)) = &tc.token() {
                
                if let Some(Token::Equal) = self.peek().map(|e| e.token()) {
                    self.consume(&Token::Equal)?;
                    let initializer = self.expression()?;
                    self.consume(&Token::Semicolon)?;
                    self.add_stack("var_decl", -1);
                    Ok(Expr::VarDecl { name: tc.token().clone(), initializer: Box::from(initializer) })    
                } else {
                    self.consume(&Token::Semicolon)?;
                    self.add_stack("var_decl", -1);
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
        self.add_stack("stmt", 1);
        if self.eof() {
            error!("Unexpected EOF, expected [stmt]");
            Err(ParseError::new(format!("Unexpected EOF, expected [stmt]"))) 
        } else if let Some(tc) = self.peek() {
            let r = match tc.token() {
                Token::Print => self.print_stmt(),
                Token::LeftBrace => self.block(),
                Token::If => self.if_stmt(),
                Token::While => self.while_loop(),
                Token::For => self.for_loop(),
                _ => self.expression_stmt()
            }?;
            self.add_stack("stmt", -1);
            Ok(r)
        } else {
            error!("Unexpected EOF, expected [stmt]");
            Err(ParseError::new(format!("Unexpected EOF, expected [stmt]"))) 
        }
    }

    fn for_loop(&mut self) -> Result<Expr> {
        self.add_stack("for_loop", 1);
        self.consume(&Token::For)?;
        self.consume(&Token::LeftParen)?;
        let initializer = match self.peek().map(|e| e.token()) {
            Some(Token::Semicolon) => {
                self.consume(&Token::Semicolon)?;
                Expr::LiteralExpr(ExprLiteralValue::NilLiteral)
            },
            Some(Token::Var) => {
                let ini = self.var_decl()?;
                ini
            },
            _ => {
                let ini = self.expression_stmt()?;
                ini
            }
        };

        let mut condition = match self.peek().map(|e| e.token()) {
            Some(Token::Semicolon) => {
                self.consume(&Token::Semicolon)?;
                Expr::LiteralExpr(ExprLiteralValue::NilLiteral)
            },
            _ => {

                let e = self.expression_stmt()?;
                e
            }
        };


        let increment = match self.peek().map(|e| e.token()) {
            Some(Token::RightParen) => Expr::LiteralExpr(ExprLiteralValue::NilLiteral),
            _ => {
                let e = self.expression()?;
                e
            }
        };
        self.consume(&Token::RightParen)?;
        let mut body = self.stmt()?;

        match &increment {
            Expr::LiteralExpr(ExprLiteralValue::NilLiteral) => (),
            inc => {
                body = Expr::BlockStmt(vec![Box::from(body), Box::from(inc.clone())]);
                ()
            }
        };

        match condition {
            Expr::LiteralExpr(ExprLiteralValue::NilLiteral) => {
                condition = Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(true));
                ()
            },
            _ => {
                ()
            }
        };
        body = Expr::WhileLoop { condition: Box::from(condition), body: Box::from(body) };
        match initializer {
            Expr::LiteralExpr(ExprLiteralValue::NilLiteral) => (),
            _ => {
                body = Expr::BlockStmt(vec![Box::from(initializer), Box::from(body)]);
                ()
            }
        };
        self.add_stack("for_loop", -1);
        Ok(body)
    }

    fn while_loop(&mut self) -> Result<Expr> {
        self.add_stack("while", 1);
        self.consume(&Token::While)?;
        self.consume(&Token::LeftParen)?;
        let cond = self.expression()?;
        self.consume(&Token::RightParen)?;
        let body = self.stmt()?;
        self.add_stack("while", -1);
        Ok(Expr::WhileLoop { condition: Box::from(cond), body: Box::from(body) })
    }

    fn if_stmt(&mut self) -> Result<Expr> {
        self.add_stack("if", 1);
        self.consume(&Token::If)?;
        self.consume(&Token::LeftParen)?;
        let condition = self.expression()?;
        self.consume(&Token::RightParen)?;
        let then_branch = self.stmt()?;
        if let Some(Token::Else) = self.peek().map(|e| e.token()) {
            self.consume(&Token::Else)?;
            let else_branch = self.stmt()?;
            self.add_stack("if", -1);
            Ok(Expr::IfStmt { condition: Box::from(condition), then_branch: Box::from(then_branch), else_branch: Box::from(else_branch)})
        } else {
            let else_branch = Expr::LiteralExpr(ExprLiteralValue::NilLiteral);
            self.add_stack("if", -1);
            Ok(Expr::IfStmt { condition: Box::from(condition), then_branch: Box::from(then_branch), else_branch: Box::from(else_branch)})

        }
    }

    fn expression_stmt(&mut self) -> Result<Expr> {
        self.add_stack("expr_stmt", 1);
        let e = self.expression()?;
        self.consume(&Token::Semicolon)?;
        self.add_stack("expr_stmt", -1);
        Ok(e)
    }

    fn print_stmt(&mut self) -> Result<Expr> {
        self.add_stack("print", 1);
        self.consume(&Token::Print)?;
        let e = self.expression()?;
        self.consume(&Token::Semicolon)?;
        self.add_stack("print", -1);
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
        self.add_stack("expr", 1);
        if self.eof() { 
            error!("Unexpected EOF, expected [equality]");
            Err(ParseError::new(format!("Unexpected EOF, expected [equality]"))) 
        }
        else {
            let r = self.assignment();
            self.add_stack("expr", -1);
            r
        }
    }

    fn assignment(&mut self) -> Result<Expr> {
        self.add_stack("assign", 1);
        let expr = self.logic_or()?;
        if let Some(Token::Equal) = self.peek().map(|e| e.token()) {
            self.consume(&Token::Equal)?;
            let value = self.assignment()?;

            if let Expr::VariableExpr(name) = expr {
                self.add_stack("assign", -1);
                Ok(Expr::AssigmentExpr { name, value: Box::from(value)})
            } else {
                return Err(ParseError::new("Invalid assignment target"));
            }
        }  else {
            self.add_stack("assign", -1);
            Ok(expr)
        }
        
    }

    fn logic_or(&mut self) -> Result<Expr> {
        self.add_stack("or", 1);
        let mut left = self.logic_and()?;
        while let Some(Token::Or) = self.peek().map(|e| e.token()) {
            self.consume(&Token::Or)?;
            let right = self.logic_and()?;
            left = Expr::new_logical_expr(left, Token::Or, right);
        }
        self.add_stack("or", -1);
        Ok(left)
    }

    fn logic_and(&mut self) -> Result<Expr> {
        self.add_stack("and", -1);
        let mut left = self.equality()?;
        while let Some(Token::And) = self.peek().map(|e| e.token()) {
            self.consume(&Token::And)?;
            let right = self.equality()?;
            left = Expr::new_logical_expr(left, Token::And, right);
        }
        self.add_stack("and", -1);
        Ok(left)
    }

    fn equality(&mut self) -> Result<Expr> {
        self.add_stack("equal", 1);
        let mut l = self.comparison()?;

        while self.token_match(&Token::BangEqual) || self.token_match(&Token::EqualEqual) {
            let o = self.tokens.pop_front().unwrap();
            let r = self.comparison()?;
            l = Expr::new_binary_expr(l, o.token().clone(), r);
        }
        self.add_stack("equal", -1);
        Ok(l)
    }

    fn comparison(&mut self) -> Result<Expr> {
        self.add_stack("compare", 1);
        let mut l = self.term()?;

        while self.token_match(&Token::Less) || self.token_match(&Token::LessEqual) || self.token_match(&Token::Greater) || self.token_match(&Token::GreaterEqual) {
            let o = self.tokens.pop_front().unwrap();
            let r = self.term()?;
            l = Expr::new_binary_expr(l, o.token().clone(), r);
        }
        self.add_stack("compare", -1);
        Ok(l)
    }

    fn term(&mut self) -> Result<Expr> {
        self.add_stack("term", 1);
        let mut l = self.factor()?;

        while self.token_match(&Token::Minus) || self.token_match(&Token::Plus) {
            let o = self.tokens.pop_front().unwrap();
            let r = self.factor()?;
            l = Expr::new_binary_expr(l, o.token().clone(), r);
        }
        self.add_stack("term", -1);
        Ok(l)
    }

    fn factor(&mut self) -> Result<Expr> {
        self.add_stack("factor", 1);
        let mut l = self.unary()?;
        while self.token_match(&Token::Star) || self.token_match(&Token::Slash) {
            let o = self.tokens.pop_front().unwrap();
            let r = self.unary()?;
            l = Expr::new_binary_expr(l, o.token().clone(), r);
        }
        self.add_stack("factor", -1);
        Ok(l)
    }

    fn unary(&mut self) -> Result<Expr> {

        self.add_stack("unary", 1);
        if self.token_match(&Token::Bang) || self.token_match(&Token::Minus) {
            let o = self.tokens.pop_front().unwrap();
            let r = self.unary()?;
            self.add_stack("unary", -1);
            Ok(Expr::UnaryExpr { operator: o.token().clone(), right: Box::from(r) })
        } else {
            let p = self.call()?;
            self.add_stack("unary", -1);
            Ok(p)
        }
    }
    fn call(&mut self) -> Result<Expr> {
        let mut expr = self.primary()?;

        while let Some(Token::LeftParen) = self.peek().map(|e| e.token()) {
            self.consume(&Token::LeftParen)?;
            expr = self.finish_call(expr)?;
        }
        Ok(expr)
    }

    fn finish_call(&mut self, expr: Expr) -> Result<Expr> {
        let mut args = Vec::new();
        if Some(&Token::RightParen) != self.peek().map(|e| e.token()) {
            loop {
                
                let next_arg = self.expression()?;
                args.push(next_arg);
                if let Some(Token::Comma) = self.peek().map(|e| e.token()) {
                    self.consume(&Token::Comma)?;

                } else {
                    break;
                }
            }
        } 
        self.consume(&Token::RightParen)?;
        Ok(Expr::CallExpr { callee: Box::from(expr), paren: Token::RightParen, arguments: args.iter().map(|e| Box::from(e.clone())).collect() })
        
    }

    fn primary(&mut self) -> Result<Expr> {
        self.add_stack("primary", 1);
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
                self.add_stack("primary", -1);
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

    fn add_stack(&mut self, method: &str, direction: i32) -> () {
        self.stack_depth += 1 * direction;

        let mut pad = String::new();
        for _ in 0..self.stack_depth {
            pad += "--";
        }
        trace!("{} {} {:?}", pad, method,&self.peek())
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
