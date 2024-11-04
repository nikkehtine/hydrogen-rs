use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    // Input file
    #[arg()]
    input: String,

    // Output file
    #[arg(short)]
    output: String,
}

fn main() {
    let args = Args::parse();

    println!("Hello, {}!", args.input);
    println!("We goin' into {} with this one", args.output);
}
