use clap::Parser;

#[derive(Debug, Parser)]
#[command(about, version, long_about = None)]
struct Args {
    // Name to Greet
    #[arg(short, long)]
    name: String,

    // NUmber of time to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let cli = Args::parse();
    for _ in 0..cli.count {
        println!("Hello {}", cli.name)
    }
}
