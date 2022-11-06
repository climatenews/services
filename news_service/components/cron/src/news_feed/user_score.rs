use db::models::news_twitter_user::NewsTwitterUser;

pub fn calc_user_score(
    news_twitter_user: &NewsTwitterUser,
    user_referenced_tweets_count: i32,
) -> i32 {
    let followers_count = news_twitter_user.followers_count;

    // Followers
    // =IFS(B3 > 1000000, 40, B3 > 100000, 30, B3 > 10000, 20, B3 > 1000, 10, B3 <= 1000, 0)
    let followers_score = if followers_count > 1000000 {
        30
    } else if followers_count > 100000 {
        25
    } else if followers_count > 10000 {
        20
    } else if followers_count > 1000 {
        15
    } else {
        10
    };
    // Listed
    // =IFS(C3 > 5000, 20, C3 > 2000, 15,C3 > 1000, 10, C3 > 500, 5, C3 <= 500, 0)
    let listed_count = news_twitter_user.listed_count;
    let listed_score = if listed_count > 5000 {
        25
    } else if listed_count > 1000 {
        20
    } else if listed_count > 500 {
        15
    } else {
        10
    };

    // Tweet References
    // >100 = 40, >75 = 30, >50 = 20, >25 = 10 , <=25 = 0
    let user_referenced_tweets_score = if user_referenced_tweets_count > 100 {
        30
    } else if user_referenced_tweets_count > 75 {
        25
    } else if user_referenced_tweets_count > 50 {
        20
    } else if user_referenced_tweets_count > 25 {
        15
    } else {
        10
    };
    followers_score + listed_score + user_referenced_tweets_score
}
