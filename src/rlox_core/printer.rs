use crate::rlox_core::shared_models::{LiteralType, Expr, ExprType, TokenType};

pub fn print_ast_grouped(root_expr: &Expr) -> String {
    let (_, expr_type) = (root_expr.token(), root_expr.expr_type());

    let res = match expr_type {
        ExprType::LiteralExpr(t, v) => match t {
            LiteralType::NilLiteral => format!("nil"),
            LiteralType::NumberLiteral | LiteralType::IdentifierLiteral => format!(r#"{}"#, v.lexeme()),
            LiteralType::StringLiteral => if let TokenType::String(s) = v.clone().token_type() { format!(r#""{}""#, s.to_string()) } else { panic!("bad") }
        },
        ExprType::BinaryExpr(l,o,r) => {
            let mut start = String::new();
            start.push('(');
            let le = l.clone();
            let lstr = print_ast_grouped(&le);
            start.push_str(&lstr);
            start.push(' ');
            start.push_str(&format!(r#"{:?}"#, o));
            start.push(' ');
            let re = r.clone();
            let rstr = print_ast_grouped(&re);
            start.push_str(&rstr);
            start.push(')');
            start
        },
        ExprType::GroupingExpr(e) => {
            let mut start = String::new();
            start.push('(');
            start.push_str("group ");
            let ge = e.clone();
            let gstr = print_ast_grouped(&ge);
            start.push_str(&gstr);
            start.push(')');
            start
        }
        ExprType::UnaryExpr(o,e) => {
            let mut start = String::new();
            start.push('(');
            start.push_str(&format!(r#"{:?}"#, o));
            start.push(' ');
            let oe = e.clone();
            let oestr = print_ast_grouped(&oe);
            start.push_str(&oestr);
            start.push(')');
            start
        }
    };
    res
}

pub fn pretty_print_ast(expr: &Expr) -> std::io::Result<()> {
    let mut curr_depth : usize = 0;
    let mut processed : Vec<(String, usize)> = vec![];
    let mut left : Vec<&Expr> = vec![expr];

    while let Some(e) = left.pop() {
        match e.expr_type() {
            ExprType::LiteralExpr(_, t) => {
                match t.token_type() {
                    TokenType::String(s) => { 
                        processed.push((s.to_string(), curr_depth));
                    },
                    _ => unimplemented!()
                }
            },
            ExprType::GroupingExpr(inner) => {
                processed.push(("grouped".to_string(), curr_depth));
                left.push(inner);
                curr_depth += 1;
            },
            ExprType::BinaryExpr(l,o,r) => {
                processed.push(("binary".to_string(), curr_depth));
                curr_depth += 1;
                left.push(l);
                left.push(r);
            },
            _ => unimplemented!()
        }
    }
    for (s, ix) in processed {
        for _ in 0..ix {
            print!("-");
        }
        println!(r#"{}"#, s);
    }
    Ok(())
}

#[allow(dead_code)]
mod test {
    #[allow(dead_code)]
    use crate::rlox_core::shared_models::{Token, TokenType};

    #[test]
    fn it_prints_lit_nil_expression_correctly() {
        let e = super::Expr::new_literal_expr(std::rc::Rc::from(Token::new(TokenType::Nil, "nil".to_string(), 1)));
        let res = super::print_ast_grouped(&e);

        assert_eq!("nil", res);
    }

    #[test]
    fn it_prints_lit_num_expr_correctly() {
        let e = super::Expr::new_literal_expr(std::rc::Rc::from(Token::new(TokenType::Number(1.2345), "1.2345".to_string(), 1)));
        let res = super::print_ast_grouped(&e);

        assert_eq!("1.2345", res);
    }

    #[test]
    fn it_prints_lit_identifier_expr_correctly() {
        let e = super::Expr::new_literal_expr(std::rc::Rc::from(Token::new(TokenType::Identifier("TestIdent".to_string()), "TestIdent".to_string(), 1)));
        let res = super::print_ast_grouped(&e);

        assert_eq!("TestIdent", res);
    }

    #[test]
    fn it_prints_lit_string_expr_correctly() {
        let e = super::Expr::new_literal_expr(std::rc::Rc::from(Token::new(TokenType::String("TestString".to_string()), "TestString".to_string(), 1)));
        let res = super::print_ast_grouped(&e);

        assert_eq!("\"TestString\"", res);
    }

    #[test]
    fn it_prints_binary_expr_correctly() {
        let lit1 = super::Expr::new_literal_expr(std::rc::Rc::from(Token::new(TokenType::String("hello".to_string()), "hello".to_string(), 1)));
        let lit2 = super::Expr::new_literal_expr(std::rc::Rc::from(Token::new(TokenType::String("bye".to_string()), "bye".to_string(), 2)));

        let e = super::Expr::new_binary_expr(std::rc::Rc::from(lit1), std::rc::Rc::from(Token::new(TokenType::EqualEqual, "==".to_string(), 3)), std::rc::Rc::from(lit2));

        let res = super::print_ast_grouped(&e);

        assert_eq!("(\"hello\" EqualEqual \"bye\")", res);
    }

    #[test]
    fn it_prints_grouped_expr_correctly() {
        let e = super::Expr::new_literal_expr(std::rc::Rc::from(Token::new(TokenType::Identifier("TestIdent".to_string()), "TestIdent".to_string(), 1)));
        let ge = super::Expr::new_grouping_expr(std::rc::Rc::from(e), std::rc::Rc::from(Token::new(TokenType::Nil, "nil".to_string(), 2)));
        let res = super::print_ast_grouped(&ge);

        assert_eq!("(group TestIdent)", res);
    }
}