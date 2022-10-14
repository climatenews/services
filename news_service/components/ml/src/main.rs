use db::init_env;
use db::queries::news_feed_url_query::NewsFeedUrlQuery;
use db::sql::news_feed_url_query::get_news_feed_urls;
use db::util::db::init_db;
use db::util::time::past_days;
use serde::Serialize;
use anyhow::{Result};
use std::{fs::OpenOptions};
use std::collections::HashSet;
use std::fs;

pub const NEWS_FEED_URLS_NUM_DAYS: i64 = 3;
pub const NEWS_FEED_URLS_LIMIT: i64 = 700;
pub const FILE_NAME: &str = "news_feed_urls.jsonl";

#[derive(Debug, Serialize)]
struct Record {
    completion: String,
    prompt: String,
}


#[tokio::main]
async fn main() -> Result<()> {
    init_env();
    //gpt3_classification().await;
    export_news_feed_urls().await?;
    Ok(())
}


async fn export_news_feed_urls() -> Result<()> {
    let db_pool = init_db().await;
    let recent_timestamp = past_days(NEWS_FEED_URLS_NUM_DAYS).unix_timestamp();
    let news_feed_urls: Vec<NewsFeedUrlQuery> =
        get_news_feed_urls(&db_pool, recent_timestamp, NEWS_FEED_URLS_LIMIT)
            .await
            .unwrap();
    fs::remove_file(FILE_NAME)?;

    let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .append(true)
    .open(FILE_NAME)?;

    let mut news_feed_url_hashset: HashSet<String> = HashSet::new();
    for news_feed_url in news_feed_urls {
        if let (Some(title), Some(description)) = (news_feed_url.title, news_feed_url.description) {
            let prompt = remove_unicode(format!("{} - {} \n\n###\n\n", title, description));

            if !news_feed_url_hashset.contains(&prompt.clone()) {
                let record = Record {
                    completion: String::from(" 1\n"),
                    prompt: prompt.clone(),
                };
                jsonl::write(&mut file, &record)?;
                news_feed_url_hashset.insert(prompt.clone());

            }
  
        }
    }




    Ok(())
}

fn remove_unicode(s: String) -> String {
    s.replace(|c: char| !c.is_ascii(), "").replace('"', "'")
}

// // TODO avoid using api library
// async fn _gpt3_classification() {
//     let api_token = std::env::var("OPENAI_API_KEY").unwrap();
//     let client = Client::new(&api_token);
//     let prompt = String::from("The following is a list of companies and the categories they fall into:\n\nApple, Facebook, Fedex\n\nApple\nCategory:",);
//     println!(
//         "{}{}",
//         prompt,
//         client.complete_prompt(prompt.as_str()).await.unwrap()
//     );
// }

// response = openai.Completion.create(
//   model="text-davinci-002",
//   prompt="The following is a list of companies and the categories they fall into:\n\nApple, Facebook, Fedex\n\nApple\nCategory:",
//   temperature=0,
//   max_tokens=6,
//   top_p=1,
//   frequency_penalty=0,
//   presence_penalty=0,
//   stop=["\n"]
// )
