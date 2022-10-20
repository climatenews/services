import { NewsFeedUrl } from "graphql/generated/graphql";
import { timeSince } from "app/time";
import Link from "next/link";

interface NewsContentProps {
  newsFeedUrls: NewsFeedUrl[];
}

// get first 10 words of a string
function getWordStr(str: string) {
  return str != null ? str.split(/\s+/).slice(0, 20).join(" ") + " ..." : "";
}

export default function NewsContent(props: NewsContentProps) {
  //{`${newsFeedUrl.urlScore}. `}
  return (
    <>
      <div className="container w-full md:max-w-3xl mx-auto">
        <div className="relative py-2 sm:py-6 lg:py-4">
          <div className="mx-auto max-w-md pt-8 text-start sm:max-w-3xl lg:max-w-7xl ">
              <h1 className="font-bold font-sans break-normal text-gray-900 pt-6 pb-2 text-xl md:text-2xl">News</h1>
              <p className="text-sm md:text-base font-normal text-gray-600">A news feed of climate related articles recently shared by <a className="hover:underline">climate scientists and climate activists.</a></p>
          </div>
        </div>


        <ul>
          {props.newsFeedUrls &&
            props.newsFeedUrls.map((newsFeedUrl: NewsFeedUrl, index: number) => {
              return (
                <li
                  className="my-2 grid lg:grid-cols-9 grid-cols-6"
                  key={newsFeedUrl.expandedUrlParsed}
                >
                  {/* Title */}
                  <div className="col-span-8">

                    <a
                      href={newsFeedUrl.expandedUrlParsed}
                      className="hover:underline"
                    >
                      <p className="text-base">
                        {newsFeedUrl?.title}
                      </p>
                      <p className="text-sm text-gray-500">({newsFeedUrl?.expandedUrlHost})</p>
                    </a>

                    {/* Host */}


                    {/* Subtitle */}
                    <p className="text-sm text-gray-400 mt-1">
                      <Link
                        href={{
                          pathname: "/news_item/[item_id]",
                          query: { item_id: newsFeedUrl.urlId }
                        }}
                      >
                        <a className="hover:underline">
                          {`Shared by @${newsFeedUrl.firstReferencedByUsername} ${newsFeedUrl.numReferences > 1
                            ? `and ${newsFeedUrl.numReferences - 1} others`
                            : ""
                            } | ${timeSince(
                              new Date(newsFeedUrl.createdAt * 1000)
                            )}`}
                        </a>
                      </Link>
                    </p>

                  </div>
                  {/* Image preview */}
                  {/* <div className="col-span-1">
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
                </div> */}

                </li>

              );
            })}
        </ul>
      </div>
    </>
  );
}
