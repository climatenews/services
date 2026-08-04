
# ML

Climate article classification, used to decide whether a news feed URL is climate related.

## Current approach

The classifier runs in the cron service and uses `gpt-5-mini` through the Chat Completions API
with structured output, so no fine-tuned model is needed:

- Request: `POST /v1/chat/completions` with `model: gpt-5-mini`, a system prompt, the article
  title and description as the user message, `reasoning_effort: minimal`, and a strict JSON schema
  response format.
- Response: `{"is_climate_related": true}` or `{"is_climate_related": false}`.
- Source: `components/cron/src/openai/api.rs`

The classification is stored on `news_feed_url.is_climate_related` by
`components/cron/src/news_feed/algorithm/news_feed_v1.rs`.

## Historical fine-tuning

The service previously classified articles with a fine-tuned `curie` model
(`curie:ft-personal-2022-10-14-18-16-36`), trained on manually labeled news feed URLs. OpenAI
retired all legacy fine-tuned models on January 4, 2024, and the fine-tuning data export and
training pipeline has been removed.

`news_feed_urls.jsonl` and `news_feed_urls_trained.jsonl` contain the manually labeled examples
from that effort. They can be used as an evaluation set to measure the accuracy of the current
classifier.
