mod scrapers;

use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let scraper = args.get(1).map(String::as_str).unwrap_or("quotes");

    match scraper {
        "quotes" => {
            println!("Running Quotes scraper (quotes.toscrape.com)...\n");
            scrapers::quotes::run().await?;
        }
        "books" => {
            println!("Running Books scraper (books.toscrape.com)...\n");
            scrapers::books::run().await?;
        }
        other => {
            eprintln!("Unknown scraper: '{}'", other);
            eprintln!("Available scrapers:");
            eprintln!("  quotes  - scrape quotes from quotes.toscrape.com");
            eprintln!("  books   - scrape books from books.toscrape.com");
            eprintln!("\nUsage: cargo run -- <scraper>");
            std::process::exit(1);
        }
    }

    Ok(())
}
