use aes::Aes128;
use base64::{engine::general_purpose, Engine as _};
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Ecb};
use serde_json::Value;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NcmError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Invalid NCM file header")]
    InvalidHeader,

    #[error("Failed to read key data")]
    KeyReadError,

    #[error("AES decryption failed")]
    AesDecryptError,

    #[error("Invalid audio key")]
    InvalidAudioKey,

    #[error("Failed to write ID3 tag: {0}")]
    TagWriteError(String),
}

const AES_CORE_KEY: [u8; 16] = [
    0x68, 0x7A, 0x48, 0x52, 0x41, 0x6D, 0x73, 0x6F, 0x35, 0x6B, 0x49, 0x6E, 0x62, 0x61, 0x78, 0x57,
];

const AES_MODIFY_KEY: [u8; 16] = [
    0x23, 0x31, 0x34, 0x6c, 0x6a, 0x6b, 0x5f, 0x21, 0x5c, 0x5d, 0x26, 0x30, 0x55, 0x3c, 0x27, 0x28,
];

const NCM_HEADER: &[u8] = b"CTENFDAM";

fn build_key_box(key: &[u8]) -> [u8; 256] {
    let mut box_ = std::array::from_fn(|i| i as u8);

    let key_len = key.len();
    let mut last_byte: u8 = 0;

    let mut i = 0;
    while i < 256 {
        let c = box_[i]
            .wrapping_add(last_byte)
            .wrapping_add(key[i % key_len]);
        box_.swap(i, c as usize);
        last_byte = c;
        i += 1;
    }

    box_
}

fn decrypt_audio(data: &[u8], key_box: &[u8; 256]) -> Vec<u8> {
    let mut result = Vec::with_capacity(data.len());

    for (i, byte) in data.iter().enumerate() {
        let j = (i + 1) & 0xff;
        let key_byte = key_box
            [(key_box[j] as usize + key_box[(key_box[j] as usize + j) & 0xff] as usize) & 0xff]
            as usize;
        result.push(*byte ^ key_byte as u8);
    }

    result
}

fn decrypt_aes128_ecb(key: &[u8], data: &[u8]) -> Result<Vec<u8>, NcmError> {
    let truncated_len = data.len() / 16 * 16;
    let truncated = &data[..truncated_len];

    let cipher =
        Ecb::<Aes128, Pkcs7>::new_from_slices(key, &[]).map_err(|_| NcmError::AesDecryptError)?;

    cipher
        .decrypt_vec(truncated)
        .map_err(|_| NcmError::AesDecryptError)
}

fn xor_bytes(data: &mut [u8], xor_key: u8) {
    for byte in data {
        *byte ^= xor_key;
    }
}

fn read_len_and_data<R: Read>(reader: &mut R) -> Result<Option<Vec<u8>>, io::Error> {
    let mut len_bytes = [0u8; 4];
    match reader.read_exact(&mut len_bytes) {
        Ok(_) => {}
        Err(_) => return Ok(None),
    }

    let length = u32::from_le_bytes(len_bytes) as usize;
    if length == 0 {
        return Ok(Some(Vec::new()));
    }

    let mut data = vec![0u8; length];
    reader.read_exact(&mut data)?;

    Ok(Some(data))
}

#[derive(Debug, Clone)]
pub struct NcmMetadata {
    pub modify_data: Option<Vec<u8>>,
    pub cover_data: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MusicMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NcmDecryptor {
    metadata: NcmMetadata,
    audio_key: Vec<u8>,
}

impl NcmDecryptor {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, NcmError> {
        let mut file = File::open(path)?;
        Self::from_reader(&mut file)
    }

    pub fn from_reader<R: Read + Seek>(reader: &mut R) -> Result<Self, NcmError> {
        reader.seek(SeekFrom::Start(0))?;

        let mut header = [0u8; 8];
        reader.read_exact(&mut header)?;

        if header != NCM_HEADER {
            return Err(NcmError::InvalidHeader);
        }

        reader.seek(SeekFrom::Start(10))?;

        let key_data = read_len_and_data(reader)?.ok_or(NcmError::KeyReadError)?;

        let mut key_data_array = key_data.clone();
        xor_bytes(&mut key_data_array, 0x64);

        let decrypted_key = decrypt_aes128_ecb(&AES_CORE_KEY, &key_data_array)?;

        let audio_key = if decrypted_key.len() > 17 {
            decrypted_key[17..].to_vec()
        } else {
            return Err(NcmError::InvalidAudioKey);
        };

        let modify_data = read_len_and_data(reader)?;

        reader.seek(SeekFrom::Current(9))?;

        let cover_data = read_len_and_data(reader)?;

        Ok(Self {
            metadata: NcmMetadata {
                modify_data,
                cover_data,
            },
            audio_key,
        })
    }

