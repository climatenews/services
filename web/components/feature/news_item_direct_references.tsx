import { NewsFeedUrlDirectReference } from "graphql/generated/graphql";

interface NewsItemDirectReferencesProps {
  newsFeedUrlDirectReferences: NewsFeedUrlDirectReference[];
}

export default function NewsItemDirectReferences(
  props: NewsItemDirectReferencesProps
) {
  return (
    <>
      <p className="text-m font-bold">Direct References</p>
      <div className="grid grid-cols-2 gap-4">
        {props.newsFeedUrlDirectReferences.map(
          (newsFeedUrlDirectReference: NewsFeedUrlDirectReference) => {
            return (
              <div
                key={newsFeedUrlDirectReference.text}
                className="border-solid border-2 border-sky-500 rounded-md p-4"
              >
                <p className="text-m font-medium">
                  <a
                    href={`https://twitter.com/${newsFeedUrlDirectReference.username}`}
                    className="hover:underline"
                  >
                    {newsFeedUrlDirectReference.username}
                  </a>
                </p>
                <p className="text-m">{newsFeedUrlDirectReference.text}</p>
              </div>
            );
          }
        )}
      </div>
    </>
  );
}
