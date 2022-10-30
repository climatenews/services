import NewsHeader from "components/generic/news_header";
import { NewsFeedUrlReference, NewsFeedUrl } from "graphql/generated/graphql";

import NewsFeedUrlReferences from "./news_feed_url_references";

interface NewsFeedUrlContentProps {
  newsFeedUrl: NewsFeedUrl;
  newsFeedUrlReferences: NewsFeedUrlReference[];
}

export default function NewsFeedUrlContent(props: NewsFeedUrlContentProps) {
  return (
    <>
      {/* TODO use grid, new header with back button */}
      <NewsHeader title="" subtitle="&larr; Back" />
      <div className="container px-4 w-full md:max-w-3xl mx-auto">
        

      <h3 className="text-2xl text-gray-900 text-left my-4">
          {props.newsFeedUrl?.title}
        </h3>

        <a className=" hover:underline">
          <img
            className="h-25 w-25 rounded lg:h-30 lg:w-30 lg:rounded-md"
            src={
              props.newsFeedUrl.previewImageThumbnailUrl
                ? props.newsFeedUrl.previewImageThumbnailUrl
                : "https://via.placeholder.com/150/FFFFFF"
            }
            alt="TODO"
          />
        </a>

        <h4 className="text-xl text-gray-900 text-left my-4">
          {props.newsFeedUrl?.description}
        </h4>
        <a className="text-blue-600 text-sm hover:underline my-2" href={props.newsFeedUrl?.expandedUrlParsed}>
          {props.newsFeedUrl?.displayUrl} &rarr;
        </a>

        <h3 className="text-xl font-bold text-gray-900 text-left my-4">
          Shares
        </h3>

        {props.newsFeedUrlReferences.length > 0 && (
          <NewsFeedUrlReferences
            newsFeedUrlReferences={props.newsFeedUrlReferences}
          />
        )}
      </div>
    </>
  );
}
