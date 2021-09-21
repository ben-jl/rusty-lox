extern crate rlox_contract;
extern crate rlox_scanner;
extern crate rlox_parser;
use std::io::Write;
use std::io::BufRead;
use rlox_contract::{ExprLiteralValue, Expr, Token, TokenContext, LiteralTokenType};
use rlox_scanner::scan;
use rlox_parser::parse;
use rlox_parser::ast_printer::print;

fn main() -> std::io::Result<()> {
    
    let args : Vec<String> = std::env::args().collect();
    let stdin = std::io::stdin();
    if args.len() == 2 {
        let source = std::fs::read_to_string(std::path::Path::new(&args[1]))?;
        run_source_fragment(&source)?;

    } else {
        loop {
            
            print!("rlox] ");
            std::io::stdout().flush()?;
            let input = read_line_from_stdin(&stdin)?;
            if input.trim() == "quit" {
                println!("Exiting...\n");
                break;
            } else {
                run_source_fragment(input.trim())?;
            }
        }

        
    }

    Ok(())
}

fn read_line_from_stdin(stdin: &std::io::Stdin) -> std::io::Result<String> {
    let mut handle = stdin.lock();
    let mut buffer = String::new();

    handle.read_line(&mut buffer)?;

    let input = buffer.trim();
    Ok(input.to_string())
}


fn run_source_fragment(source: &str) -> std::io::Result<()> {
    match scan(source) {
        Ok(s) => {
            let pr = parse(*s);

            match pr {
                Ok(e) => println!("\n{}", print(&e)),
                Err(pe) => println!("{}", pe)
            };
            Ok(())
        },
        Err(l) => {
            println!("LEXICAL ERROR: {}" , l);
            Ok(())
        }
    }
}