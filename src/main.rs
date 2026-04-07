use reqwest;
use scraper::{Html, Selector};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://quotes.toscrape.com/";

    println!("Fetching data from the: {}", url);

    let response = reqwest::get(url).await?.text().await?;

    let document = Html::parse_document(&response);

    let quote_selector = Selector::parse(".quote").unwrap();
    let text_selector = Selector::parse(".text").unwrap();
    let author_selector = Selector::parse(".author").unwrap();

    let mut quotes = Vec = Vec::new();

    for (index, quote) in document.select(&quote_selector).enumerate() {
        let text = quote.select(&text_selector).next().map(|t| t.text().collect::<Vec<_>>().join("")).unwrap_or_default();
        let author = quote.select(&author_selector).next().map(|t| t.text().collect::<Vec<_>>().join("")).unwrap_or_default();

        println!("\n{}. {} - {}", index + 1, text, author);
    }
    println!("\n");

    Ok(())
}