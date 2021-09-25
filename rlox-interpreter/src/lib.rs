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
use environment::ScopeEnvironment;
mod callable;
use callable::Callable;

pub struct Interpreter {
    scanner : Scanner,
    parser : Parser,
    pub scope: ScopeEnvironment,
}

impl Interpreter {
    pub fn default() -> Interpreter {
        let scanner = Scanner::new();
        let parser = Parser::new();
        let scope = ScopeEnvironment::new_root();
        Interpreter { scanner, parser, scope}
    }

    pub fn with_env(env: ScopeEnvironment) -> Interpreter {
        let scanner = Scanner::new();
        let parser = Parser::new();
        let scope = env;
        Interpreter {scanner, parser, scope }
    }

    pub fn execute_source<B>(&mut self, source: B) -> std::io::Result<()> where B : ToString {
        let sres = self.scanner.scan(&source.to_string());
        if let Ok(tokens) = sres {
            self.parser.add_tokens(*tokens);
            let parse_res = self.parser.parse();
            if let Ok(exprs) = parse_res {
                for e in exprs {
                    let ie = self.interpret(Box::from(e));
                    match &ie {
                        Ok(cv) => if cv == &Expr::LiteralExpr(ExprLiteralValue::NilLiteral) { continue; } else { println!("{:?}", cv)},
                        Err(e) => {return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, e.msg.clone()));}
                    }
                }
            }  else {
                error!("{:?}", parse_res);
            }
        } else {
            error!("{:?}", sres);
        }

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
                    self.parser.add_tokens(ts.to_vec());
                    let pres = self.parser.parse();
                    match pres {
                        Ok(exprs) => { 
                            for expr in exprs {
                                print!("\n");
                                match self.interpret(Box::from(expr)) {
                                    Err(e) => error!("{}", e.msg),
                                    Ok(v) => {
                                        if v == Expr::LiteralExpr(ExprLiteralValue::NilLiteral) { continue;} else {println!("{}",v)};
                                        println!("OK.")
                                    }
                                }
                            }
                        },
                        Err(pe) => { error!("{}", pe);}
                    }
                },
                Err(le) => {
                    error!("{}", le)

                }
            }
        }
        println!("Exiting...");
        Ok(())
    }

    fn interpret(&mut self, expr: Box<Expr>) -> Result<Expr> {

        let v = match *expr {
            Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(b)) => bool_literal(b),
            Expr::LiteralExpr(ExprLiteralValue::NilLiteral) => Expr::LiteralExpr(ExprLiteralValue::NilLiteral),
            Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(n)) => Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(n)),
            Expr::LiteralExpr(ExprLiteralValue::StringLiteral(s)) => Expr::LiteralExpr(ExprLiteralValue::StringLiteral(s.clone())),
            Expr::GroupingExpr(inner) => {
                return self.interpret(inner)
            },
            Expr::UnaryExpr { operator, right} => {
                let r = self.interpret(right)?;
                match (&operator,&r) {
                    (Token::Bang, Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(b))) => bool_literal(!b),
                    (Token::Bang, Expr::LiteralExpr(ExprLiteralValue::NilLiteral)) => bool_literal(true),
                    (Token::Bang, _) => bool_literal(true),
                    (Token::Minus, Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(n))) => num_literal(-1 as f64 * n),
                    (Token::Minus, _) => return Err(InterpreterError::new(format!("Expected number, got {}", r))),
                    _ => return Err(InterpreterError::new(format!("Expected unary operator, got {:?}", operator)))
                }
            },
            Expr::BinaryExpr { left, operator, right } => {
                let l = self.interpret(left)?;
                let r = self.interpret(right)?;

                match (&l, &operator, &r) {
                    (Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(n)), Token::Minus, Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(m))) => num_literal(n - m),
                    (_, Token::Minus, _) => return Err(InterpreterError::new(format!("Operator MINUS expects two numbers, got {:?} and {:?}", &l, &r))),

                    (Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(n)), Token::Plus, Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(m))) => num_literal(n + m),
                    (Expr::LiteralExpr(ExprLiteralValue::StringLiteral(s1)), Token::Plus, Expr::LiteralExpr(ExprLiteralValue::StringLiteral(s2))) => {
                        Expr::LiteralExpr(ExprLiteralValue::StringLiteral(format!(r#"{}{}"#, s1, s2)))
                    },
                    (_, Token::Plus, _) => return Err(InterpreterError::new(format!("Operator PLUS expects two numbers or two strings, got {:?} and {:?}", &l, &r))),

                    (Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(n)), Token::Slash, Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(m))) => if *m == 0 as f64 { return Err(InterpreterError::new("Divide by zero error")); } else { num_literal(n / m)},
                    (_, Token::Slash, _) => return Err(InterpreterError::new(format!("Operator SLASH expects two numbers, got {:?} and {:?}", &l, &r))),

                    (Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(n)), Token::Star, Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(m))) => num_literal(n * m),
                    (_, Token::Star, _) => return Err(InterpreterError::new(format!("Operator STAR expects two numbers, got {:?} and {:?}", &l, &r))),

                    (Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(n)), Token::Greater, Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(m))) => bool_literal(n > m),
                    (_, Token::Greater, _) => return Err(InterpreterError::new(format!("Operator GREATER expects two numbers, got {:?} and {:?}", &l, &r))),

                    (Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(n)), Token::GreaterEqual, Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(m))) => bool_literal(n >= m),
                    (_, Token::GreaterEqual, _) => return Err(InterpreterError::new(format!("Operator GREATEREQUAL expects two numbers, got {:?} and {:?}", &l, &r))),

                    (Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(n)), Token::Less, Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(m))) => bool_literal(n < m),
                    (_, Token::Less, _) => return Err(InterpreterError::new(format!("Operator LESS expects two numbers, got {:?} and {:?}", &l, &r))),

                    (Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(n)), Token::LessEqual, Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(m))) => bool_literal(n <= m),
                    (_, Token::LessEqual, _) => return Err(InterpreterError::new(format!("Operator LESSEQUAL expects two numbers, got {:?} and {:?}", &l, &r))),

                    (Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(n)), Token::BangEqual, Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(m))) => bool_literal(n != m),
                    (Expr::LiteralExpr(ExprLiteralValue::StringLiteral(s1)), Token::BangEqual, Expr::LiteralExpr(ExprLiteralValue::StringLiteral(s2))) => bool_literal(s1 != s2),
                    (Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(b1)), Token::BangEqual, Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(b2))) => bool_literal(b1 & b2),
                    (Expr::LiteralExpr(ExprLiteralValue::NilLiteral), Token::BangEqual, Expr::LiteralExpr(ExprLiteralValue::NilLiteral)) => bool_literal(false),
                    (_, Token::BangEqual, _) => bool_literal(true),

                    (Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(n)), Token::EqualEqual, Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(m))) => bool_literal(n == m),
                    (Expr::LiteralExpr(ExprLiteralValue::StringLiteral(s1)), Token::EqualEqual, Expr::LiteralExpr(ExprLiteralValue::StringLiteral(s2))) => bool_literal(s1 == s2),
                    (Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(b1)), Token::EqualEqual, Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(b2))) => bool_literal(b1 == b2),
                    (Expr::LiteralExpr(ExprLiteralValue::NilLiteral), Token::EqualEqual, Expr::LiteralExpr(ExprLiteralValue::NilLiteral)) => bool_literal(true),
                    (_, Token::EqualEqual, _) => bool_literal(false),

                    _ => return Err(InterpreterError::new("not recognized"))
                }
            },
            Expr::PrintStmt(inner) => {
                let res = self.interpret(inner)?;

                println!("{}", res);
                Expr::LiteralExpr(ExprLiteralValue::NilLiteral)
            },
            Expr::ExprStmt(inner) => {
                let res = self.interpret(inner)?;
                res
            },
            Expr::VarDecl { name, initializer } => {
                //dbg!(&name, &initializer);
                let v = self.interpret(initializer)?;
                match name {
                    Token::Literal(LiteralTokenType::IdentifierLiteral(s)) => {
                        self.scope.declare(&s, Box::from(v))?;
                        Expr::LiteralExpr(ExprLiteralValue::NilLiteral)
                    },
                    _ => return Err(InterpreterError::new("var decl requires identifier"))
                }
            },
            Expr::VariableExpr(identifier) => {
                //dbg!(&identifier);
                match identifier {
                    Token::Literal(LiteralTokenType::IdentifierLiteral(s)) => {
                        if let Some(v) = self.scope.get(&s) {
                            v.clone()
                        } else {
                            Expr::LiteralExpr(ExprLiteralValue::NilLiteral)
                        }
                    },
                    _ => return Err(InterpreterError::new("var lookup requires identifier"))
                }
            },
            Expr::AssigmentExpr { name, value } => {
                //dbg!(&name, &value);
                let v = self.interpret(value)?;
                match name {
                    Token::Literal(LiteralTokenType::IdentifierLiteral(s)) => {
                        
                        self.scope.assign(&s, Box::from(v))?;
                        Expr::LiteralExpr(ExprLiteralValue::NilLiteral)
                    },
                    _ => return Err(InterpreterError::new("expected assignment, got nothing"))
                }
            },
            Expr::BlockStmt(decs) => {
                self.scope.new_child();
                for stmt in decs {
                    //dbg!(&stmt);
                    self.interpret(stmt)?;
                }
                self.scope.set_to_previous();
                Expr::LiteralExpr(ExprLiteralValue::NilLiteral)
            },
            Expr::IfStmt { condition, then_branch, else_branch} => {
                let condition_result = self.interpret(condition)?;
                let _ = match condition_result {
                    Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(false)) => self.interpret(else_branch),
                    Expr::LiteralExpr(ExprLiteralValue::NilLiteral) => self.interpret(else_branch),
                    _ => self.interpret(then_branch)
                }?;
                Expr::LiteralExpr(ExprLiteralValue::NilLiteral)
            },
            Expr::LogicalExpr { left, operator, right } => {
                let l = self.interpret(left)?;
                match operator {
                    Token::And => {
                        match l {
                            Expr::LiteralExpr(ExprLiteralValue::NilLiteral) | Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(false)) => l,
                            _ => {
                                let r = self.interpret(right)?;
                                r
                            }
                        }
                    },
                    Token::Or => {
                        match l {
                            Expr::LiteralExpr(ExprLiteralValue::NilLiteral) | Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(false)) => {
                                let r = self.interpret(right)?;
                                r
                            },
                            _ => l
                        }
                    },
                    _ => return Err(InterpreterError::new("Expected logical operator"))
                }
            },
            Expr::WhileLoop { condition, body } => {
                let mut cond_val = self.interpret(Box::from(condition.clone()))?;
                while cond_val != Expr::LiteralExpr(ExprLiteralValue::NilLiteral) && cond_val != bool_literal(false) {
                    self.interpret(Box::from(body.clone()))?;
                    cond_val = self.interpret(Box::from(condition.clone()))?;
                }

                Expr::LiteralExpr(ExprLiteralValue::NilLiteral)
            },
            Expr::CallExpr { callee, paren: _, arguments } => {
                //dbg!(&callee, &arguments);
                let c = self.interpret(callee)?;
                let mut resolved_args = Vec::new();
                for a in arguments {
                    //dbg!(&a);
                    let result = self.interpret(a)?;
                    resolved_args.push(result);
                }
                if let Expr::FunctionExpr{name, params, body} = c {
                    let f = Expr::FunctionExpr { name,params,body };

                    let v = f.call(self.get_scope(), resolved_args)?;
                    
                    v
                } else {
                    return Err(InterpreterError::new("uncallable expr"))
                }
            },
            Expr::FunctionExpr { name, params, body } => {
                //dbg!(&name, &params, &body);
                if let Token::Literal(LiteralTokenType::IdentifierLiteral(s)) = name.clone() {
                    self.scope.declare(&s, Box::from(Expr::FunctionExpr { name, params, body}))?;
                    Expr::LiteralExpr(ExprLiteralValue::NilLiteral)

                } else {
                    return Err(InterpreterError::new("function name not identifier"))
                }
            },
            Expr::Return(_, val) => {
                //println!("returning ... {}", &self.scope);
                let v = if &Expr::LiteralExpr(ExprLiteralValue::NilLiteral) != val.as_ref() {
                    let res = self.interpret(val)?;
                    res
                } else {
                    Expr::LiteralExpr(ExprLiteralValue::NilLiteral)
                };

                //dbg!(&v);
                return Err(InterpreterError::new_return(Some(v)));
            }
        };
        Ok(v)

    }

    pub fn get_scope(&self) -> &ScopeEnvironment {
        &self.scope
    }
    
}


fn num_literal(n:f64) -> Expr {
    Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(n))
}

fn bool_literal(b:bool) -> Expr {
    Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(b))
}

fn read_line_from_stdin(stdin: &std::io::Stdin) -> std::io::Result<String> {
    let mut handle = stdin.lock();
    let mut buffer = String::new();

    handle.read_line(&mut buffer)?;

    let input = buffer.trim();
    Ok(input.to_string())
}


#[derive(Debug)]
pub struct InterpreterError {
    msg: String,
    returned: Option<Expr>
}

impl InterpreterError {
    pub fn new<B : ToString>(msg:B) -> InterpreterError {
        InterpreterError { msg: msg.to_string(), returned: None}
    }

    pub fn return_val(&self) -> Option<&Expr> {
        self.returned.as_ref()
    }

    pub fn new_return(r:Option<Expr>) -> InterpreterError{
        InterpreterError { msg: "".to_string(), returned: r}
    }
}
impl Error for InterpreterError {}
impl Display for InterpreterError {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        write!(f, "{}", self.msg)?;
        Ok(())
    }
}