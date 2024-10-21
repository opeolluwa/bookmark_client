import InformationAccessHistory from "../accessHistory";
import VaultEntry from "../VaultEntry";

export default class {
  private name: string;
  private access_history: InformationAccessHistory[];
  private description: string;
  private entries: VaultEntry[];

  constructor(name: string = "default", description: string = "") {
    this.name = name.trim();
    this.description = description.trim();
    this.entries = [];
    this.access_history = [];
    return this;
  }

  public add_entry(entry: VaultEntry | Array<VaultEntry>) {
    if (Array.isArray(entry)) {
      entry.forEach((item) => this.add_single_vault_entry(item));
    } else {
      this.add_single_vault_entry(entry);
    }
  }

  private add_single_vault_entry(entry: VaultEntry) {
    this.entries.push(entry);
  }

  get content() {
    return this.entries;
  }
}
