use crate::ncm::{MusicMetadata, NcmError};
use id3::frame::{Lyrics, Picture, PictureType};
use id3::{Tag, TagLike, Version};
use std::path::Path;

fn cover_mime_type(data: &[u8]) -> &'static str {
    if data.starts_with(&[0xff, 0xd8, 0xff]) {
        "image/jpeg"
    } else if data.starts_with(b"\x89PNG\r\n\x1a\n") {
        "image/png"
    } else {
        "image/jpeg"
    }
}

pub fn write_id3_metadata(
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
        assert_eq!(picture.picture_type, PictureType::CoverFront);
        assert_eq!(picture.data, cover);
        assert_eq!(tag.lyrics().next().unwrap().text, lyrics);
        assert!(!sidecar_path.exists());

        let _ = fs::remove_file(path);
    }
}
