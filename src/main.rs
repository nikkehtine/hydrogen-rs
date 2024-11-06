use snafu::{prelude::*, Whatever};
use std::{fs, process};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    // Input file
    #[arg()]
    input: String,

    // Output file
    #[arg(short, default_value_t = String::from("out"))]
    output: String,
}

#[derive(Debug)]
enum TokenType {
    _Return,
    _IntLit,
    _Semi,
    _Whitespace,
}

struct Token {
    token_type: TokenType,
    value: Option<String>,
}

// #[derive(Debug, Snafu)]
// enum Error

fn tokenize(input: &String) -> Result<Vec<Token>, Whatever> {
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
                    "return" => {
                        tokens.push(Token {
                            token_type: TokenType::_Return,
                            value: None,
                        });
                        buf.clear();
                    }
                    _ => {
                        whatever!("no such keyword: {}", buf)
                    }
                }
            }
            _ if c.is_numeric() => {
                buf.push(c);
                while let Some(next_c) = c_iter.next_if(|&x| x.is_numeric()) {
                    buf.push(next_c);
                }
                tokens.push(Token {
                    token_type: TokenType::_IntLit,
                    value: Some(buf.clone()),
                });
                buf.clear();
            }
            ';' => {
                tokens.push(Token {
                    token_type: TokenType::_Semi,
                    value: None,
                });
            }
            _ if c.is_whitespace() => {
                c_iter.next();
            }
            _ => {
                c_iter.next();
            }
        }
    }

    return Ok(tokens);
}

fn main() {
    let args = Args::parse();
    let _out_name = format!("{}.asm", &args.output);

    let contents = match fs::read_to_string(&args.input) {
        Ok(v) => v,
        Err(v) => {
            eprintln!("Bruh moment: {v}");
            process::exit(1);
        }
    };

    print!("{contents}");

    let tokens = match tokenize(&contents) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Bruh moment: {e}");
            process::exit(1);
        }
    };

    for e in tokens {
        println!("Token: {:?}", e.token_type);
        if let Some(v) = e.value {
            println!("  Value: {:?}", v);
        }
    }
}
