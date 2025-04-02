use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::{
    fs,
    io,
    path::{Path, PathBuf},
    time::Duration,
};
use tempfile::Builder;
use zip::ZipArchive;

use crate::steam;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AvailableLocalizations {
    localizations: Vec<Localization>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Format {
    #[serde(rename = "compatible")]
    Compatible,
    #[serde(rename = "new")]
    New,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Localization {
    pub id: String,
    pub version: String,
    pub name: String,
    pub country: String,
    pub icon: String,
    pub description: String,
    pub authors: Vec<String>,
    pub urls: Vec<String>,
    pub format: Format,
}

pub async fn fetch_available_localizations(url: &str) -> Result<AvailableLocalizations, String> {
    let client = Client::new();

    let response = client.get(url)
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .map_err(|e| format!("Request error: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()));
    }
    
    let localizations: AvailableLocalizations = response.json()
        .await
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    Ok(localizations)
}

pub async fn install_localization(localization: Localization) -> Result<(), String> {
    let temp_dir = create_temp_directory(&localization.id)?;
    let extract_path = temp_dir.path();
    
    let download_path = download_localization_file(&localization, &temp_dir).await?;
    
    println!("Extracting localization to: {:?}", extract_path);
    extract_zip_archive(&download_path, extract_path)?;
    
    let language_dir = find_language_directory(extract_path, &localization.format)?;
    
    install_to_game_directory(&language_dir, &localization)?;
    
    println!(
        "Successfully installed localization '{}' version '{}'",
        localization.id,
        localization.version
    );
    Ok(())
}

pub async fn uninstall_localization(localization: Localization) -> Result<(), String> {
    let game_path = steam::get_game_directory()?;
    let target_base_path = game_path.join("LimbusCompany_Data").join("Lang");
    let target_path = target_base_path.join(&localization.id);
    
    if !target_path.exists() {
        return Err(format!("Localization '{}' not found", localization.id));
    }
    
    fs::remove_dir_all(&target_path)
        .map_err(|e| format!("Failed to uninstall localization '{}': {}", localization.id, e))?;
    
    Ok(())
}

fn create_temp_directory(localization_id: &str) -> Result<tempfile::TempDir, String> {
    Builder::new()
        .prefix(&format!("limbus_loc_{}", localization_id))
        .tempdir()
        .map_err(|e| format!("Failed to create temporary directory: {}", e))
}

async fn download_localization_file(localization: &Localization, temp_dir: &tempfile::TempDir) -> Result<PathBuf, String> {
    let client = Client::new();
    let download_path = temp_dir.path().join("localization.zip");
    
    // Try downloading from the primary URL and then mirrors
    for url in &localization.urls {
        println!("Attempting download from: {}", url);
        match download_file(&client, url, &download_path).await {
            Ok(_) => {
                println!("Successfully downloaded localization from: {}", url);
                return Ok(download_path);
            },
            Err(e) => {
                println!("Failed to download from {}: {}. Trying next URL.", url, e);
                // Continue to the next URL
            }
        }
    }
    
    Err(format!(
        "Failed to download localization '{}' from all provided URLs.",
        localization.id
    ))
}

async fn download_file(client: &Client, url: &str, destination: &Path) -> Result<(), String> {
    let response = client
        .get(url)
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| format!("Request error: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("HTTP error {}", response.status()));
    }
    
    let mut output_file = fs::File::create(destination)
        .map_err(|e| format!("Failed to create output file: {}", e))?;
    
    let bytes = response.bytes().await
        .map_err(|e| format!("Failed to get response bytes: {}", e))?;
    
    std::io::copy(&mut std::io::Cursor::new(bytes), &mut output_file)
        .map_err(|e| format!("Failed to write data to file: {}", e))?;
    
    Ok(())
}

fn extract_zip_archive(zip_path: &Path, extract_path: &Path) -> Result<(), String> {
    let file = fs::File::open(zip_path)
        .map_err(|e| format!("Failed to open zip file: {}", e))?;
    
    let mut archive = ZipArchive::new(file)
        .map_err(|e| format!("Failed to read ZIP archive: {}", e))?;
    
    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| format!("Error reading file in zip: {}", e))?;
        
        let outpath = match file.enclosed_name() {
            Some(path) => extract_path.join(path),
            None => {
                println!("Entry {} has unsafe path, skipping.", i);
                continue;
            }
        };
        
        extract_zip_entry(&mut file, &outpath)?;
    }
    
    Ok(())
}

fn extract_zip_entry(file: &mut zip::read::ZipFile, outpath: &Path) -> Result<(), String> {
    if file.name().ends_with('/') {
        println!("Creating directory: {:?}", outpath);
        fs::create_dir_all(outpath)
            .map_err(|e| format!("Failed to create directory during extraction: {}", e))?;
    } else {
        println!("Extracting file: {:?}", outpath);
        if let Some(parent) = outpath.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).map_err(|e| {
                    format!("Failed to create parent directory during extraction: {}", e)
                })?;
            }
        }
        
        let mut outfile = fs::File::create(outpath)
            .map_err(|e| format!("Failed to create file during extraction: {}", e))?;
        
        io::copy(file, &mut outfile)
            .map_err(|e| format!("Failed to copy file during extraction: {}", e))?;
    }
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Some(mode) = file.unix_mode() {
            fs::set_permissions(outpath, fs::Permissions::from_mode(mode))
                .map_err(|e| format!("Failed to set permissions: {}", e))?;
        }
    }
    
    Ok(())
}

