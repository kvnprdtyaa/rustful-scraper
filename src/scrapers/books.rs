use scraper::{Html, Selector};
use serde::Serialize;
use std::{error::Error, fs::File};

#[derive(Debug, Serialize)]
struct Book {
    title: String,
    price: String,
    rating: String,
    availability: String,
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let mut page = 1;
    let base_url = "https://books.toscrape.com/catalogue/page-{}.html";
    let mut books: Vec<Book> = Vec::new();

    let book_selector = Selector::parse("article.product_pod").unwrap();
    let title_selector = Selector::parse("h3 > a").unwrap();
    let price_selector = Selector::parse(".price_color").unwrap();
    let rating_selector = Selector::parse(".star-rating").unwrap();
    let availability_selector = Selector::parse(".availability").unwrap();
    let next_selector = Selector::parse(".next > a").unwrap();

    loop {
        let url = if page == 1 {
            "https://books.toscrape.com/".to_string()
        } else {
            base_url.replace("{}", &page.to_string())
        };

        println!("Fetching page {}: {}", page, url);

        let response = reqwest::get(&url).await?;

        if !response.status().is_success() {
            println!("No more pages found (HTTP({}))", response.status());
            break;
        }

        let html = response.text().await?;
        let document = Html::parse_document(&html);

        let mut found_books = 0;

        for book in document.select(&book_selector) {
            let title = book
                .select(&title_selector)
                .next()
                .and_then(|e| e.value().attr("title"))
                .unwrap_or_default()
                .to_string();

            let price = book
                .select(&price_selector)
                .next()
                .map(|e| e.text().collect::<Vec<_>>().join(""))
                .unwrap_or_default();

            let rating = book
                .select(&rating_selector)
                .next()
                .and_then(|e| e.value().attr("class"))
                .unwrap_or_default()
                .replace("star-rating ", "");

            let availability = book
                .select(&availability_selector)
                .next()
                .map(|e| e.text().collect::<Vec<_>>().join("").trim().to_string())
                .unwrap_or_default();

            books.push(Book {
                title,
                price,
                rating,
                availability,
            });
            found_books += 1;
        }

        println!("\nScraped {} books on page {}", found_books, page);

        let has_next = document.select(&next_selector).next().is_some();

        if has_next {
            page += 1;
        } else {
            println!("\nNo more books to scrape.");
            break;
        }
    }

    println!("\nTotal books scraped {}.", books.len());

    save_to_json(&books)?;
    save_to_csv(&books)?;

    println!("\nData saved to books.json and books.csv\n");
    Ok(())
}

fn save_to_json(books: &[Book]) -> Result<(), Box<dyn Error>> {
    let file = File::create("books.json")?;
    serde_json::to_writer_pretty(file, books)?;
    Ok(())
}

fn save_to_csv(books: &[Book]) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path("books.csv")?;

    for book in books {
        writer.serialize(book)?;
    }

    writer.flush()?;

    Ok(())
}
