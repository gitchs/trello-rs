use serde::{Deserialize, Serialize};

use super::common::TrelloID;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomField {
    pub id: Option<TrelloID>,
    pub id_model: Option<String>,
    pub model_type: Option<String>,
    pub field_group: Option<String>,
    pub display: Option<CustomFieldDisplay>,
    #[serde(rename = "type")]
    pub field_type: Option<String>,
    pub name: Option<String>,
    pub pos: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomFieldDisplay {
    pub card_front: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomFieldItems {
    pub id: Option<TrelloID>,
    pub value: Option<serde_json::Value>,
    pub id_custom_field: Option<TrelloID>,
    pub id_model: Option<TrelloID>,
    pub model_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomFieldValue {
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomFieldOption {
    pub id: Option<TrelloID>,
    pub value: Option<serde_json::Value>,
    pub color: Option<String>,
    pub pos: Option<f64>,
}
