mod ncm;

use id3::frame::{Lyrics, Picture, PictureType};
use id3::{Tag, TagLike, Version};
use ncm::{MusicMetadata, NcmDecryptor, NcmError};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::Emitter;
use walkdir::WalkDir;

const SETTINGS_FILE_NAME: &str = ".ncm-converter-settings.json";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileInfo {
    pub path: String,
    pub name: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ProgressEvent {
    pub path: String,
    pub status: String,
    pub progress: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub input_dir: String,
    pub output_dir: String,
    pub auto_convert_on_start: bool,
}

fn settings_file_path(output_dir: impl AsRef<Path>) -> PathBuf {
    output_dir.as_ref().join(SETTINGS_FILE_NAME)
}

fn output_path_for_ncm(input_path: &Path, output_dir: &Path) -> Option<PathBuf> {
    let file_stem = input_path.file_stem()?.to_string_lossy();
    Some(output_dir.join(format!("{file_stem}.mp3")))
}

fn scan_pending_ncm(input_dir: &Path, output_dir: &Path) -> Vec<FileInfo> {
    let mut files = Vec::new();

    for entry in WalkDir::new(input_dir).into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue;
        }

        let is_ncm = entry
            .path()
            .extension()
            .is_some_and(|ext| ext.to_string_lossy().eq_ignore_ascii_case("ncm"));
        if !is_ncm {
            continue;
        }

        let Some(output_path) = output_path_for_ncm(entry.path(), output_dir) else {
            continue;
        };
        if output_path.exists() {
            continue;
        }

        if let (Some(path_str), Some(name_str)) =
            (entry.path().to_str(), entry.file_name().to_str())
        {
            files.push(FileInfo {
                path: path_str.to_string(),
                name: name_str.to_string(),
            });
        }
    }

    files
}

fn cover_mime_type(data: &[u8]) -> &'static str {
    if data.starts_with(&[0xff, 0xd8, 0xff]) {
        "image/jpeg"
    } else if data.starts_with(b"\x89PNG\r\n\x1a\n") {
        "image/png"
    } else {
        "image/jpeg"
    }
}

fn write_id3_metadata(
    path: &Path,
    music_metadata: &MusicMetadata,
    cover: Option<&[u8]>,
    lyrics: Option<&str>,
) -> Result<(), NcmError> {
    let mut tag = Tag::read_from_path(path).unwrap_or_else(|_| Tag::new());

    if let Some(title) = &music_metadata.title {
        tag.set_title(title);
    }
    if let Some(artist) = &music_metadata.artist {
        tag.set_artist(artist);
    }
    if let Some(album) = &music_metadata.album {
        tag.set_album(album);
    }

    if let Some(cover) = cover.filter(|cover| !cover.is_empty()) {
        tag.remove_all_pictures();
        tag.add_frame(Picture {
            mime_type: cover_mime_type(cover).to_string(),
            picture_type: PictureType::CoverFront,
            description: String::new(),
            data: cover.to_vec(),
        });
    }

    if let Some(lyrics) = lyrics.filter(|lyrics| !lyrics.trim().is_empty()) {
        tag.remove_all_lyrics();
        tag.add_frame(Lyrics {
            lang: "und".to_string(),
            description: String::new(),
            text: lyrics.to_string(),
        });
    }

    tag.write_to_path(path, Version::Id3v24)
        .map_err(|err| NcmError::TagWriteError(err.to_string()))
}

fn find_lyrics_path(input_path: &Path) -> Option<PathBuf> {
    let dir = input_path.parent()?;
    let stem = input_path.file_stem()?.to_string_lossy();

    ["lrc", "irc"]
        .into_iter()
        .flat_map(|ext| {
            [
                dir.join(format!("{stem}.{ext}")),
                dir.join(format!("{stem}.{}", ext.to_uppercase())),
            ]
        })
        .find(|path| path.exists())
}

fn read_matching_lyrics(input_path: &Path) -> Option<String> {
    let lyrics_path = find_lyrics_path(input_path)?;
    fs::read_to_string(lyrics_path).ok()
}

