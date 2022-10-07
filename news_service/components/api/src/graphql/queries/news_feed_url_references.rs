use crate::graphql::errors::GqlError;
use async_graphql::{ErrorExtensions, FieldResult};
use db::{
    models::news_feed_url_reference::NewsFeedUrlReference,
    queries::news_feed_url_references_query::NewsFeedUrlReferencesQuery,
    sql::news_feed_url_references_query::get_news_feed_url_references,
};
use sqlx::postgres::PgPool;

pub async fn news_feed_url_references_query<'a>(
    pool: &PgPool,
    url_id: i32,
) -> FieldResult<Vec<NewsFeedUrlReference>> {
    let news_feed_url_references_result: Option<Vec<NewsFeedUrlReferencesQuery>> =
        get_news_feed_url_references(pool, url_id).await;

    if let Some(news_feed_url_references_query) = news_feed_url_references_result {
        let mut news_feed_url_references: Vec<NewsFeedUrlReference> = vec![];
        for news_feed_url_reference_query in news_feed_url_references_query {
            let retweeted_by_usernames: Vec<String> = vec![];

            let author_username = match news_feed_url_reference_query.username {
                Some(author_username) => author_username,
                None => "TODO lookup".to_string(),
            };

            let news_feed_url_reference = NewsFeedUrlReference {
                url_id: news_feed_url_reference_query.url_id,
                tweet_id: news_feed_url_reference_query.tweet_id,
                tweet_text: news_feed_url_reference_query.text,
                tweet_created_at_str: news_feed_url_reference_query.created_at_str,
                author_username,
                retweeted_by_usernames,
            };
            news_feed_url_references.push(news_feed_url_reference);
        }
        Ok(news_feed_url_references)
    } else {
        Err(GqlError::NotFound.extend())
    }
}

// // Map of tweet ids to list of retweets
// // Used to check for retweets of a news_feed_url_reference
// let mut indirect_references_map: HashMap<i64, Vec<NewsFeedUrlReferencesQuery>> =
//     HashMap::new();
// for innews_feed_url_reference in innews_feed_url_references {
//     let map_key = innews_feed_url_reference.referenced_tweet_id;
//     if indirect_references_map.contains_key(&map_key) {
//         let retweets = indirect_references_map.get(&map_key).unwrap();
//         let mut new_retweets_vec = retweets.clone();
//         new_retweets_vec.push(innews_feed_url_reference);
//         indirect_references_map.insert(map_key, new_retweets_vec);
//     } else {
//         if innews_feed_url_reference.referenced_tweet_kind == "retweeted" {
//             indirect_references_map.insert(
//                 innews_feed_url_reference.referenced_tweet_id,
//                 vec![innews_feed_url_reference],
//             );
//         }
//     }
// }

// // TODO use dedup method
// let mut author_map: HashMap<i64, NewsFeedUrlReference> = HashMap::new();

// let mut news_feed_url_references: Vec<NewsFeedUrlReference> = vec![];
// for news_feed_url_reference in news_feed_url_references {
//     //TODO move logic into db component
//     let referenced_tweet_id = &news_feed_url_reference.tweet_id;
//     let retweeted_by_usernames: Vec<String> =
//         if indirect_references_map.contains_key(referenced_tweet_id) {
//             let retweets = indirect_references_map.get(referenced_tweet_id).unwrap();
//             retweets.iter().map(|rt| rt.username.clone()).unique().collect()
//         } else {
//             vec![]
//         };

//     let news_feed_url_reference = NewsFeedUrlReference {
//         tweet_id: news_feed_url_reference.tweet_id,
//         tweet_text: news_feed_url_reference.text,
//         tweet_created_at_str: news_feed_url_reference.created_at_str,
//         author_username: news_feed_url_reference.username,
//         retweeted_by_usernames: retweeted_by_usernames,
//         url_id: news_feed_url_reference.url_id,
//     };

//     // Avoid duplicate tweets by an author
//     if !author_map.contains_key(&news_feed_url_reference.author_id) {
//         // TODO check for oldest shared at time
//         author_map.insert(
//             news_feed_url_reference.author_id,
//             news_feed_url_reference,
//         );
//     }
// }

// // Avoid duplicate tweets by an author
// for author_id in author_map.keys() {
//     let news_feed_url_reference = author_map.get(&author_id).unwrap();
//     news_feed_url_references.push(news_feed_url_reference.clone());
// }
// // TODO add support for quoted tweets
// // TODO add support for retweeted indirect tweets

#[tokio::test]
async fn get_news_feed_urls_test() {}
