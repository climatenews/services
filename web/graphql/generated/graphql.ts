import { GraphQLClient } from "graphql-request";
import * as Dom from "graphql-request/dist/types.dom";
import gql from "graphql-tag";
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = {
  [K in keyof T]: T[K];
};
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & {
  [SubKey in K]?: Maybe<T[SubKey]>;
};
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & {
  [SubKey in K]: Maybe<T[SubKey]>;
};
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
};

export type NewsFeedUrl = {
  __typename?: "NewsFeedUrl";
  createdAt: Scalars["Int"];
  description?: Maybe<Scalars["String"]>;
  expandedUrlHost: Scalars["String"];
  expandedUrlParsed: Scalars["String"];
  numReferences: Scalars["Int"];
  title?: Maybe<Scalars["String"]>;
  urlId: Scalars["Int"];
  urlScore: Scalars["Int"];
};

export type NewsFeedUrlDirectReference = {
  __typename?: "NewsFeedUrlDirectReference";
  createdAtStr: Scalars["String"];
  text: Scalars["String"];
  username: Scalars["String"];
};

export type NewsFeedUrlIndirectReference = {
  __typename?: "NewsFeedUrlIndirectReference";
  createdAtStr: Scalars["String"];
  referencedTweetKind: Scalars["String"];
  referencedTweetText: Scalars["String"];
  text: Scalars["String"];
  username: Scalars["String"];
};

export type Query = {
  __typename?: "Query";
  newsFeedUrlDirectReferences: Array<NewsFeedUrlDirectReference>;
  newsFeedUrlIndirectReferences: Array<NewsFeedUrlIndirectReference>;
  newsFeedUrls: Array<NewsFeedUrl>;
};

export type QueryNewsFeedUrlDirectReferencesArgs = {
  urlId: Scalars["Int"];
};

export type QueryNewsFeedUrlIndirectReferencesArgs = {
  urlId: Scalars["Int"];
};

export type GetNewsFeedUrlReferencesQueryVariables = Exact<{
  urlId: Scalars["Int"];
}>;

export type GetNewsFeedUrlReferencesQuery = {
  __typename?: "Query";
  newsFeedUrlDirectReferences: Array<{
    __typename?: "NewsFeedUrlDirectReference";
    text: string;
    username: string;
    createdAtStr: string;
  }>;
  newsFeedUrlIndirectReferences: Array<{
    __typename?: "NewsFeedUrlIndirectReference";
    text: string;
    referencedTweetText: string;
    referencedTweetKind: string;
    username: string;
    createdAtStr: string;
  }>;
};

export type GetNewsFeedUrlsQueryVariables = Exact<{ [key: string]: never }>;

export type GetNewsFeedUrlsQuery = {
  __typename?: "Query";
  newsFeedUrls: Array<{
    __typename?: "NewsFeedUrl";
    urlId: number;
    urlScore: number;
    numReferences: number;
    title?: string | null;
    description?: string | null;
    expandedUrlParsed: string;
    expandedUrlHost: string;
    createdAt: number;
  }>;
};

export const GetNewsFeedUrlReferencesDocument = gql`
  query GetNewsFeedUrlReferences($urlId: Int!) {
    newsFeedUrlDirectReferences(urlId: $urlId) {
      text
      username
      createdAtStr
    }
    newsFeedUrlIndirectReferences(urlId: $urlId) {
      text
      referencedTweetText
      referencedTweetKind
      username
      createdAtStr
    }
  }
`;
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

export type SdkFunctionWrapper = <T>(
  action: (requestHeaders?: Record<string, string>) => Promise<T>,
  operationName: string,
  operationType?: string
) => Promise<T>;

const defaultWrapper: SdkFunctionWrapper = (
  action,
  _operationName,
  _operationType
) => action();

export function getSdk(
  client: GraphQLClient,
  withWrapper: SdkFunctionWrapper = defaultWrapper
) {
  return {
    GetNewsFeedUrlReferences(
      variables: GetNewsFeedUrlReferencesQueryVariables,
      requestHeaders?: Dom.RequestInit["headers"]
    ): Promise<GetNewsFeedUrlReferencesQuery> {
      return withWrapper(
        (wrappedRequestHeaders) =>
          client.request<GetNewsFeedUrlReferencesQuery>(
            GetNewsFeedUrlReferencesDocument,
            variables,
            { ...requestHeaders, ...wrappedRequestHeaders }
          ),
        "GetNewsFeedUrlReferences",
        "query"
      );
    },
    GetNewsFeedUrls(
      variables?: GetNewsFeedUrlsQueryVariables,
      requestHeaders?: Dom.RequestInit["headers"]
    ): Promise<GetNewsFeedUrlsQuery> {
      return withWrapper(
        (wrappedRequestHeaders) =>
          client.request<GetNewsFeedUrlsQuery>(
            GetNewsFeedUrlsDocument,
            variables,
            { ...requestHeaders, ...wrappedRequestHeaders }
          ),
        "GetNewsFeedUrls",
        "query"
      );
    }
  };
}
export type Sdk = ReturnType<typeof getSdk>;
