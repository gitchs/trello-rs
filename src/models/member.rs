use serde::{Deserialize, Serialize};

use super::common::{LimitsObject, TrelloID};
use crate::params::FieldQuery;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub id: Option<TrelloID>,
    pub activity_blocked: Option<bool>,
    pub avatar_hash: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub bio_data: Option<serde_json::Value>,
    pub confirmed: Option<bool>,
    pub full_name: Option<String>,
    pub id_enterprise: Option<TrelloID>,
    pub id_enterprises_deactivated: Option<Vec<serde_json::Value>>,
    pub id_member_referrer: Option<TrelloID>,
    pub id_prem_orgs_admin: Option<Vec<serde_json::Value>>,
    pub initials: Option<String>,
    pub member_type: Option<String>,
    pub non_public: Option<serde_json::Value>,
    pub non_public_available: Option<bool>,
    pub products: Option<Vec<serde_json::Value>>,
    pub url: Option<String>,
    pub username: Option<String>,
    pub status: Option<String>,
    pub aa_email: Option<String>,
    pub aa_enrolled_date: Option<String>,
    pub aa_id: Option<String>,
    pub avatar_source: Option<String>,
    pub email: Option<String>,
    pub gravatar_hash: Option<String>,
    pub id_boards: Option<Vec<TrelloID>>,
    pub id_organizations: Option<Vec<TrelloID>>,
    pub id_enterprises_admin: Option<Vec<serde_json::Value>>,
    pub limits: Option<LimitsObject>,
    pub login_types: Option<Vec<String>>,
    pub marketing_opt_in: Option<serde_json::Value>,
    pub messages_dismissed: Option<serde_json::Value>,
    pub one_time_messages_dismissed: Option<Vec<String>>,
    pub prefs: Option<MemberPrefs>,
    pub trophies: Option<Vec<serde_json::Value>>,
    pub uploaded_avatar_hash: Option<String>,
    pub uploaded_avatar_url: Option<String>,
    pub premium_features: Option<Vec<String>>,
    pub is_aa_mastered: Option<bool>,
    pub ix_update: Option<serde_json::Value>,
    pub id_boards_pinned: Option<Vec<TrelloID>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberPrefs {
    pub timezone_info: Option<serde_json::Value>,
    pub privacy: Option<serde_json::Value>,
    pub send_summaries: Option<bool>,
    pub minutes_between_summaries: Option<i64>,
    pub minutes_before_deadline_to_notify: Option<i64>,
    pub color_blind: Option<bool>,
    pub locale: Option<String>,
    pub timezone: Option<String>,
    pub two_factor: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MemberField {
    #[serde(rename = "id")]
    Id,
}

impl FieldQuery for MemberField {
    fn as_field_str(&self) -> &'static str {
        "id"
    }
}
