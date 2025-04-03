use md5::{Digest, Md5};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{
    fs, io::{self, BufReader, Read, Write},
    path::{Path, PathBuf},
    time::Duration,
};
use tempfile::Builder;
use zip::ZipArchive;
use futures::stream::StreamExt;
use log::{debug, info, warn};

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

    #[serde(untagged)]
    Unknown(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Font {
    pub url: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Localization {
    pub id: String,
    pub version: String,
    pub name: String,
    pub flag: String,
    pub icon: String,
    pub description: String,
    pub authors: Vec<String>,
    pub url: String,
    pub font: Font,
    pub format: Format,
}

pub async fn fetch_available_localizations(url: &str) -> Result<Vec<Localization>, String> {
    let client = Client::new();

    let response = client
        .get(url)
        .header("User-Agent", "Limbus Launcher")
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| format!("Request error: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()));
    }

    let localizations: AvailableLocalizations = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    Ok(localizations.localizations)
}

pub async fn install_fonts_for_localization(
    game_directory: &Option<String>,
    localization: &Localization,
) -> Result<(), String> {
    let game_path = if let Some(dir) = game_directory {
        PathBuf::from(dir)
    } else {
        steam::get_game_directory()?
    };

    let font_cache_dir = game_path.join("FontCache");
    fs::create_dir_all(&font_cache_dir)
        .map_err(|e| format!("Failed to create FontCache directory: {}", e))?;

    let font_info = &localization.font;
    let font_url = &font_info.url;
    let expected_hash = &font_info.hash;

    let extension = Path::new(font_url)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .filter(|ext| ext == "ttf" || ext == "otf")
        .unwrap_or_else(|| "ttf".to_string());

    let font_filename = format!("{}.{}", expected_hash, extension);
    let font_cache_path = font_cache_dir.join(&font_filename);

    let mut needs_download = true;
    if font_cache_path.exists() {
        debug!("Font found in cache: {:?}", font_cache_path);
        match calculate_md5(&font_cache_path) {
            Ok(calculated_hash) => {
                if calculated_hash == *expected_hash {
                    info!("Cached font hash matches. Skipping download.");
                    needs_download = false;
                } else {
                    warn!(
                        "Cached font hash mismatch (expected: {}, found: {}). Re-downloading.",
                        expected_hash, calculated_hash
                    );
                    fs::remove_file(&font_cache_path).map_err(|e| {
                        format!(
                            "Failed to remove mismatched cached font {:?}: {}",
                            font_cache_path, e
                        )
                    })?;
                }
            }
            Err(e) => {
                warn!(
                    "Failed to calculate hash for cached font {:?}: {}. Re-downloading.",
                    font_cache_path, e
                );
                fs::remove_file(&font_cache_path).map_err(|e| {
                    format!(
                        "Failed to remove potentially corrupted cached font {:?}: {}",
                            font_cache_path, e
                        )
                    })?;
            }
        }
    }

    if needs_download {
        info!("Downloading font from: {}", font_url);
        download_and_validate_font(font_url, &font_cache_path, expected_hash).await?;
    } else {
         info!("Using cached font: {:?}", font_cache_path);
    }

    let target_fonts_dir = game_path
        .join("LimbusCompany_Data")
        .join("Lang")
        .join(&localization.id)
        .join("Fonts"); // Folder name is "Fonts"

    fs::create_dir_all(&target_fonts_dir)
        .map_err(|e| format!("Failed to create target Fonts directory {:?}: {}", target_fonts_dir, e))?;

    let target_font_path = target_fonts_dir.join(&font_filename);

    let mut needs_copy = true;
    if target_font_path.exists() {
         match calculate_md5(&target_font_path) {
            Ok(target_hash) => {
                if target_hash == *expected_hash {
                    info!("Target font {:?} already exists and hash matches. Skipping copy.", target_font_path);
                    needs_copy = false;
                } else {
                     warn!("Target font {:?} exists but hash mismatches. Overwriting.", target_font_path);
                }
            }
            Err(e) => {
                 warn!("Failed to calculate hash for target font {:?}: {}. Overwriting.", target_font_path, e);
            }
         }
    }

    if needs_copy {
        debug!(
            "Copying font from cache {:?} to {:?}",
            font_cache_path, target_font_path
        );
        fs::copy(&font_cache_path, &target_font_path).map_err(|e| {
            format!(
                "Failed to copy font from cache {:?} to target {:?}: {}",
                font_cache_path, target_font_path, e
            )
        })?;
    }

    info!(
        "Successfully installed font for localization '{}'",
        localization.id
    );
    Ok(())
}

pub async fn install_localization(
    game_directory: &Option<String>,
    localization: &Localization,
) -> Result<(), String> {
    let temp_dir = create_temp_directory(&localization.id)?;
    let extract_path = temp_dir.path();

    let download_path = download_localization_file(&localization, &temp_dir).await?;

    info!("Extracting localization to: {:?}", extract_path);
    extract_zip_archive(&download_path, extract_path)?;

    let language_dir = find_language_directory(extract_path, &localization.format)?;

    install_to_game_directory(&game_directory, &language_dir, &localization)?;

    info!(
        "Successfully installed localization '{}' version '{}'",
        localization.id, localization.version
    );
    Ok(())
}

pub async fn uninstall_localization(
    game_directory: &Option<String>,
    localization: &Localization,
) -> Result<(), String> {
    let game_path = if let Some(game_directory) = game_directory {
        PathBuf::from(game_directory)
    } else {
        steam::get_game_directory()?
    };

    let target_base_path = game_path.join("LimbusCompany_Data").join("Lang");
    let target_path = target_base_path.join(&localization.id);

    if !target_path.exists() {
        return Err(format!("Localization '{}' not found", localization.id));
    }

    fs::remove_dir_all(&target_path).map_err(|e| {
        format!(
            "Failed to uninstall localization '{}': {}",
            localization.id, e
        )
    })?;

    Ok(())
}

fn create_temp_directory(localization_id: &str) -> Result<tempfile::TempDir, String> {
    Builder::new()
        .prefix(&format!("limbus_loc_{}", localization_id))
        .tempdir()
        .map_err(|e| format!("Failed to create temporary directory: {}", e))
}

async fn download_localization_file(
    localization: &Localization,
    temp_dir: &tempfile::TempDir,
) -> Result<PathBuf, String> {
    let client = Client::new();
    let download_path = temp_dir.path().join("localization.zip");

    let response = client
        .get(&localization.url)
        .header("User-Agent", "Limbus Launcher")
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| format!("Request error: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("HTTP error {}", response.status()));
    }

    let mut output_file = fs::File::create(&download_path)
        .map_err(|e| format!("Failed to create output file: {}", e))?;

    let mut stream = response.bytes_stream();
    
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.map_err(|e| format!("Failed to read chunk: {}", e.to_string()))?;
        output_file.write_all(&chunk)
            .map_err(|e| format!("Failed to write data chunk to file: {}", e))?;
    }

    output_file.flush()
        .map_err(|e| format!("Failed to flush file data: {}", e))?;

    info!("Successfully downloaded localization from: {}", &localization.url);
    Ok(download_path)
}

fn extract_zip_archive(zip_path: &Path, extract_path: &Path) -> Result<(), String> {
    let file = fs::File::open(zip_path).map_err(|e| format!("Failed to open zip file: {}", e))?;

    let mut archive =
        ZipArchive::new(file).map_err(|e| format!("Failed to read ZIP archive: {}", e))?;

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| format!("Error reading file in zip: {}", e))?;

        let outpath = match file.enclosed_name() {
            Some(path) => extract_path.join(path),
            None => {
                warn!("Entry {} has unsafe path, skipping.", i);
                continue;
            }
        };

        extract_zip_entry(&mut file, &outpath)?;
    }

    Ok(())
}

