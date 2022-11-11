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
            {/* <p className="mt-4 text-2xl font-extrabold tracking-tight text-gray-900">
              Mission
            </p>
            <p className="mx-auto mt-5 text-lg text-gray-600">
              Climate Action Collective is an open source project aiming to improve climate
              related education and amplify the voices of by climate scientists,
              organizations and activists.
            </p> */}

            <p className="mt-8 text-2xl font-extrabold tracking-tight text-gray-900">
              Climate News
            </p>
            <p className="mx-auto mt-5  text-lg text-gray-600">
              The climate news feed shows trending articles shared by climate
              scientists, organizations and activists on Twitter.
            </p>

            <p className="mx-auto mt-5  text-lg text-gray-600">
              About ~3500 Twitter accounts are imported from the lists below to generate the
              user list:
            </p>
            <ul className="mx-auto ml-5 mt-5 max-w-prose list-disc">
              <li className="mx-auto mt-2 text-lg text-gray-600">
                <Link href="https://twitter.com/i/lists/1586920047964205057">
                  <a className="font-medium hover:underline">Climate heros</a>
                </Link>
                {" - "}
                <Link href="https://twitter.com/climate_act_col">
                  <a className="font-medium hover:underline">
                    @climate_act_col
                  </a>
                </Link>
              </li>
              <li className="mx-auto mt-1  text-lg text-gray-600">
                <Link href="https://twitter.com/i/lists/1053067173961326594">
                  <a className="font-medium hover:underline">
                    scientists who do climate
                  </a>
                </Link>
                {" - "}
                <Link href="https://twitter.com/KHayhoe">
                  <a className="font-medium hover:underline">@KHayhoe</a>
                </Link>
              </li>
              <li className="mx-auto mt-1  text-lg text-gray-600">
                <Link href="https://twitter.com/i/lists/1308140854524162059">
                  <a className="font-medium hover:underline">Climate change</a>
                </Link>
                {" - "}
                <Link href="https://twitter.com/TwitterMoments">
                  <a className="font-medium hover:underline">@TwitterMoments</a>
                </Link>
                .
              </li>
            </ul>

            <p className="mx-auto mt-5  text-lg text-gray-600">
              The news feed uses a ranking algorithm to find trending articles and is based on multiple
              factors.
            </p>

            <p className="mx-auto mt-5  text-lg text-gray-600">
              Users are scored based on their follower count, the number of
              lists they appear in, and the amount of tweets that have been
              referenced by other users.
            </p>
            <p className="mx-auto mt-5  text-lg text-gray-600">
              An article score is based on the users that shared it and it will
              gradually decrease over time.
            </p>
            <p className="mx-auto mt-5  text-lg text-gray-600">
              The{" "}
              <Link href="https://developer.twitter.com/en/docs/twitter-api">
                <a className="font-medium hover:underline">Twitter API</a>
              </Link>{" "}
              is used to keep track of articles shared by users. The{" "}
              <Link href="https://openai.com">
                <a className="font-medium hover:underline">OpenAI API</a>
              </Link>{" "}
              is used to classify each article, to ensure only climate related
              articles appear in the news feed.
            </p>
            <p className="mx-auto mt-5 text-lg text-gray-600">
              Our source code is available on{" "}
              <Link href="https://github.com/climate-action">
                <a className="font-medium hover:underline">GitHub</a>
              </Link>
              {"."}
            </p>
            <p className="mt-8 text-2xl font-extrabold tracking-tight text-gray-900">
              About Us
            </p>
            <p className="mx-auto mt-5  text-lg text-gray-600">
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
