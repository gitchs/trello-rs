use serde::{Deserialize, Serialize};

use super::common::TrelloID;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoardBackground {
    pub id: Option<TrelloID>,
    pub url: Option<String>,
    pub color: Option<String>,
    pub brightness: Option<String>,
    pub tile: Option<bool>,
    pub scaled: Option<Vec<super::common::ImageDescriptor>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomBoardBackground {
    pub id: Option<TrelloID>,
    pub url: Option<String>,
    pub scaled: Option<Vec<super::common::ImageDescriptor>>,
}
