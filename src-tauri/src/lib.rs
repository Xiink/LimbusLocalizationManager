mod utils;
mod steam;
mod settings;

use tauri::{Manager, State, Emitter};
use tokio::sync::Mutex;

type SettingsState = Mutex<settings::AppSettings>;

#[tauri::command]
async fn get_available_localizations(
    state: State<'_, SettingsState>,
) -> Result<utils::AvailableLocalizations, String> {
    let settings_guard = state.lock().await;

    let active_source = settings_guard.selected_source
        .as_ref()
        .ok_or_else(|| "No active source selected".to_string())?;

    let localizations = utils::fetch_available_localizations(active_source).await?;
    Ok(localizations)
}

#[tauri::command]
async fn launch_game() -> Result<(), String> {
    steam::launch_game()?;
    Ok(())
}

#[tauri::command]
async fn get_game_directory() -> Result<String, String> {
    let game_path = steam::get_game_directory()?;
    Ok(game_path.to_string_lossy().to_string())
}

#[tauri::command]
async fn get_settings(state: State<'_, SettingsState>) -> Result<settings::AppSettings, String> {
    let settings_guard = state.lock().await;
    Ok(settings_guard.clone())
}

#[tauri::command]
async fn update_settings(
    app_handle: tauri::AppHandle,
    state: State<'_, SettingsState>,
    new_settings: settings::AppSettings,
) -> Result<(), String> {
    let mut settings_guard = state.lock().await;

    settings::save_settings(&app_handle, &settings_guard)
        .map_err(|e| e.to_string())?;

    app_handle.emit("settings_updated", &new_settings).unwrap();
    *settings_guard = new_settings;
    Ok(())
}

#[tauri::command]
async fn install_localization(
    app_handle: tauri::AppHandle,
    state: State<'_, SettingsState>,
    localization: utils::Localization,
) -> Result<(), String> {
    utils::install_localization(localization.clone()).await?;
    let mut settings_guard = state.lock().await;
    settings_guard.installed.insert(localization.id.clone(), localization);
    settings::save_settings(&app_handle, &settings_guard)
        .map_err(|e| e.to_string())?;

    app_handle.emit("settings_updated", settings_guard.clone()).unwrap();
    Ok(())
}

#[tauri::command]
async fn uninstall_localization(
    app_handle: tauri::AppHandle,
    state: State<'_, SettingsState>,
    localization: utils::Localization,
) -> Result<(), String> {
    utils::uninstall_localization(localization.clone()).await?;
    let mut settings_guard = state.lock().await;
    settings_guard.installed.remove(&localization.id);
    settings::save_settings(&app_handle, &settings_guard)
        .map_err(|e| e.to_string())?;

    app_handle.emit("settings_updated", settings_guard.clone()).unwrap();
    Ok(())
}

#[tauri::command]
async fn repair_localization(
    app_handle: tauri::AppHandle,
    state: State<'_, SettingsState>,
    localization: utils::Localization,
) -> Result<(), String> {
    install_localization(app_handle, state, localization).await?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();

            let app_settings = settings::load_settings(&app_handle).unwrap_or_else(|e| {
                eprintln!("Failed to load settings: {}", e);
                settings::AppSettings::default()
            });

            app.manage(Mutex::new(app_settings));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(
            tauri::generate_handler![
                get_available_localizations, 
                get_game_directory, 
                launch_game,
                get_settings,
                update_settings,
                install_localization,
                uninstall_localization,
                repair_localization,
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
