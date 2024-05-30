use crate::{Token, TokenType};

pub struct Tokenizer {
    pub input: String,
}

impl Tokenizer {
    pub fn tokenize(&self) -> Result<Vec<Token>, &'static str> {
        let mut buffer = String::new();
        let mut tokens: Vec<Token> = Vec::new();

        let mut chars = self.input.chars();
        while let Some(c) = chars.next() {
            if c.is_alphabetic() || !buffer.is_empty() && c.is_alphanumeric() {
                buffer.push(c);
                continue;
            } else if c.is_digit(10) {
                buffer.push(c);
                continue;
            } else if c.is_whitespace() || c.is_ascii_punctuation() {
                if buffer.as_str() == "exit" {
                    tokens.push(Token {
                        ttype: TokenType::Exit,
                        value: None,
                    });
                } else if let Ok(_) = buffer.as_str().parse::<i32>() {
                    tokens.push(Token {
                        ttype: TokenType::IntLit,
                        value: Some(buffer.clone()),
                    });
                } else if buffer.is_empty() {
                    continue;
                } else {
                    return Err("You messed up");
                }
                buffer.clear();
                if c == ';' {
                    tokens.push(Token {
                        ttype: TokenType::Semi,
                        value: None,
                    });
                }
            } else {
                return Err("You messed up");
            }
        }

        Ok(tokens)
    }
}
