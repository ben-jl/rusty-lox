extern crate rlox_contract;
extern crate rlox_scanner;
extern crate rlox_parser;


use rlox_interpreter::Interpreter;
use clap::{App,};
use simplelog::{TermLogger,LevelFilter,Config,TerminalMode,ColorChoice};

fn main() -> std::io::Result<()> {
    let matches = App::new("rlox")
                        .version("0.1")
                        .args_from_usage(
                            "[FILE]         'the input file to use (if none, enter REPL)'
                             -d --debug     'if true, will print intermediate parse trees'
                            "
                        )
                        .get_matches();
    
    if 0 == matches.occurrences_of("debug") {
        TermLogger::init(LevelFilter::Error, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).expect("Unable to construct logger");
    } else {
        TermLogger::init(LevelFilter::Trace, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).expect("Unable to construct logger");
    };
    let mut interpreter = Interpreter::default();      
    let stdin = std::io::stdin();
    if let Some(f) = matches.value_of("FILE") {
        let source = std::fs::read_to_string(std::path::Path::new(f))?;
        interpreter.execute_source(source)?;

    } else {
        let mut stdout = std::io::stdout();
        interpreter.start_repl(&stdin, &mut stdout)?;

    }

    Ok(())
}


