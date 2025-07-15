use crate::{
    backend::syscalls::{parse_sys_exit, parse_sys_write, parse_sys_read, parse_sys_open, parse_sys_sysinfo},
    frontend::tokenizer::Token,
};

#[derive(Debug)]
pub enum AstNode {
    Program(Vec<AstNode>),
    Identifier(String),
    Type(Token),
    FunctionDefinition(String, Vec<AstNode>, Vec<AstNode>),
    VariableDeclaration(String, Box<AstNode>),
    InlineAsm(Vec<String>),

    // syscall wrappers
    Syscall(String, Box<AstNode>),
    Write(Token, Token),
    Read(usize, String),
    Open(String, usize, usize),
    Exit(Token),
    Sysinfo(String),

    // deprecated
    IdentifierWithSize(String, i32),
    Number(i32),
    String(String),
}

pub fn parse(tokens: Vec<Token>) -> AstNode {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn parse(&mut self) -> AstNode {
        let mut statements = Vec::new();

        while self.current_token() != Token::EOF {
            let statement = self.parse_statement();
            statements.push(statement);
        }

        AstNode::Program(statements)
    }

    pub fn current_token(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn parse_function_definition(&mut self) -> AstNode {
        self.consume(Token::Function);

        let identifier = self.consume_identifier();

        self.consume(Token::ParentOpen);

        let mut parameters = Vec::new();
        while self.current_token() != Token::ParentClose {
            let parameter = self.parse_parameter();
            parameters.push(parameter);

            if self.current_token() == Token::Comma {
                self.consume(Token::Comma);
            }
        }
        self.consume(Token::ParentClose);

        self.consume(Token::CurlyOpen);

        let mut body = Vec::new();
        while self.current_token() != Token::CurlyClose {
            let statement = self.parse_statement();
            body.push(statement);
        }
        self.consume(Token::CurlyClose);

        AstNode::FunctionDefinition(identifier, parameters, body)
    }

    fn parse_syscall(&mut self, syscall: String) -> AstNode {
        let matched_syscall = match syscall.as_str() {
            "write" => parse_sys_write(self),
            "read" => parse_sys_read(self),
            "exit" => parse_sys_exit(self),
            "open" => parse_sys_open(self),
            "sysinfo" => parse_sys_sysinfo(self),
            _ => {
                panic!("Unknown syscall: {}", syscall);
            }
        };

        AstNode::Syscall(syscall, Box::new(matched_syscall))
    }

    fn parse_variable_declaration(&mut self) -> AstNode {
        let var_type = self.current_token();

        let identifier = self.consume_identifier();

        self.consume(Token::Equals);

        let value: AstNode = match self.current_token() {
            Token::Bool(_)
            | Token::Char(_)
            | Token::Int8(_)
            | Token::Int16(_)
            | Token::Int32(_)
            | Token::Str(_) => self.parse_datatype(),
            Token::Syscall(sys) => self.parse_syscall(sys),
            _ => panic!(
                "Unsupported value in variable declaration: {:?}",
                self.current_token()
            ),
        };

        self.consume(Token::Semicolon);

        AstNode::VariableDeclaration(identifier, Box::new(value))
    }

    fn parse_statement(&mut self) -> AstNode {
        let ast_node = match self.current_token() {
            Token::Function => self.parse_function_definition(),
            Token::Syscall(syscall) => {
                let node = self.parse_syscall(syscall);

                self.consume(Token::Semicolon);
                node
            }
            Token::TypeDefBool
            | Token::TypeDefChar
            | Token::TypeDefStr
            | Token::TypeDefInt8
            | Token::TypeDefInt16
            | Token::TypeDefInt32 => self.parse_variable_declaration(),
            Token::InlineAsm => self.parse_inline_asm(),
            _ => {
                panic!("Expected a statement, found: {:?}", self.current_token())
            }
        };

        ast_node
    }

    fn parse_datatype(&mut self) -> AstNode {
        match self.current_token() {
            Token::Bool(value) => {
                self.consume(Token::Bool(value));
                AstNode::Type(Token::Bool(value))
            }
            Token::Char(value) => {
                self.consume(Token::Char(value));
                AstNode::Type(Token::Char(value))
            }
            Token::Int8(value) => {
                self.consume(Token::Int8(value));
                AstNode::Type(Token::Int8(value))
            }
            Token::Int16(value) => {
                self.consume(Token::Int16(value));
                AstNode::Type(Token::Int16(value))
            }
            Token::Int32(value) => {
                self.consume(Token::Int32(value));
                AstNode::Type(Token::Int32(value))
            }
            Token::Str(value) => {
                self.consume(Token::Str(value.clone()));
                AstNode::Type(Token::Str(value))
            }
            _ => {
                panic!("Expected a datatype, found: {:?}", self.current_token())
            }
        }
    }

    fn parse_parameter(&mut self) -> AstNode {
        let param_type = self.current_token();

        let identifier = self.consume_identifier();

        if param_type == Token::TypeDefBool
            || param_type == Token::TypeDefChar
            || param_type == Token::TypeDefStr
            || param_type == Token::TypeDefInt8
            || param_type == Token::TypeDefInt16
            || param_type == Token::TypeDefInt32
        {
            self.consume(param_type);
        } else {
            panic!("Expected a type definition for parameter, found: {:?}", param_type);
        }

        AstNode::Identifier(identifier)
    }

    fn parse_inline_asm(&mut self) -> AstNode {
        self.consume(Token::InlineAsm);
        self.consume(Token::CurlyOpen);

        let mut asm_lines = Vec::new();

        while self.current_token() != Token::CurlyClose && self.current_token() != Token::EOF {
            if let Token::Str(ref line) = self.current_token() {
                asm_lines.push(line.clone());
                self.consume(Token::Str(line.clone()));
                if self.current_token() == Token::Comma {
                    self.consume(Token::Comma);
                }
            } else {
                panic!(
                    "Expected string in inline asm block, found: {:?}",
                    self.current_token()
                );
            }
        }

        self.consume(Token::CurlyClose);
        self.consume(Token::Semicolon);

        AstNode::InlineAsm(asm_lines)
    }

    fn consume_identifier(&mut self) -> String {
        if let Token::Identifier(id) = self.current_token() {
            self.consume(Token::Identifier(id.clone()));
            id
        } else {
            panic!("Expected identifier, found: {:?}", self.current_token());
        }
    }

    pub fn consume(&mut self, token: Token) -> Token {
        if self.current_token() == token {
            let consumed = self.tokens[self.current].clone();
            self.current += 1;
            consumed
        } else {
            panic!(
                "Expected token {:?}, found {:?}",
                token,
                self.current_token()
            );
        }
    }
}
