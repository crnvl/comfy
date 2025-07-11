use crate::tokenizer::Token;

#[derive(Debug)]
pub enum AstNode {
    Program(Vec<AstNode>),
    FunctionDefinition(
        String,       // Function name
        Vec<String>,  // Parameters
        Vec<AstNode>, // Body
    ),

    // syscall wrappers
    Write(usize, String, usize),
}

pub fn parse(tokens: Vec<Token>) -> AstNode {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

struct Parser {
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

    fn current_token(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn parse_statement(&mut self) -> AstNode {
        match self.current_token() {
            Token::Function => self.parse_function_definition(),
            Token::Syscall => 
            _ => {
                panic!("Expected a statement, found: {:?}", self.current_token())
            }
        }
    }

    fn parse_function_definition(&mut self) -> AstNode {
        self.consume(Token::Function);

        let identifier = self.consume_identifier();

        self.consume(Token::ParentOpen);

        let mut parameters = Vec::new();
        while self.current_token() != Token::ParentClose {
            if let Token::Identifier(ref id) = self.current_token() {
                parameters.push(id.clone());
                self.current += 1; // Consume identifier
                continue;
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

    fn parse_syscall(&mut self) -> AstNode {
        let identifier = self.consume_identifier();

        self.consume(Token::ParentOpen);

        let mut args = Vec::new();
        while self.current_token() != Token::ParentClose {
            if let Token::Identifier(ref id) = self.current_token() {
                args.push(id.clone());
            } else if let Token::Number(num) = self.current_token() {
                args.push(num.to_string());
            } else {
                panic!("Expected identifier or number, found: {:?}", self.current_token());
            }
            self.current += 1;
        }

        self.consume(Token::ParentClose);
    }

    fn consume(&mut self, token: Token) -> Token {
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

    fn consume_identifier(&mut self) -> String {
        if let Token::Identifier(ref id) = self.current_token() {
            self.current += 1;
            id.clone()
        } else {
            panic!("Expected identifier, found {:?}", self.current_token());
        }
    }
}
