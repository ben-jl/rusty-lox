extern crate rlox_contract;
extern crate rlox_scanner;
extern crate rlox_parser;
use std::iter::FromIterator;
use std::error::Error;
use std::fmt::Display;
use log::{error, debug};
use rlox_contract::Token;
use std::collections::VecDeque;
use rlox_contract::{Expr,ExprLiteralValue, LiteralTokenType};
use std::io::Write;
use std::io::BufRead;
use std::rc::Rc;
use rlox_scanner::Scanner;
use rlox_parser::Parser;
use rlox_parser::ast_printer::print;

pub type Result<B> = std::result::Result<B, InterpreterError>;
mod environment;
use environment::Environment;

pub struct Interpreter {
    scanner : Scanner,
    parser : Parser,
    environment: Environment
}

impl Interpreter {
    pub fn default() -> Interpreter {
        let scanner = Scanner::new();
        let parser = Parser::new();
        let environment = Environment::new_root_environment();
        Interpreter { scanner, parser, environment }
    }

    pub fn execute_source<B>(&mut self, source: B) -> std::io::Result<()> where B : ToString {
        let sres = self.scanner.scan(&source.to_string()).expect("scanner failed");
        self.parser.add_tokens(*sres);
        let pres = self.parser.parse().unwrap();
        print!("\n");
        println!("{:?}", pres);
        Ok(())
    }

    pub fn start_repl(&mut self, stdin: &std::io::Stdin, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        loop {
            print!("rlox] ");
            stdout.flush()?;
            let nxt = read_line_from_stdin(stdin)?;
            if nxt.clone() == "quit" {
                break;
            }
            let sres = self.scanner.scan(&nxt);
            match sres {
                Ok(ts) => {
                    debug!("{:?}", &ts);
                    self.parser.add_tokens(ts.to_vec());
                    let pres = self.parser.parse();
                    debug!("{:?}", pres);
                    match pres {
                        Ok(exprs) => { 
                            for expr in exprs {
                                print!("\n");
                                match self.interpret(Box::from(expr)) {
                                    Err(e) => println!("{:?}", e),
                                    v => {
                                        println!("{:?}",v);
                                        println!("OK.")
                                    }
                                }
                            }
                        },
                        Err(pe) => { println!("{}", pe);}
                    }
                },
                Err(le) => {
                    println!("{}", le);

                }
            }
        }
        println!("Exiting...");
        Ok(())
    }

