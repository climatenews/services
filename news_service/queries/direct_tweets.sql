-- Direct tweet URLS
    
SELECT 
	tu.title, 
	u.username,  
	rtu.tweet_id,
	rtu.url_id,
	tu.expanded_url,
	t.text,
	t.created_at_str

FROM
	news_referenced_tweet_url as rtu 
	JOIN news_tweet_url as tu ON tu.id = rtu.url_id
	JOIN news_tweet as t ON t.tweet_id = rtu.tweet_id
	JOIN news_twitter_user as u ON t.author_id = u.user_id	
	
WHERE
    tu.is_twitter_url = False	
ORDER by  tu.title 