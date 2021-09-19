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