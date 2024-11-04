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

fn main() {
    let args = Args::parse();
    let _out_name = format!("{}.asm", &args.output);

    let contents = match fs::read_to_string(&args.input) {
        Ok(v) => v,
        Err(v) => {
            eprintln!("Bruh moment: {}", v.to_string());
            process::exit(1);
        }
    };

    println!("{contents}");
}
