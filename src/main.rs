use reqwest;
use scraper::{Html, Selector};
use serde::Serialize;
use std::{error::Error, fs::File};

#[derive(Debug,Serialize)]
struct Quote {
    text: String,
    author: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
      let mut page = 1;
    let url = "https://quotes.toscrape.com/";

    println!("Fetching data from the: {}", url);

    let response = reqwest::get(url).await?.text().await?;

    let document = Html::parse_document(&response);

    let quote_selector = Selector::parse(".quote").unwrap();
    let text_selector = Selector::parse(".text").unwrap();
    let author_selector = Selector::parse(".author").unwrap();

    let mut quotes: Vec<Quote> = Vec::new();

    for quote in document.select(&quote_selector) {
        let text = quote.select(&text_selector).next().map(|t| t.text().collect::<Vec<_>>().join("")).unwrap_or_default();
        let author = quote.select(&author_selector).next().map(|t| t.text().collect::<Vec<_>>().join("")).unwrap_or_default();

        quotes.push(Quote { text, author });
    }

    println!("Scraped {} quotes.", quotes.len());

    save_to_json(&quotes)?;

    save_to_csv(&quotes)?;

    println!("\nData saved to quotes.json and quotes.csv\n");
    Ok(())
}

fn save_to_json(quotes: &Vec<Quote>) -> Result<(), Box<dyn Error>> {
    let file = File::create("quotes.json")?;
    
    serde_json::to_writer_pretty(file, quotes)?;
    Ok(())
}

fn save_to_csv(quotes: &Vec<Quote>) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path("quotes.csv")?;

    for quote in quotes {
        writer.serialize(quote)?;
    }

    writer.flush()?;

    Ok(())
}