fn find_language_directory(extract_path: &Path, format: &Format) -> Result<PathBuf, String> {
    match format {
        Format::Compatible => find_compatible_language_dir(extract_path),
        Format::New => {
            println!("Using 'new' format, language directory is root: {:?}", extract_path);
            Ok(extract_path.to_path_buf())
        }
    }
}

fn find_compatible_language_dir(extract_path: &Path) -> Result<PathBuf, String> {
    let localize_path = extract_path.join("Localize");
    
    if !localize_path.is_dir() {
        return Err("Localization format is 'compatible' but 'Localize' directory not found.".to_string());
    }
    
    for entry in fs::read_dir(localize_path)
        .map_err(|e| format!("Failed to read 'Localize' directory: {}", e))?
    {
        let entry = entry.map_err(|e| format!("Error reading entry in 'Localize': {}", e))?;
        let path = entry.path();
        
        if path.is_dir() && path.join("StoryData").is_dir() {
            println!("Found compatible language directory: {:?}", path);
            return Ok(path);
        }
    }
    
    Err("Could not find language directory with StoryData in 'Localize'.".to_string())
}

fn install_to_game_directory(language_dir: &Path, localization: &Localization) -> Result<(), String> {
    let game_path = steam::get_game_directory()?;
    let target_base_path = game_path.join("LimbusCompany_Data").join("Lang");
    let target_path = target_base_path.join(&localization.id);
    
    println!("Target installation path: {:?}", target_path);
    
    if target_path.exists() {
        println!("Removing existing localization at {:?}", target_path);
        fs::remove_dir_all(&target_path)
            .map_err(|e| format!("Failed to remove existing localization directory: {}", e))?;
    }
    
    fs::create_dir_all(&target_base_path)
        .map_err(|e| format!("Failed to create base Lang directory: {}", e))?;
    
    fs::create_dir(&target_path)
        .map_err(|e| format!("Failed to create target localization directory: {}", e))?;
    
    println!("Moving files from {:?} to {:?}", language_dir, target_path);
    copy_directory_contents(language_dir, &target_path)?;
    
    Ok(())
}

fn copy_directory_contents(src_dir: &Path, dest_dir: &Path) -> Result<(), String> {
    for entry in fs::read_dir(src_dir)
        .map_err(|e| format!("Failed to read language directory {:?}: {}", src_dir, e))?
    {
        let entry = entry.map_err(|e| format!("Error reading entry in language dir: {}", e))?;
        let source_path = entry.path();
        let destination_path = dest_dir.join(entry.file_name());
        
        println!("Copying {:?} -> {:?}", source_path, destination_path);
        
        if source_path.is_dir() {
            fs::create_dir_all(&destination_path)
                .map_err(|e| format!("Failed to create directory {:?}: {}", destination_path, e))?;
            copy_directory_contents(&source_path, &destination_path)?;
        } else {
            fs::copy(&source_path, &destination_path)
                .map_err(|e| format!("Failed to copy file {:?} to {:?}: {}", source_path, destination_path, e))?;
        }
    }
    
    Ok(())
}