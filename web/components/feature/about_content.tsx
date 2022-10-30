import NewsHeader from "components/generic/news_header";
import Link from "next/link";

export default function AboutContent() {
  return (
    <>
      <NewsHeader title="About" subtitle="Who we are" />
      <div className="container w-full md:max-w-3xl mx-auto">
        {/* Content */}
        <div className="relative py-4">
          <div className="mx-auto max-w-md px-4 sm:max-w-3xl">
            <p className="mt-8 text-2xl font-extrabold tracking-tight text-gray-900">
              Mission
            </p>
            <p className="mx-auto mt-5 text-xl text-gray-500">
              We aim to help tackle climate change by creating tools to improve
              climate related education.
            </p>
            <p className="mx-auto mt-5 max-w-prose text-xl text-gray-500">
              Our source code is on{" "}
              <Link href="https://github.com/climate-action">
                <a className="font-medium hover:underline">GitHub</a>
              </Link>{" "}
              and everyone is free to review and contribute to our code base.
            </p>

            <p className="mt-8 text-2xl font-extrabold tracking-tight text-gray-900">
              Climate News
            </p>
            <p className="mx-auto mt-5 max-w-prose text-xl text-gray-500">
              The news feed algorithm is score driven and based on many factors. We use Twitter to follow users.
            </p>
            <p className="mx-auto mt-5 max-w-prose text-xl text-gray-500">
              A users score is based on the number of followers they have, the number of lists they appear in and the number of
              tweets that have been referenced by other users.
            </p>
            <p className="mx-auto mt-5 max-w-prose text-xl text-gray-500">
              The article score is the sum of the users scores that shared the article. It is reduced over time by a time decay function.
            </p>
            <p className="mx-auto mt-5 max-w-prose text-xl text-gray-500">
              We trained an AI model using the{" "}
              <Link href="https://openai.com">
                <a className="font-medium hover:underline">OpenAI API</a>
              </Link>{" "}
              to classify each article. To show only climate related articles on
              the news feed.
            </p>
            <p className="mt-8 text-2xl font-extrabold tracking-tight text-gray-900">
              About Us
            </p>
            <p className="mx-auto mt-5 max-w-prose text-xl text-gray-500">
              Made with 💚 in Nanaimo, Canada by{" "}
              <Link href="https://twitter.com/patrickf_ca">
                <a className="font-medium hover:underline">
                  Patrick Fitzgerald
                </a>
              </Link>
            </p>
          </div>
        </div>
      </div>
    </>
  );
}
