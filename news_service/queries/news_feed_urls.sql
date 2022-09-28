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
    
    


