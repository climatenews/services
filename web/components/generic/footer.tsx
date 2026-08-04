import { getCurrentYear } from "app/time";

const navigation = {
  social: [
    {
      name: "Bluesky",
      href: "https://bsky.app/profile/climatenews.app",
      icon: (props: any) => (
        <svg fill="currentColor" viewBox="0 0 24 24" {...props}>
          <path d="M12 2C6.477 2 2 6.477 2 12c0 5.523 4.477 10 10 10s10-4.477 10-10c0-5.523-4.477-10-10-10zm5.51 13.5c-.06.168-.24.288-.42.3l-4.32.18v3.9c0 .24-.18.42-.42.42h-.48c-.24 0-.42-.18-.42-.42v-3.9l-4.32-.18c-.18 0-.36-.12-.42-.3-.06-.18 0-.36.12-.48l3.3-3c.06-.06.12-.12.12-.24v-4.74c0-.24.18-.42.42-.42h.48c.24 0 .42.18.42.42v4.74c0 .12.06.18.12.24l3.3 3c.12.12.18.3.12.48z"/>
        </svg>
      )
    },
    {
      name: "GitHub",
      href: "https://github.com/climatenews",
      icon: (props: any) => (
        <svg fill="currentColor" viewBox="0 0 24 24" {...props}>
          <path
            fillRule="evenodd"
            d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"
            clipRule="evenodd"
          />
        </svg>
      )
    }
  ]
};

export default function Footer() {
  return (
    <footer>
      <div className="mx-auto py-12 overflow-hidden">
        <div className="mt-8 flex justify-center space-x-6">
          {navigation.social.map((item) => (
            <a
              key={`footer_social_${item.name}`}
              href={item.href}
              className="text-gray-400 hover:text-gray-500"
              target="_blank"
              rel="noopener noreferrer"
            >
              <span className="sr-only">{item.name}</span>
              <item.icon className="h-6 w-6" aria-hidden="true" />
            </a>
          ))}
        </div>
        <p className="mt-8 text-center text-base text-gray-400">
          &copy; {getCurrentYear()} ClimateNews.app
        </p>
      </div>
    </footer>
  );
}
