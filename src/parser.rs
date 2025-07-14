use crate::{
    syscalls::{sys_exit, sys_write, sys_read},
    tokenizer::Token,
};

#[derive(Debug)]
pub enum AstNode {
    Program(Vec<AstNode>),
    Number(i32),
    String(String),
    Identifier(String, i32),
    FunctionDefinition(String, Vec<AstNode>, Vec<AstNode>),
    VariableDeclaration(String, Box<AstNode>),

    // syscall wrappers
    Write(usize, Token),
    Read(usize, String),
    Exit(Token),
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
        match syscall.as_str() {
            "write" => sys_write(self),
            "read" => sys_read(self),
            "exit" => sys_exit(self),
            _ => {
                panic!("Unknown syscall: {}", syscall);
            }
        }
    }

    fn parse_variable_declaration(&mut self) -> AstNode {
        self.consume(Token::Let);

        let identifier = self.consume_identifier();

        self.consume(Token::Equals);

        let value = match self.current_token() {
            Token::Number(_) | Token::String(_) => self.parse_datatype(),
            Token::Syscall(sys) => self.parse_syscall(sys),
            _ => panic!("Unsupported value in variable declaration: {:?}", self.current_token()),
        };

        self.consume(Token::Semicolon);

        AstNode::VariableDeclaration(identifier, Box::new(value))
    }

    fn parse_buffer_declaration(&mut self) -> AstNode {
        self.consume(Token::Buf);
        self.consume(Token::BracketOpen);

        let bufsize_token = self.current_token();
        let size = if let Token::Number(n) = bufsize_token {
            self.consume(bufsize_token.clone());
            n
        } else {
            panic!(
                "Expected a number for buffer size, found: {:?}",
                bufsize_token
            );
        };

        self.consume(Token::BracketClose);

        let identifier = self.consume_identifier();

        self.consume(Token::Semicolon);

        AstNode::Identifier(identifier, size)
    }


    fn parse_statement(&mut self) -> AstNode {
        let ast_node = match self.current_token() {
            Token::Function => self.parse_function_definition(),
            Token::Syscall(syscall) => { 
                let node = self.parse_syscall(syscall);
                
                self.consume(Token::Semicolon);
                node
            },
            Token::Let => self.parse_variable_declaration(),
            Token::Buf => self.parse_buffer_declaration(),
            _ => {
                panic!("Expected a statement, found: {:?}", self.current_token())
            }
        };

        ast_node
    }

    fn parse_datatype(&mut self) -> AstNode {
        match self.current_token() {
            Token::Number(number) => {
                self.consume(Token::Number(number));
                AstNode::Number(number)
            }
            Token::String(string) => {
                self.consume(Token::String(string.clone()));
                AstNode::String(string.clone())
            }
            _ => {
                panic!("Expected a datatype, found: {:?}", self.current_token())
            }
        }
    }

    fn parse_parameter(&mut self) -> AstNode {
        match self.current_token() {
            Token::Identifier(_) => self.consume_sized_identifier(),
            _ => {
                panic!(
                    "Expected a parameter identifier, found: {:?}",
                    self.current_token()
                )
            }
        }
    }

    fn consume_sized_identifier(&mut self) -> AstNode {
        let identifier = self.consume_identifier();

        self.consume(Token::Colon);

        let size = if let Token::Number(size) = self.current_token() {
            self.consume(Token::Number(size));
            size
        } else {
            panic!(
                "Expected size after identifier, found: {:?}",
                self.current_token()
            );
        };

        AstNode::Identifier(identifier, size)
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
