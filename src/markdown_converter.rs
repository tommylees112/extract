use serde_json::json;
use std::error::Error;

pub async fn convert_to_markdown(content: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .map_err(|_| "ANTHROPIC_API_KEY environment variable not set")?;

    let system_prompt = include_str!("system_prompt.txt");

    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", &api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&json!({
            "model": "claude-3-sonnet-20240229",
            "max_tokens": 1000,
            "temperature": 0,
            "system": system_prompt,
            "messages": [
                {
                    "role": "user", 
                    "content": [
                        {
                            "type": "text",
                            "text": content
                        }
                    ]
                }
            ]
        }))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    println!("API Response: {}", serde_json::to_string_pretty(&response)?);

    match response.get("content").and_then(|c| c[0].get("text")) {
        Some(text) => Ok(text.as_str().unwrap_or("").to_string()),
        None => {
            if let Some(error) = response.get("error") {
                Err(format!("API Error: {}", error).into())
            } else {
                Err("Unknown API response format".into())
            }
        }
    }
}
