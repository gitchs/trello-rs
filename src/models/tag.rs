use serde::{Deserialize, Serialize};

use super::common::TrelloID;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: Option<TrelloID>,
    pub name: Option<String>,
}
