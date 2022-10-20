export default function AboutContent() {
  return (
    <div>
      {/* Content */}
      <div className="relative py-8 sm:py-12 lg:py-16">
        <div className="mx-auto max-w-md px-4 text-center sm:max-w-3xl sm:px-6 lg:max-w-7xl lg:px-8">
          <img
            className="mx-auto h-24 w-auto mt-10"
            src="/cac_logo.svg"
            alt="Climate Action Collective logo"
          />

          {/* <h1 className="text-base font-semibold uppercase tracking-wider text-primary">
            Climate Action Collective
          </h1> */}
          <p className="mt-4 text-3xl font-extrabold tracking-tight text-gray-900 sm:text-4xl">
            About Us
          </p>
          <p className="mx-auto mt-5 max-w-prose text-base text-gray-500">
            Climate Action Collective is a benefit company registered in BC,
            Canada in 2022.
          </p>
          <p className="mx-auto mt-1 max-w-prose text-base text-gray-500">
            We aim to improve climate related education and make
            tools for individuals and corporations to take meaningful climate
            action.
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
  );
}
