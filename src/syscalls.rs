use crate::{
    parser::{AstNode, Parser},
    tokenizer::Token,
};

pub fn sys_write(parser: &mut Parser) -> AstNode {
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

    parser.consume(Token::Semicolon);

    AstNode::Write(fd as usize, write_data)
}

pub fn sys_exit(parser: &mut Parser) -> AstNode {
    parser.consume(Token::Syscall("exit".to_string()));

    parser.consume(Token::ParentOpen);

    let code = match parser.current_token() {
        Token::Number(n) => Token::Number(n),
        Token::Identifier(id) => Token::Identifier(id),
        _ => panic!("Expected exit code (number)"),
    };
    parser.consume(code.clone());

    parser.consume(Token::ParentClose);
    parser.consume(Token::Semicolon);

    AstNode::Exit(code)
}
