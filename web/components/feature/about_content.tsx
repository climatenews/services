import NewsHeader from "components/generic/news_header";

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
              Our aim is to help tackle climate change by creating open source tools to improve climate related education. 
          </p> 
          <p className="mx-auto mt-5 max-w-prose text-xl text-gray-500">
              We plan to make tools for individuals and corporations to take meaningful climate action.
          </p>
          <p className="mx-auto mt-5 max-w-prose text-xl text-gray-500">
            Our source code is hosted on GitHub and everyone is free to review and contribute to the codebase.
          </p>
          
          <p className="mt-8 text-2xl font-extrabold tracking-tight text-gray-900">
            Climate News
          </p>
          <p className="mx-auto mt-5 max-w-prose text-xl text-gray-500">
          Our first feature, Climate News, shows trending articles shared by climate scientists and activists on Twitter.
          </p>
          <p className="mx-auto mt-5 max-w-prose text-xl text-gray-500">
          We use an news feed algorithm to give each person a score based on follwer count, the number of lists they apper in and how many times they have been referenced by other climate scientists and activists.
          </p>

          <p className="mt-8 text-2xl font-extrabold tracking-tight text-gray-900">
            About Us
          </p>
          <p className="mx-auto mt-5 max-w-prose text-xl text-gray-500">
            Climate Action Collective is a project by Patrick Fitzgerald.
          </p>
          <p className="mx-auto mt-5 max-w-prose text-xl text-gray-500">
            Made with love in Nanaimo, BC, Canada
          </p>
          </div>
        </div>
      </div>
    </>
  );
}