    pub fn decrypt_audio<P: AsRef<Path>>(
        &self,
        input_path: P,
        output_path: P,
    ) -> Result<(), NcmError> {
        let mut input_file = File::open(&input_path)?;

        let mut header = [0u8; 8];
        input_file.read_exact(&mut header)?;

        if header != NCM_HEADER {
            return Err(NcmError::InvalidHeader);
        }

        input_file.seek(SeekFrom::Start(10))?;

        read_len_and_data(&mut input_file)?;
        read_len_and_data(&mut input_file)?;

        input_file.seek(SeekFrom::Current(9))?;

        read_len_and_data(&mut input_file)?;

        let key_box = build_key_box(&self.audio_key);

        let chunk_size = 0x8000;
        let mut buffer = vec![0u8; chunk_size];
        let mut output_file = File::create(&output_path)?;

        let mut found_mp3_start = false;
        let mut mp3_start_offset = 0usize;
        let mut first_chunk = Vec::new();

        loop {
            let bytes_read = input_file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }

            let chunk = &buffer[..bytes_read];
            let decrypted = decrypt_audio(chunk, &key_box);

            if !found_mp3_start {
                first_chunk.extend_from_slice(&decrypted);

                let search_limit = std::cmp::min(50000, first_chunk.len());
                for i in 0..search_limit.saturating_sub(2) {
                    let frame = &first_chunk[i..i + 2];
                    if frame == [0xFF, 0xFB]
                        || frame == [0xFF, 0xFA]
                        || frame == [0xFF, 0xF3]
                        || frame == [0xFF, 0xF2]
                    {
                        found_mp3_start = true;
                        mp3_start_offset = i;
                        break;
                    }
                }

                if found_mp3_start {
                    output_file.write_all(&first_chunk[mp3_start_offset..])?;
                }
            } else {
                output_file.write_all(&decrypted)?;
            }
        }

        if !found_mp3_start && !first_chunk.is_empty() {
            output_file.write_all(&first_chunk)?;
        }

        Ok(())
    }

    pub fn cover_data(&self) -> Option<&[u8]> {
        self.metadata.cover_data.as_deref()
    }

    pub fn music_metadata(&self) -> MusicMetadata {
        MusicMetadata::from_modify_data(self.metadata.modify_data.as_deref())
    }
}

impl MusicMetadata {
    pub fn from_modify_data(data: Option<&[u8]>) -> Self {
        let Some(data) = data else {
            return Self::default();
        };

        parse_music_metadata(data).unwrap_or_default()
    }
}

fn parse_music_metadata(data: &[u8]) -> Option<MusicMetadata> {
    let mut decoded = data.to_vec();
    xor_bytes(&mut decoded, 0x63);

    let prefix = b"163 key(Don't modify):";
    if decoded.starts_with(prefix) {
        decoded = decoded[prefix.len()..].to_vec();
    }

    let encrypted = general_purpose::STANDARD.decode(decoded).ok()?;
    let decrypted = decrypt_aes128_ecb(&AES_MODIFY_KEY, &encrypted).ok()?;
    let json_bytes = decrypted.strip_prefix(b"music:").unwrap_or(&decrypted);
    let value: Value = serde_json::from_slice(json_bytes).ok()?;

    Some(MusicMetadata {
        title: value
            .get("musicName")
            .and_then(Value::as_str)
            .filter(|value| !value.trim().is_empty())
            .map(ToOwned::to_owned),
        artist: parse_artists(value.get("artist")),
        album: value
            .get("album")
            .and_then(Value::as_str)
            .filter(|value| !value.trim().is_empty())
            .map(ToOwned::to_owned),
    })
}

fn parse_artists(value: Option<&Value>) -> Option<String> {
    let artists = value?.as_array()?;
    let names = artists
        .iter()
        .filter_map(|artist| artist.as_array()?.first()?.as_str())
        .filter(|name| !name.trim().is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();

    if names.is_empty() {
        None
    } else {
        Some(names.join(" / "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use block_modes::block_padding::Pkcs7;
    use block_modes::{BlockMode, Ecb};

    fn encrypted_modify_data(json: &str) -> Vec<u8> {
        let cipher = Ecb::<Aes128, Pkcs7>::new_from_slices(&AES_MODIFY_KEY, &[]).unwrap();
        let encrypted = cipher.encrypt_vec(format!("music:{json}").as_bytes());
        let mut prefixed = format!(
            "163 key(Don't modify):{}",
            general_purpose::STANDARD.encode(encrypted)
        )
        .into_bytes();
        for byte in &mut prefixed {
            *byte ^= 0x63;
        }
        prefixed
    }

    #[test]
    fn parses_music_metadata_from_ncm_modify_data() {
        let data = encrypted_modify_data(
            r#"{"musicName":"Song Title","artist":[["Artist One",1],["Artist Two",2]],"album":"Album Name"}"#,
        );

        let metadata = MusicMetadata::from_modify_data(Some(&data));

        assert_eq!(metadata.title.as_deref(), Some("Song Title"));
        assert_eq!(metadata.artist.as_deref(), Some("Artist One / Artist Two"));
        assert_eq!(metadata.album.as_deref(), Some("Album Name"));
    }
}