fn extract_zip_entry(file: &mut zip::read::ZipFile, outpath: &Path) -> Result<(), String> {
    if file.name().ends_with('/') {
        debug!("Creating directory: {:?}", outpath);
        fs::create_dir_all(outpath)
            .map_err(|e| format!("Failed to create directory during extraction: {}", e))?;
    } else {
        debug!("Extracting file: {:?}", outpath);
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
            debug!(
                "Using 'new' format, language directory is root: {:?}",
                extract_path
            );
            Ok(extract_path.to_path_buf())
        },
        Format::Unknown(unknown) => {
            Err(format!("Unknown localization format: {}", unknown))
        }
    }
}

fn find_compatible_language_dir(extract_path: &Path) -> Result<PathBuf, String> {
    let localize_path = extract_path.join("Localize");

    if !localize_path.is_dir() {
        return Err(
            "Localization format is 'compatible' but 'Localize' directory not found.".to_string(),
        );
    }

    for entry in fs::read_dir(localize_path)
        .map_err(|e| format!("Failed to read 'Localize' directory: {}", e))?
    {
        let entry = entry.map_err(|e| format!("Error reading entry in 'Localize': {}", e))?;
        let path = entry.path();

        if path.is_dir() && path.join("StoryData").is_dir() {
            debug!("Found compatible language directory: {:?}", path);
            return Ok(path);
        }
    }

    Err("Could not find language directory with StoryData in 'Localize'.".to_string())
}

