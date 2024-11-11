use tauri::Runtime;
use tauri_plugin_store::StoreExt;

pub fn extract_token<R: Runtime>(app: &tauri::AppHandle<R>) -> String {
    app.store("store.json")
        .expect("error fetching store")
        .get("access token")
        .expect("error fetching the token")
        .take()["value"]
        .clone()
        .as_str()
        .expect("couldn't covert to string ")
        .to_string()
}
