mod gpu;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about = "Ethereum Vanity Generator GPU")]
struct Args {
    #[arg(long, default_value = "13")]
    prefix: String,
    #[arg(long, default_value = "a3")]
    suffix: String,
    #[arg(long, default_value_t = 4)]
    threads: usize,
    #[arg(long, default_value_t = 100000)]
    batch: u32,
}

fn main() {
    let args = Args::parse();

    println!("Target Prefix: {}", &args.prefix);
    println!("Target Suffix: {}", &args.suffix);
    println!("Batch size: {}", args.batch);

    match gpu::run_gpu(&args.prefix, &args.suffix, args.batch) {
        Ok(_) => println!("GPU batch completed"),
        Err(e) => eprintln!("GPU failed: {}", e),
    }
}
