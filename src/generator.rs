use std::collections::HashMap;

use crate::{
    parser::AstNode,
    tokenizer::Token,
    utils::{generate_num_varname, generate_str_varname},
};

pub struct Generator {
    rodata: Vec<String>,
    bss: Vec<String>,
    text: Vec<String>,
}

impl Generator {
    pub fn new() -> Self {
        let mut self_data = Self {
            rodata: Vec::new(),
            bss: Vec::new(),
            text: Vec::new(),
        };

        self_data.text.push(".global _start".to_string());

        self_data
    }

    pub fn generate(&mut self, ast: &AstNode) -> String {
        match ast {
            AstNode::Program(statements) => {
                for statement in statements.iter() {
                    self.generate(statement);
                }
            }
            AstNode::FunctionDefinition(name, params, body) => {
                if name == "main" {
                    self.text.push("_start:".to_string());
                } else {
                    self.text.push(format!("_{}:", name));

                    for param in params.iter() {
                        if let AstNode::Identifier(param_name, size) = param {
                            println!("Adding parameter {} with size {}", param_name, size);
                            self.bss
                                .push(format!(".lcomm {}_{}, {}", name, param_name, size));
                        } else {
                            panic!("Expected identifier for function parameter");
                        }
                    }
                }

                for body_statement in body.iter() {
                    self.generate(body_statement);
                }
            }
            AstNode::Exit(code) => {
                let syscall_number = 1;
                self.text.push(format!(
                    "\tmov r7, #{}\n\tmov r0, #{}\n\tsvc #0\n",
                    syscall_number, code
                ));
            }
            AstNode::Write(fd, string) => {
                let syscall_number = 4;
                let fd_str = fd.to_string();

                let string_varname = generate_str_varname();
                self.rodata
                    .push(format!("{}: .asciz \"{}\"", string_varname, string));
                let length_str = format!("{}_len", string_varname);
                self.rodata
                    .push(format!("{} = .-{}", length_str, string_varname));

                self.text.push(format!(
                    "\tmov r7, #{}\n\tmov r0, #{}\n\tldr r1, ={}\n\tldr r2, ={}\n\tsvc #0\n",
                    syscall_number, fd_str, string_varname, length_str
                ));
            }
            _ => {
                panic!("Unsupported AST node type for code generation");
            }
        }

        self.generate_asm()
    }

    pub fn generate_asm(&mut self) -> String {
        let mut assembly_code = String::new();
        assembly_code.push_str("\n.section .rodata\n");
        for rodata_item in self.rodata.iter() {
            assembly_code.push_str("\t");
            assembly_code.push_str(rodata_item.as_str());
            assembly_code.push('\n');
        }

        assembly_code.push_str("\n.section .bss\n");
        for bss_item in self.bss.iter() {
            assembly_code.push_str("\t");
            assembly_code.push_str(bss_item.as_str());
            assembly_code.push('\n');
        }

        assembly_code.push_str("\n.section .text\n");
        for text_item in self.text.iter() {
            assembly_code.push_str(text_item.as_str());
            assembly_code.push('\n');
        }

        assembly_code
    }
}
