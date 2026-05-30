use std::fs;
use std::process::{self, ExitCode, Command};
use lexer::Token;
use logos::Logos;
use std::io::Write;

mod lexer;
mod parser;
mod codegen;

fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().collect();
    let argc = argv.len();

    if argc != 2 {
        eprintln!("Incorrect usage");
        eprintln!("toru <input.tr|input.toru>");
        process::exit(1);
    }

    let contents = fs::read_to_string(&argv[1]).unwrap_or_else(|_| {
        eprintln!("Input file not found: '{}'", &argv[1]);
        process::exit(1);
    });

    let tokens = Token::lexer(&contents)
        .filter_map(|t| t.ok())
        .collect();

    println!("{:?}", tokens);

    let parsed = parser::parse(tokens);
    
    println!("{:?}", parsed);

    let output = codegen::tokens_to_asm(&parsed);

    println!("{}", output);

    let mut file = fs::File::create("out.asm").unwrap_or_else(|_| {
        eprintln!("Failed to create assembly output file");
        process::exit(1);
    });

    file.write_all(output.as_bytes()).unwrap_or_else(|_| {
        eprintln!("Failed to write to asm output file");
        process::exit(1);
    });

    Command::new("nasm")
        .arg("-felf64")
        .arg("out.asm")
        .status()
        .unwrap();

    Command::new("ld")
        .arg("out.o")
        .arg("-otest")
        .status()
        .unwrap();

    ExitCode::from(0)
}