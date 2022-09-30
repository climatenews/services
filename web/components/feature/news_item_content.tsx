import {
  NewsFeedUrlDirectReference,
  NewsFeedUrlIndirectReference
} from "graphql/generated/graphql";

interface NewsItemContentProps {
  newsFeedUrlDirectReferences: NewsFeedUrlDirectReference[];
  newsFeedUrlIndirectReferences: NewsFeedUrlIndirectReference[];
}

export default function NewsItemContent(props: NewsItemContentProps) {
  return (
    <div className="container mx-auto ">
      <h3 className="text-2xl font-bold text-gray-900 text-left my-4">
        Shares
      </h3>

      {props.newsFeedUrlDirectReferences &&
        props.newsFeedUrlDirectReferences.map(
          (newsFeedUrlDirectReference: NewsFeedUrlDirectReference) => {
            return (
              <>
                <p className="text-m font-bold">Direct References</p>
                <ul>
                  <li key={newsFeedUrlDirectReference.text}>
                    <p className="text-m font-bold">
                      {newsFeedUrlDirectReference.username}
                    </p>
                    <p className="text-m">{newsFeedUrlDirectReference.text}</p>
                  </li>
                </ul>
              </>
            );
          }
        )}

      {props.newsFeedUrlIndirectReferences &&
        props.newsFeedUrlIndirectReferences.map(
          (newsFeedUrlIndirectReference: NewsFeedUrlIndirectReference) => {
            return (
              <>
                <p className="text-m font-bold">Indirect References</p>
                <ul>
                  <li key={newsFeedUrlIndirectReference.text}>
                    <p className="text-m font-bold">
                      {newsFeedUrlIndirectReference.username}
                    </p>
                    <p className="text-m">
                      {newsFeedUrlIndirectReference.text}
                    </p>
                  </li>
                </ul>
              </>
            );
          }
        )}
    </div>
  );
}
