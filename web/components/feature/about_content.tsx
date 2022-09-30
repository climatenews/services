import { BeakerIcon } from "@heroicons/react/24/solid";

const features = [
  // {
  //     name: 'Transparent fee structure',
  //     description: 'A flat fee of 0.50% for all trades using Walet Swap.',
  //     icon: VariableIcon,
  // },
  {
    name: "Open source",
    description:
      "The source code for Walet is hosted on GitHub and everyone is free to review, audit, and contribute to the Walet codebase.",
    icon: BeakerIcon
  },
  // {
  //     name: 'Multiple DEXs',
  //     description: 'Support for Uniswap V2, Sushiswap, Curve. With more to come soon.',
  //     icon: CollectionIcon,
  // },
  // {
  //     name: 'Fast Quotes',
  //     description: 'Off-chain routing to ensure qutes are recieved as quick as possible.',
  //     icon: LightningBoltIcon,
  // },
  {
    name: "No 3rd party trackers",
    description: "Walet has taken a strong stance on privacy and trackers.",
    icon: BeakerIcon
  },
  {
    name: "Powerful API",
    description: "An easy to use GraphQL API to get quotes and make trades.",
    icon: BeakerIcon
  }
];

export default function AboutContent() {
  return (
    <div>
      {/* Content */}
      <div className="relative py-8 sm:py-12 lg:py-16">
        <div className="mx-auto max-w-md px-4 text-center sm:max-w-3xl sm:px-6 lg:max-w-7xl lg:px-8">
          {/* <img
                        className="mx-auto h-24 w-auto my-2"
                        src="/walet-logo-mobile.svg"
                        alt="Walet logo"
                    /> */}
          <h2 className="text-base font-semibold uppercase tracking-wider text-primary">
            Climate Action Collective
          </h2>
          <p className="mt-2 text-3xl font-extrabold tracking-tight text-gray-900 sm:text-4xl">
            Get smart, take action.
          </p>
          <p className="mx-auto mt-5 max-w-prose text-xl text-gray-500">
            Climate Action Collective is a global initiative to move leaders to
            act on climate.
          </p>
          <div className="mt-12">
            <div className="grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-3">
              {features.map((feature) => (
                <div key={feature.name} className="pt-6">
                  <div className="flow-root rounded-lg bg-gray-50 px-6 pb-8">
                    <div className="-mt-6">
                      <div>
                        <span className="inline-flex items-center justify-center rounded-md bg-primary p-3 shadow-lg">
                          <feature.icon
                            className="h-6 w-6 text-blue-500"
                            aria-hidden="true"
                          />
                        </span>
                      </div>
                      <h3 className="mt-8 text-lg font-medium tracking-tight text-gray-900">
                        {feature.name}
                      </h3>
                      <p className="mt-5 text-base text-gray-500">
                        {feature.description}
                      </p>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
