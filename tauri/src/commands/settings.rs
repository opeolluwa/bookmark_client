use sqlite_wasm_bindgen::entities::settings::Settings;

use crate::state::AppState;

#[tauri::command]
pub async fn fetch_default_settings(state: tauri::State<'_, AppState>) -> Result<Settings, String> {
    let conn = state.conn.lock().unwrap();
    let settings = Settings::fetch(&conn).unwrap();
    Ok(settings)
}

// change the application initialization status, this is used to determine if the application has been initialized or not yet, it will prevent the application from reinitializing the application settings and from displaying the walkthrough screen
#[tauri::command]
pub async fn update_application_initialization_status(
    state: tauri::State<'_, AppState>,
    status: bool,
) -> Result<(), String> {
    let conn = state.conn.lock().unwrap();
    Settings::change_initialization_status(status, &conn);
    Ok(())
}

#[cfg(test)]
mod test {

    #[test]
    fn test_settings() {
        // let connection = sqlite_wasm_bindgen::database::SqliteWasm::init().unwrap();
        // let conn = Mutex::new(connection);
        // let state = tauri::State::from(AppState { conn });
        // let settings = super::fetch_default_settings(state).unwrap();
        // println!("{:#?}", settings);
    }
}
