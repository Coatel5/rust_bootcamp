use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(default_value = "World")]
    name: String,

    #[arg(long)]
    upper: bool,

    #[arg(long, default_value = "1")]
    repeat: u32,
}

fn main() {
    let args = Args::parse();

    let message = if args.upper {
        format!("HELLO, {}!", args.name.to_uppercase())
    } else {
        format!("Hello, {}!", args.name)
    };

    for _ in 0..args.repeat {
        println!("{message}");
    }
}