use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TokenType {
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, This, True, Var, While,
    Dot, Print, Return, Super,
    Minus, For, If, Nil, Or, 
    Plus, And, Class, Else, False, Fun,
    Semicolon, Identifier, String, Number,
    Slash, Greater, GreaterEqual, Less, LessEqual,
    Star, Bang, BangEqual, Equal, EqualEqual,
    Eof
}

#[derive(Debug)]
pub struct Token {
    token_type : TokenType,
    lexeme : String,
    line: i64
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: i64) -> Token {
        Token { token_type, lexeme, line }
    }
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>
}

impl Scanner {
    pub fn from_raw_source_string(source: String) -> Scanner {
        let tokens = Vec::new();
        Scanner { source, tokens } 
    }

    pub fn scan_tokens(&mut self) -> std::io::Result<Box<&[Token]>> {
        let mut start:i64 = 0;
        let mut current:i64 = 0;
        let mut line:i64 = 1;

        while !self.is_at_end(current) {
            start = current;
            let tok_type = self.scan_token(start, &mut current)?;
            
            self.tokens.push(Token::new(tok_type, self.source[start as usize..(current) as usize].to_string(), line));
            
        }

        self.tokens.push(Token::new(TokenType::Eof, "".to_string(), line));

        Ok(Box::from(self.tokens.as_slice()))
    }

    fn is_at_end(&self,  current: i64) -> bool {
        current >= self.source.len() as i64
    }

    fn scan_token(&self,start: i64, current: &mut i64) -> std::io::Result<TokenType> {
        *current += 1;
        let tok = match &self.source[start as usize..*current as usize] {
            
            
                    "(" => TokenType::LeftParen,
                    ")" => TokenType::RightParen,
                    "}" => TokenType::RightBrace,
                    "{" => TokenType::LeftBrace,
                    "," => TokenType::Comma,
                    "." => TokenType::Dot,
                    "-" => TokenType::Minus,
                    "+" => TokenType::Plus,
                    ";" => TokenType::Semicolon,
                    "*" => TokenType::Star,
                    _ => panic!("{:?}, {}, {}", &self.source[start as usize..*current as usize], start, current)
            
            
            
            
        };
        Ok(tok)
    }
}



impl Display for Token {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        write!(f, "{:<10} {:<15} {}", self.line, self.token_type, self.lexeme);
        Ok(())
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{:?}", self);
        Ok(())
    }
}