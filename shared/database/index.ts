import Database from '@tauri-apps/plugin-sql'

export const database = await Database.load('sqlite:test.db')
