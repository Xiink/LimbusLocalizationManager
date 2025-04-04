use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tauri::path::BaseDirectory;
use tauri::Manager;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LocalizationSource {
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppSettings {
    pub sources: HashMap<String, LocalizationSource>,
    pub selected_source: Option<String>,
    pub game_directory: Option<String>,
    pub language: Option<String>,
}

impl AppSettings {
    pub fn default() -> Self {
        Self {
            sources: HashMap::new(),
            selected_source: None,
            game_directory: None,
            language: None,
        }
    }
}

fn get_config_path(app_handle: &tauri::AppHandle) -> Result<PathBuf, anyhow::Error> {
    let config_dir = app_handle.path().app_config_dir()?;
    Ok(config_dir.join("config.toml"))
}

fn load_default_settings(app_handle: &tauri::AppHandle) -> Result<AppSettings, anyhow::Error> {
    let config_path = get_config_path(app_handle)?;

    if let Some(parent_dir) = config_path.parent() {
        fs::create_dir_all(parent_dir)?;
    }

    let resource_path = app_handle
        .path()
        .resolve("resources/default_config.toml", BaseDirectory::Resource)?;

    println!("Loading default config from: {:?}", resource_path);

    if !resource_path.exists() {
        eprintln!(
            "Warning: default_config.toml not found at {:?}. Using hardcoded default.",
            resource_path
        );
        let default_settings = AppSettings::default();
        save_settings(app_handle, &default_settings)?;
        return Ok(default_settings);
    }

    let default_config_content = fs::read_to_string(&resource_path)?;
    let default_settings: AppSettings = toml::from_str(&default_config_content)?;

    save_settings(app_handle, &default_settings)?;

    Ok(default_settings)
}

pub fn load_settings(app_handle: &tauri::AppHandle) -> Result<AppSettings, anyhow::Error> {
    let config_path = get_config_path(app_handle)?;

    if config_path.exists() {
        let config_content = fs::read_to_string(&config_path)?;
        let settings: AppSettings = toml::from_str(&config_content).unwrap_or_else(|e| {
            eprintln!("Failed to parse config file: {}", e);
            load_default_settings(app_handle).unwrap()
        });
        return Ok(settings);
    }

    println!(
        "Config file not found at {:?}. Loading default config.",
        config_path
    );

    load_default_settings(app_handle)
}

pub fn save_settings(
    app_handle: &tauri::AppHandle,
    settings: &AppSettings,
) -> Result<(), anyhow::Error> {
    let config_path = get_config_path(app_handle)?;
    let config_content = toml::to_string(settings)?;
    fs::write(&config_path, config_content)?;
    println!("Settings saved to: {:?}", config_path);
    Ok(())
}
