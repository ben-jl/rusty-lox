extern crate rlox_contract;
extern crate rlox_scanner;
extern crate rlox_parser;
use log::{error, debug};
use rlox_contract::Token;
use std::collections::VecDeque;
use rlox_contract::{Expr,ExprLiteralValue};
use std::io::Write;
use std::io::BufRead;
use rlox_scanner::Scanner;
use rlox_parser::Parser;
use rlox_parser::ast_printer::print;

pub struct Interpreter {
    scanner : Scanner,
    parser : Parser
}

impl Interpreter {
    pub fn default() -> Interpreter {
        let scanner = Scanner::new();
        let parser = Parser::new();
        Interpreter { scanner, parser }
    }

    pub fn execute_source<B>(&mut self, source: B) -> std::io::Result<()> where B : ToString {
        let sres = self.scanner.scan(&source.to_string()).expect("scanner failed");
        self.parser.add_tokens(*sres);
        let pres = self.parser.parse().unwrap();
        print!("\n");
        println!("{}", print(&pres));
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
                    self.parser.add_tokens(*ts);
                    let pres = self.parser.parse();
                    match pres {
                        Ok(expr) => { 
                            println!("{}", print(&expr));
                            print!("\n");
                            match self.execute(expr) {
                                Err(e) => println!("{:?}", e),
                                _ => println!("OK.")
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

    fn execute(&self, expr: Expr) -> std::io::Result<()> {
        let mut calc_stack = VecDeque::new();
        let mut literals = VecDeque::new();
        calc_stack.push_front(IntermediateValue::Expression(expr));

        while let Some(e) = calc_stack.pop_front() {
            debug!("intermediate {:?}", calc_stack);
            debug!("literals     {:?}", literals);
            match e {
                IntermediateValue::Operator(t) => {
                    match t {
                        Token::Minus => {
                            let l = literals.pop_front();
                            let r = literals.pop_front();
                            match (&l,&r)  {
                                (Some(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(nl))), Some(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(nr)))) => {
                                    literals.push_front(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(nl - nr)));

                                },
                                (_,_) => {
                                    println!("Expected numbers, found {:?} and {:?}", l, r);
                                }
                            }

                        },
                        Token::Plus => {
                            let l = literals.pop_front();
                            let r = literals.pop_front();
                            match (&l,&r)  {
                                (Some(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(nl))), Some(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(nr)))) => {
                                    literals.push_front(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(nl + nr)));

                                },
                                (_,_) => {
                                    println!("Expected numbers, found {:?} and {:?}", l, r);
                                }
                            }

                        },
                        Token::Greater => {
                            let l = literals.pop_front();
                            let r = literals.pop_front();
                            match (&l,&r)  {
                                (Some(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(nl))), Some(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(nr)))) => {
                                    if nl > nr {
                                        literals.push_front(Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(true)));
                                    } else {
                                        literals.push_front(Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(false)));
                                    }
                                },
                                (_,_) => {
                                    println!("Expected numbers, found {:?} and {:?}", l, r);
                                }
                            }
                        },
                        Token::GreaterEqual => {
                            let l = literals.pop_front();
                            let r = literals.pop_front();
                            match (&l,&r)  {
                                (Some(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(nl))), Some(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(nr)))) => {
                                    if nl >= nr {
                                        literals.push_front(Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(true)));
                                    } else {
                                        literals.push_front(Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(false)));
                                    }
                                },
                                (_,_) => {
                                    println!("Expected numbers, found {:?} and {:?}", l, r);
                                }
                            }
                        },
                        Token::LessEqual => {
                            let l = literals.pop_front();
                            let r = literals.pop_front();
                            match (&l,&r)  {
                                (Some(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(nl))), Some(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(nr)))) => {
                                    if nl <= nr {
                                        literals.push_front(Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(true)));
                                    } else {
                                        literals.push_front(Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(false)));
                                    }
                                },
                                (_,_) => {
                                    println!("Expected numbers, found {:?} and {:?}", l, r);
                                }
                            }
                        },
                        Token::EqualEqual => {
                            let l = literals.pop_front();
                            let r = literals.pop_front();
                            match (&l,&r)  {
                                (Some(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(nl))), Some(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(nr)))) => {
                                    if nl == nr {
                                        literals.push_front(Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(true)));
                                    } else {
                                        literals.push_front(Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(false)));
                                    }
                                },
                                (Some(Expr::LiteralExpr(ExprLiteralValue::StringLiteral(sl))), Some(Expr::LiteralExpr(ExprLiteralValue::StringLiteral(sr)))) => {
                                    if sl == sr {
                                        literals.push_front(Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(true)));
                                    } else {
                                        literals.push_front(Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(false)));
                                    }
                                },
                                (_,_) => {
                                    println!("Expected numbers or strings, found {:?} and {:?}", l, r);
                                }
                            }
                        },
                        Token::BangEqual => {
                            let l = literals.pop_front();
                            let r = literals.pop_front();
                            match (&l,&r)  {
                                (Some(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(nl))), Some(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(nr)))) => {
                                    if nl != nr {
                                        literals.push_front(Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(true)));
                                    } else {
                                        literals.push_front(Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(false)));
                                    }
                                },
                                (Some(Expr::LiteralExpr(ExprLiteralValue::StringLiteral(sl))), Some(Expr::LiteralExpr(ExprLiteralValue::StringLiteral(sr)))) => {
                                    if sl != sr {
                                        literals.push_front(Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(true)));
                                    } else {
                                        literals.push_front(Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(false)));
                                    }
                                },
                                (_,_) => {
                                    println!("Expected numbers or strings, found {:?} and {:?}", l, r);
                                }
                            }
                        },
                        Token::Less => {
                            let l = literals.pop_front();
                            let r = literals.pop_front();
                            match (&l,&r)  {
                                (Some(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(nl))), Some(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(nr)))) => {
                                    if nl < nr {
                                        literals.push_front(Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(true)));
                                    } else {
                                        literals.push_front(Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(false)));
                                    }
                                },
                                (_,_) => {
                                    println!("Expected numbers, found {:?} and {:?}", l, r);
                                }
                            }
                        },
                        Token::Star => {
                            let l = literals.pop_front();
                            let r = literals.pop_front();
                            match (&l,&r) {
                                (Some(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(nl))), Some(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(nr)))) => {
                                    literals.push_front(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(nl * nr)));
                                },
                                (_,_) => {
                                    println!("Expected numbers, found {:?} and {:?}", l, r);
                                }
                            }
                        },
                        Token::Slash => {
                            let l = literals.pop_front();
                            let r = literals.pop_front();
                            match (&l,&r) {
                                (Some(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(nl))), Some(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(nr)))) => {
                                    literals.push_front(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(nl / nr)));
                                },
                                (_,_) => {
                                    println!("Expected numbers, found {:?} and {:?}", l, r);
                                }
                            }
                        }
                        _ => unimplemented!()
                    }
                },
                IntermediateValue::Expression(ex) => {
                    match ex {
                        Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(n)) => {
                            literals.push_front(Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(n)));
                        },
                        Expr::LiteralExpr(ExprLiteralValue::StringLiteral(s)) => {
                            literals.push_front(Expr::LiteralExpr(ExprLiteralValue::StringLiteral(s)));
                        },
                        Expr::BinaryExpr { left, operator, right} => {
                            calc_stack.push_front(IntermediateValue::Operator(operator));
                            calc_stack.push_front(IntermediateValue::Expression(*right));
                            calc_stack.push_front(IntermediateValue::Expression(*left));
                        },
                        Expr::GroupingExpr(inner) => {
                            calc_stack.push_front(IntermediateValue::Expression(*inner));
                        },
                        Expr::UnaryExpr { operator, right } => {
                            calc_stack.push_front(IntermediateValue::Operator(operator));
                            calc_stack.push_front(IntermediateValue::Expression(*right));
                        }
                        _ => panic!("NOT FOUND {:?}", ex)
                    }
                }
            }
        }
        for l in literals.iter() {
            println!("{:?}", l);
        }
        Ok(())
    }
}

#[derive(Debug)]
enum IntermediateValue {
    Operator(Token),
    Expression(Expr)
}

fn read_line_from_stdin(stdin: &std::io::Stdin) -> std::io::Result<String> {
    let mut handle = stdin.lock();
    let mut buffer = String::new();

    handle.read_line(&mut buffer)?;

    let input = buffer.trim();
    Ok(input.to_string())
}

