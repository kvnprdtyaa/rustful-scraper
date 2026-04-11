mod scrapers;

use std::{env, error::Error, io::{self, Write}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let choice = if let Some(arg) = args.get(1) {
        arg.clone()
    } else {
        println!("=== Rustful Scraper ===");
        println!("Available scrapers:");
        println!("  1. quotes  - Scrape quotes from quotes.toscrape.com");
        println!("  2. books   - Scrape books from books.toscrape.com");
        println!();
        print!("Enter scraper name or number (1/2): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_string();

        match input.as_str() {
            "1" => "quotes".to_string(),
            "2" => "books".to_string(),
            other => other.to_string(),
        }
    };

    match choice.as_str() {
        "quotes" => {
            println!("\nRunning Quotes scraper (quotes.toscrape.com)...\n");
            scrapers::quotestoscrape::run().await?;
        }
        // "books" => {
        //     println!("\nRunning Books scraper (books.toscrape.com)...\n");
        //     scrapers::books::run().await?;
        // }
        other => {
            eprintln!("Unknown scraper: '{}'", other);
            eprintln!("Available scrapers: quotes, books");
            eprintln!("Usage: cargo run -- <scraper>");
            std::process::exit(1);
        }
    }

    Ok(())
}
