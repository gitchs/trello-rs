use serde::{Deserialize, Serialize};

use super::common::TrelloID;
use crate::params::FieldQuery;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    pub id: Option<TrelloID>,
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub date_last_activity: Option<String>,
    pub prefs: Option<OrganizationPrefs>,
    pub id_enterprise: Option<TrelloID>,
    pub offering: Option<String>,
    pub url: Option<String>,
    pub id_boards: Option<Vec<TrelloID>>,
    pub memberships: Option<Vec<super::common::Membership>>,
    pub premium_features: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationPrefs {
    pub board_visibility_restrict: Option<serde_json::Value>,
    pub board_delete_restrict: Option<serde_json::Value>,
    pub attachment_restrictions: Option<Vec<serde_json::Value>>,
    pub permission_level: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationExport {
    pub id: Option<TrelloID>,
    pub status: Option<String>,
    pub started_at: Option<String>,
    pub size: Option<String>,
    pub export_url: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OrganizationField {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "name")]
    Name,
}

impl FieldQuery for OrganizationField {
    fn as_field_str(&self) -> &'static str {
        match self {
            Self::Id => "id",
            Self::Name => "name",
        }
    }
}
