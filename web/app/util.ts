export function capitalize(s: String): String {
  return s[0].toUpperCase() + s.slice(1);
}

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
