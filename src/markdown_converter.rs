use serde_json::json;
use std::error::Error;

pub async fn convert_to_markdown(content: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let api_key = std::env::var("OPENAI_API_KEY")
        .map_err(|_| "OPENAI_API_KEY environment variable not set")?;

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": "gpt-3.5-turbo",
            "messages": [
                {
                    "role": "system",
                    "content": "Convert the following webpage content to well-formatted markdown."
                },
                {
                    "role": "user",
                    "content": content
                }
            ]
        }))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    Ok(response["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("Error: No response from API")
        .to_string())
}