    fn interpret(&mut self, expr: Box<Expr>) -> Result<ComputedValue> {
        debug!("{:?}", expr);
        let v = match *expr {
            Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(b)) => ComputedValue::BooleanValue(b),
            Expr::LiteralExpr(ExprLiteralValue::NilLiteral) => ComputedValue::NilValue,
            Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(n)) => ComputedValue::NumberValue(n),
            Expr::LiteralExpr(ExprLiteralValue::StringLiteral(s)) => ComputedValue::StringValue(s.clone()),
            Expr::GroupingExpr(inner) => {
                return self.interpret(inner)
            },
            Expr::UnaryExpr { operator, right} => {
                let r = self.interpret(right)?;
                match (&operator,&r) {
                    (Token::Bang, ComputedValue::BooleanValue(b)) => ComputedValue::BooleanValue(!b),
                    (Token::Bang, ComputedValue::NilValue) => ComputedValue::BooleanValue(true),
                    (Token::Bang, _) => ComputedValue::BooleanValue(true),
                    (Token::Minus, ComputedValue::NumberValue(n)) => ComputedValue::NumberValue(-1 as f64 * n),
                    (Token::Minus, _) => return Err(InterpreterError::new(format!("Expected number, got {:?}", r))),
                    _ => return Err(InterpreterError::new(format!("Expected unary operator, got {:?}", operator)))
                }
            },
            Expr::BinaryExpr { left, operator, right } => {
                let l = self.interpret(left)?;
                let r = self.interpret(right)?;

                match (&l, &operator, &r) {
                    (ComputedValue::NumberValue(n), Token::Minus, ComputedValue::NumberValue(m)) => ComputedValue::NumberValue(n - m),
                    (_, Token::Minus, _) => return Err(InterpreterError::new(format!("Operator MINUS expects two numbers, got {:?} and {:?}", &l, &r))),

                    (ComputedValue::NumberValue(n), Token::Plus, ComputedValue::NumberValue(m)) => ComputedValue::NumberValue(n + m),
                    (ComputedValue::StringValue(s1), Token::Plus, ComputedValue::StringValue(s2)) => {
                        ComputedValue::StringValue(format!(r#"{}{}"#, s1, s2))
                    },
                    (_, Token::Plus, _) => return Err(InterpreterError::new(format!("Operator PLUS expects two numbers or two strings, got {:?} and {:?}", &l, &r))),

                    (ComputedValue::NumberValue(n), Token::Slash, ComputedValue::NumberValue(m)) => if *m == 0 as f64 { return Err(InterpreterError::new("Divide by zero error")); } else { ComputedValue::NumberValue(n / m)},
                    (_, Token::Slash, _) => return Err(InterpreterError::new(format!("Operator SLASH expects two numbers, got {:?} and {:?}", &l, &r))),

                    (ComputedValue::NumberValue(n), Token::Star, ComputedValue::NumberValue(m)) => ComputedValue::NumberValue(n * m),
                    (_, Token::Star, _) => return Err(InterpreterError::new(format!("Operator STAR expects two numbers, got {:?} and {:?}", &l, &r))),

                    (ComputedValue::NumberValue(n), Token::Greater, ComputedValue::NumberValue(m)) => ComputedValue::BooleanValue(n > m),
                    (_, Token::Greater, _) => return Err(InterpreterError::new(format!("Operator GREATER expects two numbers, got {:?} and {:?}", &l, &r))),

                    (ComputedValue::NumberValue(n), Token::GreaterEqual, ComputedValue::NumberValue(m)) => ComputedValue::BooleanValue(n >= m),
                    (_, Token::GreaterEqual, _) => return Err(InterpreterError::new(format!("Operator GREATEREQUAL expects two numbers, got {:?} and {:?}", &l, &r))),

                    (ComputedValue::NumberValue(n), Token::Less, ComputedValue::NumberValue(m)) => ComputedValue::BooleanValue(n < m),
                    (_, Token::Less, _) => return Err(InterpreterError::new(format!("Operator LESS expects two numbers, got {:?} and {:?}", &l, &r))),

                    (ComputedValue::NumberValue(n), Token::LessEqual, ComputedValue::NumberValue(m)) => ComputedValue::BooleanValue(n <= m),
                    (_, Token::LessEqual, _) => return Err(InterpreterError::new(format!("Operator LESSEQUAL expects two numbers, got {:?} and {:?}", &l, &r))),

                    (ComputedValue::NumberValue(n), Token::BangEqual, ComputedValue::NumberValue(m)) => ComputedValue::BooleanValue(n != m),
                    (ComputedValue::StringValue(s1), Token::BangEqual, ComputedValue::StringValue(s2)) => ComputedValue::BooleanValue(s1 != s2),
                    (ComputedValue::BooleanValue(b1), Token::BangEqual, ComputedValue::BooleanValue(b2)) => ComputedValue::BooleanValue(b1 & b2),
                    (ComputedValue::NilValue, Token::BangEqual, ComputedValue::NilValue) => ComputedValue::BooleanValue(false),
                    (_, Token::BangEqual, _) => ComputedValue::BooleanValue(true),

                    (ComputedValue::NumberValue(n), Token::EqualEqual, ComputedValue::NumberValue(m)) => ComputedValue::BooleanValue(n == m),
                    (ComputedValue::StringValue(s1), Token::EqualEqual, ComputedValue::StringValue(s2)) => ComputedValue::BooleanValue(s1 == s2),
                    (ComputedValue::BooleanValue(b1), Token::EqualEqual, ComputedValue::BooleanValue(b2)) => ComputedValue::BooleanValue(b1 == b2),
                    (ComputedValue::NilValue, Token::EqualEqual, ComputedValue::NilValue) => ComputedValue::BooleanValue(true),
                    (_, Token::EqualEqual, _) => ComputedValue::BooleanValue(false),

                    _ => return Err(InterpreterError::new("not recognized"))
                }
            },
            Expr::PrintStmt(inner) => {
                let res = self.interpret(inner)?;
                println!("{:?}", res);
                ComputedValue::NilValue
            },
            Expr::ExprStmt(inner) => {
                let res = self.interpret(inner)?;
                res
            },
            Expr::VarDecl { name, initializer } => {
                let v = self.interpret(initializer)?;
                match name {
                    Token::Literal(LiteralTokenType::IdentifierLiteral(s)) => {
                        self.environment.put(&s, v);
                        ComputedValue::NilValue
                    },
                    _ => return Err(InterpreterError::new("var decl requires identifier"))
                }
            },
            Expr::VariableExpr(identifier) => {
                match identifier {
                    Token::Literal(LiteralTokenType::IdentifierLiteral(s)) => {
                        if let Some(v) = self.environment.get(&s) {
                            v.clone()
                        } else {
                            ComputedValue::NilValue
                        }
                    },
                    _ => return Err(InterpreterError::new("var lookup requires identifier"))
                }
            }
        };
        debug!("Intermediate {:?}", v);
        Ok(v)

    }

}


#[derive(Debug, PartialEq, Clone)]
enum ComputedValue {
    BooleanValue(bool),
    NumberValue(f64),
    StringValue(String),
    NilValue
}

fn read_line_from_stdin(stdin: &std::io::Stdin) -> std::io::Result<String> {
    let mut handle = stdin.lock();
    let mut buffer = String::new();

    handle.read_line(&mut buffer)?;

    let input = buffer.trim();
    Ok(input.to_string())
}


#[derive(Debug)]
struct InterpreterError {
    msg: String
}

impl InterpreterError {
    pub fn new<B : ToString>(msg:B) -> InterpreterError {
        InterpreterError { msg: msg.to_string()}
    }
}
impl Error for InterpreterError {}
impl Display for InterpreterError {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        write!(f, "{}", self.msg)?;
        Ok(())
    }
}