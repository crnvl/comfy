use crate::{
    parser::AstNode,
    tokenizer::Token,
    utils::{
        generate_str_varname, load_syscall_return_value_into_label, store_syscall_return_value,
    },
};

pub struct Generator {
    pub rodata: Vec<String>,
    pub bss: Vec<String>,
    pub text: Vec<String>,

    last_fun_name: String,
}

pub fn generate(ast_nodes: &AstNode) -> Generator {
    let mut generator = Generator::new();
    generator.generate(ast_nodes);
    generator
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

    fn generate(&mut self, ast: &AstNode) {
        match ast {
            AstNode::Program(statements) => {
                for stmt in statements {
                    self.generate(stmt);
                }
            }

            AstNode::FunctionDefinition(name, params, body) => {
                let fun_name = if name == "main" {
                    "_start".to_string()
                } else {
                    name.clone()
                };

                self.last_fun_name = fun_name.clone();
                self.text.push(format!("{}:", fun_name));

                // Declare params in .bss
                for param in params {
                    if let AstNode::Identifier(param_name, size) = param {
                        self.bss
                            .push(format!(".lcomm {}_{}, {}", fun_name, param_name, size));
                    }
                }

                for stmt in body {
                    self.generate(stmt);
                }
            }

            AstNode::VariableDeclaration(name, value) => {
                let label = format!("{}_{}", self.last_fun_name, name);

                match &**value {
                    AstNode::Number(n) => {
                        self.rodata.push(format!("{}: .word {}", label, n));
                    }
                    AstNode::String(s) => {
                        self.rodata.push(format!("{}: .asciz \"{}\"", label, s));
                        self.rodata.push(format!("{}_len = .-{}", label, label));
                    }
                    AstNode::Syscall(_, _) => {
                        self.bss.push(format!(".lcomm {}, 4", label));
                        self.generate(value);
                        load_syscall_return_value_into_label(&mut self.text, &label);
                    }
                    _ => panic!("Unsupported variable declaration value: {:?}", value),
                }
            }

            AstNode::Identifier(name, size) => {
                let label = format!("{}_{}", self.last_fun_name, name);
                self.bss.push(format!(".lcomm {}, {}", label, size));
                self.bss.push(format!("{}_len = {}", label, size));
            }

            AstNode::Syscall(name, inner) => match name.as_str() {
                "write" => self.generate_write(inner),
                "read" => self.generate_read(inner),
                "exit" => self.generate_exit(inner),
                _ => panic!("Unknown syscall: {}", name),
            },

            _ => panic!("Unsupported AST node in code generation: {:?}", ast),
        }
    }

    fn generate_write(&mut self, inner: &AstNode) {
        let (fd, data) = match inner {
            AstNode::Write(fd, token) => (fd, token),
            _ => panic!("Invalid write syscall inner node"),
        };

        let syscall_number = 4;
        let fd_str = fd.to_string();

        match data {
            Token::String(s) => {
                let var = generate_str_varname();
                self.rodata.push(format!("{}: .asciz \"{}\"", var, s));
                self.rodata.push(format!("{}_len = .-{}", var, var));

                self.text.push(format!(
                    "\tmov r7, #{}\n\tmov r0, #{}\n\tldr r1, ={}\n\tldr r2, ={}_len\n\tsvc #0\n",
                    syscall_number, fd_str, var, var
                ));
            }

            Token::Identifier(id) => {
                let label = format!("{}_{}", self.last_fun_name, id);
                self.text.push(format!(
                    "\tmov r7, #{}\n\tmov r0, #{}\n\tldr r1, ={}\n\tldr r2, ={}_len\n\tsvc #0\n",
                    syscall_number, fd_str, label, label
                ));
            }

            _ => panic!("Unsupported write token: {:?}", data),
        }
    }

    fn generate_read(&mut self, inner: &AstNode) {
        let (fd, buffer) = match inner {
            AstNode::Read(fd, buffer) => (fd, buffer),
            _ => panic!("Invalid read syscall inner node"),
        };

        let syscall_number = 3;
        let fd_str = fd.to_string();
        let label = format!("{}_{}", self.last_fun_name, buffer);

        self.text.push(format!(
            "\tmov r7, #{}\n\tmov r0, #{}\n\tldr r1, ={}\n\tsvc #0\n",
            syscall_number, fd_str, label
        ));
        store_syscall_return_value(&mut self.text);
    }

    fn generate_exit(&mut self, inner: &AstNode) {
        let code = match inner {
            AstNode::Exit(token) => token,
            _ => panic!("Invalid exit syscall inner node"),
        };

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
            _ => panic!("Unsupported exit code: {:?}", code),
        }
    }
}
