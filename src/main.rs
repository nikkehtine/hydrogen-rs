use crate::tokenizer::tokenize;
use hydro::{Token, TokenType};
use std::{env, fmt::Write, fs};

mod tokenizer;

pub fn assemble(tokens: &Vec<Token>) -> Result<String, &'static str> {
    let iter = tokens.iter().enumerate();

    let mut exit_code = 0;
    let mut output = String::new();

    output.write_str("global _start\n_start:\n").unwrap();

    for (i, token) in iter {
        match token.ttype {
            TokenType::Exit => {
                if i != 0 {
                    return Err("exit in wrong place");
                }
            }
            TokenType::IntLit => {
                if i != 1 {
                    return Err("int in wrong place");
                } else {
                    exit_code = match &token.value {
                        Some(v) => v.parse().unwrap(),
                        None => panic!("wrong value of int"),
                    }
                }
            }
            TokenType::Semi => {
                if i != 2 {
                    return Err("semi in wrong place");
                }
            }
        }
    }

    output.write_str("    mov rax, 60\n").unwrap();
    output
        .write_str(format!("    mov rdi, {exit_code}\n").as_str())
        .unwrap();
    output.write_str("    syscall\n").unwrap();

    Ok(output)
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

    println!("{:?}", tokens);

    let output = match assemble(&tokens) {
        Ok(val) => val,
        Err(e) => {
            println!("ERROR: {}: {}", e, input);
            std::process::exit(1);
        }
    };

    match fs::write("./out.asm", output) {
        Ok(_) => {}
        Err(e) => {
            println!("ERROR: {}: {}", e, input);
            std::process::exit(1);
        }
    };
}
