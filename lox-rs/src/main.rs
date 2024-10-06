extern crate lox_rs;

use lox_rs::lox;
use lox_rs::lox::ast::StmtVisitor;

use lox::ast::{Interpreter, Stmt};
use lox::parser::Parser;
use lox::scanner::Scanner;
use std::io::Write;
use std::{env, fs, io};

fn run(source: &String, intp: &mut Interpreter) -> Result<(), &'static str> {
    let mut scanner: Scanner = Scanner::new(source.clone());

    match scanner.scan_tokens() {
        Ok(tokens) => {
            let mut parser: Parser = Parser::new(tokens);
            let stmts: Vec<Stmt> = if let Ok(stmts) = parser.parse() {
                stmts
            } else {
                return Err("Some errors occurred i guess idk");
            };

            for stmt in stmts {
                intp.visit_stmt(&stmt);
            }
            Ok(())
        }
        Err(errs) => {
            for err in errs.iter() {
                eprintln!("{:?}", err);
            }
            Err("Execution Error")
        }
    }
}

fn run_file(file: &String) {
    let contents = fs::read_to_string(file).expect("Error:: Reading the file");
    let mut intp = Interpreter::default();
    run(&contents, &mut intp).expect("Execution Error");
}

fn run_prompt() {
    let mut lock = io::stdout().lock();
    let mut intp = Interpreter::default();

    loop {
        write!(lock, ">").unwrap();
        io::stdout().flush().unwrap();

        let mut cmd = String::new();
        io::stdin()
            .read_line(&mut cmd)
            .expect("Error reading stdin");

        run(&cmd, &mut intp).unwrap_or(());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}
