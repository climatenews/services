import NewsHeader from "components/generic/news_header";

export default function AboutContent() {
  return (
    <>
      <NewsHeader title="About" subtitle="Who we are" />
      <div className="container px-4 w-full md:max-w-3xl mx-auto">
        {/* Content */}
        <div className="relative py-4">
          <div className="mx-auto max-w-md px-4 text-center sm:max-w-3xl sm:px-6 lg:max-w-7xl lg:px-8">
            <p className="mx-auto mt-5 max-w-prose">
              Climate Action Collective is a benefit company registered in BC,
              Canada in 2022.
            </p>
            <p className="mx-auto mt-1 max-w-prose">
              We aim to improve climate related education and make tools for
              individuals and corporations to take meaningful climate action.
            </p>
            {/* <p className="mt-8 text-3xl font-extrabold tracking-tight text-gray-900 sm:text-4xl">
            Engaging climate content
          </p>
          <p className="mx-auto mt-5 max-w-prose text-xl text-gray-500">
            Lorem ipsum dolor sit amet,
            consectetur adipiscing elit, sed do eiusmod tempor incididunt ut.
            Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do
            eiusmod tempor incididunt ut.
          </p> */}
          </div>
        </div>
      </div>
    </>
  );
}
