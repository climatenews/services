import type { NextPage } from "next";
import Footer from "components/generic/footer";
import NewsItemContent from "components/feature/news_item_content";
import {
  NewsFeedUrlDirectReference,
  NewsFeedUrlIndirectReference,
  getSdk
} from "graphql/generated/graphql";
import { graphQLClient } from "graphql/client";

interface NewsItemPageProps {
  newsFeedUrlDirectReferences: NewsFeedUrlDirectReference[];
  newsFeedUrlIndirectReferences: NewsFeedUrlIndirectReference[];
}

const NewsItemPage: NextPage<NewsItemPageProps> = ({
  newsFeedUrlDirectReferences,
  newsFeedUrlIndirectReferences
}) => {
  return (
    <>
      <NewsItemContent
        newsFeedUrlDirectReferences={newsFeedUrlDirectReferences}
        newsFeedUrlIndirectReferences={newsFeedUrlIndirectReferences}
      />
      <Footer />
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
      newsFeedUrlDirectReferences: response.newsFeedUrlDirectReferences,
      newsFeedUrlIndirectReferences: response.newsFeedUrlIndirectReferences
    }
  };
}

export default NewsItemPage;
