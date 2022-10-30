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
                className="border-solid border-2 border-gray-400 rounded-md p-4"
              >
                <p className="text-m font-medium">
                  <a
                    href={`https://twitter.com/${newsFeedUrlReference.authorUsername}`}
                    className="hover:underline"
                  >
                    @{newsFeedUrlReference.authorUsername}
                  </a>
                </p>
                <p className="text-m">
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
