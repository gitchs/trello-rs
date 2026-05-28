use serde::{Deserialize, Serialize};

use super::common::TrelloID;
use crate::params::Color;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub id: Option<TrelloID>,
    pub id_board: Option<TrelloID>,
    pub name: Option<String>,
    pub color: Option<Color>,
}
