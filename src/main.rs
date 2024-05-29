use std::{env, fmt::Debug, fs};

#[derive(Debug)]
pub enum TokenType {
    Return,
    IntLit,
    Semi,
}

#[derive(Debug)]
pub struct Token {
    ttype: TokenType,
    value: Option<String>,
}

pub fn tokenize(s: &str) -> Result<Vec<Token>, &'static str> {
    let mut buffer = String::new();
    let mut tokens: Vec<Token> = Vec::new();

    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c.is_alphabetic() || !buffer.is_empty() && c.is_alphanumeric() {
            buffer.push(c);
            continue;
        }
        if c.is_whitespace() && buffer.as_str() == "return" {
            tokens.push(Token {
                ttype: TokenType::Return,
                value: None,
            });
        }
        if tokens.is_empty() {
            return Err("You messed up");
        }
    }

    Ok(tokens)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Incorrect usage");
        println!("Correct usage is");
        println!("    hydro <input.hy>");
        std::process::exit(1);
    };

    let input = &args[1];

    let contents = match fs::read_to_string(input) {
        Ok(val) => val,
        Err(e) => {
            println!("ERROR: {}: {}", e, input);
            std::process::exit(1);
        }
    };

    let tokens = match tokenize(&contents) {
        Ok(val) => val,
        Err(e) => {
            println!("ERROR: {}: {}", e, input);
            std::process::exit(1);
        }
    };

    println!("{:?}", tokens)
}
