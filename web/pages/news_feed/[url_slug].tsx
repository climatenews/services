import type { NextPage } from "next";
import NewsFeedUrlContent from "components/feature/news_feed_url_content";
import {
  NewsFeedUrlReference,
  NewsFeedUrl,
  getSdk
} from "graphql/generated/graphql";
import { graphQLClient } from "graphql/client";
import Meta from "components/generic/meta";
import NavBar from "components/generic/navbar";

interface NewsFeedUrlPageProps {
  newsFeedUrl: NewsFeedUrl;
  newsFeedUrlReferences: NewsFeedUrlReference[];
}

const NewsFeedUrlPage: NextPage<NewsFeedUrlPageProps> = ({
  newsFeedUrl,
  newsFeedUrlReferences
}) => {
  return (
    <>
      <Meta
        title={newsFeedUrl.title ?? undefined}
        description={newsFeedUrl.description ?? undefined}
      />
      <NavBar pageRoute="/" />
      <NewsFeedUrlContent
        newsFeedUrl={newsFeedUrl}
        newsFeedUrlReferences={newsFeedUrlReferences}
      />
    </>
  );
};

export async function getStaticPaths() {
  const sdk = getSdk(graphQLClient);
  const response = await sdk.GetNewsFeedUrlSlugs();
  const paths = response.newsFeedUrlSlugs.map((urlSlug) => ({
    params: { url_slug: urlSlug }
  }));
  return {
    paths,
    fallback: false
  };
}

export async function getStaticProps(context: any) {
  const { url_slug } = context.params;
  const sdk = getSdk(graphQLClient);
  const response = await sdk.GetNewsFeedUrlAndReferences({
    urlSlug: url_slug
  });
  return {
    props: {
      newsFeedUrl: response.newsFeedUrl,
      newsFeedUrlReferences: response.newsFeedUrlReferences
    }
  };
}

export default NewsFeedUrlPage;
