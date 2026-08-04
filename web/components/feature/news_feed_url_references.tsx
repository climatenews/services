import { repostedByText, unescapeHTML } from "app/util";
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
            const postRkey = newsFeedUrlReference.postUri
              .split("/")
              .pop();
            return (
              <div
                key={newsFeedUrlReference.postUri}
                className="border-solid border-2 border-gray-300 rounded-md p-4"
              >
                <div className="flex flex-row">
                  <a
                    href={`https://bsky.app/profile/${newsFeedUrlReference.authorHandle}`}
                    className="hover:underline"
                    target="_blank"
                    rel="noopener noreferrer"
                  >
                    <p className="text-m font-medium">
                      @{newsFeedUrlReference.authorHandle}
                    </p>
                  </a>{" "}
                  <a
                    href={`https://bsky.app/profile/${newsFeedUrlReference.authorHandle}/post/${postRkey}`}
                    className="ml-2 hover:underline"
                    target="_blank"
                    rel="noopener noreferrer"
                  >
                    <img
                      className="mx-auto h-5 w-5"
                      src={"/bluesky_icon.svg"}
                      alt="Bluesky"
                    />
                  </a>
                </div>

                <p className="text-m mt-2">
                  {unescapeHTML(newsFeedUrlReference.postText)}
                </p>

                {newsFeedUrlReference.repostedByHandles.length > 0 && (
                  <div className="flex flex-row mt-2">
                    <img
                      className="h-4 w-4 mt-1 mr-1"
                      src={"/repost_icon.svg"}
                      alt="repost_icon"
                    />
                    <p className="text-m font-light italic text-gray-800 ">
                      {repostedByText(
                        newsFeedUrlReference.repostedByHandles
                      )}
                    </p>
                  </div>
                )}
              </div>
            );
          }
        )}
      </div>
    </>
  );
}
