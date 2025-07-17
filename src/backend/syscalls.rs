use crate::{
    frontend::parser::{AstNode, Parser},
    frontend::tokenizer::Token,
};

pub fn parse_sys_write(parser: &mut Parser) -> AstNode {
    parser.consume(Token::Syscall("write".to_string()));

    parser.consume(Token::ParentOpen);

    let fd = match parser.current_token() {
        Token::Int32Container(n) => {
            parser.consume(Token::Int32Container(n));
            Token::Int32Container(n)
        }
        Token::Identifier(id) => {
            parser.consume(Token::Identifier(id.clone()));
            Token::Identifier(id)
        }
        _ => panic!("Expected file descriptor (number)"),
    };

    parser.consume(Token::Comma);

    let write_data = match parser.current_token() {
        Token::StrContainer(s) => {
            parser.consume(Token::StrContainer(s.clone()));
            Token::StrContainer(s)
        }
        Token::Identifier(id) => {
            parser.consume(Token::Identifier(id.clone()));
            Token::Identifier(id)
        }
        _ => panic!("Expected write data (string or identifier)"),
    };

    parser.consume(Token::ParentClose);

    AstNode::Write(fd, write_data)
}

pub fn parse_sys_read(parser: &mut Parser) -> AstNode {
    parser.consume(Token::Syscall("read".to_string()));

    parser.consume(Token::ParentOpen);

    let fd = match parser.current_token() {
        Token::Int32Container(n) => n,
        _ => panic!("Expected file descriptor (number)"),
    };
    parser.consume(Token::Int32Container(fd));

    parser.consume(Token::Comma);

    let buffer = match parser.current_token() {
        Token::Identifier(id) => id,
        _ => panic!("Expected buffer identifier"),
    };

    parser.consume(Token::Identifier(buffer.clone()));
    parser.consume(Token::ParentClose);

    AstNode::Read(fd as usize, buffer)
}

pub fn parse_sys_exit(parser: &mut Parser) -> AstNode {
    parser.consume(Token::Syscall("exit".to_string()));

    parser.consume(Token::ParentOpen);

    let code = match parser.current_token() {
        Token::Int32Container(n) => Token::Int32Container(n),
        Token::Identifier(id) => Token::Identifier(id),
        _ => panic!("Expected exit code (number or identifier)"),
    };
    parser.consume(code.clone());

    parser.consume(Token::ParentClose);

    AstNode::Exit(code)
}

pub fn parse_sys_open(parser: &mut Parser) -> AstNode {
    parser.consume(Token::Syscall("open".to_string()));

    parser.consume(Token::ParentOpen);

    let filename = match parser.current_token() {
        Token::StrContainer(s) => s,
        _ => panic!("Expected filename (string)"),
    };
    parser.consume(Token::StrContainer(filename.clone()));

    parser.consume(Token::Comma);

    let flags = match parser.current_token() {
        Token::Int32Container(n) => n,
        _ => panic!("Expected flags (number)"),
    };
    parser.consume(Token::Int32Container(flags));

    parser.consume(Token::Comma);

    let mode = match parser.current_token() {
        Token::Int32Container(n) => n,
        _ => panic!("Expected mode (number)"),
    };
    parser.consume(Token::Int32Container(mode));

    parser.consume(Token::ParentClose);

    AstNode::Open(filename, flags as usize, mode as usize)
}

pub fn parse_sys_sysinfo(parser: &mut Parser) -> AstNode {
    parser.consume(Token::Syscall("sysinfo".to_string()));

    parser.consume(Token::ParentOpen);

    let buffer = match parser.current_token() {
        Token::Identifier(id) => id,
        _ => panic!("Expected buffer identifier"),
    };

    parser.consume(Token::Identifier(buffer.clone()));
    parser.consume(Token::ParentClose);

    AstNode::Sysinfo(buffer)
}
