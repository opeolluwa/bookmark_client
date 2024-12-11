import { Client, Stronghold } from "@tauri-apps/plugin-stronghold";
import { appDataDir } from "@tauri-apps/api/path";

const initStronghold = async (name: string) => {
  const vaultPath = `${await appDataDir()}/vault.hold`;
  const vaultPassword = "evU8td7Dwhzk4HileRdcHvvtLfEnF5msZUW2";
  const stronghold = await Stronghold.load(vaultPath, vaultPassword);

  let client: Client;
  const clientName = name;
  try {
    client = await stronghold.loadClient(clientName);
  } catch {
    client = await stronghold.createClient(clientName);
  }

  return {
    stronghold,
    client,
  };
};

// Insert a record to the store
export async function insertRecord(store: any, key: string, value: string) {
  const data = Array.from(new TextEncoder().encode(value));
  await store.insert(key, data);
}

// Read a record from store
export async function getRecord(store: any, key: string): Promise<string> {
  const data = await store.get(key);
  return new TextDecoder().decode(new Uint8Array(data));
}

// the available stores

export const {
  stronghold: authenticationStronghold,
  client: authenticationClient,
} = await initStronghold("authentication");

// const store = client.getStore();
// const key = "my_key";

// // Insert a record to the store
// insertRecord(store, key, "secret value");

// // Read a record from store
// const value = await getRecord(store, key);
// console.log(value); // 'secret value'

// // Save your updates
// await stronghold.save();

// // Remove a record from store
// await store.remove(key);
