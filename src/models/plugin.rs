use serde::{Deserialize, Serialize};

use super::common::TrelloID;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plugin {
    pub id: Option<TrelloID>,
    pub name: Option<String>,
    pub locale: Option<String>,
    pub description: Option<String>,
    pub overview: Option<String>,
    pub listing: Option<PluginListing>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginData {
    pub id: Option<TrelloID>,
    pub id_plugin: Option<TrelloID>,
    pub scope: Option<String>,
    pub id_model: Option<TrelloID>,
    pub value: Option<String>,
    pub access: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginListing {
    pub id: Option<TrelloID>,
    pub name: Option<String>,
    pub locale: Option<String>,
    pub description: Option<String>,
    pub overview: Option<String>,
}
