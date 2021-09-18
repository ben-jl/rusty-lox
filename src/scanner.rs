
pub fn scan(source: &str) -> Vec<Result<Token, LexicalError>> {
    let mut tokens : Vec<Result<Token, LexicalError>> = Vec::new();

    let mut line : i64 = 1;
    let mut current_idx : usize = 0;

    while current_idx <= source.len() - 1 {

        match Token::next(&source[current_idx..], line) {
            Ok((t, cidx, l)) => {

                if t.token_type == TokenType::Eof {
                    break;
                }
                line = l;
                current_idx += cidx as usize;
                tokens.push(Ok(t));
            },
            Err(l) => {
                current_idx += l.error_lexeme.len();
                tokens.push(Err(l));
            }
        }
        //current_idx += 1;
    }
    tokens
}

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

    fn next(source: &str, line: i64) -> Result<(Token, i64, i64), LexicalError> {
        let csiter = source.chars();
        let cs : Vec<char> = csiter.collect();
        if cs.len() == 0 { Ok((Token::new(TokenType::Eof, "EOF".to_string(), line), 0, line)) }
         else {
             match cs[0] {
                 '\r'|' '|'\t' => Ok((Token::new(TokenType::WhiteSpace, slice_to_lexeme(source, 0, 1), line), 1, line)),
                 '\n' => Ok((Token::new(TokenType::WhiteSpace, slice_to_lexeme(source, 0, 1), line), 1, line + 1)),
                 '(' => Ok((Token::new(TokenType::LeftParen, slice_to_lexeme(source,0,1), line), 1, line)),
                 ')' => Ok((Token::new(TokenType::RightParen, slice_to_lexeme(source, 0, 1), line), 1, line)),
                 '{' => Ok((Token::new(TokenType::LeftBrace, slice_to_lexeme(source,0,1), line), 1, line)),
                 '}' => Ok((Token::new(TokenType::RightBrace, slice_to_lexeme(source, 0, 1), line), 1, line)),
                 ',' => Ok((Token::new(TokenType::Comma, slice_to_lexeme(source,0,1), line), 1, line)),
                 '.' => Ok((Token::new(TokenType::Dot, slice_to_lexeme(source, 0, 1), line), 1, line)),
                 '-' => Ok((Token::new(TokenType::Minus, slice_to_lexeme(source,0,1), line), 1, line)),
                 '+' => Ok((Token::new(TokenType::Plus, slice_to_lexeme(source, 0, 1), line), 1, line)),
                 ';' => Ok((Token::new(TokenType::Semicolon, slice_to_lexeme(source,0,1), line), 1, line)),
                 '*' => Ok((Token::new(TokenType::Star, slice_to_lexeme(source, 0, 1), line), 1, line)),
                 c => {
                    if cs.len() == 1 { return Err(LexicalError { line, error_lexeme: String::from("unexpected EOF"), message: String::from("uhoh")}); }
                    match (c, cs[1]) {
                        ('!', '=') => Ok((Token::new(TokenType::BangEqual, slice_to_lexeme(source, 0, 2), line), 2, line)),
                        ('!', _) => Ok((Token::new(TokenType::Bang, slice_to_lexeme(source, 0, 1), line), 1, line)),
                        ('=', '=') => Ok((Token::new(TokenType::EqualEqual, slice_to_lexeme(source, 0, 2), line), 2, line)),
                        ('=', _) => Ok((Token::new(TokenType::Equal, slice_to_lexeme(source, 0, 1), line), 1, line)),
                        ('>', '=') => Ok((Token::new(TokenType::GreaterEqual, slice_to_lexeme(source, 0, 2), line), 2, line)),
                        ('>', _) => Ok((Token::new(TokenType::Greater, slice_to_lexeme(source, 0, 1), line), 1, line)),
                        ('<', '=') => Ok((Token::new(TokenType::LessEqual, slice_to_lexeme(source, 0, 2), line), 2, line)),
                        ('<', _) => Ok((Token::new(TokenType::Less, slice_to_lexeme(source, 0, 1), line), 1, line)),
                        ('/', '/') => {
                            let mut eoc : usize = 0;
                            let mut line_count = line;
                            for (ix, nc) in cs[0..].iter().enumerate() {
                                
                                if *nc == '\n' {
                                    line_count += 1;
                                    break;
                                }
                                eoc = ix;
                            }

                            Ok((Token::new(TokenType::Comment(slice_to_lexeme(source, 0, eoc + 1)), slice_to_lexeme(source, 0, eoc + 1), line_count), eoc as i64, line_count))
                        },
                        ('/', _) => Ok((Token::new(TokenType::Slash, slice_to_lexeme(source, 0, 1), line), 1, line)),
                        ('"', _) => {
                            let mut idx: usize = 0;
                            let mut line_count = line;
                            for (ix, nc) in cs[1..].iter().enumerate() {
                                idx = ix;
                                if *nc == '"' {
                                    break;
                                } else if *nc == '\n' {
                                    line_count += 1;
                                }
                            }
                            let string_contents = cs[1..idx+1].iter().collect();
                            Ok((Token::new(TokenType::String(string_contents), slice_to_lexeme(source, 0, idx+2), line_count), idx as i64+2, line_count))
                        },
                        _ => {
                            let mut idx : usize = 0;
                            let mut line_count = line;
                            for (_, nc) in cs[0..].iter().enumerate() {
                                //idx = ix;
                                if *nc == '\n' {
                                    line_count += 1;
                                    break;
                                } else if *nc == '\r' || *nc == ' ' || *nc == '\t' {
                                    break;
                                } else {
                                    idx += 1;
                                }
                            }
                            let chunk : String = cs[0..idx as usize].iter().collect();
                            match &chunk[..] {
                                "and" => Ok((Token::new(TokenType::And, slice_to_lexeme(source, 0, idx), line_count), idx as i64, line_count)),
                                "class" => Ok((Token::new(TokenType::Class, slice_to_lexeme(source, 0, idx), line_count), idx as i64, line_count)),
                                "else" => Ok((Token::new(TokenType::Else, slice_to_lexeme(source, 0, idx), line_count), idx as i64, line_count)),
                                "false" => Ok((Token::new(TokenType::False, slice_to_lexeme(source, 0, idx), line_count), idx as i64, line_count)),
                                "fun" => Ok((Token::new(TokenType::Fun, slice_to_lexeme(source, 0, idx ), line_count), idx as i64, line_count)),
                                "for" => Ok((Token::new(TokenType::For, slice_to_lexeme(source, 0, idx), line_count), idx as i64, line_count)),
                                "if" => Ok((Token::new(TokenType::If, slice_to_lexeme(source, 0, idx), line_count), idx as i64, line_count)),
                                "nil" => Ok((Token::new(TokenType::Nil, slice_to_lexeme(source, 0, idx), line_count), idx as i64, line_count)),
                                "or" => Ok((Token::new(TokenType::Or, slice_to_lexeme(source, 0, idx), line_count), idx as i64, line_count)),
                                "print" => Ok((Token::new(TokenType::Print, slice_to_lexeme(source, 0, idx), line_count), idx as i64, line_count)),
                                "return" => Ok((Token::new(TokenType::Return, slice_to_lexeme(source, 0, idx), line_count), idx as i64, line_count)),
                                "super" => Ok((Token::new(TokenType::Super, slice_to_lexeme(source, 0, idx), line_count), idx as i64, line_count)),
                                "this" => Ok((Token::new(TokenType::This, slice_to_lexeme(source, 0, idx), line_count), idx as i64, line_count)),
                                "true" => Ok((Token::new(TokenType::True, slice_to_lexeme(source, 0, idx), line_count), idx as i64, line_count)),
                                "var" => Ok((Token::new(TokenType::Var, slice_to_lexeme(source, 0, idx), line_count), idx as i64, line_count)),
                                "while" => Ok((Token::new(TokenType::While, slice_to_lexeme(source, 0, idx), line_count), idx as i64, line_count)),
                                other => {
                                    if other.contains('.') {
                                        let num: f64 = other.parse().unwrap();

                                        return Ok((Token::new(TokenType::Number(num), slice_to_lexeme(source, 0, other.len()), line_count), idx as i64, line_count))
                                    } else {
                                        let chars :Vec<char> = other.chars().collect();
                                        if chars[0].is_alphabetic() && chars.iter().all(|c| c.is_alphanumeric() || *c == '_') {
                                            return Ok((Token::new(TokenType::Identifier(other.to_string()), slice_to_lexeme(source, 0, chars.len()), line_count), idx as i64, line_count))

                                        }else {
                                            println!("{:?}", other);
                                            return Err(LexicalError { line, error_lexeme: String::from("bad"), message: String::from("sad")})
                                        }
                                    }
                                }
                            }
                        }
                    }
                 },
             }        
         }
    }
}

