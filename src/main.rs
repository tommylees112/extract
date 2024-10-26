use clap::Parser;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use scraper::{Html, Selector};
use std::error::Error;

/// Simple program to extract text from a webpage
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// URL of the webpage to extract text from
    url: String,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    // Parse command-line arguments using clap::Parser
    let args = Args::parse();

    // Validate the URL
    let url = args.url;
    let parsed_url = reqwest::Url::parse(&url)?;

    // Create an HTTP client with a custom User-Agent
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("Mozilla/5.0 (compatible; Extractor/1.0)"),
    );
    let client = Client::builder().default_headers(headers).build()?;

    // Fetch the webpage content
    let res = client.get(parsed_url).send()?;

    // Check if the request was successful
    if !res.status().is_success() {
        return Err(format!("Failed to fetch the URL: {}", res.status()).into());
    }

    // Get the response body as text
    let body = res.text()?;

    // Parse the HTML
    let document = Html::parse_document(&body);

    // Try to select 'article', 'main', or 'body' elements
    let selectors = ["article", "main", "body"];
    let mut found_content = false;

    // Print the URL to the top of the page
    println!("URL: {}", url);
    for selector_str in &selectors {
        let selector = Selector::parse(selector_str)?;
        for element in document.select(&selector) {
            // Collect text from the selected element
            let text = element.text().collect::<Vec<_>>().join(" ");
            if !text.trim().is_empty() {
                println!("{}", text);
                found_content = true;
                break;
            }
        }

        if found_content {
            break;
        }
    }

    // Select and print links as Markdown
    println!("\n*Links*");
    let link_selector = Selector::parse("a")?;
    for element in document.select(&link_selector) {
        if let Some(href) = element.value().attr("href") {
            let mut link_text = element.text().collect::<Vec<_>>().join(" ");
            link_text = link_text.trim().to_string();
            println!("[{}]({})", link_text, href);
        }
    }

    if !found_content {
        return Err("No suitable content found.".into());
    }

    Ok(())
}
