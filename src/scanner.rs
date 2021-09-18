
#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: i64
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, line: i64) -> Token {
        Token { token_type, lexeme, line }
    }

    fn next(source: &str, line: i64) -> Result<(Token, i64), LexicalError> {
        let csiter = source.chars();
        let cs : Vec<char> = csiter.collect();
        if cs.len() == 0 { Ok((Token::new(TokenType::Eof, "EOF".to_string(), line), 0)) }
         else {
             match cs[0] {
                 '\r'|' '|'\t' => Ok((Token::new(TokenType::WhiteSpace, slice_to_lexeme(source, 0, 1), line), 1)),
                 '\n' => Ok((Token::new(TokenType::WhiteSpace, slice_to_lexeme(source, 0, 1), line+1), 1)),
                 '(' => Ok((Token::new(TokenType::LeftParen, slice_to_lexeme(source,0,1), line), 1)),
                 ')' => Ok((Token::new(TokenType::RightParen, slice_to_lexeme(source, 0, 1), line), 1)),
                 '{' => Ok((Token::new(TokenType::LeftBrace, slice_to_lexeme(source,0,1), line), 1)),
                 '}' => Ok((Token::new(TokenType::RightBrace, slice_to_lexeme(source, 0, 1), line), 1)),
                 ',' => Ok((Token::new(TokenType::LeftParen, slice_to_lexeme(source,0,1), line), 1)),
                 '.' => Ok((Token::new(TokenType::RightParen, slice_to_lexeme(source, 0, 1), line), 1)),
                 '-' => Ok((Token::new(TokenType::LeftParen, slice_to_lexeme(source,0,1), line), 1)),
                 '+' => Ok((Token::new(TokenType::RightParen, slice_to_lexeme(source, 0, 1), line), 1)),
                 ';' => Ok((Token::new(TokenType::LeftParen, slice_to_lexeme(source,0,1), line), 1)),
                 '*' => Ok((Token::new(TokenType::RightParen, slice_to_lexeme(source, 0, 1), line), 1)),
                 c => {
                    if cs.len() == 1 { return Err(LexicalError { line, error_lexeme: String::from("unexpected EOF"), message: String::from("uhoh")}); }
                    match (c, cs[1]) {
                        ('!', '=') => Ok((Token::new(TokenType::BangEqual, slice_to_lexeme(source, 0, 2), line), 2)),
                        ('!', _) => Ok((Token::new(TokenType::Bang, slice_to_lexeme(source, 0, 1), line), 1)),
                        ('=', '=') => Ok((Token::new(TokenType::EqualEqual, slice_to_lexeme(source, 0, 2), line), 2)),
                        ('=', _) => Ok((Token::new(TokenType::Equal, slice_to_lexeme(source, 0, 1), line), 1)),
                        ('>', '=') => Ok((Token::new(TokenType::GreaterEqual, slice_to_lexeme(source, 0, 2), line), 2)),
                        ('>', _) => Ok((Token::new(TokenType::Greater, slice_to_lexeme(source, 0, 1), line), 1)),
                        ('<', '=') => Ok((Token::new(TokenType::LessEqual, slice_to_lexeme(source, 0, 2), line), 2)),
                        ('<', _) => Ok((Token::new(TokenType::Less, slice_to_lexeme(source, 0, 1), line), 1)),
                        ('/', '/') => {
                            let mut eoc : usize = 0;
                            let mut line_count = line;
                            for (ix, nc) in cs[0..].iter().enumerate() {
                                eoc = ix;
                                if *nc == '\n' {
                                    line_count += 1;
                                    break;
                                }
                            }

                            Ok((Token::new(TokenType::Comment(slice_to_lexeme(source, 0, eoc)), slice_to_lexeme(source, 0, eoc), line_count), eoc as i64))
                        },
                        ('/', _) => Ok((Token::new(TokenType::Slash, slice_to_lexeme(source, 0, 1), line), 1)),
                        _ => {
                            let mut idx : usize = 0;
                            let mut line_count = line;
                            for (ix, nc) in cs[0..].iter().enumerate() {
                                idx = ix;
                                if *nc == '\n' {
                                    line_count += 1;
                                    break;
                                } else if *nc == '\r' || *nc == ' ' || *nc == '\t' {
                                    break;
                                }
                            }
                            let chunk : String = cs[0..idx as usize + 1].iter().collect();
                            match &chunk[..] {
                                "and" => Ok((Token::new(TokenType::And, slice_to_lexeme(source, 0, idx + 1), line_count), idx as i64 + 1)),
                                "class" => Ok((Token::new(TokenType::Class, slice_to_lexeme(source, 0, idx + 1), line_count), idx as i64 + 1)),
                                "else" => Ok((Token::new(TokenType::Else, slice_to_lexeme(source, 0, idx + 1), line_count), idx as i64 + 1)),
                                "false" => Ok((Token::new(TokenType::False, slice_to_lexeme(source, 0, idx + 1), line_count), idx as i64 + 1)),
                                "fun" => Ok((Token::new(TokenType::Fun, slice_to_lexeme(source, 0, idx + 1), line_count), idx as i64 + 1)),
                                "for" => Ok((Token::new(TokenType::For, slice_to_lexeme(source, 0, idx + 1), line_count), idx as i64 + 1)),
                                "if" => Ok((Token::new(TokenType::If, slice_to_lexeme(source, 0, idx + 1), line_count), idx as i64 + 1)),
                                "nil" => Ok((Token::new(TokenType::Nil, slice_to_lexeme(source, 0, idx + 1), line_count), idx as i64 + 1)),
                                "or" => Ok((Token::new(TokenType::Or, slice_to_lexeme(source, 0, idx + 1), line_count), idx as i64 + 1)),
                                "print" => Ok((Token::new(TokenType::Print, slice_to_lexeme(source, 0, idx + 1), line_count), idx as i64 + 1)),
                                "return" => Ok((Token::new(TokenType::Return, slice_to_lexeme(source, 0, idx + 1), line_count), idx as i64 + 1)),
                                "super" => Ok((Token::new(TokenType::Super, slice_to_lexeme(source, 0, idx + 1), line_count), idx as i64 + 1)),
                                "this" => Ok((Token::new(TokenType::This, slice_to_lexeme(source, 0, idx + 1), line_count), idx as i64 + 1)),
                                "true" => Ok((Token::new(TokenType::True, slice_to_lexeme(source, 0, idx + 1), line_count), idx as i64 + 1)),
                                "var" => Ok((Token::new(TokenType::Var, slice_to_lexeme(source, 0, idx + 1), line_count), idx as i64 + 1)),
                                "while" => Ok((Token::new(TokenType::While, slice_to_lexeme(source, 0, idx + 1), line_count), idx as i64 + 1)),
                                other => {
                                    if other.starts_with('"') {
                                        unimplemented!()
                                    } else if false {
                                        unimplemented!()
                                    } else {
                                        unimplemented!();
                                    }
                                }
                            }
                        }
                    }

                 },
                //  _ => Err(LexicalError { line, error_lexeme: String::from("bad"), message: String::from("sad")})
             }

             
         }
    }

    
}

