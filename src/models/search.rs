use serde::{Deserialize, Serialize};

use super::{board::Board, card::Card, member::Member, organization::Organization};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResults {
    pub cards: Option<Vec<Card>>,
    pub boards: Option<Vec<Board>>,
    pub members: Option<Vec<Member>>,
    pub organizations: Option<Vec<Organization>>,
    pub options: Option<SearchOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchOptions {
    pub terms: Option<Vec<serde_json::Value>>,
    pub modifiers: Option<Vec<serde_json::Value>>,
    pub model_types: Option<Vec<String>>,
    pub partial: Option<bool>,
    pub total: Option<i64>,
}
