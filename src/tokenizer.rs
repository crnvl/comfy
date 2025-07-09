#[derive(Debug)]
pub enum Token {
    Number(i32),
    String(String),

    Function,
    Identifier(String),
    ParentOpen,
    ParentClose,
    CurlyOpen,
    CurlyClose,
    Comma,
    Semicolon,

    EOF, // End of File
    Unknown,
}

pub fn tokenize(script: String) -> Vec<Token> {
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
                tokens.push(Token::String(string));
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
                tokens.push(Token::Number(number.parse().unwrap()));
            }

            _ => tokens.push(Token::Unknown),
        }
    }

    tokens.push(Token::EOF);
    tokens
}
