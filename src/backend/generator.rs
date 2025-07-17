use crate::{
    backend::arm32::{
        asm::{
            load_syscall_return_value_into_label, store_syscall_return_value, syscall_1arg,
            syscall_2args, syscall_3args,
        },
        section,
        syscall_mapper::{Architecture, get_syscall_num_or_panic},
    },
    extra::utils::generate_str_varname,
    frontend::parser::AstNode,
    frontend::tokenizer::Token,
};

pub struct Generator {
    pub section_writer: section::SectionWriter,
    last_fun_name: String,
    arch: Architecture,
}

pub fn generate(ast_nodes: &AstNode, arch: Architecture) -> Generator {
    let mut generator = Generator::new(arch);
    generator.generate(ast_nodes);
    generator
}

impl Generator {
    pub fn new(arch: Architecture) -> Self {
        let self_data = Self {
            section_writer: section::SectionWriter::new(),
            last_fun_name: String::new(),
            arch,
        };

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
                self.section_writer.push_text(format!("{}:", fun_name));

                // Declare params in .bss
                for param in params {
                    if let AstNode::IdentifierWithSize(param_name, size) = param {
                        self.section_writer
                            .declare_bss_with_name_prefix(&fun_name, param_name, *size);
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
                        self.section_writer.push_rodata_word(&label, *n);
                    }
                    AstNode::String(s) => {
                        self.section_writer.push_rodata_str_with_len(&label, s);
                    }
                    AstNode::Syscall(_, _) => {
                        self.section_writer.declare_bss(&label, 4);
                        self.generate(value);
                        load_syscall_return_value_into_label(&mut self.section_writer.text, &label);
                    }
                    _ => panic!("Unsupported variable declaration value: {:?}", value),
                }
            }

            AstNode::IdentifierWithSize(name, size) => {
                let label = format!("{}_{}", self.last_fun_name, name);
                self.section_writer.declare_bss_with_len(&label, *size);
            }

            AstNode::InlineAsm(asm_lines) => {
                self.section_writer.push_text("\t@ ===Inline Assembly===");
                for line in asm_lines {
                    self.section_writer.push_text(format!("\t{}", line));
                }
                self.section_writer.push_text("\t@ =====================\n");
            }

            AstNode::Syscall(name, inner) => match name.as_str() {
                "write" => self.generate_write(inner),
                "read" => self.generate_read(inner),
                "open" => self.generate_open(inner),
                "exit" => self.generate_exit(inner),
                "sysinfo" => self.generate_sysinfo(inner),
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

        let syscall_number: u32 = get_syscall_num_or_panic(self.arch, "write");
        let fd_str = match fd {
            Token::Int32Container(n) => n.to_string(),
            Token::Identifier(id) => id.clone(),
            _ => panic!("Unsupported file descriptor type: {:?}", fd),
        };

        match data {
            Token::StrContainer(s) => {
                let var = generate_str_varname();
                self.section_writer.push_rodata_str_with_len(&var, s);

                match fd {
                    Token::Int32Container(n) => {
                        let instr = syscall_3args(
                            syscall_number,
                            &n.to_string(),
                            &var,
                            &format!("{}_len", var),
                        );
                        self.section_writer.push_text(&instr);
                    }
                    Token::Identifier(id) => {
                        let label = format!("{}_{}", self.last_fun_name, id);

                        // !! Exception: Inline asm for simplicity and since we need
                        // to load the value into a register from a label
                        // TODO: Refactor this to use a more generic approach
                        self.section_writer.push_text(format!(
                            "\tmov r7, #{}\n\tldr r0, ={}\n\tldr r0, [r0]\n\tldr r1, ={}\n\tldr r2, ={}_len\n\tsvc #0\n",
                            syscall_number, label, var, var
                        ));
                    }
                    _ => panic!("Unsupported file descriptor type: {:?}", fd),
                }
            }

            Token::Identifier(id) => {
                let label = format!("{}_{}", self.last_fun_name, id);
                let instr =
                    syscall_3args(syscall_number, &fd_str, &label, &format!("{}_len", label));
                self.section_writer.push_text(&instr);
            }

            _ => panic!("Unsupported write token: {:?}", data),
        }
        store_syscall_return_value(&mut self.section_writer.text);
    }

    fn generate_read(&mut self, inner: &AstNode) {
        let (fd, buffer) = match inner {
            AstNode::Read(fd, buffer) => (fd, buffer),
            _ => panic!("Invalid read syscall inner node"),
        };

        let syscall_number = get_syscall_num_or_panic(self.arch, "read");
        let fd_str = fd.to_string();
        let label = format!("{}_{}", self.last_fun_name, buffer);

        let instr = syscall_2args(syscall_number, &fd_str, &label);
        self.section_writer.push_text(&instr);

        store_syscall_return_value(&mut self.section_writer.text);
    }

    fn generate_exit(&mut self, inner: &AstNode) {
        let code = match inner {
            AstNode::Exit(token) => token,
            _ => panic!("Invalid exit syscall inner node"),
        };

        let syscall_number = get_syscall_num_or_panic(self.arch, "exit");

        let asm = match code {
            Token::Int32Container(n) => syscall_1arg(syscall_number, &n.to_string()),
            Token::Identifier(id) => syscall_1arg(syscall_number, &id),
            _ => panic!("Unsupported exit code: {:?}", code),
        };

        self.section_writer.push_text(&asm);
    }

    fn generate_open(&mut self, inner: &AstNode) {
        let (path, flags, mode) = match inner {
            AstNode::Open(path, flags, mode) => (path, flags, mode),
            _ => panic!("Invalid open syscall inner node"),
        };

        let syscall_number = get_syscall_num_or_panic(self.arch, "open");

        let var = generate_str_varname();
        self.section_writer.push_rodata_str_with_len(&var, path);

        let flags_str = flags.to_string();
        let mode_str = mode.to_string();

        let instr = syscall_3args(syscall_number, &var, &flags_str, &mode_str);
        self.section_writer.push_text(&instr);

        store_syscall_return_value(&mut self.section_writer.text);
    }

    fn generate_sysinfo(&mut self, inner: &AstNode) {
        let buffer = match inner {
            AstNode::Sysinfo(buffer) => buffer,
            _ => panic!("Invalid sysinfo syscall inner node"),
        };

        let syscall_number = get_syscall_num_or_panic(self.arch, "sysinfo");
        let label = format!("{}_{}", self.last_fun_name, buffer);

        let instr = syscall_1arg(syscall_number, &label);
        self.section_writer.push_text(&instr);

        store_syscall_return_value(&mut self.section_writer.text);
    }
}
