use db::init_env;
use db::queries::news_feed_url_query::NewsFeedUrlQuery;
use db::sql::news_feed_url_query::get_news_feed_urls;
use db::util::db::init_db;
use db::util::time::past_days;
use openai_api::Client;
use serde::Serialize;

pub const NEWS_FEED_URLS_NUM_DAYS: i64 = 3;
pub const NEWS_FEED_URLS_LIMIT: i64 = 100;

#[tokio::main]
async fn main() {
    init_env();
    //gpt3_classification().await;
    export_news_feed_urls().await;
}

#[derive(Debug, Serialize)]
struct Record {
    title_and_description: String,
}

async fn export_news_feed_urls() {
    let db_pool = init_db().await;
    let recent_timestamp = past_days(NEWS_FEED_URLS_NUM_DAYS).unix_timestamp();
    let news_feed_urls: Vec<NewsFeedUrlQuery> =
        get_news_feed_urls(&db_pool, recent_timestamp, NEWS_FEED_URLS_LIMIT)
            .await
            .unwrap();

    let mut wtr = csv::Writer::from_path("news_feed_urls.csv").unwrap();

    for news_feed_url in news_feed_urls {
        if let (Some(title), Some(description)) = (news_feed_url.title, news_feed_url.description) {
            wtr.serialize(Record {
                title_and_description: format!("{}-{}", title, description),
            })
            .unwrap();
        }
    }

    wtr.flush().unwrap();
}

// TODO avoid using api library
async fn _gpt3_classification() {
    let api_token = std::env::var("OPENAI_API_KEY").unwrap();
    let client = Client::new(&api_token);
    let prompt = String::from("The following is a list of companies and the categories they fall into:\n\nApple, Facebook, Fedex\n\nApple\nCategory:",);
    println!(
        "{}{}",
        prompt,
        client.complete_prompt(prompt.as_str()).await.unwrap()
    );
}

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
