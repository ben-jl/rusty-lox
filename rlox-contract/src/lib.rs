use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralTokenType {
    NumberLiteral(f64),
    IdentifierLiteral(String),
    StringLiteral(String)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LeftParen, RightParen, LeftBrace, RightBrace, Comma, Dot, Minus, Plus, Semicolon, Star,
    Slash, 
    Bang, BangEqual, Equal, EqualEqual, Greater, GreaterEqual, Less, LessEqual,

    Literal(LiteralTokenType),

    And, Class, Else, False, Fun, For, If, Nil, Or, Print, Return, Super, This,
    True, Var, While,

    Eof
}

#[derive(Debug,PartialEq,Clone)]
pub struct TokenContext {
    token: Token,
    line_number: usize,
    start_char_offset: usize,
    lexeme: String
}

impl TokenContext {
    pub fn new<B>(token: Token, line_number: usize, start_char_offset: usize, lexeme: B) -> TokenContext where B : ToString {
        TokenContext { token, line_number, start_char_offset, lexeme: lexeme.to_string() }
    }

    pub fn length(&self) -> usize {
        self.lexeme.len()
    }

    pub fn token(&self) -> &Token {
        &self.token
    }
}

impl Display for TokenContext {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        write!(f, "({} {}:{})", self.token(), self.line_number, self.start_char_offset)
     }
}

impl Token {
    pub fn from_number(n : f64) -> Token {
        Token::Literal(LiteralTokenType::NumberLiteral(n))
    }

    pub fn from_identifier<B>(symbols: B ) -> Token where B : ToString {
        Token::Literal(LiteralTokenType::IdentifierLiteral(symbols.to_string()))
    }

    pub fn from_string<B>(string: B) -> Token where B : ToString {
        Token::Literal(LiteralTokenType::StringLiteral(string.to_string()))
    }

}

impl Display for Token {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        match self {
            Token::Literal(lt) => match lt {
                LiteralTokenType::NumberLiteral(n) => write!(f, "Number {}", n),
                LiteralTokenType::StringLiteral(s) => write!(f, "String {}", s),
                LiteralTokenType::IdentifierLiteral(i) => write!(f, "Ident {}", i)
            },
            t => write!(f, "{:?}", t)
        }?;
        Ok(())
     }
}

#[derive(Debug,Clone)]
pub enum Expr {
    BinaryExpr { left: Box<Expr>, operator: Token, right: Box<Expr> },
    GroupingExpr(Box<Expr>),
    LiteralExpr(ExprLiteralValue),
    UnaryExpr { operator: Token, right: Box<Expr> },
    PrintStmt(Box<Expr>),
    ExprStmt(Box<Expr>),
    VarDecl {name: Token, initializer: Box<Expr>  },
    VariableExpr(Token),
    AssigmentExpr { name: Token, value: Box<Expr> },
    BlockStmt(Vec<Box<Expr>>),
    IfStmt { condition: Box<Expr>, then_branch: Box<Expr>, else_branch: Box<Expr>},
    LogicalExpr { left: Box<Expr>, operator: Token, right: Box<Expr>},
    WhileLoop { condition: Box<Expr>, body: Box<Expr> }
}

#[derive(Debug,Clone)]
pub enum ExprLiteralValue {
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
    NilLiteral
}



impl Expr {
    pub fn new_binary_expr(left: Expr, operator: Token, right: Expr) -> Expr {
        let l = Box::from(left);
        let r = Box::from(right);

        let o = operator.clone();
        Expr::BinaryExpr { left: l, operator: o, right: r}
    }

    pub fn new_logical_expr(left: Expr, operator: Token, right: Expr) -> Expr {
        let l = Box::from(left);
        let r = Box::from(right);

        let o = operator.clone();
        Expr::LogicalExpr { left: l, operator: o, right: r}
    }
}