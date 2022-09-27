
import { GraphQLClient } from 'graphql-request'

// TODO use ENV variable
export const graphQLClient = new GraphQLClient('http://news_api:8000/graphql')