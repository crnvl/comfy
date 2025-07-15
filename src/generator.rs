use crate::{parser::{AstNode, FileDescriptor}, utils::generate_str_varname};

pub struct Generator {
    pub rodata: Vec<String>,
    pub bss: Vec<String>,
    pub text: Vec<String>,
    pub last_fun_name: String,
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

    /// Generic argument type for syscalls that can handle different kinds of values
    fn load_arg_to_register(&self, arg: &str, target_reg: &str) -> String {
        if arg.starts_with('#') {
            // Immediate value
            format!("\tmov {}, {}\n", target_reg, arg)
        } else {
            // Label/variable - load from memory
            format!("\tldr {}, ={}\n", target_reg, arg)
        }
    }

    /// Generate code for loading a file descriptor (either immediate value or from variable)
    fn load_fd_to_register(&self, fd: &FileDescriptor, target_reg: &str) -> String {
        match fd {
            FileDescriptor::Number(n) => format!("\tmov {}, #{}\n", target_reg, n),
            FileDescriptor::Identifier(id) => {
                let label = if self.last_fun_name.is_empty() {
                    id.clone()
                } else {
                    format!("{}_{}", self.last_fun_name, id)
                };
                format!("\tldr {}, ={}\n\tldr {}, [{}]\n", target_reg, label, target_reg, target_reg)
            }
        }
    }

    /// Generic syscall wrapper that handles variable file descriptors
    fn syscall_3args_with_fd(&self, syscall_number: i32, fd: &FileDescriptor, arg2: &str, arg3: &str) -> String {
        let mut result = String::new();
        result.push_str(&format!("\tmov r7, #{}\n", syscall_number));
        result.push_str(&self.load_fd_to_register(fd, "r0"));
        result.push_str(&format!("\tldr r1, ={}\n", arg2));
        result.push_str(&format!("\tldr r2, ={}\n", arg3));
        result.push_str("\tsvc #0\n");
        result
    }

    /// Generic syscall wrapper that can handle any 3 arguments
    fn syscall_3args(&self, syscall_number: i32, arg1: &str, arg2: &str, arg3: &str) -> String {
        let mut result = String::new();
        result.push_str(&format!("\tmov r7, #{}\n", syscall_number));
        result.push_str(&self.load_arg_to_register(arg1, "r0"));
        result.push_str(&self.load_arg_to_register(arg2, "r1"));
        result.push_str(&self.load_arg_to_register(arg3, "r2"));
        result.push_str("\tsvc #0\n");
        result
    }

    pub fn generate(&mut self, ast: &AstNode) {
        match ast {
            AstNode::Program(statements) => {
                for statement in statements.iter() {
                    self.generate(statement);
                }
            }
            AstNode::FunctionDefinition(name, params, body) => {
                // Track the current function name for variable resolution
                self.last_fun_name = name.clone();
                
                if name == "main" {
                    self.text.push("_start:".to_string());
                } else {
                    self.text.push(format!("_{}:", name));
                }

                // Process parameters for all functions, including main
                for param in params.iter() {
                    if let AstNode::Identifier(param_name, size) = param {
                        self.bss
                            .push(format!(".lcomm {}_{}, {}", name, param_name, size));
                    } else {
                        panic!("Expected identifier for function parameter");
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

                let string_varname = generate_str_varname();
                self.rodata
                    .push(format!("{}: .asciz \"{}\"", string_varname, string));
                let length_str = format!("{}_len", string_varname);
                self.rodata
                    .push(format!("{} = .-{}", length_str, string_varname));

                // Use the generic syscall wrapper that handles both numeric and variable file descriptors
                let instr = self.syscall_3args_with_fd(syscall_number, fd, &string_varname, &length_str);
                self.text.push(instr);
            }
            _ => {
                panic!("Unsupported AST node type for code generation");
            }
        }
    }
}
