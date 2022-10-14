
#OpenAI

openai tools fine_tunes.prepare_data -f news_feed_urls.jsonl
openai api fine_tunes.create -t "news_feed_urls_prepared_train.jsonl" -v "news_feed_urls_prepared_valid.jsonl" --compute_classification_metrics --classification_n_classes 1


 prompt has to end with the indicator string ` \n\n###\n\n`