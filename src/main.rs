use crate::{generator::Generator, parser::parse};

mod generator;
mod parser;
mod syscalls;
mod tokenizer;
mod utils;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    let script = match std::fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path, e);
            std::process::exit(1);
        }
    };

    let tokens = tokenizer::tokenize(&script);
    println!("Tokens: {:#?}\n", tokens);

    let ast_nodes = parse(tokens);
    println!("AST Nodes: {:#?}\n", ast_nodes);

    let mut generator = Generator::new();
    let assembly_code = generator.generate(&ast_nodes);

    println!("Generated Assembly Code:\n{}", assembly_code);

    // write to file
    let output_file = "output.s";
    match std::fs::write(output_file, assembly_code) {
        Ok(_) => println!("Assembly code written to {}", output_file),
        Err(e) => {
            eprintln!("Error writing to file {}: {}", output_file, e);
            std::process::exit(1);
        }
    }
}
