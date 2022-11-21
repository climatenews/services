### Climate Action Collective Services

# Running locally
## Prerequisites
- Docker & Docker Compose
- [Twitter API key](https://developer.twitter.com/en/docs/authentication/oauth-2-0/bearer-tokens)
- [OpenAI API key](https://openai.com/api/)

## Setting up the .env.dev file
```
# copy the sample .env file 
cp .env.sample .env.dev
```
Set the OPENAI_API_KEY & TWITTER_BEARER_TOKEN variables in `.env.dev`.

## Test the app with Compose
```
# Start the app
docker-compose up -d --build

# tail the logs
docker-compose logs --tail="all" -f
```

# Deploying
## Deploy the stack to the swarm
```
# Deploy the stack
# TODO use docker secrets
env $(cat .env.dev | xargs) docker stack deploy --compose-file docker-compose.yaml climate_action_stack

# Display stack info
docker stack ps climate_action_stack

# Display service info
docker service ps climate_action_stack_web


# Remove the stack
docker stack rm climate_action_stack

# Stack logs for a service
docker service logs climate_action_stack_db --follow
docker service logs climate_action_stack_news_api --follow
docker service logs climate_action_stack_web --follow

```