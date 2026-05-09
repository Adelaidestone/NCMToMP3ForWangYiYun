use crate::models::FileInfo;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn is_ncm_file(path: &Path) -> bool {
    path.extension()
        .is_some_and(|ext| ext.to_string_lossy().eq_ignore_ascii_case("ncm"))
}

fn file_info_from_path(path: &Path, name: &std::ffi::OsStr) -> Option<FileInfo> {
    Some(FileInfo {
        path: path.to_str()?.to_string(),
        name: name.to_str()?.to_string(),
    })
}

pub fn output_path_for_ncm(input_path: &Path, output_dir: &Path) -> Option<PathBuf> {
    let file_stem = input_path.file_stem()?.to_string_lossy();
    Some(output_dir.join(format!("{file_stem}.mp3")))
}

pub fn scan_ncm(input_dir: &Path) -> Vec<FileInfo> {
    WalkDir::new(input_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| is_ncm_file(entry.path()))
        .filter_map(|entry| file_info_from_path(entry.path(), entry.file_name()))
        .collect()
}

pub fn scan_pending_ncm(input_dir: &Path, output_dir: &Path) -> Vec<FileInfo> {
    WalkDir::new(input_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| is_ncm_file(entry.path()))
        .filter(|entry| {
            output_path_for_ncm(entry.path(), output_dir)
                .is_some_and(|output_path| !output_path.exists())
        })
        .filter_map(|entry| file_info_from_path(entry.path(), entry.file_name()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

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
}
