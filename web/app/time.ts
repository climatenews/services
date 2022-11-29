interface Interval {
  label: String;
  seconds: number;
}
const intervals: Interval[] = [
  { label: "year", seconds: 31536000 },
  { label: "month", seconds: 2592000 },
  { label: "day", seconds: 86400 },
  { label: "hour", seconds: 3600 },
  { label: "minute", seconds: 60 },
  { label: "second", seconds: 1 }
];

export function timeSince(timestamp?: number): string {
  if (timestamp === undefined) {
    return "";
  } else {
    const date = new Date(timestamp * 1000);
    const seconds = Math.floor((Date.now() - date.getTime()) / 1000);
    const interval: Interval | undefined = intervals.find(
      (i) => i.seconds < seconds
    );
    if (interval) {
      const count = Math.floor(seconds / interval.seconds);
      return `${count} ${interval.label}${count !== 1 ? "s" : ""} ago`;
    }
    return "Invalid date";
  }
}

export const getCurrentYear = () => new Date().getFullYear();
