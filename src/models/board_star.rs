use serde::{Deserialize, Serialize};

use super::common::TrelloID;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoardStar {
    pub id: Option<TrelloID>,
    pub id_board: Option<TrelloID>,
    pub pos: Option<i64>,
}