#[tauri::command]
async fn scan_ncm_files(path: String) -> Vec<FileInfo> {
    let mut files = Vec::new();

    for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext.to_string_lossy().to_lowercase() == "ncm" {
                    if let (Some(path_str), Some(name_str)) =
                        (entry.path().to_str(), entry.file_name().to_str())
                    {
                        files.push(FileInfo {
                            path: path_str.to_string(),
                            name: name_str.to_string(),
                        });
                    }
                }
            }
        }
    }

    files
}

#[tauri::command]
async fn scan_pending_ncm_files(input_dir: String, output_dir: String) -> Vec<FileInfo> {
    scan_pending_ncm(Path::new(&input_dir), Path::new(&output_dir))
}

#[tauri::command]
async fn load_settings(output_dir: String) -> Result<Option<AppSettings>, String> {
    let path = settings_file_path(output_dir);
    if !path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&path).map_err(|err| format!("读取设置文件失败: {}", err))?;
    let settings =
        serde_json::from_str(&content).map_err(|err| format!("解析设置文件失败: {}", err))?;

    Ok(Some(settings))
}

#[tauri::command]
async fn save_settings(settings: AppSettings) -> Result<(), String> {
    if settings.output_dir.trim().is_empty() {
        return Err("输出文件夹不能为空".to_string());
    }

    let path = settings_file_path(&settings.output_dir);
    let content = serde_json::to_string_pretty(&settings)
        .map_err(|err| format!("序列化设置失败: {}", err))?;
    fs::write(&path, content).map_err(|err| format!("保存设置文件失败: {}", err))
}

