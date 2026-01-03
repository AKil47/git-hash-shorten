use clap::Parser;
use ghash::{GitRepository, resolve};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The full git hash to shorten
    hash: String,
}

use std::io::Read;

fn main() {
    let cli = Cli::parse();
    
    let hash_input = if cli.hash == "-" {
        let mut buffer = String::new();
        if let Err(e) = std::io::stdin().read_to_string(&mut buffer) {
             eprintln!("Error reading from stdin: {}", e);
             std::process::exit(1);
        }
        buffer
    } else {
        cli.hash
    };

    let hash = hash_input.trim();

    // Open the repository in the current directory
    let repo_result = GitRepository::open(".");
    
    let repo = match repo_result {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error opening repository: {}", e);
            std::process::exit(1);
        }
    };
    
    match resolve(&repo, hash) {
        Ok(short) => println!("{}", short),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
