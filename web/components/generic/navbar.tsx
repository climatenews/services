import Link from "next/link";

const navigation = [
  { name: "News", href: "/" },
  { name: "About", href: "/about" }
];

function classNames(...classes: any[]) {
  return classes.filter(Boolean).join(" ");
}

function isCurrentRoute(pageRoute: string, href: string): boolean {
  return pageRoute === href;
}

interface NavBarProps {
  pageRoute: string;
}

export default function NavBar(props: NavBarProps) {
  return (
    <nav id="header" className="container px-4 w-full md:max-w-3xl mx-auto ">
      <div className="w-full mx-auto flex flex-wrap items-center justify-between py-6">
        <div className="flex flex-1">
          <div className="flex flex-shrink-0 items-center">
            <img
              className="block h-8 w-auto"
              src="/cac_logo.svg"
              alt="Your Company"
            />
          </div>
          <div className="flex flex-shrink-0 items-center">
            <Link href="/">
              <a className="text-gray-900 text-base no-underline hover:no-underline font-extrabold text-lg pl-4">
                Climate Action Collective
              </a>
            </Link>
          </div>
        </div>

        {/* Mobile button */}
        <div className="block lg:hidden">
          <button
            id="nav-toggle"
            className="flex items-center px-2 py-2 rounded text-gray-500  hover:text-gray-900 hover:border-green-500 appearance-none focus:outline-none"
          >
            <svg
              width="36px"
              height="36px"
              viewBox="0 0 48 48"
              xmlns="http://www.w3.org/2000/svg"
            >
              <title>Hmaburger menu</title>
              <path
                d="M41,14H7a2,2,0,0,1,0-4H41A2,2,0,0,1,41,14Z"
                fill="#000000"
              />
              <path
                d="M41,26H7a2,2,0,0,1,0-4H41A2,2,0,0,1,41,26Z"
                fill="#000000"
              />
              <path
                d="M41,38H7a2,2,0,0,1,0-4H41A2,2,0,0,1,41,38Z"
                fill="#000000"
              />
            </svg>
          </button>
        </div>

        <div
          className="w-full flex-grow lg:flex lg:items-center lg:w-auto hidden lg:block mt-2 lg:mt-0 bg-gray-100 md:bg-transparent z-20"
          id="nav-content"
        >
          <ul className="list-reset lg:flex justify-end flex-1 items-center">
            {navigation.map((item) => (
              <li className="mr-3" key={item.name}>
                <a
                  href={item.href}
                  className={classNames(
                    isCurrentRoute(props.pageRoute, item.href)
                      ? "text-gray-900 font-bold"
                      : "text-gray-600 hover:text-gray-900 hover:text-underline",
                    "py-2 px-4 inline-block no-underline"
                  )}
                  aria-current={
                    isCurrentRoute(props.pageRoute, item.href)
                      ? "page"
                      : undefined
                  }
                >
                  {item.name}
                </a>
              </li>
            ))}

            {/* <li className="mr-3">
              <a
                className=" "
                href="#"
              >
                News
              </a>
            </li>
            <li className="mr-3">
              <a
                className="text-gray-600 hover:text-gray-900 hover:text-underline"
                href="#"
              >
                Jobs
              </a>
            </li>
            <li className="mr-3">
              <a
                className="inline-block text-gray-600 no-underline hover:text-gray-900 hover:text-underline py-2 px-4"
                href="#"
              >
                About
              </a>
            </li> */}
          </ul>
        </div>
      </div>
    </nav>
  );
}
