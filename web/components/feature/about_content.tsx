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
            <p className="mt-6 text-2xl font-extrabold tracking-tight text-gray-900">
              Mission
            </p>
            <p className="mx-auto mt-5 text-xl text-gray-500">
              Climate Action Collective is an open source project that aims to
              help tackle climate change by creating tools to improve climate
              related education and amplify the voices of by climate scientists,
              organizations and activists.
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
              The news feed shows trending articles shared by climate
              scientists, organizations and activists. 
            </p>
            <p className="mx-auto mt-5 max-w-prose text-xl text-gray-500">
              Users are imported from the following 3 Twitter lists:
            </p>
            <p className="mx-auto mt-2 max-w-prose text-xl text-gray-500">
              <Link href="https://twitter.com/i/lists/1586920047964205057">
                <a className="font-medium hover:underline">Climate heros</a>
              </Link>
              {" by "}
              <Link href="https://twitter.com/climate_act_col">
                <a className="font-medium hover:underline">@climate_act_col</a>
              </Link>
            </p>
            <p className="mx-auto mt-1 max-w-prose text-xl text-gray-500">
              <Link href="https://twitter.com/i/lists/1053067173961326594">
                <a className="font-medium hover:underline">
                  scientists who do climate
                </a>
              </Link>
              {" by "}
              <Link href="https://twitter.com/KHayhoe">
                <a className="font-medium hover:underline">@KHayhoe</a>
              </Link>
            </p>
            <p className="mx-auto mt-1 max-w-prose text-xl text-gray-500">
              <Link href="https://twitter.com/i/lists/1308140854524162059">
                <a className="font-medium hover:underline">Climate change</a>
              </Link>
              {" by "}
              <Link href="https://twitter.com/TwitterMoments">
                <a className="font-medium hover:underline">@TwitterMoments</a>
              </Link>
              .
            </p>

            <p className="mx-auto mt-5 max-w-prose text-xl text-gray-500">
              The news feed uses a score driven algorithm and is based on
              multiple factors. 
            </p>
            <p className="mx-auto mt-5 max-w-prose text-xl text-gray-500">
              Users are given a score based on the number of followers they
              have, the number of lists they appear in and the number of their
              tweets that have been referenced by other users.
            </p>
            <p className="mx-auto mt-5 max-w-prose text-xl text-gray-500">
              An article is given a score based on the score of the users that
              shared the article and the time since it was first shared. The
              score for an article will gradually decrease over time.
            </p>
            <p className="mx-auto mt-5 max-w-prose text-xl text-gray-500">
             The{" "}
              <Link href="https://developer.twitter.com/en/docs/twitter-api">
                <a className="font-medium hover:underline">Twitter API</a>
              </Link>{" "}
              to keep track of articles shared by users.
              The{" "}
              <Link href="https://openai.com">
                <a className="font-medium hover:underline">OpenAI API</a>
              </Link>{" "}
              is used to classify each article, to ensure only climate related
              articles appear in the news feed.

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
              {"."}
            </p>
          </div>
        </div>
      </div>
    </>
  );
}
