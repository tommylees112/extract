# Webpage Text and Link Extractor

This is a simple Rust command-line application that extracts text and links from a specified webpage. It uses the `clap` crate for command-line argument parsing, `reqwest` for HTTP requests, and `scraper` for HTML parsing.

## Features

- Extracts text content from `article`, `main`, or `body` elements of a webpage.
- Extracts links from the webpage and formats them in Markdown.
- Handles errors gracefully and provides informative error messages.

## Prerequisites

- Rust and Cargo installed on your system. You can download them from [rust-lang.org](https://www.rust-lang.org/).

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/tommylees112/extract.git
   cd webpage-extractor
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

## Usage

Run the program with the URL of the webpage you want to extract text and links from:

```bash
cargo run --release -- <URL>
```

Replace `<URL>` with the actual URL of the webpage.

## Example

```bash
cargo run --release -- https://example.com
```

This will output the text content and links in Markdown format from the specified webpage.

## Dependencies

- **clap**: For command-line argument parsing.
- **reqwest**: For making HTTP requests.
- **scraper**: For parsing and extracting data from HTML documents.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue.

## Acknowledgments

- Thanks to the authors of the `clap`, `reqwest`, and `scraper` crates for their excellent libraries.
