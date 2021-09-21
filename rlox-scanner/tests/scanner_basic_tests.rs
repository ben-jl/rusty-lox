extern crate  rlox_scanner;


#[cfg(test)]
mod scanner_basic_tests {

    #[test]
    fn simple_scanner_integration_test_works() {
        let foo = rlox_scanner::scan("hello").unwrap();
        assert_eq!(rlox_contract::TokenContext::new(rlox_contract::Token::from_identifier("hello"), 1, 0, "hello"),foo[0]);
    }

    #[test]
    fn larger_source_scanner_test_multiple_lines_works() {
        let source = r#"class foo {
            "hello now"
        }
        // bye
        "#;
        let res = rlox_scanner::scan(&source).unwrap();
        let expected = vec![
            rlox_contract::TokenContext::new(rlox_contract::Token::Class, 1, 0, "class"),
            rlox_contract::TokenContext::new(rlox_contract::Token::from_identifier("foo"), 1, 6, "foo"),
            rlox_contract::TokenContext::new(rlox_contract::Token::LeftBrace, 1, 10, "{"),
            rlox_contract::TokenContext::new(rlox_contract::Token::from_string("\"hello now\""), 2, 12, "\"hello now\""),
            rlox_contract::TokenContext::new(rlox_contract::Token::RightBrace, 3, 8, "}"),
        ];
        assert_eq!(6, res.len());
        assert_eq!(expected[0], res[0]);
        assert_eq!(expected[1], res[1]);
        assert_eq!(expected[2], res[2]);
        assert_eq!(expected[3], res[3]);
        assert_eq!(expected[4], res[4]);
        assert_eq!(rlox_contract::TokenContext::new(rlox_contract::Token::Eof, 5,16, ""), res[5]);
        
    }
}