import InformationAccessHistory from "../accessHistory";

export default class VaultInformation {
  title: string;
  description: string;
  dateAdded: string;
  lastModified: string;
  accessHistory?: InformationAccessHistory[];
  constructor(title: string, description: string) {
    this.title = title;
    this.description = description;
    this.lastModified = new Date().toLocaleDateString("en-us", {
      year: "numeric",
      day: "numeric",
      month: "long",
      weekday: "long",
    });
    this.dateAdded = new Date().toLocaleDateString("en-us", {
      year: "numeric",
      day: "numeric",
      month: "long",
      weekday: "long",
    });
    return this;
  }
}
