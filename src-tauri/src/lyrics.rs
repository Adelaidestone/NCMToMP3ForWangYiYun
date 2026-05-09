use std::fs;
use std::path::{Path, PathBuf};

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

pub fn read_matching_lyrics(input_path: &Path) -> Option<String> {
    let lyrics_path = find_lyrics_path(input_path)?;
    fs::read_to_string(lyrics_path).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

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
