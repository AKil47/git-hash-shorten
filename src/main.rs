use clap::Parser;
use ghash::{GitRepository, resolve};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The full git hash to shorten
    hash: String,
}

fn main() {
    let cli = Cli::parse();
    
    // Open the repository in the current directory
    let repo_result = GitRepository::open(".");
    
    let repo = match repo_result {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error opening repository: {}", e);
            std::process::exit(1);
        }
    };
    
    match resolve(&repo, &cli.hash) {
        Ok(short) => println!("{}", short),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
