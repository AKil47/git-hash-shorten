use clap::Parser;
use git2::Repository;
use ghash::compute_shortest_prefix;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The full git hash to shorten
    hash: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    // Open the repository in the current directory
    let repo = Repository::open(".")?;
    let odb = repo.odb()?;
    
    let mut hashes = Vec::new();
    // Collect all OIDs from the object database
    odb.foreach(|oid| {
        hashes.push(oid.to_string());
        true
    })?;
    
    // Sort for binary search
    hashes.sort();
    
    // Normalize input hash to lowercase
    let target = cli.hash.to_lowercase();
    
    match compute_shortest_prefix(&hashes, &target) {
        Some(short) => println!("{}", short),
        None => {
            eprintln!("Error: Hash '{}' not found in repository", target);
            std::process::exit(1);
        }
    }
    
    Ok(())
}
