curl https://api.openai.com/v1/completions \
  -H 'Content-Type: application/json' \
  -H 'Authorization: Bearer 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' \
  -d '{
  "model": "text-davinci-002",
  "prompt": "Text: Progress Snapshot - Climate Change Committee\nCategory: Climate\n---\nText: 2030 in Focus: Getting the Next\nDecade Right on Net-Zero - Canadian Climate Institute\nCategory: Climate\n---\nText: August 2022 Temperature Update - Berkeley Earth\nCategory: Climate\n---\nText: Jan. 6 committee announces next hearing will be held Oct. 13\nCategory: Other\n---\nText: Surface air temperature for September 2022\nCategory:",
  "max_tokens": 6,
  "temperature": 0
}'




Text: Progress Snapshot - Climate Change Committee
Category: Climate
---
Text: 2030 in Focus: Getting the Next Decade Right on Net-Zero - Canadian Climate Institute
Category: Climate
---
Text: August 2022 Temperature Update - Berkeley Earth
Category: Climate
---
Text: Jan. 6 committee announces next hearing will be held Oct. 13
Category: Other
---
Text: Surface air temperature for September 2022
Category: