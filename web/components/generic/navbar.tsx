import { Fragment } from "react";
import { Disclosure, Menu, Transition } from "@headlessui/react";
import { Bars3Icon, BellIcon, XMarkIcon } from "@heroicons/react/24/outline";

const navigation = [
  { name: "News", href: "/" },
  { name: "Jobs", href: "/jobs" },
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
    <nav id="header" className="fixed w-full z-10 top-0 ">
      <div className="w-full md:max-w-4xl mx-auto flex flex-wrap items-center justify-between my-6 lg:pl-4">

        <div className="flex flex-1 items-center justify-center sm:items-stretch sm:justify-start">
          <div className="flex flex-shrink-0 items-center">
            <img
              className="block h-8 w-auto"
              src="/cac_logo.svg"
              alt="Your Company"
            />
          </div>
          <div className="flex flex-shrink-0 items-center">
          <a
            className="text-gray-900 text-base no-underline hover:no-underline font-extrabold text-lg pl-4"
            href="#"
          >
            Climate Action Collective
          </a>
          </div>
        </div>


        <div className="block lg:hidden pr-4">
          <button
            id="nav-toggle"
            className="flex items-center px-3 py-2 border rounded text-gray-500 border-gray-600 hover:text-gray-900 hover:border-green-500 appearance-none focus:outline-none"
          >
            <svg
              className="fill-current h-3 w-3"
              viewBox="0 0 20 20"
              xmlns="http://www.w3.org/2000/svg"
            >
              <title>Menu</title>
              <path d="M0 3h20v2H0V3zm0 6h20v2H0V9zm0 6h20v2H0v-2z" />
            </svg>
          </button>
        </div>

        <div
          className="w-full flex-grow lg:flex lg:items-center lg:w-auto hidden lg:block mt-2 lg:mt-0 bg-gray-100 md:bg-transparent z-20"
          id="nav-content"
        >
          <ul className="list-reset lg:flex justify-end flex-1 items-center">
            
            
          {navigation.map((item) => (
            <li className="mr-3">
                        <a
                          key={item.name}
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
