use crate::{Token, TokenType};
use snafu::{prelude::*, Whatever};

pub fn tokenize(input: String) -> Result<Vec<Token>, Whatever> {
    let mut c_iter = input.chars().peekable();

    let mut tokens: Vec<Token> = Vec::new();
    let mut buf = String::new();

    while let Some(c) = c_iter.next() {
        match c {
            _ if c.is_alphabetic() => {
                buf.push(c);
                while let Some(next_c) = c_iter.next_if(|&x| x.is_alphanumeric()) {
                    buf.push(next_c);
                }
                match &buf[..] {
                    "exit" => {
                        tokens.push(Token {
                            token_type: TokenType::Exit,
                            value: None,
                        });
                        buf.clear();
                    }
                    _ => {
                        whatever!("no such keyword: {}", &buf)
                    }
                }
            }
            _ if c.is_numeric() => {
                buf.push(c);
                while let Some(next_c) = c_iter.next_if(|&x| x.is_numeric()) {
                    buf.push(next_c);
                }
                tokens.push(Token {
                    token_type: TokenType::IntLit,
                    value: Some(buf.clone()),
                });
                buf.clear();
            }
            ';' => {
                tokens.push(Token {
                    token_type: TokenType::Semi,
                    value: None,
                });
            }
            _ if c.is_whitespace() => continue,
            // _ if c.is_whitespace() => tokens.push(Token {
            //     token_type: TokenType::_Whitespace,
            //     value: None,
            // }),
            _ => {
                whatever!("unrecognized character: {c}")
            }
        }
    }

    return Ok(tokens);
}
