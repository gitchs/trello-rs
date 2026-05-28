use serde::{Deserialize, Serialize};

use super::common::TrelloID;
use crate::params::FieldQuery;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub id: Option<TrelloID>,
    pub identifier: Option<String>,
    pub id_member: Option<TrelloID>,
    pub date_created: Option<String>,
    pub date_expires: Option<String>,
    pub permissions: Option<Vec<TokenPermission>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenPermission {
    pub id_model: Option<String>,
    pub model_type: Option<String>,
    pub read: Option<bool>,
    pub write: Option<bool>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TokenField {
    #[serde(rename = "identifier")]
    Identifier,
    #[serde(rename = "idMember")]
    IdMember,
    #[serde(rename = "dateCreated")]
    DateCreated,
    #[serde(rename = "dateExpires")]
    DateExpires,
    #[serde(rename = "permissions")]
    Permissions,
}

impl FieldQuery for TokenField {
    fn as_field_str(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::IdMember => "idMember",
            Self::DateCreated => "dateCreated",
            Self::DateExpires => "dateExpires",
            Self::Permissions => "permissions",
        }
    }
}
