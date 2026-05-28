use serde::{Deserialize, Serialize};

use super::common::{Limits, TrelloID};
use crate::params::FieldQuery;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrelloList {
    pub id: Option<TrelloID>,
    pub name: Option<String>,
    pub closed: Option<bool>,
    pub pos: Option<f64>,
    pub soft_limit: Option<String>,
    pub id_board: Option<String>,
    pub subscribed: Option<bool>,
    pub limits: Option<Limits>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ListField {
    #[serde(rename = "id")]
    Id,
}

impl FieldQuery for ListField {
    fn as_field_str(&self) -> &'static str {
        "id"
    }
}
