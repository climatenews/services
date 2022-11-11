import { unescapeHTML } from "app/util";
import { NewsFeedUrlReference } from "graphql/generated/graphql";

interface NewsFeedUrlReferencesProps {
  newsFeedUrlReferences: NewsFeedUrlReference[];
}

export default function NewsFeedUrlDirectReferences(
  props: NewsFeedUrlReferencesProps
) {
  return (
    <>
      <div className="grid lg:grid-cols-2 sm:grid-cols-1 gap-4">
        {props.newsFeedUrlReferences.map(
          (newsFeedUrlReference: NewsFeedUrlReference) => {
            return (
              <div
                key={newsFeedUrlReference.tweetId}
                className="border-solid border-2 border-gray-300 rounded-md p-4"
              >
                <div className="flex flex-row">
                  <a
                    href={`https://twitter.com/${newsFeedUrlReference.authorUsername}`}
                  >
                    <p className="text-m font-medium">
                      @{newsFeedUrlReference.authorUsername}
                    </p>
                  </a>{" "}
                  <a
                    href={`https://twitter.com/${newsFeedUrlReference.authorUsername}/status/${newsFeedUrlReference.tweetId}`}
                    className="ml-2 hover:underline"
                  >
                    <img
                      className="mx-auto h-5 w-5 rounded lg:h-5 lg:w-5"
                      src={"/twitter_icon.svg"}
                      alt="twitter_icon"
                    />
                  </a>
                </div>

                <p className="text-m mt-1">
                  {unescapeHTML(newsFeedUrlReference.tweetText)}
                </p>
                {/* <p className="text-m font-bold">
                  Retweeted by{" "}
                  {newsFeedUrlReference.retweetedByUsernames.join(", ")}
                </p> */}
              </div>
            );
          }
        )}
      </div>
    </>
  );
}
