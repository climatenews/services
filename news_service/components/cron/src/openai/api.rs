use crate::{
    news_feed::constants::REQUEST_SLEEP_DURATION,
    openai::models::{ChatCompletion, ChatCompletionArgs, Message, ResponseFormat},
    slack::send_main_cron_message,
};
use anyhow::{bail, Error, Result};
use db::models::news_bsky_post_url::NewsBskyPostUrl;
use log::info;
use reqwest::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    StatusCode,
};
use serde::Deserialize;
use serde_json::json;
use tokio::time::{sleep, Duration};

const BASE_URL: &str = "https://api.openai.com/v1";
const MODEL_NAME: &str = "gpt-5-mini";
const SYSTEM_PROMPT: &str =
    "You are a climate news classifier. Given a news article title and description, determine \
    whether the article is related to climate change, the climate crisis, climate policy, or \
    climate-related environmental issues. Respond only with a JSON object matching the provided \
    schema.";

#[derive(Deserialize)]
struct ClimateClassification {
    is_climate_related: bool,
}

pub async fn fetch_news_post_url_climate_classification(
    news_post_url: NewsBskyPostUrl,
) -> Result<bool> {
    let title_and_description = format!(
        "{} - {}",
        news_post_url.title.unwrap_or_default(),
        news_post_url.description.unwrap_or_default()
    );

    let result = fetch_text_climate_classification(title_and_description.clone()).await;
    info!(
        "OpenAI classification - result: {:?} - text: {}",
        result,
        title_and_description.clone()
    );
    result
}

async fn fetch_text_climate_classification(text: String) -> Result<bool> {
    let completion = openai_chat_completion_request(text).await?;
    let classification: ClimateClassification = serde_json::from_str(&completion)
        .map_err(|err| Error::new(err).context("OpenAI classification - invalid response error"))?;
    Ok(classification.is_climate_related)
}

pub async fn openai_chat_completion_request(text: String) -> Result<String> {
    let args = ChatCompletionArgs {
        model: String::from(MODEL_NAME),
        messages: vec![
            Message {
                role: String::from("system"),
                content: String::from(SYSTEM_PROMPT),
            },
            Message {
                role: String::from("user"),
                content: text,
            },
        ],
        response_format: ResponseFormat {
            format_type: String::from("json_schema"),
            json_schema: json!({
                "name": "climate_classification",
                "strict": true,
                "schema": {
                    "type": "object",
                    "properties": {
                        "is_climate_related": {
                            "type": "boolean"
                        }
                    },
                    "required": ["is_climate_related"],
                    "additionalProperties": false
                }
            }),
        },
        reasoning_effort: String::from("minimal"),
    };
    let url = format!("{}/chat/completions", BASE_URL);
    let client = reqwest::Client::new();
    let body = serde_json::to_string(&args)?;
    let api_token = std::env::var("OPENAI_API_KEY")?;
    let response = client
        .post(url)
        .body(body.clone())
        .header(AUTHORIZATION, format!("Bearer {}", api_token))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await;
    sleep(Duration::from_millis(REQUEST_SLEEP_DURATION)).await;

    match response {
        Err(e) => Err(Error::new(e).context("OpenAI chat completion API error".to_string())),
        Ok(response) => {
            if response.status() == StatusCode::OK {
                let mut result: ChatCompletion = response.json().await?;
                let choice = result.choices.remove(0);
                Ok(choice.message.content.unwrap_or_default())
            } else {
                let result = response.text().await?;
                send_main_cron_message(format!(
                    "openai_chat_completion_request failed: {:?}",
                    result
                )).await;
                bail!("openai - server error - {}", result)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use db::init_env;

    #[tokio::test]
    async fn fetch_climate_classification_test_1() {
        init_env();
        let text = "Italy: Floods and rain kill at least 10 overnight - officials - Rescuers are searching for four others missing after torrential rainfall hit the Marche region overnight.";
        let is_climate_related = fetch_text_climate_classification(text.to_string())
            .await
            .unwrap();
        assert!(is_climate_related);
    }

    #[tokio::test]
    async fn fetch_climate_classification_test_2() {
        init_env();
        let text = "Former President Donald Trump invoked his Fifth Amendment right more than 440 times on Wednesday during a deposition by lawyers from New York Attorney General Letitia James’ office, according to multiple sources. - Former President Donald Trump invoked his Fifth Amendment right more than 440 times on Wednesday during a deposition by lawyers from New York Attorney General Letitia James’ office, according to multiple sources.";
        let is_climate_related = fetch_text_climate_classification(text.to_string())
            .await
            .unwrap();
        assert!(!is_climate_related);
    }
}
