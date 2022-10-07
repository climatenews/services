import { NewsFeedUrlReference } from "graphql/generated/graphql";

interface NewsItemReferencesProps {
  newsFeedUrlReferences: NewsFeedUrlReference[];
}

export default function NewsItemDirectReferences(
  props: NewsItemReferencesProps
) {
  return (
    <>
      {/* <p className="text-m font-bold">Shares</p> */}
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
                    {newsFeedUrlReference.authorUsername}
                  </a>
                </p>
                <p className="text-m">{newsFeedUrlReference.tweetText}</p>
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
