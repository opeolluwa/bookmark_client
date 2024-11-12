export const parseDate = (dateString: string): Date => {
  return new Date(dateString.split("+")[0].trim());
};
