mod eth;
mod matcher;
mod difficulty;

use clap::Parser;
use difficulty::estimate_attempts;
use eth::{generate_wallet, EthWallet};
use matcher::matches_pattern;
use std::io::{self, Write};
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about = "Ethereum Similar Vanity Generator - Multi Core")]
struct Args {
    #[arg(long, default_value = "13")]
    prefix: String,

    #[arg(long, default_value = "a3")]
    suffix: String,

    #[arg(long, default_value_t = 4)]
    threads: usize,
}

fn main() {
    let args = Args::parse();

    let (total_chars, expected_attempts) = estimate_attempts(&args.prefix, &args.suffix);

    println!("======================================");
    println!(" Rust Similar Wallet Generator - v2");
    println!("======================================");
    println!("Target Prefix: {}", if args.prefix.is_empty() { "(none)" } else { &args.prefix });
    println!("Target Suffix: {}", if args.suffix.is_empty() { "(none)" } else { &args.suffix });
    println!("Fixed Chars:   {}", total_chars);
    println!("Expected Avg Attempts: {:.0}", expected_attempts);
    println!("Threads:       {}", args.threads);
    println!("Searching...\n");

    let start = Instant::now();
    let found = Arc::new(AtomicBool::new(false));
    let attempts = Arc::new(AtomicU64::new(0));
    let result: Arc<Mutex<Option<EthWallet>>> = Arc::new(Mutex::new(None));

    let mut handles = vec![];

    for _ in 0..args.threads {
        let prefix = args.prefix.clone();
        let suffix = args.suffix.clone();
        let found_clone = Arc::clone(&found);
        let attempts_clone = Arc::clone(&attempts);
        let result_clone = Arc::clone(&result);

        let handle = thread::spawn(move || {
            while !found_clone.load(Ordering::Relaxed) {
                let wallet = generate_wallet();
                attempts_clone.fetch_add(1, Ordering::Relaxed);

                if matches_pattern(&wallet.address, &prefix, &suffix) {
                    let already_found = found_clone.swap(true, Ordering::SeqCst);

                    if !already_found {
                        let mut lock = result_clone.lock().unwrap();
                        *lock = Some(wallet);
                    }

                    break;
                }
            }
        });

        handles.push(handle);
    }

    // Progress printer loop
    while !found.load(Ordering::Relaxed) {
        let elapsed = start.elapsed().as_secs_f64();
        let current_attempts = attempts.load(Ordering::Relaxed);
        let rate = current_attempts as f64 / elapsed.max(0.0001);

        print!(
            "\rAttempts: {} | Elapsed: {:.2}s | Rate: {:.2}/s",
            current_attempts, elapsed, rate
        );
        io::stdout().flush().unwrap();

        thread::sleep(Duration::from_millis(500));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed = start.elapsed().as_secs_f64();
    let final_attempts = attempts.load(Ordering::Relaxed);
    let final_rate = final_attempts as f64 / elapsed.max(0.0001);

    let lock = result.lock().unwrap();

    if let Some(wallet) = &*lock {
        println!("\n\nMATCH FOUND!\n");
        println!("Address:     {}", wallet.address);
        println!("Private Key: {}", wallet.private_key);
        println!("Public Key:  {}", wallet.public_key);
        println!();
        println!("Attempts:    {}", final_attempts);
        println!("Elapsed:     {:.2}s", elapsed);
        println!("Attempts/s:  {:.2}", final_rate);
    } else {
        println!("\nNo match found.");
    }
}
