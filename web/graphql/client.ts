
import { GraphQLClient } from 'graphql-request'

// TODO use ENV variable
export const graphQLClient = new GraphQLClient('http://0.0.0.0:8080/graphql')