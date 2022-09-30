import {
  NewsFeedUrlDirectReference,
  NewsFeedUrlIndirectReference
} from "graphql/generated/graphql";

import NewsItemDirectReferences from "./news_item_direct_references";
import NewsItemIndirectReferences from "./news_item_indirect_references";

interface NewsItemContentProps {
  newsFeedUrlDirectReferences: NewsFeedUrlDirectReference[];
  newsFeedUrlIndirectReferences: NewsFeedUrlIndirectReference[];
}

export default function NewsItemContent(props: NewsItemContentProps) {
  return (
    <div className="container mx-auto ">
      <h3 className="text-2xl font-bold text-gray-900 text-left my-4">
        Shares
      </h3>

      {props.newsFeedUrlDirectReferences.length > 0 && (
        <NewsItemDirectReferences
          newsFeedUrlDirectReferences={props.newsFeedUrlDirectReferences}
        />
      )}
      {props.newsFeedUrlIndirectReferences.length > 0 && (
        <NewsItemIndirectReferences
          newsFeedUrlIndirectReferences={props.newsFeedUrlIndirectReferences}
        />
      )}
    </div>
  );
}
