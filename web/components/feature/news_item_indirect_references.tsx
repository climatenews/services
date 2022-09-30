import { NewsFeedUrlIndirectReference } from "graphql/generated/graphql";
import {capitalize } from "app/util"

interface NewsItemIndirectReferencesProps {
  newsFeedUrlIndirectReferences: NewsFeedUrlIndirectReference[];
}

export default function NewsItemIndirectReferences(
  props: NewsItemIndirectReferencesProps
) {
  return (
    <>
      <p className="text-m font-bold mt-4">Indirect References</p>
      <div className="grid grid-cols-2 gap-4">
        {props.newsFeedUrlIndirectReferences.map(
          (newsFeedUrlIndirectReference: NewsFeedUrlIndirectReference) => {
            return (
              <div
                key={newsFeedUrlIndirectReference.text}
                className="border-solid border-2 border-sky-500 rounded-md p-4"
              >
                <p className="text-m font-medium">
                  {`${capitalize(newsFeedUrlIndirectReference.referencedTweetKind)} by `}
                  <a
                    href={`https://twitter.com/${newsFeedUrlIndirectReference.username}`}
                    className="hover:underline"
                  >
                    {newsFeedUrlIndirectReference.username}
                  </a>
                </p>

                <p className="text-m">{newsFeedUrlIndirectReference.text}</p>
                {/* <p className="text-m">{newsFeedUrlIndirectReference.referencedTweetText}</p> */}
              </div>
            );
          }
        )}
      </div>
    </>
  );
}
