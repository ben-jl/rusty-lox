mod scanner;

fn main() -> std::io::Result<()> {
    let source = "{()*-;+".to_string();
    let mut scanner = scanner::Scanner::from_raw_source_string(source);
    let toks = scanner.scan_tokens()?;
    for t in toks.iter() {
        println!("{:?}", t);
    }

    Ok(())
}
