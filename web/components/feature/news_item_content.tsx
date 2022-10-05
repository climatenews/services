import {
  NewsFeedUrlReference
} from "graphql/generated/graphql";

import NewsItemReferences from "./news_item_references";

interface NewsItemContentProps {
  newsFeedUrlReferences: NewsFeedUrlReference[];
}

export default function NewsItemContent(props: NewsItemContentProps) {
  return (
    <div className="container mx-auto ">
      <h3 className="text-2xl font-bold text-gray-900 text-left my-4">
        Shares
      </h3>

      {props.newsFeedUrlReferences.length > 0 && (
        <NewsItemReferences
          newsFeedUrlReferences={props.newsFeedUrlReferences}
        />
      )}
    </div>
  );
}
