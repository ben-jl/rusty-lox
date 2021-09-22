extern crate rlox_contract;
extern crate rlox_scanner;
extern crate rlox_parser;

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
        self.scanner.add_source(source.to_string());
        let sres = self.scanner.scan().expect("scanner failed");
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
            self.scanner.add_source(nxt);
            let sres = self.scanner.scan();
            match sres {
                Ok(ts) => {
                    self.parser.add_tokens(*ts);
                    let pres = self.parser.parse();
                    match pres {
                        Ok(expr) => { 
                            println!("{}", print(&expr));
                            println!("OK.");
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
}

fn read_line_from_stdin(stdin: &std::io::Stdin) -> std::io::Result<String> {
    let mut handle = stdin.lock();
    let mut buffer = String::new();

    handle.read_line(&mut buffer)?;

    let input = buffer.trim();
    Ok(input.to_string())
}