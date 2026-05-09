mod commands;
mod conversion;
mod id3_tags;
mod lyrics;
mod models;
mod ncm;
mod scanner;
mod settings;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::scan_ncm_files,
            commands::scan_pending_ncm_files,
            commands::load_settings,
            commands::save_settings,
            commands::convert_files,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
