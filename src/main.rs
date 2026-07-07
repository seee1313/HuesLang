mod ast;
mod codegen;
mod parser;
mod token;
use crate::codegen::CodeGen;
use clap::Parser;
use inkwell::context::Context;
use logos::Logos;
use std::fs;
use std::process::Command;

#[derive(Parser)]
#[command(name = "huec", about = "HuesLang Compiler 0.1 Version")]
struct Cli {
    #[arg(short = 'c')]
    compile_only: bool,
    source_file: String,
}

fn main() {
    let args = Cli::parse();
    let source_file = &args.source_file;
    let read_file = fs::read_to_string(source_file).expect("File Not Found...");
    let tokens: Vec<crate::token::Token> = crate::token::Token::lexer(&read_file)
        .filter_map(|t| t.ok())
        .collect();
    let parsed_ast = crate::parser::parse_manager(tokens);
    let mut path = std::path::PathBuf::from(source_file);
    path.set_extension("ll");

    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "HuesLang");
    codegen.generate_program(parsed_ast.expect("..."));
    let ir = codegen.get_ir();
    fs::write(&path, ir).expect("can't create ll file");

    let status = Command::new("clang")
        .arg(&path)
        .status()
        .expect("clang tot found");

    if status.success() {
        println!("Compile Succes!");
    } else {
        println!("Compile Err...");
    }
}
