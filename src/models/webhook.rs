use serde::{Deserialize, Serialize};

use super::common::TrelloID;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Webhook {
    pub id: Option<TrelloID>,
    pub description: Option<String>,
    pub id_model: Option<TrelloID>,
    pub callback_url: Option<String>,
    pub active: Option<bool>,
    pub consecutive_failures: Option<f64>,
    pub first_consecutive_fail_date: Option<String>,
}
