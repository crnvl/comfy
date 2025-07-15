use crate::{
    parser::{AstNode, FileDescriptor, Parser},
    tokenizer::Token,
};

pub fn sys_write(parser: &mut Parser) -> AstNode {
    parser.consume(Token::Syscall("write".to_string()));

    parser.consume(Token::ParentOpen);

    let fd = match parser.current_token() {
        Token::Number(n) => {
            parser.consume(Token::Number(n));
            FileDescriptor::Number(n as usize)
        }
        Token::Identifier(id) => {
            parser.consume(Token::Identifier(id.clone()));
            FileDescriptor::Identifier(id)
        }
        _ => panic!("Expected file descriptor (number or identifier)"),
    };

    parser.consume(Token::Comma);

    let string = match parser.current_token() {
        Token::String(s) => s,
        _ => panic!("Expected string argument"),
    };
    parser.consume(Token::String(string.clone()));

    parser.consume(Token::ParentClose);

    parser.consume(Token::Semicolon);

    AstNode::Write(fd, string)
}

pub fn sys_exit(parser: &mut Parser) -> AstNode {
    parser.consume(Token::Syscall("exit".to_string()));

    parser.consume(Token::ParentOpen);

    let code = match parser.current_token() {
        Token::Number(n) => n,
        _ => panic!("Expected exit code (number)"),
    };
    parser.consume(Token::Number(code));

    parser.consume(Token::ParentClose);
    parser.consume(Token::Semicolon);

    AstNode::Exit(code)
}
