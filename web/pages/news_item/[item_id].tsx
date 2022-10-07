import type { NextPage } from "next";
import NewsItemContent from "components/feature/news_item_content";
import { NewsFeedUrlReference, getSdk } from "graphql/generated/graphql";
import { graphQLClient } from "graphql/client";
import Meta from "components/generic/meta";
import NavBar from "components/generic/navbar";

interface NewsItemPageProps {
  newsFeedUrlReferences: NewsFeedUrlReference[];
}

const NewsItemPage: NextPage<NewsItemPageProps> = ({
  newsFeedUrlReferences
}) => {
  return (
    <>
      <Meta />
      <NavBar pageRoute="/" />
      <NewsItemContent newsFeedUrlReferences={newsFeedUrlReferences} />
    </>
  );
};

export async function getServerSideProps(context: any) {
  const { item_id } = context.query;
  const sdk = getSdk(graphQLClient);
  const response = await sdk.GetNewsFeedUrlReferences({
    urlId: Number(item_id)
  });
  return {
    props: {
      newsFeedUrlReferences: response.newsFeedUrlReferences
    }
  };
}

export default NewsItemPage;
