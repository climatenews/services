-- User references without username 
SELECT
	t1.author_id,
	t1.tweet_id,
	t2.author_id as referenced_author_id,
	t2.tweet_id as referenced_tweet_id,
	rt.referenced_tweet_kind
FROM
	news_referenced_tweet as rt
	JOIN news_tweet as t1 ON t1.tweet_id = rt.tweet_id
	JOIN news_tweet as t2 ON t2.tweet_id = rt.referenced_tweet_id
	
WHERE 
	 t1.author_id != t2.author_id               
	AND (rt.referenced_tweet_kind = 'retweeted' OR rt.referenced_tweet_kind = 'quoted')
	AND t2.author_id = 21786618
	
	
-- User reference with username    
SELECT
	u1.username, 
	t1.author_id,
	t1.tweet_id,
	u2.username, 
	t2.author_id as referenced_author_id,
	t2.tweet_id as referenced_tweet_id,
	rt.referenced_tweet_kind
FROM
	news_referenced_tweet as rt
	JOIN news_tweet as t1 ON t1.tweet_id = rt.tweet_id
	JOIN news_tweet as t2 ON t2.tweet_id = rt.referenced_tweet_id
	JOIN news_twitter_user as u1 ON t1.author_id = u1.user_id
	JOIN news_twitter_user as u2 ON t2.author_id = u2.user_id
	
WHERE 
	t1.author_id != t2.author_id               
	AND (rt.referenced_tweet_kind = 'retweeted' OR rt.referenced_tweet_kind = 'quoted')
	AND t2.author_id = 21786618               


-- Twitter User reference with username    
SELECT
	u1.username, 
	t1.author_id,
	t1.tweet_id,
	u2.username, 
	t2.author_id as referenced_author_id,
	t2.tweet_id as referenced_tweet_id,
	rt.referenced_tweet_kind
FROM
	news_referenced_tweet as rt
	JOIN news_tweet as t1 ON t1.tweet_id = rt.tweet_id
	JOIN news_tweet as t2 ON t2.tweet_id = rt.referenced_tweet_id
	JOIN news_twitter_user as u1 ON t1.author_id = u1.user_id
	LEFT JOIN news_twitter_user as u2 ON t2.author_id = u2.user_id
	
WHERE 
	t1.author_id != t2.author_id               
	AND (rt.referenced_tweet_kind = 'retweeted' OR rt.referenced_tweet_kind = 'quoted')             
	


		