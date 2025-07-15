#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Function,
    Identifier(String),
    Syscall(String),
    ParentOpen,
    ParentClose,
    CurlyOpen,
    CurlyClose,
    Comma,
    Semicolon,
    Colon,
    Equals,

    BracketOpen,
    BracketClose,
    InlineAsm,

    EOF,
    Unknown,

    // comfy base types
    TypeDefBool,
    TypeDefChar,
    TypeDefStr,
    TypeDefInt8,
    TypeDefInt16,
    TypeDefInt32,
    Bool(bool),
    Char(char),
    Str(String),
    Int8(i8),
    Int16(i16),
    Int32(i32),

    // deprecated
    Number(i32),
    String(String),
}

pub fn tokenize(script: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut iter = script.chars().peekable();

    while let Some(ch) = iter.next() {
        match ch {
            ' ' | '\n' | '\t' => continue, // Skip whitespace

            '(' => tokens.push(Token::ParentOpen),
            ')' => tokens.push(Token::ParentClose),
            '{' => tokens.push(Token::CurlyOpen),
            '}' => tokens.push(Token::CurlyClose),
            ',' => tokens.push(Token::Comma),
            ';' => tokens.push(Token::Semicolon),
            ':' => tokens.push(Token::Colon),
            '=' => tokens.push(Token::Equals),
            '[' => tokens.push(Token::BracketOpen),
            ']' => tokens.push(Token::BracketClose),

            '$' => {
                let mut syscall = String::new();
                while let Some(&next_ch) = iter.peek() {
                    if next_ch.is_alphanumeric() || next_ch == '_' {
                        syscall.push(iter.next().unwrap());
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Syscall(syscall));
            }

            '"' => {
                let mut string = String::new();
                while let Some(&next_ch) = iter.peek() {
                    if next_ch == '"' {
                        iter.next(); // Consume the closing quote
                        break;
                    } else {
                        string.push(iter.next().unwrap());
                    }
                }
                tokens.push(Token::Str(string));
            }

            '\'' => {
                if let Some(&next_ch) = iter.peek() {
                    iter.next(); // Consume the character
                    if next_ch == '\'' {
                        tokens.push(Token::Char('\'')); // Empty char
                    } else {
                        tokens.push(Token::Char(next_ch));
                        iter.next(); // Consume the character
                    }
                } else {
                    tokens.push(Token::Unknown); // Unmatched quote
                }
            }

            c if c.is_alphabetic() || c == '_' => {
                let mut identifier = c.to_string();
                while let Some(&next_ch) = iter.peek() {
                    if next_ch.is_alphanumeric() || next_ch == '_' {
                        identifier.push(iter.next().unwrap());
                    } else {
                        break;
                    }
                }

                match identifier.as_str() {
                    "fn" => tokens.push(Token::Function),
                    "asm" => tokens.push(Token::InlineAsm),
                    "true" => tokens.push(Token::Bool(true)),
                    "false" => tokens.push(Token::Bool(false)),
                    "bool" => tokens.push(Token::TypeDefBool),
                    "char" => tokens.push(Token::TypeDefChar),
                    "str" => tokens.push(Token::TypeDefStr),
                    "int8" => tokens.push(Token::TypeDefInt8),
                    "int16" => tokens.push(Token::TypeDefInt16),
                    "int32" => tokens.push(Token::TypeDefInt32),
                    _ => tokens.push(Token::Identifier(identifier)),
                }
            }

            c if c.is_digit(10) => {
                let mut number = c.to_string();
                while let Some(&next_ch) = iter.peek() {
                    if next_ch.is_digit(10) {
                        number.push(iter.next().unwrap());
                    } else {
                        break;
                    }
                }

                let num_value = number.parse::<i32>().unwrap();
                match num_value {
                    n if n >= i8::MIN as i32 && n <= i8::MAX as i32 => {
                        tokens.push(Token::Int8(n as i8))
                    }
                    n if n >= i16::MIN as i32 && n <= i16::MAX as i32 => {
                        tokens.push(Token::Int16(n as i16))
                    }
                    _ => tokens.push(Token::Int32(num_value)),
                }
            }

            _ => tokens.push(Token::Unknown),
        }
    }

    tokens.push(Token::EOF);
    tokens
}
