-- News feed by url_score
SELECT
	nfu.url_id, 
	nfu.url_score,
	nfu.num_references,
    tu.title,
    tu.description
FROM
	news_feed_url as nfu
    JOIN news_tweet_url as tu ON tu.id = nfu.url_id
	
ORDER BY
    url_score DESC
LIMIT 20    
    
    
--Tweets by URL id
SELECT
	u.username, 
	t.text, 
	nrtu.url_id, 
    tu.title,
    tu.description
FROM
	news_referenced_tweet_url as nrtu
    JOIN news_tweet_url as tu ON tu.id = nrtu.url_id
    JOIN news_tweet as t ON t.tweet_id = nrtu.tweet_id
	JOIN news_twitter_user as u ON t.author_id = u.user_id	
	
WHERE
	nrtu.url_id = 20349