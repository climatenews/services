-- Indirect referenced tweet URLS
    
SELECT 
	tu.title, 
	u.username,  
	rt.referenced_tweet_id,
	rt.tweet_id,
	rtu.url_id,
	tu.expanded_url,
	t.text,
	t.created_at_str
 
FROM
    news_referenced_tweet as rt   
   	JOIN news_tweet as t ON t.tweet_id = rt.tweet_id  
   	JOIN news_twitter_user as u ON t.author_id = u.user_id   
	JOIN news_referenced_tweet_url as rtu ON rtu.tweet_id = rt.referenced_tweet_id 
	JOIN news_tweet_url as tu ON tu.id = rtu.url_id  
WHERE
    tu.is_twitter_url = False	
	AND (rt.referenced_tweet_kind = 'retweeted' OR rt.referenced_tweet_kind = 'quoted')
	AND t.in_reply_to_user_id IS NULL
	AND rt.referenced_tweet_id = 1570145799300067330  
ORDER by t.created_at DESC

    