fn slice_to_lexeme(source: &str, start: usize, end: usize) -> String {
    (*source)[start..end].to_string()
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    LeftParen, RightParen, LeftBrace, RightBrace, Comma, Dot, Minus, Plus, Semicolon, Star,
    Slash, Comment(String), WhiteSpace,
    Bang, BangEqual, Equal, EqualEqual, Greater, GreaterEqual, Less, LessEqual,

    Identifier(String), String(String), Number(f64),

    And, Class, Else, False, Fun, For, If, Nil, Or, Print, Return, Super, This,
    True, Var, While,

    Eof
}



#[derive(Debug)]
pub struct LexicalError {
    line: i64,
    error_lexeme: String,
    message: String
}

impl std::fmt::Display for LexicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "ERROR (line {}): {} at {}", self.line, self.message, self.error_lexeme);
        Ok(())
    }
}

impl std::error::Error for LexicalError { }

mod test {
    use super::*;

    #[test]
    pub fn test_token_next_works_simplest_case() {
        let source = "(";
        let nxt = Token::next(source, 1);

        if nxt.is_err() { panic!("Expected some, got error {:?}", nxt); }
        else {
            let (t, cs) = nxt.unwrap();
            assert_eq!(t.token_type, TokenType::LeftParen);
            assert_eq!(t.line, 1);
            assert_eq!(t.lexeme, "(");
            assert_eq!(cs, 1);
        }
    }

    #[test]
    pub fn test_token_next_works_one_or_two_chars_case_two() {
        let source = "!=";
        let nxt = Token::next(source, 1);

        if nxt.is_err() { panic!("Expected some, got error {:?}", nxt); }
        else {
            let (t, cs) = nxt.unwrap();
            assert_eq!(t.token_type, TokenType::BangEqual);
            assert_eq!(t.line, 1);
            assert_eq!(t.lexeme, "!=");
            assert_eq!(cs, 2);
        }
    }

    #[test]
    pub fn test_token_next_works_one_or_two_chars_case_one() {
        let source = "! ";
        let nxt = Token::next(source, 1);

        if nxt.is_err() { panic!("Expected some, got error {:?}", nxt); }
        else {
            let (t, cs) = nxt.unwrap();
            assert_eq!(t.token_type, TokenType::Bang);
            assert_eq!(t.line, 1);
            assert_eq!(t.lexeme, "!");
            assert_eq!(cs, 1);
        }
    }

    #[test]
    pub fn test_token_next_reads_comment_to_end_of_line() {
        let source = "//im a comment\n";
        let nxt = Token::next(source, 1);

        if nxt.is_err() { panic!("Expected some, got error {:?}", nxt); }
        else {
            let (t, cs) = nxt.unwrap();
            assert_eq!(t.token_type, TokenType::Comment(String::from("//im a comment")));
            assert_eq!(t.line, 2);
            assert_eq!(t.lexeme, "//im a comment");
            assert_eq!(cs, 14);
        }
    }

    #[test]
    pub fn test_token_next_reads_keyword_and() {
        let source = "and";
        let nxt = Token::next(source, 1);

        if nxt.is_err() { panic!("Expected some, got error {:?}", nxt); }
        else {
            let (t, cs) = nxt.unwrap();
            assert_eq!(t.token_type, TokenType::And);
            assert_eq!(t.line, 1);
            assert_eq!(t.lexeme, "and");
            assert_eq!(cs, 3);
        }
    }
}