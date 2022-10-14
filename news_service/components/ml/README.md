
# OpenAI
Example notebook: https://github.com/openai/openai-cookbook/blob/main/examples/Fine-tuned_classification.ipynb


### Data Preparation
```
openai tools fine_tunes.prepare_data -f news_feed_urls.jsonl
export OPENAI_API_KEY=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
```

### Fine-tuning
```
openai api fine_tunes.create -t "news_feed_urls_prepared_train.jsonl" -v "news_feed_urls_prepared_valid.jsonl" --compute_classification_metrics --classification_positive_class " 1"
```
Note prompt has to end with the indicator string ` \n\n###\n\n`
```
Created fine-tune: ft-fqXnErmYT0dAuVR4Urok8hEe
Uploaded model: curie:ft-personal-2022-10-14-18-16-36
Uploaded result file: file-66KckvW4bbg4qcfy1LPR7Vmi
```

### Results
```
openai api fine_tunes.results -i ft-fqXnErmYT0dAuVR4Urok8hEe > result.csv
```
### Testing

```
openai api completions.create  --max-tokens 2 -m curie:ft-personal-2022-10-14-18-16-36 -p "Italy: Floods and rain kill at least 10 overnight - officials - Rescuers are searching for four others missing after torrential rainfall hit the Marche region overnight.\n\n###\n\n"

openai api completions.create  --max-tokens 2 -m curie:ft-personal-2022-10-14-18-16-36 -p "Vi är Aurora, vi ska stämma staten - Vi är en växande grupp barn, unga och vuxna som jobbar på att starta upp en stämning av svenska staten för deras katastrofala miljö- och klimatpolitik.\n\n###\n\n"

openai api completions.create  --max-tokens 2 -m curie:ft-personal-2022-10-14-18-16-36 -p "Former President Donald Trump invoked his Fifth Amendment right more than 440 times on Wednesday during a deposition by lawyers from New York Attorney General Letitia James’ office, according to multiple sources. - Former President Donald Trump invoked his Fifth Amendment right more than 440 times on Wednesday during a deposition by lawyers from New York Attorney General Letitia James’ office, according to multiple sources.\n\n###\n\n"

openai api completions.create  --max-tokens 2 -m curie:ft-personal-2022-10-14-18-16-36 -p "China’s Climate Goals Hinge on a $440 Billion Nuclear Buildout - China is planning at least 150 new reactors in the next 15 years, more than the rest of the world has built in the past 35.\n\n###\n\n"
```