use std::path::Path;

use crate::{generator::generate, parser::parse, tokenizer::tokenize};

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
    let verbose = args.get(2).map_or(false, |arg| arg == "--verbose");

    let script = match std::fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path, e);
            std::process::exit(1);
        }
    };

    let tokens = tokenize(&script);
    if verbose {
        println!("Tokens: {:?}", tokens);
    }
    let ast_nodes = parse(tokens);
    if verbose {
        println!("AST Nodes: {:?}", ast_nodes);
    }

    let generator = generate(&ast_nodes);

    let assembly_code = utils::generate_assembly(generator.rodata, generator.bss, generator.text);
    if verbose {
        println!("Generated Assembly Code:\n{}", assembly_code);
    }

    let input_path = Path::new(file_path);
    let file_stem = input_path.file_stem().unwrap_or_default().to_string_lossy();
    let build_dir = Path::new("./build");
    if let Err(e) = std::fs::create_dir_all(build_dir) {
        eprintln!("Error creating build directory: {}", e);
        std::process::exit(1);
    }

    let output_file = build_dir.join(format!("{}.s", file_stem));

    match std::fs::write(&output_file, assembly_code) {
        Ok(_) => println!("Assembly code written to {}", output_file.display()),
        Err(e) => {
            eprintln!("Error writing to file {}: {}", output_file, e);
            std::process::exit(1);
        }
    }
}
