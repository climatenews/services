import { mkdir, writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

const SITE_ROOT = path.resolve(__dirname, "..");
const PUBLIC_DIR = path.join(SITE_ROOT, "public");
const SITEMAP_DIR = path.join(PUBLIC_DIR, "sitemap");

const GRAPHQL_API_URL =
  process.env.GRAPHQL_API_URL || "http://localhost:8000/graphql";
const PUBLIC_DOMAIN = process.env.PUBLIC_DOMAIN || "climatenews.app";
const BASE_URL = `https://${PUBLIC_DOMAIN}`;

const SITE_LAUNCH_YEAR = 2026;
const SITE_LAUNCH_MONTH = 8;

function getMonthsSinceLaunch() {
  const currentDate = new Date();
  const toYear = currentDate.getFullYear();
  const toMonth = currentDate.getMonth();
  const months = [{ year: SITE_LAUNCH_YEAR, month: SITE_LAUNCH_MONTH }];

  for (let year = SITE_LAUNCH_YEAR; year <= toYear; year++) {
    let monthNum = year === SITE_LAUNCH_YEAR ? SITE_LAUNCH_MONTH : 0;
    const monthLimit = year === toYear ? toMonth : 11;

    for (; monthNum <= monthLimit; monthNum++) {
      months.push({ year, month: monthNum + 1 });
    }
  }
  return months;
}

async function fetchSlugs(month, year) {
  const query = `
    query GetSitemapNewsFeedUrlSlugs($month: Int!, $year: Int!) {
      sitemapNewsFeedUrlSlugs(month: $month, year: $year)
    }
  `;
  const response = await fetch(GRAPHQL_API_URL, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ query, variables: { month, year } })
  });
  if (!response.ok) {
    throw new Error(
      `GraphQL request failed (${response.status}): ${await response.text()}`
    );
  }
  const payload = await response.json();
  if (payload.errors) {
    throw new Error(JSON.stringify(payload.errors));
  }
  return payload.data.sitemapNewsFeedUrlSlugs;
}

function generateMainSiteMap() {
  return `<?xml version="1.0" encoding="UTF-8"?>
  <urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
    <url>
      <loc>${BASE_URL}</loc>
    </url>
    <url>
      <loc>${BASE_URL}/about</loc>
    </url>
  </urlset>`;
}

function generateSiteMap(urlSlugs) {
  return `<?xml version="1.0" encoding="UTF-8"?>
  <urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  ${urlSlugs
    .map((urlSlug) => {
      return `
    <url>
      <loc>${BASE_URL}/news_feed/${urlSlug}</loc>
    </url>
  `;
    })
    .join("")}
  </urlset>`;
}

function generateSiteMapIndex(months) {
  return `<?xml version="1.0" encoding="UTF-8"?>
  <sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
    <sitemap>
      <loc>${BASE_URL}/sitemap/main.xml</loc>
    </sitemap>
    ${months
      .map((month) => {
        return `
    <sitemap>
      <loc>${BASE_URL}/sitemap/${month.month}-${month.year}.xml</loc>
    </sitemap>
  `;
      })
      .join("")}
  </sitemapindex>`;
}

async function generate() {
  await mkdir(SITEMAP_DIR, { recursive: true });

  const months = getMonthsSinceLaunch();

  await writeFile(path.join(PUBLIC_DIR, "sitemap.xml"), generateSiteMapIndex(months));
  await writeFile(
    path.join(SITEMAP_DIR, "main.xml"),
    generateMainSiteMap()
  );

  for (const { month, year } of months) {
    const urlSlugs = await fetchSlugs(month, year);
    await writeFile(
      path.join(SITEMAP_DIR, `${month}-${year}.xml`),
      generateSiteMap(urlSlugs)
    );
  }

  console.log(
    `Generated sitemaps in ${SITEMAP_DIR} for ${months.length} month(s)`
  );
}

generate().catch((error) => {
  console.error("Failed to generate sitemaps:", error);
  process.exit(1);
});
