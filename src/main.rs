use std::io::Write;
use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let args : Vec<String> = std::env::args().collect();
    let stdout = std::io::stdout();
    let stdin = std::io::stdin();


    if args.len() == 2 {
        let source_file = &args[1];
        let source = std::fs::read_to_string(std::path::Path::new(source_file))?;
        
        write_out(&stdout, &source)?;
        write_out(&stdout, "\n")?;
        
    } else {    
        let mut should_exit = false;

        while !should_exit {
            write_out(&stdout, "rlox] ")?;
            let next = read_line_from_stdin(&stdin)?;
            if next == "quit" {
                write_out(&stdout, "ok...see ya!\n")?;
                should_exit = true;
            } else {
                write_out(&stdout, &next)?;  
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