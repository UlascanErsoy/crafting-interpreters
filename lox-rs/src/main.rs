extern crate lox_rs;

use lox_rs::lox;

use std::{env, fs, io};
use std::io::Write;
use lox::scanner::Scanner;



fn run(source: &String) -> Result<(), &'static str> {

    let mut scanner: Scanner = Scanner::new(source.clone());
    
    match scanner.scan_tokens() {
        Ok(tokens) => {
             for token in tokens.iter() {
                println!("{:?}", token);
            }
            Ok(())
        },
        Err(errs) => {
            for err in errs.iter() {
                eprintln!("{:?}", err);
            }
            Err("Execution Error")
        }
    }
     
}

fn run_file(file: &String){
    let contents = fs::read_to_string(file)
                    .expect("Error:: Reading the file");

    run(&contents).expect("Execution Error");
}

fn run_prompt(){
    
    let mut lock = io::stdout().lock();

    loop {
        write!(lock, ">").unwrap();
        io::stdout().flush().unwrap();

        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd)
                    .expect("Error reading stdin");

        run(&cmd).unwrap_or(());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
    }else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}
