use crate::id3_tags::write_id3_metadata;
use crate::lyrics::read_matching_lyrics;
use crate::models::ProgressEvent;
use crate::ncm::{NcmDecryptor, NcmError};
use rayon::prelude::*;
use std::path::PathBuf;
use tauri::Emitter;

fn output_path_for_conversion(
    path: &std::path::Path,
    output_dir: Option<&str>,
) -> Result<PathBuf, NcmError> {
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

    let output_path = if let Some(dir) = output_dir {
        let mut path = PathBuf::from(dir);
        path.push(format!("{}.mp3", base_name));
        path
    } else {
        let mut path = path.to_path_buf();
        path.set_extension("mp3");
        path
    };

    Ok(output_path)
}

fn emit_progress(
    app: &tauri::AppHandle,
    path: &str,
    status: &str,
    progress: f64,
    error: Option<String>,
) {
    let _ = app.emit(
        "file-progress",
        ProgressEvent {
            path: path.to_string(),
            status: status.to_string(),
            progress,
            error,
        },
    );
}

fn conversion_error_message(error: NcmError) -> String {
    match error {
        NcmError::Io(io_err) => format!("IO错误: {}", io_err),
        NcmError::InvalidHeader => "无效的NCM文件格式".to_string(),
        NcmError::KeyReadError => "无法读取密钥数据".to_string(),
        NcmError::AesDecryptError => "AES解密失败".to_string(),
        NcmError::InvalidAudioKey => "无效的音频密钥".to_string(),
        NcmError::TagWriteError(tag_err) => format!("写入封面标签失败: {}", tag_err),
    }
}

pub fn convert_file_batch(files: Vec<String>, output_dir: Option<String>, app: tauri::AppHandle) {
    files.par_iter().for_each(|file_path| {
        let path = PathBuf::from(file_path);

        emit_progress(&app, file_path, "processing", 0.0, None);

        let result = || -> Result<(), NcmError> {
            let decryptor = NcmDecryptor::from_path(&path)?;
            let output_path = output_path_for_conversion(&path, output_dir.as_deref())?;

            emit_progress(&app, file_path, "processing", 30.0, None);
            decryptor.decrypt_audio(&path, &output_path)?;

            emit_progress(&app, file_path, "processing", 80.0, None);
            let _ = write_id3_metadata(
                &output_path,
                &decryptor.music_metadata(),
                decryptor.cover_data(),
                read_matching_lyrics(&path).as_deref(),
            );

            Ok(())
        }();

        match result {
            Ok(_) => emit_progress(&app, file_path, "success", 100.0, None),
            Err(error) => emit_progress(
                &app,
                file_path,
                "error",
                0.0,
                Some(conversion_error_message(error)),
            ),
        }
    });
}
