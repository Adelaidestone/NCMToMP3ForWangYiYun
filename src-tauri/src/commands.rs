use crate::conversion::convert_file_batch;
use crate::models::{AppSettings, FileInfo};
use crate::scanner::{scan_ncm, scan_pending_ncm};
use crate::settings::{load_settings_from_output_dir, save_settings_to_output_dir};
use std::path::Path;

#[tauri::command]
pub async fn scan_ncm_files(path: String) -> Vec<FileInfo> {
    scan_ncm(Path::new(&path))
}

#[tauri::command]
pub async fn scan_pending_ncm_files(input_dir: String, output_dir: String) -> Vec<FileInfo> {
    scan_pending_ncm(Path::new(&input_dir), Path::new(&output_dir))
}

#[tauri::command]
pub async fn load_settings(output_dir: String) -> Result<Option<AppSettings>, String> {
    load_settings_from_output_dir(output_dir)
}

#[tauri::command]
pub async fn save_settings(settings: AppSettings) -> Result<(), String> {
    save_settings_to_output_dir(&settings)
}

#[tauri::command]
pub async fn convert_files(
    files: Vec<String>,
    output_dir: Option<String>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    convert_file_batch(files, output_dir, app);
    Ok(())
}

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to NCM Converter.", name)
}
