use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    // short = 'n', long = "name"
    #[clap(short, long)]
    name: String,

    /// Age of the person
    #[clap(short, long)]
    age: u8,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}[{}]!", args.name, args.age);
    }
}