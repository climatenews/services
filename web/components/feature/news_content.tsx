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
              <li
                className="my-2 grid lg:grid-cols-9 grid-cols-6"
                key={newsFeedUrl.expandedUrlParsed}
              >

                {/* Title */}
                <div className="lg:col-start-3 col-span-5">
                  <p className="text-lg lg:text-xl">
                  <Link
                      href={{
                        pathname: "/news_item/[item_id]",
                        query: { item_id: newsFeedUrl.urlId }
                      }}
                    >
                    <a
                      className="hover:underline"
                    >
                      {newsFeedUrl?.title}
                    </a>
                    </Link>
                  </p>

                  <p className="text-xs text-sky-400">
                  <a
                      className="hover:underline"
                      href={newsFeedUrl.expandedUrlParsed}
                    >
                     {newsFeedUrl.displayUrl}
                    </a>
                  </p>

                  {/* Subtitle */}
                  <p className="text-sm text-gray-400 mt-1">
                  <Link
                      href={{
                        pathname: "/news_item/[item_id]",
                        query: { item_id: newsFeedUrl.urlId }
                      }}
                    >
                      <a className="hover:underline">
                        {`${newsFeedUrl.numReferences} ${
                          newsFeedUrl.numReferences == 1 ? "Share" : "Shares"
                        }`}
                      </a>
                    </Link>
                    {` | Tweeted by @${newsFeedUrl.firstReferencedByUsername} ${timeSince(new Date(newsFeedUrl.createdAt * 1000))}`}

                  </p>
                </div>
                {/* Image preview */}
                <div className="col-span-1">
                  <Link
                    href={{
                      pathname: "/news_item/[item_id]",
                      query: { item_id: newsFeedUrl.urlId }
                    }}
                  >
                    <a className=" hover:underline">
                      <img
                        className="mx-auto h-15 w-15 rounded lg:h-20 lg:w-20 lg:rounded-md"
                        src={
                          newsFeedUrl.previewImageThumbnailUrl
                            ? newsFeedUrl.previewImageThumbnailUrl
                            : "https://via.placeholder.com/150/FFFFFF"
                        }
                        alt="TODO"
                      />
                    </a>
                  </Link>
                </div>
              </li>
            );
          })}
      </ul>
    </div>
  );
}
