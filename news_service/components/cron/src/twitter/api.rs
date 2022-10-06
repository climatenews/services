use crate::news_feed::constants::{MAX_TWEET_RESULTS, REQUEST_SLEEP_DURATION};
use crate::util::convert::i64_to_numeric_id;
use crate::util::helpers::past_365_days;
use tokio::time::{sleep, Duration};
use twitter_v2::authorization::BearerToken;
use twitter_v2::id::NumericId;
use twitter_v2::prelude::PaginableApiResponse;
use twitter_v2::query::{Exclude, TweetField, UserField};
use twitter_v2::{Error, Tweet, TwitterApi, User};
use log::info;

static TWEET_FIELDS: [TweetField; 6] = [
    TweetField::AuthorId,
    TweetField::CreatedAt,
    TweetField::InReplyToUserId,
    TweetField::ConversationId,
    TweetField::Entities,
    TweetField::ReferencedTweets,
];

static USER_FIELDS: [UserField; 5] = [
    UserField::PublicMetrics,
    UserField::ProfileImageUrl,
    UserField::Description,
    UserField::Name,
    UserField::Verified,
];

pub async fn get_users_by_username(
    twitter_api: &TwitterApi<BearerToken>,
    usernames: Vec<&str>,
) -> Option<Vec<User>> {
    //TODO split in to max 100 users per request
    info!("API - get_users_by_username: {}", usernames.len());
    let users_response = twitter_api
        .get_users_by_usernames(usernames)
        .user_fields(USER_FIELDS)
        .send()
        .await;
    parse_error_response(&users_response).await;
    let users = users_response.unwrap().into_data();
    users
}

pub async fn get_users_by_author_id(
    twitter_api: &TwitterApi<BearerToken>,
    author_ids: Vec<i64>,
) -> Vec<User> {
    info!("API - get_users_by_author_id: {}", author_ids.len());
    let mut user_vec: Vec<User> = vec![];
    let split_author_ids_vec =
        split_requests_into_max_amount(author_ids.iter().map(|i| i64_to_numeric_id(*i)).collect());
    for split_author_ids in split_author_ids_vec {
        let users_response = twitter_api
            .get_users(split_author_ids)
            .user_fields(USER_FIELDS)
            .send()
            .await;

        parse_error_response(&users_response).await;
        let users = users_response.unwrap().into_data().unwrap();
        user_vec = [user_vec, users].concat();
    }

    user_vec
}

pub async fn get_user_tweets(
    twitter_api: &TwitterApi<BearerToken>,
    user_id: NumericId,
    last_tweet_id: Option<NumericId>,
) -> Vec<Tweet> {
    let start_time = past_365_days();

    info!("API - get_user_tweets: {}", user_id);
    let mut tweets: Vec<Tweet> = vec![];
    let tweets_api_response = if let Some(last_tweet_id) = last_tweet_id {
        // Use last_tweet_id
        twitter_api
            .get_user_tweets(user_id)
            .max_results(MAX_TWEET_RESULTS)
            .tweet_fields(TWEET_FIELDS)
            .start_time(start_time)
            .since_id(last_tweet_id)
            .exclude([Exclude::Replies])
            .send()
            .await
    } else {
        twitter_api
            .get_user_tweets(user_id)
            .max_results(MAX_TWEET_RESULTS)
            .tweet_fields(TWEET_FIELDS)
            .start_time(start_time)
            .exclude([Exclude::Replies])
            .send()
            .await
    };
    parse_error_response(&tweets_api_response).await;

    let tweets_api_response = tweets_api_response.unwrap();

    if let Some(new_tweets) = tweets_api_response.clone().into_data() {
        tweets = [tweets, new_tweets].concat();
    }
    let mut next_page_response = tweets_api_response.next_page().await;
    parse_error_response(&next_page_response).await;
    loop {
        match next_page_response {
            Err(e) => {
                info!("next_page_response error: {}", e);
                break;
            } //return Err(anyhow!(e)),
            Ok(None) => {
                // info!("next_page_response None");
                break;
            }
            Ok(Some(ref next_page_result)) => {
                // info!("next_page_response Some");
                let new_tweets: Option<Vec<Tweet>> = next_page_result.clone().into_data();
                if let Some(new_tweets) = new_tweets {
                    tweets = [tweets, new_tweets].concat();
                }
                let new_response = next_page_result.next_page().await;
                parse_error_response(&new_response).await;
                next_page_response = new_response;
            }
        }
    }
    tweets
}

pub async fn get_tweets(
    twitter_api: &TwitterApi<BearerToken>,
    tweet_ids: Vec<NumericId>,
) -> Option<Vec<Tweet>> {
    info!("API - get_tweets - num_tweet_ids: {:?} ", tweet_ids.len());
    let tweets_response = twitter_api
        .get_tweets(tweet_ids)
        .tweet_fields(TWEET_FIELDS)
        .send()
        .await;
    parse_error_response(&tweets_response).await;
    let tweets = tweets_response.unwrap().into_data();
    tweets
}

