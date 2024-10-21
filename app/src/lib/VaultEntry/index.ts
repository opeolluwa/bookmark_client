"use client";

import {AppDatabase} from "@/database";
import { v4 as uuidv4 } from "uuid";

export default class VaultEntry {
  title: string;
  description: string;
  keywords: string[] | undefined;
  date_added: string;
  last_modified: string;

  constructor(title: string, description: string, keywords: string[]) {
    this.title = title.trim();
    this.description = description.trim();
    this.keywords = keywords.map((keyword) => keyword.trim().toLowerCase());
    this.last_modified = new Date().toLocaleDateString("en-us", {
      year: "numeric",
      day: "numeric",
      month: "long",
      weekday: "long",
    });
    this.date_added = new Date().toLocaleDateString("en-us", {
      year: "numeric",
      day: "numeric",
      month: "long",
      weekday: "long",
    });
    return this;
  }

//   async save() {
//     const uuid = uuidv4();
//     const query = `INSERT INTO vault_entries (id, title, description, keywords, date_added, last_modified)
// VALUES ('${uuid}', '${this.title}', '${
//       this.description
//     }', '["${this.keywords?.join('","')}"]', '${this.date_added}', '${
//       this.last_modified
//     }');`;

//     const result = await AppDatabase.execute(query);
//     return result;
//   }
}
