use clap::Parser;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use scraper::{Html, Selector};
use std::error::Error;

mod markdown_converter;
use markdown_converter::convert_to_markdown;

/// Simple program to extract text from a webpage
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// URL of the webpage to extract text from
    url: String,
    
    /// Convert output to markdown using LLM
    #[arg(long, short, alias = "md")]
    markdown: bool,
}

async fn run() -> Result<(), Box<dyn Error>> {
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
    let res = client.get(parsed_url).send().await?;

    // Check if the request was successful
    if !res.status().is_success() {
        return Err(format!("Failed to fetch the URL: {}", res.status()).into());
    }

    // Get the response body as text
    let body = res.text().await?;

    // Parse the HTML
    let document = Html::parse_document(&body);

    // Try to select 'article', 'main', or 'body' elements
    let selectors = ["article", "main", "body"];
    let mut found_content = false;

    let mut output = String::new();
    for selector_str in &selectors {
        let selector = Selector::parse(selector_str)?;
        for element in document.select(&selector) {
            // Collect text from the selected element
            let text = element.text().collect::<Vec<_>>().join(" ");
            if !text.trim().is_empty() {
                output.push_str(&text);
                output.push('\n');
                found_content = true;
                break;
            }
        }
        if found_content {
            break;
        }
    }

    if !found_content {
        return Err("No suitable content found.".into());
    }

    // Only print the markdown version if requested, otherwise print original
    if args.markdown {
        let markdown = markdown_converter::convert_to_markdown(&output).await?;
        println!("{}", markdown);
    } else {
        // Print URL and original content
        println!("URL: {}", url);
        println!("{}", output);
        
        // Print links section
        println!("\n*Links*");
        let link_selector = Selector::parse("a")?;
        for element in document.select(&link_selector) {
            if let Some(href) = element.value().attr("href") {
                let mut link_text = element.text().collect::<Vec<_>>().join(" ");
                link_text = link_text.trim().to_string();
                println!("[{}]({})", link_text, href);
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
