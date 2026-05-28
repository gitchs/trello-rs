use serde::{Deserialize, Serialize};

use super::common::TrelloID;
use crate::params::FieldQuery;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub id: Option<TrelloID>,
    pub id_member_creator: Option<TrelloID>,
    pub data: Option<ActionData>,
    #[serde(rename = "type")]
    pub action_type: Option<String>,
    pub date: Option<String>,
    pub limits: Option<ActionLimits>,
    pub display: Option<ActionDisplay>,
    pub member_creator: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionData {
    pub text: Option<String>,
    pub card: Option<ActionCardRef>,
    pub board: Option<ActionBoardRef>,
    pub list: Option<ActionListRef>,
    pub list_before: Option<ActionListRef>,
    pub list_after: Option<ActionListRef>,
    pub organization: Option<ActionOrgRef>,
    pub old: Option<serde_json::Value>,
    pub attachment: Option<serde_json::Value>,
    pub checklist: Option<serde_json::Value>,
    pub check_item: Option<serde_json::Value>,
    pub member: Option<serde_json::Value>,
    pub plugin: Option<serde_json::Value>,
    pub sticker: Option<serde_json::Value>,
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionCardRef {
    pub id: Option<TrelloID>,
    pub name: Option<String>,
    pub id_short: Option<i64>,
    pub short_link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionBoardRef {
    pub id: Option<TrelloID>,
    pub name: Option<String>,
    pub short_link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionListRef {
    pub id: Option<TrelloID>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionOrgRef {
    pub id: Option<TrelloID>,
    pub name: Option<String>,
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionLimits {
    pub reactions: Option<ReactionLimits>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReactionLimits {
    pub per_action: Option<super::common::LimitsObject>,
    pub unique_per_action: Option<super::common::LimitsObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionDisplay {
    pub translation_key: Option<String>,
    pub entities: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReactionSummary {
    pub emoji: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ActionField {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "idMemberCreator")]
    IdMemberCreator,
    #[serde(rename = "data")]
    Data,
    #[serde(rename = "type")]
    ActionType,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "limits")]
    Limits,
    #[serde(rename = "display")]
    Display,
    #[serde(rename = "memberCreator")]
    MemberCreator,
}

impl FieldQuery for ActionField {
    fn as_field_str(&self) -> &'static str {
        match self {
            Self::Id => "id",
            Self::IdMemberCreator => "idMemberCreator",
            Self::Data => "data",
            Self::ActionType => "type",
            Self::Date => "date",
            Self::Limits => "limits",
            Self::Display => "display",
            Self::MemberCreator => "memberCreator",
        }
    }
}
