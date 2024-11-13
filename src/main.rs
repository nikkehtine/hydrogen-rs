use clap::Parser;
use snafu::{prelude::*, Whatever};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    process::{self, Command, Output},
};

pub mod tokenizer;

#[derive(Parser, Debug)]
struct Args {
    // Input file
    #[arg()]
    input: String,

    // Output file
    #[arg(short, default_value_t = String::from("out"))]
    output: String,

    // Verbosity
    #[arg(short, default_value_t = false)]
    verbose: bool,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Exit,
    IntLit,
    Semi,
    _Whitespace,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    value: Option<String>,
}

fn assemble_tokens(tokens: Vec<Token>) -> Result<String, Whatever> {
    let mut output = String::from("global _start\n_start:\n");
    let mut t_iter = tokens.iter().peekable();
    while let Some(token) = t_iter.next() {
        match &token.token_type {
            TokenType::Exit => {
                if let Some(token1) = t_iter.next() {
                    match &token1.token_type {
                        TokenType::IntLit => {
                            if let Some(token2) = t_iter.next() {
                                match &token2.token_type {
                                    TokenType::Semi => {
                                        match &token1.value {
                                            Some(v) => {
                                                output += "    mov rax, 60\n";
                                                output += &format!("    mov rdi, {}\n", v);
                                                output += "    syscall\n";
                                            }
                                            None => whatever!("value required"),
                                        };
                                    }
                                    _ => whatever!("unexpected token: {:?}", &token.token_type),
                                }
                            } else {
                                whatever!(
                                    "unexpected token after {:#?}:\n  reading {:?}\n  expected {:?}",
                                    &token1.token_type,
                                    &t_iter.peek(),
                                    TokenType::Semi
                                )
                            }
                        }
                        _ => whatever!("unexpected token: {:?}", &token1.token_type),
                    }
                } else {
                    whatever!(
                        "unexpected token after {:#?}:\n  reading {:?}\n  expected {:?}",
                        &token.token_type,
                        &t_iter.peek(),
                        TokenType::IntLit
                    )
                }
            }
            TokenType::_Whitespace => continue,
            _ => whatever!("unexpected token: {:?}", &token.token_type),
        }
    }
    return Ok(output);
}

fn run_nasm(filename: &str, input: &str) -> Result<Output, std::io::Error> {
    Command::new("nasm")
        .args(["-felf64", "-o", &format!("{filename}.o"), &input])
        .output()
}

fn run_ld(filename: &str) -> Result<Output, std::io::Error> {
    Command::new("ld")
        .args(["-o", &filename, &format!("{filename}.o")])
        .output()
}

fn main() {
    let args = Args::parse();
    let out_file = Path::new(&args.output);
    let out_name = out_file.file_stem().unwrap().to_str().unwrap();

    let contents = match fs::read_to_string(&args.input) {
        Ok(v) => v,
        Err(v) => {
            eprintln!("Bruh moment: {v}");
            process::exit(1);
        }
    };

    let tokens = match tokenizer::tokenize(contents) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Bruh moment: {e}");
            process::exit(1);
        }
    };
    if args.verbose {
        for t in &tokens {
            println!("Token: {:?}", &t.token_type);
            if let Some(v) = &t.value {
                println!("  Value: {:?}", v);
            }
        }
    }

    let output_asm = match assemble_tokens(tokens) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Bruh moment: {e}");
            process::exit(1);
        }
    };

    match File::create(out_file).and_then(|mut f| f.write_all(output_asm.as_bytes())) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Bruh moment: {e}");
            process::exit(1);
        }
    }

    match run_nasm(out_name, out_file.to_str().unwrap()) {
        Ok(o) => {
            if !o.stdout.is_empty() {
                println!("{}", &String::from_utf8_lossy(&o.stdout));
            }
            if !o.stderr.is_empty() {
                eprintln!("{}", &String::from_utf8_lossy(&o.stderr));
            }
        }
        Err(e) => {
            eprintln!("Bruh moment: {e}");
            process::exit(1);
        }
    };

    match run_ld(out_name) {
        Ok(o) => {
            if !o.stdout.is_empty() {
                println!("{:?}", String::from_utf8_lossy(&o.stdout));
            }
            if !o.stderr.is_empty() {
                eprintln!("{:?}", String::from_utf8_lossy(&o.stderr));
            }
        }
        Err(e) => {
            eprintln!("Bruh moment: {e}");
            process::exit(1);
        }
    };
}
