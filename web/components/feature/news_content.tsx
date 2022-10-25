import { NewsFeedUrl } from "graphql/generated/graphql";
import { timeSince } from "app/time";
import Link from "next/link";
import NewsHeader from "components/generic/news_header";

interface NewsContentProps {
  newsFeedUrls: NewsFeedUrl[];
}

function sharedByText(newsFeedUrl: NewsFeedUrl): String {
  var sharedByText = `Shared by @${newsFeedUrl.firstReferencedByUsername}`
  var numReferencesText = ""
  if(newsFeedUrl.numReferences > 2){
    numReferencesText = `and ${newsFeedUrl.numReferences - 1} others`
  }else if(newsFeedUrl.numReferences == 2){
    numReferencesText = `and 1 other`
  }
  var dateText = timeSince(new Date(newsFeedUrl.createdAt * 1000))
  return `${sharedByText}${numReferencesText} | ${dateText}`
}

export default function NewsContent(props: NewsContentProps) {
  //{`${newsFeedUrl.urlScore}. `}
  
  return (
    <>
      <NewsHeader title="News" subtitle="Trending articles shared by climate scientists and activists"/>
      <div className="container px-4 w-full md:max-w-3xl mx-auto">
        <ul>
          {props.newsFeedUrls &&
            props.newsFeedUrls.map(
              (newsFeedUrl: NewsFeedUrl, index: number) => {
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
                            <b>{newsFeedUrl?.title}</b> ({newsFeedUrl?.expandedUrlHost})
                          </p>
                          {/* <p className="ml-4 text-sm text-gray-500">
                            
                          </p> */}
                        </div>
                      </a>


                      {/* Host */}

                      {/* Subtitle */}
                      <p className="text-base text-gray-400 mt-1">
                        <Link
                          href={{
                            pathname: "/news_item/[item_id]",
                            query: { item_id: newsFeedUrl.urlId }
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
              }
            )}
        </ul>
      </div>
    </>
  );
}

