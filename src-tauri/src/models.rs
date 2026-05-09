use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
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