//split items into max 100 elements
pub fn split_requests_into_max_amount<T>(items: Vec<T>) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut count = 0usize;
    let items_len = items.len();

    let mut split_items_vec: Vec<Vec<T>> = vec![];
    loop {
        if count + MAX_TWEET_RESULTS >= items_len {
            // last elements in vec
            split_items_vec.push(items[count..items_len].to_vec());
            break;
        }
        split_items_vec.push(items[count..count + MAX_TWEET_RESULTS].to_vec());
        count += MAX_TWEET_RESULTS;
    }
    split_items_vec
}

// Retry logic:

pub async fn parse_error_response<T>(response: &Result<T, Error>) {
    sleep(Duration::from_millis(REQUEST_SLEEP_DURATION)).await;
    match response {
        Err(twitter_v2::Error::Request(ref e)) => {
            if e.is_timeout() {
                log::warn!("Request error timeout: {}", e);
            } else if e.is_connect() {
                log::warn!("Request error connect: {}", e);
            } else if e.status() == Some(reqwest::StatusCode::TOO_MANY_REQUESTS) {
                log::warn!("Request error too many requests: {}", e);
            } else {
                log::warn!("Request error unhandled: {}", e);
            }
        }
        Err(twitter_v2::Error::Api(ref e)) => {
            if e.status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                log::warn!("Api error due to 429 response {}", e);
            } else {
                log::warn!("Api error unhandled: {}", e);
            }
        }
        Err(e) => {
            log::warn!("generic error unhandled {}", e);
        }
        Ok(_) => {}
    }
}

// Source: https://github.com/jpopesculian/twitter-v2-rs/issues/8
// type DefaultRateLimiter = governor::RateLimiter<governor::state::NotKeyed, governor::state::InMemoryState, governor::clock::DefaultClock>;
// type OptionResponse<Auth, T, Meta> = twitter_v2::Result<Option<twitter_v2::ApiResponse<Auth, Vec<T>, Meta>>>;
// type Response<Auth, T, Meta> = twitter_v2::Result<twitter_v2::ApiResponse<Auth, Vec<T>, Meta>>;

// pub async fn get_list_members<Auth>(api: &TwitterApi<Auth>, rl: &DefaultRateLimiter, list_id: u64) -> Result<Vec<User>>
// where Auth: Authorization + Send + Sync + std::fmt::Debug {
//   get_all_pages(rl,
//                 async || {
//                   api.get_list_members(list_id)
//                     .user_fields([UserField::Id, UserField::Name])
//                     .max_results(100)
//                     .send()
//                     .await
//                 },
//                 None)
//     .await
//     .with_context(|| format!("Failed to fetch list members for list {list_id}"))
// }

// pub async fn retry<Auth, T, Meta, F, Fut>(rl: &DefaultRateLimiter, func: F) -> OptionResponse<Auth, T, Meta>
// where Auth: Authorization + Send + Sync + std::fmt::Debug,
//       T: serde::de::DeserializeOwned + Clone + Debug + Send + Sync,
//       F: Fn() -> Fut,
//       Fut: std::future::Future<Output = OptionResponse<Auth, T, Meta>> {
//   loop {
//     rl.until_ready().await;
//     match func().await {
//       Err(twitter_v2::Error::Request(ref e)) if e.is_timeout() || e.is_connect() => {
//         warn!(error=?e, "retrying due to request error");
//       },
//       Err(twitter_v2::Error::Api(ref e)) if e.status == reqwest::StatusCode::TOO_MANY_REQUESTS => {
//         warn!(error=?e, "retrying due to 429 response");
//         // TODO(db48x): extend the twitter_v2 crate to expose the
//         //   rate–limiting information provided by the twitter api
//       },
//       response => {
//         if let Ok(Some(ref r)) = response {
//           tracing::info!(url=%r.url(), "success");
//         }
//         return response;
//       }
//     }
//   }
// }

// pub async fn get_all_pages<Auth, T, Meta, F, Fut>(rl: &DefaultRateLimiter,
//                                                   func: F,
//                                                   expected: Option<usize>) -> Result<Vec<T>>
// where Auth: Authorization + Send + Sync + std::fmt::Debug,
//       T: serde::de::DeserializeOwned + Clone + Debug + Send + Sync,
//       Meta: twitter_v2::meta::PaginationMeta + serde::de::DeserializeOwned + Send + Sync,
//       F: Fn() -> Fut,
//       Fut: std::future::Future<Output = Response<Auth, T, Meta>> {
//   let mut items: Vec<T> = match expected {
//     Some(v) => Vec::with_capacity(v),
//     _ => Vec::new(),
//   };
//   let mut response: OptionResponse<Auth, T, Meta> = retry(rl, async || func().await.map(Some)).await;
//   loop {
//     match response {
//       Err(e) => return Err(anyhow!(e)),
//       Ok(None) => break,
//       Ok(Some(ref r)) => {
//         if let Some(new_items) = r.data() {
//           items.extend_from_slice(&new_items);
//         }
//         if matches!(r.meta(), Some(meta) if meta.next_token().is_some()) {
//           response = retry(rl, async || r.next_page().await).await;
//         } else {
//           break;
//         }
//       }
//     }
//   }
//   Ok(items)
// }
