use crate::{parser::AstNode, tokenizer::Token, utils::generate_str_varname};

pub struct Generator {
    pub rodata: Vec<String>,
    pub bss: Vec<String>,
    pub text: Vec<String>,

    last_fun_name: String,
}

impl Generator {
    pub fn new() -> Self {
        let mut self_data = Self {
            rodata: Vec::new(),
            bss: Vec::new(),
            text: Vec::new(),
            last_fun_name: String::new(),
        };

        self_data.text.push(".global _start".to_string());

        self_data
    }

    pub fn generate(&mut self, ast: &AstNode) {
        match ast {
            AstNode::Program(statements) => {
                for statement in statements.iter() {
                    self.generate(statement);
                }
            }
            AstNode::FunctionDefinition(name, params, body) => {
                let fun_name: String;

                if name == "main" {
                    fun_name = "_start".to_string();
                } else {
                    fun_name = name.clone();

                    for param in params.iter() {
                        if let AstNode::Identifier(param_name, size) = param {
                            let unique_param_name = format!("{}_{}", fun_name, param_name);

                            self.bss
                                .push(format!(".lcomm {}, {}", unique_param_name, size));
                        } else {
                            panic!("Expected identifier for function parameter");
                        }
                    }
                }

                self.last_fun_name = fun_name.clone();
                self.text.push(format!("{}:", fun_name));

                for body_statement in body.iter() {
                    self.generate(body_statement);
                }
            }
            AstNode::VariableDeclaration(id, value) => {
                match &**value {
                    AstNode::Number(n) => {
                        self.rodata
                            .push(format!("{}_{}: .word {}", self.last_fun_name, id, n));
                    }
                    AstNode::String(s) => {
                        let unique_name = format!("{}_{}", self.last_fun_name, id);
                        self.rodata
                            .push(format!("{}: .asciz \"{}\"", unique_name, s));
                        self.rodata
                            .push(format!("{}_len = .-{}", unique_name, unique_name));
                    }
                    _ => panic!("Unsupported variable type for declaration"),
                };
            }
            AstNode::Exit(code) => {
                let syscall_number = 1;

                match code {
                    Token::Number(n) => {
                        self.text.push(format!(
                            "\tmov r7, #{}\n\tmov r0, #{}\n\tsvc #0\n",
                            syscall_number, n
                        ));
                    }
                    Token::Identifier(id) => {
                        self.text.push(format!(
                            "\tmov r7, #{}\n\tldr r0, ={}\n\tsvc #0\n",
                            syscall_number, id
                        ));
                    }
                    _ => panic!("Unsupported exit code type"),
                }
            }
            AstNode::Write(fd, write_data) => {
                let syscall_number = 4;
                let fd_str = fd.to_string();

                let string_varname = generate_str_varname();

                match write_data {
                    Token::String(string) => {
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
                    Token::Identifier(id) => {
                        let unique_id = format!("{}_{}", self.last_fun_name, id);

                        let length_str = format!("{}_len", unique_id);
                        self.text.push(format!(
                            "\tmov r7, #{}\n\tmov r0, #{}\n\tldr r1, ={}\n\tldr r2, ={}\n\tsvc #0\n",
                            syscall_number, fd_str, unique_id, length_str
                        ));
                    }
                    _ => {
                        panic!("Unsupported write data type: {:?}", write_data);
                    }
                }
            }
            _ => {
                panic!("Unsupported AST node type for code generation");
            }
        }
    }
}
