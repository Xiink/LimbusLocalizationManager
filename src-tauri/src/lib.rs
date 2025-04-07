mod settings;
mod steam;
mod utils;

use dashmap::DashMap;
use log::{error, info};
use serde::{Deserialize, Serialize};
use tauri::{Emitter, Manager, State};
use tokio::sync::Mutex;

#[derive(Clone, Serialize, Deserialize)]
struct AppState {
    settings: settings::AppSettings,
    installed_metadata: Option<utils::InstalledMetadata>,
}

impl AppState {
    fn new(app_handle: &tauri::AppHandle) -> Self {
        let mut app_state = Self {
            settings: settings::load_settings(app_handle).unwrap_or_else(|e| {
                error!("Failed to load settings: {}", e);
                settings::AppSettings::default()
            }),
            installed_metadata: None,
        };

        app_state.load_installed_metadata().unwrap_or_else(|e| {
            error!("Failed to load installed metadata: {}", e);
        });

        app_state
    }

    fn update_settings(
        &mut self,
        app_handle: &tauri::AppHandle,
        new_settings: &settings::AppSettings,
    ) -> anyhow::Result<()> {
        settings::save_settings(app_handle, &new_settings)?;
        self.settings = new_settings.clone();
        Ok(())
    }

    fn save_settings(&self, app_handle: &tauri::AppHandle) -> anyhow::Result<()> {
        settings::save_settings(app_handle, &self.settings)?;
        Ok(())
    }

    fn update_game_directory(&mut self, game_directory: &Option<String>) -> anyhow::Result<()> {
        let game_path = if let Some(game_directory) = game_directory {
            if !steam::validate_game_directory(&game_directory).is_ok() {
                return Err(anyhow::anyhow!("Invalid game directory"));
            }

            std::path::PathBuf::from(game_directory)
        } else {
            steam::get_game_directory()?
        };

        let installed_metadata = utils::load_installed_metadata(&game_path)?;

        self.installed_metadata = Some(installed_metadata);
        self.settings.game_directory = game_directory.clone();
        Ok(())
    }

    fn load_installed_metadata(&mut self) -> anyhow::Result<()> {
        let game_path = if let Some(game_directory) = &self.settings.game_directory {
            std::path::PathBuf::from(game_directory)
        } else {
            steam::get_game_directory()?
        };

        self.installed_metadata = Some(utils::load_installed_metadata(&game_path)?);
        Ok(())
    }

