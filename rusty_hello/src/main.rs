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

    let message: String;

    if args.upper {
        message = format!("HELLO, {}!", args.name.to_uppercase());
    } else {
        message = format!("Hello, {}!", args.name);
    }

    
    let mut i: u32 = 0;
    while i < args.repeat {
        println!("{}", &message);  
        i = i + 1;
    }
}