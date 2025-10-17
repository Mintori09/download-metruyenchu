use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Chapter {
    pub name: String,
    pub link: String,
    #[serde(default)]
    pub is_download: bool,
}
