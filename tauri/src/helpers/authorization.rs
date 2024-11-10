use serde_json::Value;
use tauri::Runtime;
use tauri_plugin_store::StoreExt;

pub fn extract_token<R: Runtime>(app: &tauri::AppHandle<R>) -> std::option::Option<Value> {
    app.store("store.json")
        .expect("error fetching store")
        .get("access token")
}
