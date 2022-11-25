# Climate Action Collective - Services

## Overview
`web` - Next.js frontend

## Running locally
### Prerequisites
- Docker & Docker Compose
- [Twitter API key](https://developer.twitter.com/en/docs/authentication/oauth-2-0/bearer-tokens)
- [OpenAI API key](https://openai.com/api/)

### Setting up the .env.dev file
```bash
# copy the sample .env file 
cp .env.sample .env.dev
```
Set the `OPENAI_API_KEY` & `TWITTER_BEARER_TOKEN` variables in `.env.dev`

### Test the app with Docker Compose
```bash
# Start the app
docker-compose up -d --build

# tail the logs
docker-compose logs --tail="all" -f
```

## Deploying
### Deploy the stack with Docker Swarm
```bash
# Initialize a docker swarm
docker swarm init 

# Deploy the stack
# TODO use docker secrets?
env $(cat .env.dev | xargs) docker stack deploy --compose-file docker-compose.yaml climate_action_stack 

# Remove the stack
docker stack rm climate_action_stack

# Display stack info
docker stack ps climate_action_stack

# Display service info
docker service ps climate_action_stack_db
docker service ps climate_action_stack_news_api
docker service ps climate_action_stack_news_cron
docker service ps climate_action_stack_web


# Stack logs for a service
docker service logs climate_action_stack_db --follow
docker service logs climate_action_stack_news_api --follow
docker service logs climate_action_stack_news_cron --follow
docker service logs climate_action_stack_web --follow

```