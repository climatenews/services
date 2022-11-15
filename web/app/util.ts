import { NewsFeedUrl } from "graphql/generated/graphql";
import { timeSince } from "app/time";

export const unescapeHTML = (str: string) =>
  str.replace(
    /&amp;|&lt;|&gt;|&#39;|&quot;/g,
    (tag) =>
      ({
        "&amp;": "&",
        "&lt;": "<",
        "&gt;": ">",
        "&#39;": "'",
        "&quot;": '"'
      }[tag] || tag)
  );

export function sharedByText(newsFeedUrl: NewsFeedUrl): String {
  var sharedByText = `Shared by @${newsFeedUrl.firstReferencedByUsername}`;
  var numReferencesText = "";
  if (newsFeedUrl.numReferences > 2) {
    numReferencesText = ` and ${newsFeedUrl.numReferences - 1} others`;
  } else if (newsFeedUrl.numReferences == 2) {
    numReferencesText = ` and 1 other`;
  }
  return `${sharedByText}${numReferencesText} | ${dateText(newsFeedUrl)}`;
}

export function retweetedByText(retweetedByUsernames: String[]): String {
  var retweetedByText = "";
  if (retweetedByUsernames.length > 0){
    retweetedByText = `Retweeted by ${retweetedByUsernames.reduce((res, k, i) => [`@${res}`, `@${k}`].join(i  === retweetedByUsernames.length - 1 ? ' and ' : ', '))}`
  }
  return retweetedByText;
}

export function dateText(newsFeedUrl: NewsFeedUrl): String {
  return timeSince(new Date(newsFeedUrl.createdAt * 1000));
}


