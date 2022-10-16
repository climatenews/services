use crate::openai::models::{Completion, CompletionArgs};
use log::info;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

const BASE_URL: &str = "https://api.openai.com/v1";

pub async fn fetch_climate_classification(_text: &str) -> bool {
    completion(String::from("Say this is a test")).await;
    false
}

pub async fn completion(prompt: String) {
    let args = CompletionArgs {
        prompt,
        temperature: 0.0,
        max_tokens: 6,
        stop: vec![String::from("\n")],
        engine: String::from("text-davinci-002"),
    };
    let url = format!("{}/completions", BASE_URL);
    let client = reqwest::Client::new();
    let body = serde_json::to_string(&args).unwrap();

    let api_token = std::env::var("OPENAI_API_KEY").unwrap();
    let response = client
        .post(url)
        .body(body.clone())
        .header(AUTHORIZATION, format!("Bearer {}", api_token))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await;

    match response {
        Err(e) => {
            panic!("error: {:?}", e);
        }
        Ok(response) => {
            let result: Completion = response.json().await.unwrap();
            info!("Response: {}", result.choices[0].text);
            info!("Model used: {}", result.model);
        }
    }
}
