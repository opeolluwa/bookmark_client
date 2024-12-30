use std::sync::Mutex;

use sqlite_wasm_bindgen::rusqlite::Connection;

#[derive(Debug)]
pub struct AppState {
    pub conn: Mutex<Connection>,
}
