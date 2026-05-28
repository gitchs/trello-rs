use serde::{Deserialize, Serialize};

use super::common::TrelloID;
use crate::params::Pos;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedSearch {
    pub id: Option<TrelloID>,
    pub name: Option<String>,
    pub query: Option<String>,
    pub pos: Option<Pos>,
}
