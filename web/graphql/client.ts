import { GraphQLClient } from "graphql-request";

const graphqlApiUrl =
  process.env.GRAPHQL_API_URL || "http://0.0.0.0:8000/graphql";

const graphqlFetch: typeof fetch = async (input, init) => {
  const response = await fetch(input, init);
  const contentType = response.headers.get("content-type") || "";

  // graphql-request@5 does not parse this media type as JSON.
  if (contentType.includes("application/graphql-response+json")) {
    const body = await response.text();
    const headers = new Headers(response.headers);
    headers.set("content-type", "application/json");
    return new Response(body, {
      status: response.status,
      statusText: response.statusText,
      headers
    });
  }

  return response;
};

export const graphQLClient = new GraphQLClient(graphqlApiUrl, {
  fetch: graphqlFetch
});
