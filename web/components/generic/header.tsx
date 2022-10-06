import { getCurrentYear } from "app/time";
import { useState } from "react";

interface HeaderProps {
  title?: string;
}

export default function Header(props: HeaderProps) {
  const [isNavOpen, setIsNavOpen] = useState(false);

  return (
    <div className="flex items-center justify-between my-4">
      <h3 className="text-2xl font-bold text-gray-900 text-left">
        {props.title}
      </h3>
      <nav>
        <section className="flex">
          <div
            className="space-y-2"
            onClick={() => setIsNavOpen((prev) => !prev)}
          >
            <span className="block h-0.5 w-8 animate-pulse bg-gray-900"></span>
            <span className="block h-0.5 w-8 animate-pulse bg-gray-900"></span>
            <span className="block h-0.5 w-8 animate-pulse bg-gray-900"></span>
          </div>

          <div className={isNavOpen ? "showMenuNav" : "hideMenuNav"}>
            <div
              className="absolute top-0 right-0 px-8 py-8"
              onClick={() => setIsNavOpen(false)}
            >
              <svg
                className="h-8 w-8 text-gray-900"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
              >
                <line x1="18" y1="6" x2="6" y2="18" />
                <line x1="6" y1="6" x2="18" y2="18" />
              </svg>
            </div>
            <ul className="flex flex-col items-center justify-around min-h-[150px]">
              <li className="text-3xl font-bold border-b border-gray-400 my-8">
                <a href="/">Climate News</a>
              </li>
              <li className="text-3xl font-bold my-8">
                <a href="/about">About</a>
              </li>
            </ul>
            
          </div>
        </section>
      </nav>
      <style>{`
      .hideMenuNav {
        display: none;
      }
      .showMenuNav {
        display: block;
        position: absolute;
        width: 100%;
        height: 100vh;
        top: 0;
        left: 0;
        background: white;
        z-index: 10;
        display: flex;
        flex-direction: column;
        justify-content: space-evenly;
        align-items: center;
      }
    `}</style>
    </div>
  );
}
