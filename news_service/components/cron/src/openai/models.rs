use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize)]
pub struct ChatCompletionArgs {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(rename = "response_format")]
    pub response_format: ResponseFormat,
    #[serde(rename = "reasoning_effort")]
    pub reasoning_effort: String,
}

#[derive(Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    pub format_type: String,
    #[serde(rename = "json_schema")]
    pub json_schema: Value,
}

#[derive(Deserialize)]
pub struct ChatCompletion {
    pub id: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChatCompletionChoice>,
}

#[derive(Deserialize)]
pub struct ChatCompletionChoice {
    pub index: u64,
    pub message: ChatCompletionMessage,
    pub finish_reason: String,
}

#[derive(Deserialize)]
pub struct ChatCompletionMessage {
    pub role: String,
    pub content: Option<String>,
}