#[tauri::command]
async fn convert_files(
    files: Vec<String>,
    output_dir: Option<String>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let output_dir_cloned = output_dir.clone();

    files.par_iter().for_each(|file_path| {
        let path = PathBuf::from(file_path);

        let _ = app.emit(
            "file-progress",
            ProgressEvent {
                path: file_path.clone(),
                status: "processing".to_string(),
                progress: 0.0,
                error: None,
            },
        );

        let result = || -> Result<(), NcmError> {
            let decryptor = NcmDecryptor::from_path(&path)?;

            let file_name = path.file_name().ok_or_else(|| {
                NcmError::Io(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid file name",
                ))
            })?;

            let file_name_str = file_name.to_string_lossy();
            let base_name = path
                .file_stem()
                .map(|stem| stem.to_string_lossy())
                .unwrap_or(file_name_str);

            let output_path = if let Some(ref dir) = output_dir_cloned {
                let mut p = PathBuf::from(dir);
                p.push(format!("{}.mp3", base_name));
                p
            } else {
                let mut p = path.clone();
                p.set_extension("mp3");
                p
            };

            let _ = app.emit(
                "file-progress",
                ProgressEvent {
                    path: file_path.clone(),
                    status: "processing".to_string(),
                    progress: 30.0,
                    error: None,
                },
            );

            decryptor.decrypt_audio(&path, &output_path)?;

            let _ = app.emit(
                "file-progress",
                ProgressEvent {
                    path: file_path.clone(),
                    status: "processing".to_string(),
                    progress: 80.0,
                    error: None,
                },
            );

            let _ = write_id3_metadata(
                &output_path,
                &decryptor.music_metadata(),
                decryptor.cover_data(),
                read_matching_lyrics(&path).as_deref(),
            );

            Ok(())
        }();

        match result {
            Ok(_) => {
                let _ = app.emit(
                    "file-progress",
                    ProgressEvent {
                        path: file_path.clone(),
                        status: "success".to_string(),
                        progress: 100.0,
                        error: None,
                    },
                );
            }
            Err(e) => {
                let error_msg = match e {
                    NcmError::Io(io_err) => format!("IO错误: {}", io_err),
                    NcmError::InvalidHeader => "无效的NCM文件格式".to_string(),
                    NcmError::KeyReadError => "无法读取密钥数据".to_string(),
                    NcmError::AesDecryptError => "AES解密失败".to_string(),
                    NcmError::InvalidAudioKey => "无效的音频密钥".to_string(),
                    NcmError::TagWriteError(tag_err) => format!("写入封面标签失败: {}", tag_err),
                };

                let _ = app.emit(
                    "file-progress",
                    ProgressEvent {
                        path: file_path.clone(),
                        status: "error".to_string(),
                        progress: 0.0,
                        error: Some(error_msg),
                    },
                );
            }
        }
    });

    Ok(())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to NCM Converter.", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            scan_ncm_files,
            scan_pending_ncm_files,
            load_settings,
            save_settings,
            convert_files,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;
    use id3::Tag;
    use std::fs;

    #[test]
    fn embeds_cover_art_in_mp3_id3_tag() {
        let path = std::env::temp_dir().join(format!(
            "ncm-converter-cover-test-{}.mp3",
            std::process::id()
        ));
        let sidecar_path = path.with_extension("jpg");
        let cover = vec![0xff, 0xd8, 0xff, 0xe0, 0x00, 0x10, b'J', b'F', b'I', b'F'];

        fs::write(&path, [0xff, 0xfb, 0x90, 0x64]).unwrap();
        let _ = fs::remove_file(&sidecar_path);

        let metadata = MusicMetadata {
            title: Some("Song Title".to_string()),
            artist: Some("Artist One / Artist Two".to_string()),
            album: Some("Album Name".to_string()),
        };

        let lyrics = "[00:01.00]第一句歌词\n[00:02.00]第二句歌词";

        write_id3_metadata(&path, &metadata, Some(&cover), Some(lyrics)).unwrap();

        let tag = Tag::read_from_path(&path).unwrap();
        let picture = tag.pictures().next().unwrap();

        assert_eq!(tag.title(), Some("Song Title"));
        assert_eq!(tag.artist(), Some("Artist One / Artist Two"));
        assert_eq!(tag.album(), Some("Album Name"));
        assert_eq!(picture.mime_type, "image/jpeg");
        assert_eq!(picture.picture_type, id3::frame::PictureType::CoverFront);
        assert_eq!(picture.data, cover);
        assert_eq!(tag.lyrics().next().unwrap().text, lyrics);
        assert!(!sidecar_path.exists());

        let _ = fs::remove_file(path);
    }

    #[test]
    fn scan_pending_ncm_skips_files_with_existing_mp3_outputs() {
        let root =
            std::env::temp_dir().join(format!("ncm-converter-pending-test-{}", std::process::id()));
        let input_dir = root.join("input");
        let output_dir = root.join("output");

        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&input_dir).unwrap();
        fs::create_dir_all(&output_dir).unwrap();

        fs::write(input_dir.join("new-song.ncm"), []).unwrap();
        fs::write(input_dir.join("done-song.ncm"), []).unwrap();
        fs::write(input_dir.join("ignore.txt"), []).unwrap();
        fs::write(output_dir.join("done-song.mp3"), []).unwrap();

        let pending = scan_pending_ncm(&input_dir, &output_dir);

        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].name, "new-song.ncm");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn settings_file_lives_in_output_directory() {
        let output_dir = std::env::temp_dir().join("ncm-converter-output");

        assert_eq!(
            settings_file_path(&output_dir),
            output_dir.join(SETTINGS_FILE_NAME)
        );
    }

    #[test]
    fn reads_same_name_lrc_lyrics_next_to_ncm_file() {
        let root =
            std::env::temp_dir().join(format!("ncm-converter-lyrics-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();

        let ncm_path = root.join("song.ncm");
        let lrc_path = root.join("song.lrc");
        let lyrics = "[00:10.00]hello";

        fs::write(&ncm_path, []).unwrap();
        fs::write(&lrc_path, lyrics).unwrap();

        assert_eq!(read_matching_lyrics(&ncm_path).as_deref(), Some(lyrics));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn missing_same_name_lyrics_returns_none() {
        let ncm_path = std::env::temp_dir()
            .join("ncm-converter-missing-lyrics")
            .join("song.ncm");

        assert!(read_matching_lyrics(&ncm_path).is_none());
    }
}
