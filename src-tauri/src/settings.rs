use crate::models::AppSettings;
use std::fs;
use std::path::{Path, PathBuf};

pub const SETTINGS_FILE_NAME: &str = ".ncm-converter-settings.json";

pub fn settings_file_path(output_dir: impl AsRef<Path>) -> PathBuf {
    output_dir.as_ref().join(SETTINGS_FILE_NAME)
}

pub fn load_settings_from_output_dir(
    output_dir: impl AsRef<Path>,
) -> Result<Option<AppSettings>, String> {
    let path = settings_file_path(output_dir);
    if !path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&path).map_err(|err| format!("读取设置文件失败: {}", err))?;
    let settings =
        serde_json::from_str(&content).map_err(|err| format!("解析设置文件失败: {}", err))?;

    Ok(Some(settings))
}

pub fn save_settings_to_output_dir(settings: &AppSettings) -> Result<(), String> {
    if settings.output_dir.trim().is_empty() {
        return Err("输出文件夹不能为空".to_string());
    }

    let path = settings_file_path(&settings.output_dir);
    let content =
        serde_json::to_string_pretty(settings).map_err(|err| format!("序列化设置失败: {}", err))?;
    fs::write(&path, content).map_err(|err| format!("保存设置文件失败: {}", err))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_file_lives_in_output_directory() {
        let output_dir = std::env::temp_dir().join("ncm-converter-output");

        assert_eq!(
            settings_file_path(&output_dir),
            output_dir.join(SETTINGS_FILE_NAME)
        );
    }
}
