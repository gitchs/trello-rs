use serde::{Deserialize, Serialize};

use super::common::TrelloID;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Checklist {
    pub id: Option<TrelloID>,
    pub name: Option<String>,
    pub id_board: Option<TrelloID>,
    pub id_card: Option<TrelloID>,
    pub pos: Option<f64>,
    pub check_items: Option<Vec<CheckItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckItem {
    pub id_checklist: Option<TrelloID>,
    pub state: Option<String>,
    pub id: Option<TrelloID>,
    pub name: Option<String>,
    pub name_data: Option<String>,
    pub pos: Option<String>,
}
