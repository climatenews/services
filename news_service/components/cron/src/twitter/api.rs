use tokio::time::{sleep, Duration};
use twitter_v2::authorization::BearerToken;
use twitter_v2::id::NumericId;
use twitter_v2::prelude::PaginableApiResponse;
use twitter_v2::query::{Exclude, TweetField, UserField};
use twitter_v2::{Tweet, TwitterApi, User};

use crate::news_feed::constants::{MAX_TWEET_RESULTS, REQUEST_SLEEP_DURATION};
use crate::util::convert::i64_to_numeric_id;
use crate::util::helpers::past_365_days;

static TWEET_FIELDS: [TweetField; 6] = [
    TweetField::AuthorId,
    TweetField::CreatedAt,
    TweetField::InReplyToUserId,
    TweetField::ConversationId,
    TweetField::Entities,
    TweetField::ReferencedTweets,
];

static USER_FIELDS: [UserField; 1] = [UserField::PublicMetrics];

pub async fn get_users_by_username(
    twitter_api: &TwitterApi<BearerToken>,
    usernames: Vec<&str>,
) -> Option<Vec<User>> {
    //TODO split in to max 100 users per request
    println!("API - get_users_by_username: {}", usernames.len());
    let users = twitter_api
        .get_users_by_usernames(usernames)
        .user_fields(USER_FIELDS)
        .send()
        .await
        .unwrap()
        .into_data();
    sleep(Duration::from_millis(REQUEST_SLEEP_DURATION)).await;
    users
}

pub async fn get_users_by_author_id(
    twitter_api: &TwitterApi<BearerToken>,
    author_ids: Vec<i64>,
) -> Vec<User> {
    println!("API - get_users_by_author_id: {}", author_ids.len());
    let mut user_vec: Vec<User> = vec![];
    let split_author_ids_vec =
        split_requests_into_max_amount(author_ids.iter().map(|i| i64_to_numeric_id(*i)).collect());
    for split_author_ids in split_author_ids_vec {
        let users = twitter_api
            .get_users(split_author_ids)
            .user_fields(USER_FIELDS)
            .send()
            .await
            .unwrap()
            .into_data()
            .unwrap();
        user_vec = [user_vec, users].concat();
        sleep(Duration::from_millis(REQUEST_SLEEP_DURATION)).await;
    }

    user_vec
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

pub async fn get_user_tweets(
    twitter_api: &TwitterApi<BearerToken>,
    user_id: NumericId,
    last_tweet_id: Option<NumericId>,
) -> Vec<Tweet> {
    let start_time = past_365_days();

    println!("API - get_user_tweets: {}", user_id);
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
            .unwrap()
    } else {
        twitter_api
            .get_user_tweets(user_id)
            .max_results(MAX_TWEET_RESULTS)
            .tweet_fields(TWEET_FIELDS)
            .start_time(start_time)
            .exclude([Exclude::Replies])
            .send()
            .await
            .unwrap()
    };

    if let Some(new_tweets) = tweets_api_response.clone().into_data() {
        tweets = [tweets, new_tweets].concat();
    }
    sleep(Duration::from_millis(REQUEST_SLEEP_DURATION)).await;
    let mut next_page_response = tweets_api_response.next_page().await;
    sleep(Duration::from_millis(REQUEST_SLEEP_DURATION)).await;
    loop {
        match next_page_response {
            Err(e) => {
                println!("next_page_response error: {}", e);
                break;
            } //return Err(anyhow!(e)),
            Ok(None) => {
                // println!("next_page_response None");
                break;
            }
            Ok(Some(ref next_page_result)) => {
                // println!("next_page_response Some");
                let new_tweets: Option<Vec<Tweet>> = next_page_result.clone().into_data();
                if let Some(new_tweets) = new_tweets {
                    tweets = [tweets, new_tweets].concat();
                }
                let new_response = next_page_result.next_page().await;
                sleep(Duration::from_millis(REQUEST_SLEEP_DURATION)).await;
                next_page_response = new_response;
            }
        }
    }
    println!("API - num_tweets: {}", tweets.len());
    tweets
}

pub async fn get_tweets(
    twitter_api: &TwitterApi<BearerToken>,
    tweet_ids: Vec<NumericId>,
) -> Option<Vec<Tweet>> {
    println!("API - get_tweets - num_tweet_ids: {:?} ", tweet_ids.len());
    let tweets = twitter_api
        .get_tweets(tweet_ids)
        .tweet_fields(TWEET_FIELDS)
        .send()
        .await
        .unwrap()
        .into_data();
    sleep(Duration::from_millis(REQUEST_SLEEP_DURATION)).await;
    tweets
}
