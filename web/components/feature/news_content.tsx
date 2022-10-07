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
                className="my-2 grid grid-cols-12 justify-items-start"
                key={newsFeedUrl.expandedUrlParsed}
              >

                <div className="col-start-4 col-span-5 justify-items-start">
                  <p className="text-xl">
                    <a
                      className="hover:underline"
                      href={newsFeedUrl.expandedUrlParsed}
                    >
                      {newsFeedUrl?.title}
                    </a>
                  </p>

                  <p className="text-base text-gray-400">
                    {newsFeedUrl.expandedUrlHost}
                    {` | ${timeSince(new Date(newsFeedUrl.createdAt * 1000))}`}
                  </p>

                  <p className="text-lg text-gray-400">
                    <Link
                      href={{
                        pathname: "/news_item/[item_id]",
                        query: { item_id: newsFeedUrl.urlId }
                      }}
                    >
                      <a className=" hover:underline">
                        {`${newsFeedUrl.numReferences} ${
                          newsFeedUrl.numReferences == 1 ? "Share" : "Shares"
                        }`}
                      </a>
                    </Link>
                  </p>
                </div>
                <div className="col-span-1 justify-items-start ml-2">
                  <Link
                    href={{
                      pathname: "/news_item/[item_id]",
                      query: { item_id: newsFeedUrl.urlId }
                    }}
                  >
                    <a className=" hover:underline">
                      <img
                        className="mx-auto h-20 w-20 rounded-md"
                        src={
                          newsFeedUrl.previewImageThumbnailUrl
                            ? newsFeedUrl.previewImageThumbnailUrl
                            : "https://via.placeholder.com/150"
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
