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

    let script = match std::fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path, e);
            std::process::exit(1);
        }
    };

    let tokens = tokenize(&script);
    let ast_nodes = parse(tokens);
    let generator = generate(&ast_nodes);

    let assembly_code = utils::generate_assembly(generator.rodata, generator.bss, generator.text);

    let output_file = "output.s";
    match std::fs::write(output_file, assembly_code) {
        Ok(_) => println!("Assembly code written to {}", output_file),
        Err(e) => {
            eprintln!("Error writing to file {}: {}", output_file, e);
            std::process::exit(1);
        }
    }
}
