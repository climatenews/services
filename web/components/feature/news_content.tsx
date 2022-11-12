import { NewsFeedUrl } from "graphql/generated/graphql";
import Link from "next/link";
import NewsHeader from "components/generic/news_header";
import { sharedByText } from "app/util";

interface NewsContentProps {
  newsFeedUrls: NewsFeedUrl[];
}

export default function NewsContent(props: NewsContentProps) {
  //{`${newsFeedUrl.urlScore}. `}

  return (
    <>
      <NewsHeader
        title="Climate News"
        subtitle="Trending articles shared by climate scientists, organizations and activists."
      />
      <div className="container px-4 w-full md:max-w-3xl mx-auto">
        <ul>
          {props.newsFeedUrls &&
            props.newsFeedUrls.map((newsFeedUrl: NewsFeedUrl) => {
              return (
                <li
                  className="my-4 grid grid-cols-12"
                  key={newsFeedUrl.expandedUrlParsed}
                >
                  {/* Title */}
                  <div className="col-span-10">
                    <a
                      href={newsFeedUrl.expandedUrlParsed}
                      className="hover:underline"
                    >
                      <div className="flex flex-row">
                        <p className="text-base">
                          <b>{newsFeedUrl?.title}</b> (
                          {newsFeedUrl?.expandedUrlHost})
                        </p>
                      </div>
                    </a>

                    {/* Shares */}
                    <p className="text-base text-gray-500 mt-1">
                      <Link
                        href={{
                          pathname: "/news_feed/[url_id]",
                          query: { url_id: newsFeedUrl.urlId }
                        }}
                      >
                        <a className="hover:underline">
                          {sharedByText(newsFeedUrl)}
                        </a>
                      </Link>
                    </p>
                  </div>

                  {/* Image preview */}
                  <div className="col-span-2">
                    <Link
                      href={{
                        pathname: "/news_feed/[url_id]",
                        query: { url_id: newsFeedUrl.urlId }
                      }}
                    >
                      <a className=" hover:underline">
                        <img
                          className="mx-auto h-15 w-15 rounded lg:h-20 lg:w-20 lg:rounded-md"
                          src={
                            newsFeedUrl.previewImageThumbnailUrl
                              ? newsFeedUrl.previewImageThumbnailUrl
                              : "news_article_placeholder.png"
                          }
                          alt=""
                        />
                      </a>
                    </Link>
                  </div>
                </li>
              );
            })}
        </ul>
      </div>
    </>
  );
}
