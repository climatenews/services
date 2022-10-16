use crate::openai::models::{Completion, CompletionArgs};
use log::info;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

const BASE_URL: &str = "https://api.openai.com/v1";
const PROMPT_END: &str = " \n\n###\n\n";

pub async fn fetch_climate_classification(text: &str) -> bool {
    let prompt = format!("{}{}", text, PROMPT_END);
    let completion = completion(prompt).await;
    return match completion.as_str() {
        " 0" => {
            false
         },
         " 1" => {
            true
         },
         _ => {
             panic!("Invalid completion string");
         }
    }
}

pub async fn completion(prompt: String) -> String {
    let args = CompletionArgs {
        prompt,
        temperature: 1.0,
        max_tokens: 1,
        // stop: vec![String::from("\n")],
        model: String::from("curie:ft-personal-2022-10-14-18-16-36"),
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
            info!("status: {}", response.status());
            let mut result: Completion = response.json().await.unwrap();
            // info!("Response: '{}'", result.choices[0].text);
            let choice = result.choices.remove(0);
            return choice.text;
        }
    }
}

#[cfg(test)]
mod tests {

    use db::init_env;
    use super::*;

    #[tokio::test]
    async fn fetch_climate_classification_test_1() {
        init_env();
        let text = "Italy: Floods and rain kill at least 10 overnight - officials - Rescuers are searching for four others missing after torrential rainfall hit the Marche region overnight.";
        let is_climate_related = fetch_climate_classification(text).await;
        assert_eq!(is_climate_related, true);
    }

    #[tokio::test]
    async fn fetch_climate_classification_test_2() {
        init_env();
        let text = "Former President Donald Trump invoked his Fifth Amendment right more than 440 times on Wednesday during a deposition by lawyers from New York Attorney General Letitia James’ office, according to multiple sources. - Former President Donald Trump invoked his Fifth Amendment right more than 440 times on Wednesday during a deposition by lawyers from New York Attorney General Letitia James’ office, according to multiple sources.";
        let is_climate_related = fetch_climate_classification(text).await;
        assert_eq!(is_climate_related, false);
    }
}