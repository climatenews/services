import { GraphQLClient } from "graphql-request";

const graphqlApiUrl = process.env.GRAPHQL_API_URL || "http://news_api:8000/graphql";
export const graphQLClient = new GraphQLClient(graphqlApiUrl);
