import create from "zustand";
import { NewsFeedUrl, getSdk } from "graphql/generated/graphql";
import { graphQLClient } from "graphql/client";
import { log } from "./log";

interface NewsFeedUrlState {
  newsFeedUrls: NewsFeedUrl[];
  isLoading: boolean;
  getNewsFeedUrls: () => void;
}

export const useNewsFeedUrlStore = create<NewsFeedUrlState>(
  log((set: any, get: any) => ({
    newsFeedUrls: [],
    isLoading: false,
    getNewsFeedUrls: async () => {
      if (get().newsFeedUrls.length == 0) {
        set({ isLoading: true });
        const sdk = getSdk(graphQLClient);
        const response = await sdk.GetNewsFeedUrls();
        set({ newsFeedUrls: response.newsFeedUrls, isLoading: false });
      }
    }
  }))
);
