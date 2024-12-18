use tauri::Runtime;

#[tauri::command]
async fn create_app_settings<R: Runtime>(
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
) -> Result<(), String> {
    Ok(())
}
