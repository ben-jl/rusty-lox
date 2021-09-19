use std::io::Write;
use std::io::BufRead;

pub mod rlox_core;
use rlox_core::scan;

fn main() -> std::io::Result<()> {
    
    let args : Vec<String> = std::env::args().collect();
    let stdout = std::io::stdout();
    let stdin = std::io::stdin();


    if args.len() == 2 {
        let source_file = &args[1];
        let source = std::fs::read_to_string(std::path::Path::new(source_file))?;
        let toks = scan(&source);
        let mut tokens = Vec::new();
        for r in toks {
            tokens.push(r.clone().unwrap().clone());
            let ftok = format!("{:?}\n", r);
            println!("{:?}", ftok);
            //write_out(&stdout, &ftok)?;
        }
        let r = rlox_core::parse_expr(&tokens);
        if let Ok(e) = r {
            let r = rlox_core::print_ast_grouped(&e);
            //write_out(&stdout, &r)?;
            println!("{:?}", &r);
            rlox_core::run(e)?;
            
        } else {
            r.unwrap();
            //write_out(&stdout, &format!("{:?}\n", r))?;
        }
                
        //write_out(&stdout, "\n")?;
        
    } else {    
        let mut should_exit = false;

        while !should_exit {
            write_out(&stdout, "rlox] ")?;
            let next = read_line_from_stdin(&stdin)?;
            if next == "quit" {
                write_out(&stdout, "ok...see ya!\n")?;
                should_exit = true;
            } else {
                let results = scan(&next);
                let mut tokens = Vec::new();
                for r in results {
                    tokens.push(r.clone().unwrap().clone());
                    let ftok = format!("{:?}\n", r);
                    write_out(&stdout, &ftok)?;
                }

                let e = rlox_core::parse_expr(&tokens).unwrap();
                let r = rlox_core::print_ast_grouped(&e);
                write_out(&stdout, &r)?;
                write_out(&stdout, "\n")?;
                rlox_core::run(e)?;
                write_out(&stdout, "OK.")?;  
                write_out(&stdout, "\n")?;
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

fn write_out(stdout: &std::io::Stdout, content: &str) -> std::io::Result<()> {
    let bytes = content.as_bytes();
    let mut lock = stdout.lock();
    lock.write(bytes)?;
    lock.flush()?;
    Ok(())
}