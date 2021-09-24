extern crate rlox_contract;
use std::fmt::Display;
use rlox_contract::{TokenContext, Token, LiteralTokenType};
use std::error::Error;

type Result<T> = std::result::Result<T, LexicalError>;

pub struct Scanner {

}

impl Scanner {


    pub fn new() -> Scanner {
        Scanner {  }
    }



    pub fn scan(&self, source: &str) -> Result<Box<Vec<TokenContext>>> {
        
        if source.len() == 0 {
            return Ok(Box::from(vec![]));
        }
    
        let mut tokens : Vec<TokenContext> = Vec::new();
        
        let mut current_idx = 0;
        let mut line = 1;
        let mut char_idx : usize= 0;
        let chars : Vec<char> = source.chars().collect();
    
        while current_idx < source.len() {
            if chars[current_idx].is_whitespace() {
                char_idx += 1;
                if chars[current_idx] == '\n' {
                    line += 1;
                    char_idx = 0;
                }
                current_idx += 1;
            } else {
                let ctx = match &chars[current_idx] {
                    '(' => TokenContext::new(Token::LeftParen, line, char_idx, '('),
                    ')' => TokenContext::new(Token::RightParen, line, char_idx, ')'),
                    '{' => TokenContext::new(Token::LeftBrace, line, char_idx, '{'),
                    '}' => TokenContext::new(Token::RightBrace, line, char_idx, '}'),
                    ',' => TokenContext::new(Token::Comma, line, char_idx, ','),
                    '.' => TokenContext::new(Token::Dot, line, char_idx, '.'),
                    '*' => TokenContext::new(Token::Star, line, char_idx, '*'),
                    '-' => TokenContext::new(Token::Minus, line, char_idx, '-'),
                    '+' => TokenContext::new(Token::Plus, line, char_idx, '+'),
                    ';' => TokenContext::new(Token::Semicolon, line, char_idx, ';'),
                    c => {
                        match c {
                            '!'|'='|'>'|'<' => {
                                if chars.len() > current_idx + 1 && !&chars[current_idx+1].is_whitespace() {
                                    match (c, &chars[current_idx + 1]) {
                                        ('!', '=') => TokenContext::new(Token::BangEqual, line, char_idx, "!="),
                                        ('>', '=') => TokenContext::new(Token::GreaterEqual, line, char_idx, ">="),
                                        ('<', '=') => TokenContext::new(Token::LessEqual, line ,char_idx, "<="),
                                        ('=', '=') => TokenContext::new(Token::EqualEqual, line, char_idx, "=="),
                                        ('!', _) => TokenContext::new(Token::Bang, line, char_idx, "!"),
                                        ('=', _) => TokenContext::new(Token::Equal, line, char_idx, "="),
                                        ('>', _) => TokenContext::new(Token::Greater, line, char_idx, ">"),
                                        ('<', _) => TokenContext::new(Token::Less, line, char_idx, "<"),
                                        _ => return Err(LexicalError::new("unexpected, should never be here"))
                                    }
                                } else {
                                    match c {
                                        '!' => TokenContext::new(Token::Bang, line, char_idx, '!'),
                                        '=' => TokenContext::new(Token::Equal, line, char_idx, "="),
                                        '>' => TokenContext::new(Token::Greater, line, char_idx, ">"),
                                        '<' => TokenContext::new(Token::Less, line, char_idx, "<"),
                                        _ => return Err(LexicalError::new("unexpected 2, should never be here"))
                                    }
                                }
                            },
                            '/' => {
                                
                                if chars.len() > current_idx + 1 && chars[current_idx + 1] == '/' {
                                    for (idx, j) in chars[current_idx..].iter().enumerate() {
                                        current_idx+=1;   
                                        
                                        if *j == '\n' {
                                            line += 1;
                                            break;
                                        }
                                    }
    
                                    
                                    continue;
                                } else {
                                   TokenContext::new(Token::Slash, line, char_idx, "/")                                
                                }
                            }
                            _ => {
                                if c.is_digit(10) {
                                    let mut num_str = String::new();
    
                                    for (ix, nxt_c) in chars[current_idx..].iter().enumerate() {
                                        let wc = nxt_c.clone();
                                        if !(wc.is_digit(10)) && wc != '.' {
                                            break;
                                        } else {
                                            num_str.push(wc);
                                        }
                                    }
                                    let num:f64 = num_str.parse().unwrap();
                                    TokenContext::new(Token::Literal(LiteralTokenType::NumberLiteral(num)), line, char_idx, num_str)
    
                                    
                                }else if *c == '"' {
                                    let mut chunk = String::new();
                                    if current_idx + 1 > chars.len() {
                                        return Err(LexicalError::new(format!("Unexpected EOF, expected \"")));
                                    }
                                    chunk.push('"');
                                    let start_char_idx = char_idx;
                                    for (ix, nxt_c) in chars[current_idx+1..].iter().enumerate() {
                                        chunk.push(nxt_c.clone());
                                        char_idx += 1;
                                        if nxt_c.clone() == '"' {
                                            break;
                                        }
                                        if nxt_c.clone() == '\n' {
                                            line += 1;
                                            char_idx = 0;
                                        }
                                    }
                                    TokenContext::new(Token::Literal(LiteralTokenType::StringLiteral(chunk.clone())), line, start_char_idx, chunk.clone())
                                } else {
                                    let mut chunk = String::new();
                                    for (ix, nxt_c) in chars[current_idx..].iter().enumerate() {
                                        
                                        if nxt_c.is_whitespace() || *nxt_c == ';' || *nxt_c == '(' || *nxt_c == ')' {
                                            break;
                                        } else if !nxt_c.is_alphanumeric() && *nxt_c != '_' {
                                            return Err(LexicalError::new(format!("UNEXPECTED CHAR {} at {}:{}", nxt_c, line, char_idx)));
                                        }
                                        else {
                                            chunk.push(*nxt_c);
                                        }
    
                                    }
                                    match chunk.as_str() {
                                        "and" => TokenContext::new(Token::And, line, char_idx, chunk),
                                        "class" => TokenContext::new(Token::Class, line, char_idx, chunk),
                                        "else" => TokenContext::new(Token::Else, line, char_idx, chunk),
                                        "false" => TokenContext::new(Token::False, line, char_idx, chunk),
                                        "fun" => TokenContext::new(Token::Fun, line, char_idx, chunk),
                                        "for" => TokenContext::new(Token::For, line, char_idx, chunk),
                                        "if" => TokenContext::new(Token::If, line, char_idx, chunk),
                                        "nil" => TokenContext::new(Token::Nil, line, char_idx, chunk),
                                        "or" => TokenContext::new(Token::Or, line, char_idx, chunk),
                                        "print" => TokenContext::new(Token::Print, line, char_idx, chunk),
                                        "return" => TokenContext::new(Token::Return, line, char_idx, chunk),
                                        "super" => TokenContext::new(Token::Super, line, char_idx, chunk),
                                        "this" => TokenContext::new(Token::This, line, char_idx, chunk),
                                        "true" => TokenContext::new(Token::True, line, char_idx, chunk),
                                        "var" => TokenContext::new(Token::Var, line, char_idx, chunk),
                                        "while" => TokenContext::new(Token::While, line, char_idx, chunk),
                                        _ => TokenContext::new(Token::from_identifier(&chunk), line, char_idx, chunk)
                                    }
                                    
                                }
                            }
                        }
                        
                    }
                };
                current_idx += &ctx.length();
                char_idx += &ctx.length();
                tokens.push(ctx);
                
            }
            
            
        }
        tokens.push(TokenContext::new(Token::Eof, line, char_idx, ""));
        Ok(Box::from(tokens))
    }
}



