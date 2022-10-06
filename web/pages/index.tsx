import type { NextPage } from "next";
import NewsContent from "components/feature/news_content";
import { NewsFeedUrl, getSdk } from "graphql/generated/graphql";
import { graphQLClient } from "graphql/client";
import Header from "components/generic/header";

interface NewsPageProps {
  newsFeedUrls: NewsFeedUrl[];
}

const NewsPage: NextPage<NewsPageProps> = ({ newsFeedUrls }) => {
  return (
    <>
      <Header title="Climate News" />
      <NewsContent newsFeedUrls={newsFeedUrls} />
    </>
  );
};

export async function getServerSideProps(context: any) {
  const sdk = getSdk(graphQLClient);
  const response = await sdk.GetNewsFeedUrls();
  return {
    props: {
      newsFeedUrls: response.newsFeedUrls
    }
  };
}

export default NewsPage;
