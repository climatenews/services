import { NewsFeedUrl } from "graphql/generated/graphql";
import { timeSince } from "app/time";
import Link from "next/link";

interface NewsContentProps {
  newsFeedUrls: NewsFeedUrl[];
}

export default function NewsContent(props: NewsContentProps) {
  //{`${newsFeedUrl.urlScore}. `}
  return (
    <div className="container mx-auto ">
      <ul>
        {props.newsFeedUrls &&
          props.newsFeedUrls.map((newsFeedUrl: NewsFeedUrl, index: number) => {
            return (
              <li key={newsFeedUrl.expandedUrlParsed}>
                <div className="flex items-baseline">
                  <p className="text-lg mr-1">
                    <a
                      className="hover:underline"
                      href={newsFeedUrl.expandedUrlParsed}
                    >
                      {newsFeedUrl?.title}
                    </a>
                  </p>
                  <p className="text-xs text-gray-400">
                    ({newsFeedUrl.expandedUrlHost})
                  </p>
                </div>
                <p className="text-s text-gray-400">
                  <Link
                    href={{
                      pathname: "/news_item/[item_id]",
                      query: { item_id: newsFeedUrl.urlId }
                    }}
                  >
                    <a className="hover:underline">
                      {newsFeedUrl.numReferences} Shares
                    </a>
                  </Link>
                  {` | ${timeSince(new Date(newsFeedUrl.createdAt * 1000))}`}
                </p>
              </li>
            );
          })}
      </ul>
    </div>
  );
}
