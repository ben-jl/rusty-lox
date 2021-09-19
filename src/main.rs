use std::io::Write;
use std::io::BufRead;
use std::vec::{Vec, IntoIter};
pub mod rlox_core;
use rlox_core::scan;
use rlox_core::parse_expr;
use rlox_core::print_ast_grouped;

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
    let scanned = scan(&source);

        let mut has_lex_error = false;
        let mut tokens = Vec::new();
        for s in scanned.iter() {
            if let Ok(sp) = s {
                println!("{:?}", &sp);
                tokens.push(sp.clone());
                
            } else {
                has_lex_error = true;
                println!("{:?}", s);
            }
        }

        print!("\n");        
        if !has_lex_error {
            let parsed = parse_expr(&tokens[..]);
            let mut has_parse_err = false;
            if let Ok(expr) = parsed {
                println!(r#"{}"#, print_ast_grouped(&expr));
            } else {
                has_parse_err = true;
                println!("{:?}", parsed);
            }
            
        }

        Ok(())
}