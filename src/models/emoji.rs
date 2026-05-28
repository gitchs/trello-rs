use serde::{Deserialize, Serialize};

use super::common::TrelloID;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Emoji {
    pub trello: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomEmoji {
    pub id: Option<TrelloID>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomSticker {
    pub id: Option<TrelloID>,
    pub url: Option<String>,
    pub scaled: Option<Vec<serde_json::Value>>,
}
