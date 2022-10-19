import { NewsFeedUrl } from "graphql/generated/graphql";
import { timeSince } from "app/time";
import Link from "next/link";

interface NewsContentProps {
  newsFeedUrls: NewsFeedUrl[];
}

// get first 10 words of a string
function getWordStr(str: string) {
  return str != null ? str.split(/\s+/).slice(0, 20).join(" ") + " ...": "";
}

export default function NewsContent(props: NewsContentProps) {
  //{`${newsFeedUrl.urlScore}. `}
  return (
    <div className="container mx-auto p-2">
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
                  <p className="text-lg">
                      <a href={newsFeedUrl.expandedUrlParsed} className="hover:underline">{newsFeedUrl?.title}</a>
                  </p>
                  <p className="text-sm">
                      {getWordStr(newsFeedUrl?.description)}
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
                        {`Shared by @${
                          newsFeedUrl.firstReferencedByUsername
                        } ${newsFeedUrl.numReferences > 1 ? `and ${newsFeedUrl.numReferences - 1} others` :  ""} | ${timeSince(new Date(newsFeedUrl.createdAt * 1000))}`}
                      </a>
                    </Link>
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