fn slice_to_lexeme(source: &str, start: usize, end: usize) -> String {
    (*source)[start..end].to_string()
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
            let (t, cs, _) = nxt.unwrap();
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
            let (t, cs, _) = nxt.unwrap();
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
            let (t, cs, _) = nxt.unwrap();
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
            let (t, cs, _) = nxt.unwrap();
            assert_eq!(t.token_type, TokenType::Comment(String::from("//im a comment")));
            assert_eq!(t.line, 2);
            assert_eq!(t.lexeme, "//im a comment");
            assert_eq!(cs, 13);
        }
    }

    #[test]
    pub fn test_token_next_reads_keyword_and() {
        let source = "and";
        let nxt = Token::next(source, 1);

        if nxt.is_err() { panic!("Expected some, got error {:?}", nxt); }
        else {
            let (t, cs, _) = nxt.unwrap();
            assert_eq!(t.token_type, TokenType::And);
            assert_eq!(t.line, 1);
            assert_eq!(t.lexeme, "and");
            assert_eq!(cs, 3);
        }
    }

    #[test]
    pub fn test_creates_string_token_correctly_in_next() {
        let source = "\"im a string\"";
        let nxt = Token::next(source, 1);
        if nxt.is_err() {
            panic!("Expected some, got error {:?}", nxt); 
        }
        else {
            let (t,cs, _) = nxt.unwrap();
            assert_eq!(t.token_type, TokenType::String("im a string".to_string()));
            assert_eq!(t.line, 1);
            assert_eq!(t.lexeme, "\"im a string\"");
            assert_eq!(cs, 13);
        }
    }

    #[test]
    pub fn test_creates_number_token_correctly_in_next() {
        let source = "1.345";
        let nxt = Token::next(source, 1);
        if nxt.is_err() {
            panic!("Expected some, got error {:?}", nxt);
        } else {
            let (t,cs,_) = nxt.unwrap();
            assert_eq!(t.token_type, TokenType::Number(1.345));
            assert_eq!(t.line, 1);
            assert_eq!(t.lexeme, "1.345");
            assert_eq!(cs, 5);
        }
    }

    #[test]
    pub fn test_creates_identifier_token_correctly_in_next() {
        let source = "identwhat";
        let nxt = Token::next(source, 1);
        if nxt.is_err() {
            panic!("Expected some, got error {:?}", nxt);
        } else {
            let (t,cs, _) = nxt.unwrap();
            assert_eq!(t.token_type, TokenType::Identifier("identwhat".to_string()));
            assert_eq!(t.line, 1);
            assert_eq!(t.lexeme, "identwhat");
            assert_eq!(cs, 9);
        }
    }

    #[test]
    pub fn test_pulls_out_full_comments() {
        let source = "// see ya";
        let nxt = Token::next(source, 1);
        if nxt.is_err() {
            panic!("Expected some, got error {:?}", nxt);
        } else {
            let (t,cs,_) = nxt.unwrap();
            assert_eq!(t.token_type, TokenType::Comment("// see ya".to_string()));
            assert_eq!(t.line, 1);
            assert_eq!(t.lexeme, "// see ya");
            assert_eq!(cs, 8);
        }
    }

    #[test]
    pub fn test_gets_full_identifier_after_newline() {
        let source = "\nlater";
        let vals = scan(&source);
        let snd = vals[1].as_ref();
        let sndt = snd.unwrap();

        assert_eq!(TokenType::Identifier("later".to_string()), sndt.token_type);
    }
}