fn install_to_game_directory(
    game_directory: &Option<String>,
    language_dir: &Path,
    localization: &Localization,
) -> Result<(), String> {
    let game_path = if let Some(game_directory) = game_directory {
        PathBuf::from(game_directory)
    } else {
        steam::get_game_directory()?
    };

    let target_base_path = game_path.join("LimbusCompany_Data").join("Lang");
    let target_path = target_base_path.join(&localization.id);

    debug!("Target installation path: {:?}", target_path);

    if target_path.exists() {
        info!("Removing existing localization at {:?}", target_path);
        fs::remove_dir_all(&target_path)
            .map_err(|e| format!("Failed to remove existing localization directory: {}", e))?;
    }

    fs::create_dir_all(&target_base_path)
        .map_err(|e| format!("Failed to create base Lang directory: {}", e))?;

    fs::create_dir(&target_path)
        .map_err(|e| format!("Failed to create target localization directory: {}", e))?;

    debug!("Moving files from {:?} to {:?}", language_dir, target_path);
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

        debug!("Copying {:?} -> {:?}", source_path, destination_path);

        if source_path.is_dir() {
            fs::create_dir_all(&destination_path)
                .map_err(|e| format!("Failed to create directory {:?}: {}", destination_path, e))?;
            copy_directory_contents(&source_path, &destination_path)?;
        } else {
            fs::copy(&source_path, &destination_path).map_err(|e| {
                format!(
                    "Failed to copy file {:?} to {:?}: {}",
                    source_path, destination_path, e
                )
            })?;
        }
    }

    Ok(())
}

fn calculate_md5(file_path: &Path) -> Result<String, String> {
    let file = fs::File::open(file_path)
        .map_err(|e| format!("Failed to open file for hashing {:?}: {}", file_path, e))?;

    let mut reader = BufReader::with_capacity(64 * 1024, file);
    let mut hasher = Md5::new();
    let mut buffer = [0; 1024];

    loop {
        let n = reader.read(&mut buffer).map_err(|e| {
            format!(
                "Failed to read file chunk for hashing {:?}: {}",
                file_path, e
            )
        })?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

async fn download_and_validate_font(
    url: &str,
    save_path: &Path,
    expected_hash: &str,
) -> Result<(), String> {
    let client = Client::new();

    info!("Starting download from {} to {:?}", url, save_path);

    let response = client
        .get(url)
        .header("User-Agent", "Limbus Launcher")
        .timeout(Duration::from_secs(300))
        .send()
        .await
        .map_err(|e| format!("Font download request error from {}: {}", url, e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Font download from {} failed with HTTP status {}",
            url,
            response.status()
        ));
    }

    if let Some(parent_dir) = save_path.parent() {
        fs::create_dir_all(parent_dir).map_err(|e| format!("Failed to create directory for font file {:?}: {}", parent_dir, e))?;
    }

    let temp_save_path = save_path.with_extension("tmp_download");

    let mut dest = fs::File::create(&temp_save_path).map_err(|e| {
        format!(
            "Failed to create temporary font file {:?}: {}",
            temp_save_path, e
        )
    })?;

    let mut hasher = Md5::new();
    let mut stream = response.bytes_stream();

    while let Some(chunk_result) = stream.next().await {
        let chunk =
            chunk_result.map_err(|e| format!("Error reading download stream from {}: {}", url, e))?;
        hasher.update(&chunk);
        dest.write_all(&chunk).map_err(|e| {
            format!(
                "Failed to write chunk to temp file {:?}: {}",
                temp_save_path, e
            )
        })?;
    }

    dest.sync_all().map_err(|e| format!("Failed to sync temporary font file {:?}: {}", temp_save_path, e))?;
    drop(dest);

    let calculated_hash = format!("{:x}", hasher.finalize());

    if calculated_hash != expected_hash {
        fs::remove_file(&temp_save_path).ok();
        Err(format!(
            "Font hash mismatch for {}. Expected: {}, Calculated: {}. Download saved to {:?} was discarded.",
            url, expected_hash, calculated_hash, temp_save_path
        ))
    } else {
        fs::rename(&temp_save_path, save_path).map_err(|e| format!("Failed to rename temporary font file {:?} to {:?}: {}", temp_save_path, save_path, e))?;
        info!(
            "Font downloaded successfully to {:?} and hash validated ({})",
            save_path, calculated_hash
        );
        Ok(())
    }
}
