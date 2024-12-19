use tauri::Runtime;

#[tauri::command]
pub async fn create_app_settings<R: Runtime>(
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn fetch_default_settings<R: Runtime>(
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
) -> Result<(), String> {
    Ok(())
}
