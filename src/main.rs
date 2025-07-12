use crate::parser::parse;

mod tokenizer;
mod parser;
mod syscalls;

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
    println!("Tokens: {:?}\n", tokens);

    let ast_nodes = parse(tokens);
    println!("AST Nodes: {:#?}", ast_nodes);
}