#[derive(Debug,Clone)]
pub struct LexicalError {
    message: String
}
impl LexicalError {
    fn new<B>(message: B) -> LexicalError where B : ToString {
        LexicalError { message: message.to_string() }
    }
}
impl Error for LexicalError {}
impl Display for LexicalError {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        write!(f, "{}", self.message)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{TokenContext, Token, LiteralTokenType};

    #[test]
    fn it_parses_out_single_token_lexeme() {
        let source= "(";
        let res = super::Scanner::new().scan(source).unwrap();
        assert_eq!(TokenContext::new(Token::LeftParen, 1, 0, "("), res[0]);
    }

    #[test]
    fn it_parses_out_simple_two_char_lexeme() {
        let source = "<=";
        let res = super::Scanner::new().scan(source).unwrap();
        assert_eq!(TokenContext::new(Token::LessEqual, 1, 0, "<="), res[0]);

    }

    #[test]
    fn it_skips_whitespace_incrementing_char_counter() {
        let source = "  *";
        let res = super::Scanner::new().scan(source).unwrap();
        assert_eq!(TokenContext::new(Token::Star, 1, 2, "*"), res[0]);
    }

    #[test]
    fn it_parses_potential_two_char_lexeme_into_one_char_lexeme_if_subs_not_present() {
        let source = "!)";
        let res = super::Scanner::new().scan(source).unwrap();
        assert_eq!(TokenContext::new(Token::Bang, 1, 0, "!"), res[0]);
    }

    #[test]
    fn it_increments_line_numbers_for_new_lines() {
        let source = r#"

        !=
        "#;
        let res = super::Scanner::new().scan(source).unwrap();
        assert_eq!(TokenContext::new(Token::BangEqual, 3, 8, "!="), res[0]);
    }

    #[test]
    fn it_skips_comments_and_continues_on_next_line() {
        let source = r#"// this is a comment
        !=
        "#;

        let res = super::Scanner::new().scan(source).unwrap();

        assert_eq!(2, res.len());
        assert_eq!(TokenContext::new(Token::BangEqual, 2, 8, "!="), res[0]);
    }

    #[test]
    fn it_parses_slash_correctly() {
        let source = "/ ";
        let res = super::Scanner::new().scan(source).unwrap();
        assert_eq!(2, res.len());
        assert_eq!(TokenContext::new(Token::Slash, 1, 0, "/"), res[0]);
    }

    #[test]
    fn it_parses_number_correctly() {
        let source = "2.1";
        let res = super::Scanner::new().scan(source).unwrap();
        assert_eq!(TokenContext::new(Token::Literal(LiteralTokenType::NumberLiteral(2.1)), 1, 0, "2.1"), res[0]);
    }

    #[test]
    fn it_parses_number_correctly_no_decimal_part() {
        let source = "2.";
        let res = super::Scanner::new().scan(source).unwrap();
        assert_eq!(TokenContext::new(Token::Literal(LiteralTokenType::NumberLiteral(2.0)), 1, 0, "2."), res[0]);
    }

    #[test]
    fn it_parses_simple_keyword_out_correctly() {
        let source = "and";
        let res = super::Scanner::new().scan(source).unwrap();
        assert_eq!(TokenContext::new(Token::And, 1, 0, "and"), res[0]);
    }

    #[test]
    fn it_parses_comments_correctly_at_eof() {
        let source = "and //hello";
        let res = super::Scanner::new().scan(source).unwrap();
        assert_eq!(2, res.len());
    }

    #[test]
    fn it_parses_strings_correctly() {
        let source = "\"hello\"";
        let res = super::Scanner::new().scan(source).unwrap();
        assert_eq!(TokenContext::new(Token::from_string("\"hello\""), 1, 0, "\"hello\""), res[0]);
    }
}
