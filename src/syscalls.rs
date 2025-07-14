use crate::{
    parser::{AstNode, Parser},
    tokenizer::Token,
};

pub fn parse_sys_write(parser: &mut Parser) -> AstNode {
    parser.consume(Token::Syscall("write".to_string()));

    parser.consume(Token::ParentOpen);

    let fd = match parser.current_token() {
        Token::Number(n) => n,
        _ => panic!("Expected file descriptor (number)"),
    };
    parser.consume(Token::Number(fd));

    parser.consume(Token::Comma);

    let write_data = match parser.current_token() {
        Token::String(s) => Token::String(s),
        Token::Identifier(id) => Token::Identifier(id),
        _ => panic!("Expected write data (string or identifier)"),
    };
    parser.consume(write_data.clone());

    parser.consume(Token::ParentClose);

    AstNode::Write(fd as usize, write_data)
}

pub fn parse_sys_read(parser: &mut Parser) -> AstNode {
    parser.consume(Token::Syscall("read".to_string()));

    parser.consume(Token::ParentOpen);

    let fd = match parser.current_token() {
        Token::Number(n) => n,
        _ => panic!("Expected file descriptor (number)"),
    };
    parser.consume(Token::Number(fd));

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
        Token::Number(n) => Token::Number(n),
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
        Token::String(s) => s,
        _ => panic!("Expected filename (string)"),
    };
    parser.consume(Token::String(filename.clone()));

    parser.consume(Token::Comma);

    let flags = match parser.current_token() {
        Token::Number(n) => n,
        _ => panic!("Expected flags (number)"),
    };
    parser.consume(Token::Number(flags));

    parser.consume(Token::Comma);

    let mode = match parser.current_token() {
        Token::Number(n) => n,
        _ => panic!("Expected mode (number)"),
    };
    parser.consume(Token::Number(mode));

    parser.consume(Token::ParentClose);

    AstNode::Open(filename, flags as usize, mode as usize)
}