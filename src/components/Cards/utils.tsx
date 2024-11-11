export const parseDate = (dateString: string) => {
  return new Date(dateString.split("+")[0].trim()).toLocaleDateString("en-US", {
    year: "numeric",
    weekday: "short",
    day: "numeric",
    month: "short",
    timeZone: "UTC",
    minute: "numeric",
    hour: "numeric",
  });
};
