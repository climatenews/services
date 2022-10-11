import { Fragment } from "react";
import { Disclosure, Menu, Transition } from "@headlessui/react";
import { Bars3Icon, BellIcon, XMarkIcon } from "@heroicons/react/24/outline";

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
    <Disclosure as="nav" className="">
      {({ open }) => (
        <>
          <div className="mx-auto max-w-7xl px-2 sm:px-6 lg:px-8 py-4 sm:px-3 lg:px-4 border-b border-slate-900/5 bg-transparent">
            <div className="relative flex h-16 items-center justify-between">
              {/* Mobile menu button*/}
              {/* <div className="border-b absolute inset-y-0 left-0 flex items-center sm:hidden">
                <Disclosure.Button className="ba inline-flex items-center justify-center rounded-md p-2 text-gray-400 hover:bg-gray-700 hover:text-white focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white">
                  <span className="sr-only">Open main menu</span>
                  {open ? (
                    <XMarkIcon className="block h-6 w-6" aria-hidden="true" />
                  ) : (
                    <Bars3Icon className="block h-6 w-6" aria-hidden="true" />
                  )}
                </Disclosure.Button>
              </div> */}
              <div>
                <div className="flex flex-1 items-center justify-center sm:items-stretch sm:justify-start">
                  <div className="flex flex-shrink-0 items-center">
                    <img
                      className="block h-8 w-auto lg:hidden"
                      src="/cac_logo.jpg"
                      alt="Your Company"
                    />
                    <img
                      className="hidden h-8 w-auto lg:block"
                      src="/cac_logo.jpg"
                      alt="Your Company"
                    />
                  </div>
                  <div className="flex flex-shrink-0 items-center">
                    <p className="px-2 py-2 font-bold text-base lg:text-lg">
                      Climate Action Collective
                    </p>
                  </div>
                </div>
                <div className="flex flex-1 items-center justify-center sm:items-stretch sm:justify-start">
                  <div className="hidden sm:block">
                    <div className="flex space-x-4">
                      {navigation.map((item) => (
                        <a
                          key={item.name}
                          href={item.href}
                          className={classNames(
                            isCurrentRoute(props.pageRoute, item.href)
                              ? "underline underline-offset-4 font-bold"
                              : "",
                            "pr-3 py-2 text-sm lg:text-lg"
                          )}
                          aria-current={
                            isCurrentRoute(props.pageRoute, item.href)
                              ? "page"
                              : undefined
                          }
                        >
                          {item.name}
                        </a>
                      ))}
                    </div>
                  </div>
                </div>
              </div>

              {/* <div className="absolute inset-y-0 right-0 flex items-center pr-2 sm:static sm:inset-auto sm:ml-6 sm:pr-0">
                <button
                  type="button"
                  className="rounded-full bg-gray-800 p-1 text-gray-400 hover:text-white focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800"
                >
                  <span className="sr-only">View notifications</span>
                  <BellIcon className="h-6 w-6" aria-hidden="true" />
                </button>
              </div> */}
            </div>
          </div>

          <Disclosure.Panel className="sm:hidden">
            <div className="space-y-1 px-2 pt-2 pb-3">
              {navigation.map((item) => (
                <Disclosure.Button
                  key={item.name}
                  as="a"
                  href={item.href}
                  className={classNames(
                    isCurrentRoute(props.pageRoute, item.href) ? "" : "",
                    "block px-3 py-2 rounded-md text-base font-medium"
                  )}
                  aria-current={
                    isCurrentRoute(props.pageRoute, item.href)
                      ? "page"
                      : undefined
                  }
                >
                  {item.name}
                </Disclosure.Button>
              ))}
            </div>
          </Disclosure.Panel>
        </>
      )}
    </Disclosure>
  );
}
