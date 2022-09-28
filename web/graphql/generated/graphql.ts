import { GraphQLClient } from 'graphql-request';
import * as Dom from 'graphql-request/dist/types.dom';
import gql from 'graphql-tag';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
};

export type NewsFeedUrl = {
  __typename?: 'NewsFeedUrl';
  createdAt: Scalars['Int'];
  description?: Maybe<Scalars['String']>;
  expandedUrlHost: Scalars['String'];
  expandedUrlParsed: Scalars['String'];
  numReferences: Scalars['Int'];
  title?: Maybe<Scalars['String']>;
  urlId: Scalars['Int'];
  urlScore: Scalars['Int'];
};

export type Query = {
  __typename?: 'Query';
  newsFeedUrls: Array<NewsFeedUrl>;
};

export type GetNewsFeedUrlsQueryVariables = Exact<{ [key: string]: never; }>;


export type GetNewsFeedUrlsQuery = { __typename?: 'Query', newsFeedUrls: Array<{ __typename?: 'NewsFeedUrl', urlId: number, urlScore: number, numReferences: number, title?: string | null, description?: string | null, expandedUrlParsed: string, expandedUrlHost: string, createdAt: number }> };


export const GetNewsFeedUrlsDocument = gql`
    query GetNewsFeedUrls {
  newsFeedUrls {
    urlId
    urlScore
    numReferences
    title
    description
    expandedUrlParsed
    expandedUrlHost
    createdAt
  }
}
    `;

export type SdkFunctionWrapper = <T>(action: (requestHeaders?:Record<string, string>) => Promise<T>, operationName: string, operationType?: string) => Promise<T>;


const defaultWrapper: SdkFunctionWrapper = (action, _operationName, _operationType) => action();

export function getSdk(client: GraphQLClient, withWrapper: SdkFunctionWrapper = defaultWrapper) {
  return {
    GetNewsFeedUrls(variables?: GetNewsFeedUrlsQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<GetNewsFeedUrlsQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<GetNewsFeedUrlsQuery>(GetNewsFeedUrlsDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'GetNewsFeedUrls', 'query');
    }
  };
}
export type Sdk = ReturnType<typeof getSdk>;