use std::{
    fs::{self, read_to_string, File},
    path::PathBuf,
};

use anyhow::Result;
use platform_dirs::AppDirs;
use simpleini::Ini;

const APP_NAME: &'static str = "scratchpad-rs";
const DEFAULT_STORAGE_FILENAME: &'static str = "scratchpad.txt";
const PREFERENCES_FILENAME: &'static str = "preferences.ini";

fn default_settings_path(filename: &str) -> PathBuf {
    let app_dirs = AppDirs::new(Some(APP_NAME), true).unwrap();
    let config_file_path = app_dirs.config_dir.join(filename);
    fs::create_dir_all(&app_dirs.config_dir).unwrap();
    config_file_path
}

pub fn preferences_save(key: &str, val: &str) -> Result<()> {
    let path = default_settings_path(PREFERENCES_FILENAME);
    if !path.exists() {
        File::create(&path)?;
    }
    let mut ini = Ini::from_file(&path)?;
    ini.set(key, val);
    ini.to_file(&path)?;

    Ok(())
}

pub fn preferences_read(key: &str, default: &str) -> Result<String> {
    let path = default_settings_path(PREFERENCES_FILENAME);
    if !path.exists() {
        File::create(&path)?;
    }
    let ini = Ini::from_file(&path)?;
    let value = match ini.get(key) {
        Some(v) => v,
        None => default,
    };

    Ok(value.to_string())
}

pub fn load_scratchpad_contents() -> Result<String> {
    let path = default_settings_path(DEFAULT_STORAGE_FILENAME);
    let storage = preferences_read("storage_file", path.to_str().unwrap())?;
    let pb = PathBuf::from(&storage);
    if !pb.exists() {
        return Ok(String::from(""));
    }

    let result = read_to_string(&storage)?;

    Ok(result)
}

pub fn save_scratchpad_contents(contents: &str) -> Result<()> {
    let default_path = default_settings_path(DEFAULT_STORAGE_FILENAME);
    let storage_path = preferences_read("storage_file", default_path.to_str().unwrap())?;
    let pb = PathBuf::from(storage_path);

    fs::write(&pb, contents.as_bytes())?;

    Ok(())
}