    fn save_installed_metadata(&self) -> anyhow::Result<()> {
        let game_path = if let Some(game_directory) = &self.settings.game_directory {
            std::path::PathBuf::from(game_directory)
        } else {
            steam::get_game_directory()?
        };

        if let Some(metadata) = &self.installed_metadata {
            utils::save_installed_metadata(&game_path, metadata)?;
        }

        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct RemoteLocalizations {
    source: String,
    localizations: Vec<utils::Localization>,
}

type AppStateMutex = Mutex<AppState>;
type RemoteLocalizationsMutex = Mutex<Option<RemoteLocalizations>>;
type LocalizationLocks = DashMap<(String, std::path::PathBuf), Mutex<()>>;

#[tauri::command]
async fn get_latest_version() -> Result<String, String> {
    utils::get_latest_version().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_available_localizations(
    app_handle: tauri::AppHandle,
    app_state: State<'_, AppStateMutex>,
    remote_localizations: State<'_, RemoteLocalizationsMutex>,
) -> Result<Vec<utils::Localization>, String> {
    let active_source_name;
    let source_url;

    {
        let app_state_guard = app_state.lock().await;

        active_source_name = app_state_guard
            .settings
            .selected_source
            .as_ref()
            .ok_or_else(|| "No active source selected".to_string())?
            .clone();

        source_url = app_state_guard
            .settings
            .sources
            .get(&active_source_name)
            .ok_or_else(|| "No active source selected".to_string())?
            .url
            .clone();
    }

    let localizations = utils::fetch_available_localizations(&source_url)
        .await
        .map_err(|e| {
            error!("Failed to fetch available localizations: {:?}", e);
            e.to_string()
        })?;

    // Update remote_localizations state
    let mut remote_localizations_guard = remote_localizations.lock().await;
    let remote_localizations = RemoteLocalizations {
        source: active_source_name,
        localizations: localizations.clone(),
    };

    *remote_localizations_guard = Some(remote_localizations.clone());
    app_handle
        .emit("remote_localizations_updated", remote_localizations)
        .unwrap();
    Ok(localizations)
}

#[tauri::command]
async fn get_app_state(state: State<'_, AppStateMutex>) -> Result<AppState, String> {
    let app_state_guard = state.lock().await;
    Ok(app_state_guard.clone())
}

#[tauri::command]
async fn update_settings(
    app_handle: tauri::AppHandle,
    state: State<'_, AppStateMutex>,
    remote_localizations: State<'_, RemoteLocalizationsMutex>,
    new_settings: settings::AppSettings,
) -> Result<(), String> {
    let mut app_state_guard = state.lock().await;

    if new_settings.selected_source != app_state_guard.settings.selected_source {
        let mut remote_localizations_guard = remote_localizations.lock().await;
        *remote_localizations_guard = None;
    }

    app_state_guard
        .update_settings(&app_handle, &new_settings)
        .map_err(|e| {
            error!("Failed to update settings: {:?}", e);
            e.to_string()
        })?;

    app_handle
        .emit("app_state_updated", app_state_guard.clone())
        .map_err(|e| {
            error!("Failed to emit app state updated: {:?}", e);
            e.to_string()
        })?;

    Ok(())
}

#[tauri::command]
async fn install_localization(
    app_handle: tauri::AppHandle,
    state: State<'_, AppStateMutex>,
    localization_lock: State<'_, LocalizationLocks>,
    localization: utils::Localization,
) -> Result<(), String> {
    if steam::is_game_running() {
        return Err("Game is running".to_string());
    }

    let game_path;
    let source;

    {
        let app_state_guard = state.lock().await;

        source = app_state_guard
            .settings
            .selected_source
            .clone()
            .ok_or_else(|| "No active source selected".to_string())?;

        let game_directory = app_state_guard.settings.game_directory.clone();
        game_path = if let Some(game_directory) = game_directory {
            std::path::PathBuf::from(game_directory)
        } else {
            steam::get_game_directory().map_err(|e| {
                error!("Failed to get game directory: {:?}", e);
                e.to_string()
            })?
        };
    }

    let lock = localization_lock
        .entry((localization.id.clone(), game_path.clone()))
        .or_insert_with(|| Mutex::new(()));
    let _aquired_lock = lock.lock().await;

    utils::install_localization(&game_path, &localization)
        .await
        .map_err(|e| {
            error!("Failed to install localization: {:?}", e);
            e.to_string()
        })?;

    utils::install_fonts_for_localization(&game_path, &localization)
        .await
        .map_err(|e| {
            error!("Failed to install fonts for localization: {:?}", e);
            e.to_string()
        })?;

    {
        let mut app_state_guard = state.lock().await;

        match app_state_guard.installed_metadata {
            Some(ref mut installed_metadata) => {
                installed_metadata.installed.insert(
                    localization.id.clone(),
                    utils::InstalledLocalization {
                        id: localization.id.clone(),
                        version: localization.version.clone(),
                        source,
                    },
                );
            }
            None => {
                app_state_guard.installed_metadata = Some(
                    utils::InstalledMetadata::with_localization(&localization, &source),
                );
            }
        }

        app_state_guard.save_installed_metadata().map_err(|e| {
            error!("Failed to save installed metadata: {:?}", e);
            e.to_string()
        })?;

        app_handle
            .emit("app_state_updated", app_state_guard.clone())
            .map_err(|e| {
                error!("Failed to emit app state updated: {:?}", e);
                e.to_string()
            })?;
    }

    Ok(())
}

#[tauri::command]
async fn uninstall_localization(
    app_handle: tauri::AppHandle,
    state: State<'_, AppStateMutex>,
    localization_lock: State<'_, LocalizationLocks>,
    localization: utils::Localization,
) -> Result<(), String> {
    if steam::is_game_running() {
        return Err("Game is running".to_string());
    }

    let game_path;

    {
        let app_state_guard = state.lock().await;

        game_path = if let Some(game_directory) = &app_state_guard.settings.game_directory {
            std::path::PathBuf::from(game_directory)
        } else {
            steam::get_game_directory().map_err(|e| {
                error!("Failed to get game directory: {:?}", e);
                e.to_string()
            })?
        };
    }

    let lock = localization_lock
        .entry((localization.id.clone(), game_path.clone()))
        .or_insert_with(|| Mutex::new(()));
    let _aquired_lock = lock.lock().await;

    utils::uninstall_localization(&game_path, &localization)
        .await
        .map_err(|e| {
            error!("Failed to uninstall localization: {:?}", e);
            e.to_string()
        })?;

    {
        let mut app_state_guard = state.lock().await;

        app_state_guard
            .installed_metadata
            .as_mut()
            .unwrap()
            .installed
            .remove(&localization.id);
        app_state_guard.save_installed_metadata().map_err(|e| {
            error!("Failed to save installed metadata: {:?}", e);
            e.to_string()
        })?;

        app_handle
            .emit("app_state_updated", app_state_guard.clone())
            .map_err(|e| {
                error!("Failed to emit app state updated: {:?}", e);
                e.to_string()
            })?;
    }

    Ok(())
}

#[tauri::command]
async fn repair_localization(
    app_handle: tauri::AppHandle,
    state: State<'_, AppStateMutex>,
    localization_lock: State<'_, LocalizationLocks>,
    localization: utils::Localization,
) -> Result<(), String> {
    install_localization(app_handle, state, localization_lock, localization).await?;
    Ok(())
}

#[tauri::command]
async fn set_game_directory(
    app_handle: tauri::AppHandle,
    state: State<'_, AppStateMutex>,
    directory: Option<String>,
) -> Result<(), String> {
    let mut app_state_guard = state.lock().await;

    app_state_guard
        .update_game_directory(&directory)
        .map_err(|e| {
            error!("Failed to update game directory: {:?}", e);
            e.to_string()
        })?;

    app_state_guard.save_settings(&app_handle).map_err(|e| {
        error!("Failed to save settings: {:?}", e);
        e.to_string()
    })?;

    app_handle
        .emit("app_state_updated", app_state_guard.clone())
        .map_err(|e| {
            error!("Failed to emit app state updated: {:?}", e);
            e.to_string()
        })?;

    Ok(())
}

#[tauri::command]
async fn update_and_play(
    app_handle: tauri::AppHandle,
    state: State<'_, AppStateMutex>,
    localization_lock: State<'_, LocalizationLocks>,
    remote_localizations_state: State<'_, RemoteLocalizationsMutex>,
) -> Result<(), String> {
    app_handle.emit("play:started", ()).unwrap();

    if steam::is_game_running() {
        app_handle.emit("play:game_running", ()).unwrap();
        return Err("Game is already running".to_string());
    }

    let active_source;
    let source_url;
    let game_path;

    {
        let app_state_guard = state.lock().await;
        active_source = app_state_guard
            .settings
            .selected_source
            .as_ref()
            .ok_or_else(|| "No active source selected".to_string())?
            .clone();

        source_url = app_state_guard
            .settings
            .sources
            .get(&active_source)
            .ok_or_else(|| "No active source selected".to_string())?
            .url
            .clone();

        game_path = if let Some(game_directory) = &app_state_guard.settings.game_directory {
            std::path::PathBuf::from(game_directory)
        } else {
            steam::get_game_directory().map_err(|e| {
                error!("Failed to get game directory: {:?}", e);
                e.to_string()
            })?
        };
    }

    let remote_localizations = utils::fetch_available_localizations(&source_url)
        .await
        .map_err(|e| {
            error!("Failed to fetch available localizations: {:?}", e);
            e.to_string()
        })?;

    let mut remote_localizations_guard = remote_localizations_state.lock().await;
    let remote_localizations_payload = RemoteLocalizations {
        source: active_source.clone(),
        localizations: remote_localizations.clone(),
    };

    *remote_localizations_guard = Some(remote_localizations_payload.clone());
    app_handle
        .emit(
            "remote_localizations_updated",
            remote_localizations_payload.clone(),
        )
        .unwrap();

    let localizations_to_update: Vec<_> = state
        .lock()
        .await
        .installed_metadata
        .as_ref()
        .ok_or_else(|| "No installed metadata found".to_string())?
        .installed
        .values()
        .filter_map(|localization| {
            let remote_localization = remote_localizations
                .iter()
                .find(|l| l.id == localization.id);

            if let None = remote_localization {
                info!(
                    "Localization {} not found in remote source",
                    &localization.id
                );
                app_handle
                    .emit("play:unknown_localization", &localization.id)
                    .unwrap();
                return None;
            }

            let remote_localization = remote_localization.unwrap();
            if remote_localization.version == localization.version {
                info!("Localization {} is up to date", &localization.id);
                app_handle
                    .emit("play:up_to_date", &localization.id)
                    .unwrap();
                return None;
            }

            Some((localization.id.clone(), remote_localization.clone()))
        })
        .collect();

    for (localization_id, remote_localization) in localizations_to_update {
        info!(
            "Updating localization {} to version {}",
            &localization_id, &remote_localization.version
        );
        app_handle.emit("play:updating", &localization_id).unwrap();

        let lock = localization_lock
            .entry((localization_id.clone(), game_path.clone()))
            .or_insert_with(|| Mutex::new(()));
        let _aquired_lock = lock.lock().await;

        utils::install_localization(&game_path, &remote_localization)
            .await
            .map_err(|e| {
                error!("Failed to install localization: {:?}", e);
                e.to_string()
            })?;

        utils::install_fonts_for_localization(&game_path, &remote_localization)
            .await
            .map_err(|e| {
                error!("Failed to install fonts for localization: {:?}", e);
                e.to_string()
            })?;

        app_handle
            .emit("play:update_finished", &localization_id)
            .unwrap();

        state
            .lock()
            .await
            .installed_metadata
            .as_mut()
            .unwrap()
            .installed
            .insert(
                remote_localization.id.clone(),
                utils::InstalledLocalization {
                    id: remote_localization.id.clone(),
                    version: remote_localization.version.clone(),
                    source: active_source.clone(),
                },
            );
    }

    let state_guard = state.lock().await;
    state_guard.save_installed_metadata().map_err(|e| {
        error!("Failed to save installed metadata: {:?}", e);
        e.to_string()
    })?;

    app_handle
        .emit("app_state_updated", state_guard.clone())
        .map_err(|e| {
            error!("Failed to emit app state updated: {:?}", e);
            e.to_string()
        })?;

    app_handle.emit("play:starting_game", ()).unwrap();
    steam::launch_game().map_err(|e| {
        error!("Failed to launch game: {:?}", e);
        e.to_string()
    })?;

    app_handle.emit("play:finished", ()).unwrap();
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }))
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let app_handle = app.handle();

            let app_state = AppState::new(&app_handle);

            app.manage(Mutex::new(app_state));
            app.manage(Mutex::new(None::<RemoteLocalizations>));

            let localization_locks_mutex: LocalizationLocks = DashMap::new();
            app.manage(localization_locks_mutex);

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_available_localizations,
            get_app_state,
            get_latest_version,
            update_settings,
            install_localization,
            uninstall_localization,
            repair_localization,
            set_game_directory,
            update_and_play,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
