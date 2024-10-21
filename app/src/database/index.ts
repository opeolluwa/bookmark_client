import Database from "@tauri-apps/plugin-sql";

export const AppDatabase = await Database.load("sqlite:vault.db");

