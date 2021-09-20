use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: i64
}

impl Token {
    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn line(&self) -> i64 {
        self.line
    }

    pub fn new(token_type: TokenType, lexeme: String, line: i64) -> Token {
        Token { token_type, lexeme, line }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    LeftParen, RightParen, LeftBrace, RightBrace, Comma, Dot, Minus, Plus, Semicolon, Star,
    Slash, Comment(String), WhiteSpace,
    Bang, BangEqual, Equal, EqualEqual, Greater, GreaterEqual, Less, LessEqual,

    Identifier(String), String(String), Number(f64),

    And, Class, Else, False, Fun, For, If, Nil, Or, Print, Return, Super, This,
    True, Var, While,

    Eof
}

#[derive(Debug,Clone)]
pub struct Expr {
    expr_type: ExprType,
    token: Rc<Token>
}

impl Expr {
    pub fn new_binary_expr(left_expr: Rc<Expr>, operator: Rc<Token>, right_expr: Rc<Expr>) -> Expr {
        let left = left_expr.clone();
        let right = right_expr.clone();
        let token = operator.clone();
        let tok_type = Rc::from(operator.token_type().clone());

        let expr_type = ExprType::BinaryExpr(left, tok_type, right);
        Expr { expr_type, token }
    }

    pub fn new_grouping_expr(grouped_expr: Rc<Expr>, ftoken: Rc<Token>) -> Expr {
        let expr_type = ExprType::GroupingExpr(grouped_expr.clone());

        let token = ftoken.clone();
        Expr { expr_type, token } 
    }

    pub fn new_literal_expr(ftoken: Rc<Token>) -> Expr {
        let tok_type = ftoken.token_type();
        let lit_type = match tok_type.clone() {
            TokenType::Identifier(_) => LiteralType::IdentifierLiteral,
            TokenType::String(_) => LiteralType::StringLiteral,
            TokenType::Number(_) => LiteralType::NumberLiteral,
            TokenType::Nil => LiteralType::NilLiteral,
            _ => panic!("BAD LITERAL EXPR {:?}", ftoken)
        };

        let token = ftoken.clone();
        let expr_type = ExprType::LiteralExpr(lit_type, token.clone());
        Expr { expr_type, token }
    }

    pub fn new_unary_expr(operator: Rc<Token>, operand: Rc<Expr>) -> Expr {
        let expr_type = ExprType::UnaryExpr(operator.clone(), operand.clone());
        let token = operator.clone();
        Expr { expr_type, token }
    }

    pub fn expr_type(&self) -> &ExprType {
        &self.expr_type
    }

    pub fn token(&self) -> &Token {
        &self.token
    }
}

#[derive(Debug,Clone)]
pub enum ExprType {
    BinaryExpr(Rc<Expr>, Rc<TokenType>, Rc<Expr>),
    GroupingExpr(Rc<Expr>),
    LiteralExpr(LiteralType, Rc<Token>),
    UnaryExpr(Rc<Token>, Rc<Expr>)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum LiteralType {
    StringLiteral,
    NumberLiteral,
    IdentifierLiteral,
    NilLiteral
}


#[derive(Debug, Clone)]
pub struct LexicalError {
    line: i64,
    error_lexeme: String,
    message: String
}

impl LexicalError {
    pub fn new(line:i64, error_lexeme: String, message: String) -> LexicalError {
        LexicalError { line, error_lexeme, message} 
    }

    pub fn line(&self) -> i64 {
        self.line
    }

    pub fn error_lexeme(&self) -> &str {
        &self.error_lexeme
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for LexicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "ERROR (line {}): {} at {}", self.line, self.message, self.error_lexeme)?;
        Ok(())
    }
}

impl std::error::Error for LexicalError { }

mod test {
    #[test]
    fn test_create_simple_expr() {
        let lit1 = super::Expr::new_literal_expr(super::Rc::from(super::Token::new(super::TokenType::String("hello".to_string()), "hello_there".to_string(), 1)));
        let lit2 = super::Expr::new_literal_expr(super::Rc::from(super::Token::new(super::TokenType::String("bye".to_string()), "bye_now".to_string(), 2)));

        let e = super::Expr::new_binary_expr(super::Rc::from(lit1), super::Rc::from(super::Token::new(super::TokenType::GreaterEqual, ">=".to_string(), 3)), super::Rc::from(lit2));
        let ms = match *e.expr_type() {
            super::ExprType::BinaryExpr(_,_,_) => true,
            _ => false
        };
        assert_eq!(ms, true);
        
    }
}