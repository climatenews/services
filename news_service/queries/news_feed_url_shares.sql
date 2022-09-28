
-- Direct Tweets by URL id
SELECT 
	t.text,
	t.created_at_str,
	u.username
FROM
    news_referenced_tweet as rt   
   	JOIN news_tweet as t ON t.tweet_id = rt.tweet_id  
   	JOIN news_twitter_user as u ON t.author_id = u.user_id   
	JOIN news_referenced_tweet_url as rtu ON rtu.tweet_id = rt.referenced_tweet_id 
	JOIN news_tweet_url as tu ON tu.id = rtu.url_id  
WHERE
    rtu.url_id = 43

-- Indirect Tweets by URL id
SELECT 
	t.text,
	t.created_at_str,
	u.username
FROM
	news_referenced_tweet_url as rtu 
	JOIN news_tweet_url as tu ON tu.id = rtu.url_id
	JOIN news_tweet as t ON t.tweet_id = rtu.tweet_id
	JOIN news_twitter_user as u ON t.author_id = u.user_id		
WHERE
    rtu.url_id = 43

