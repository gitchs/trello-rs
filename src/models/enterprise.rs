use serde::{Deserialize, Serialize};

use super::common::TrelloID;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Enterprise {
    pub id: Option<TrelloID>,
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub logo_hash: Option<String>,
    pub logo_url: Option<String>,
    pub prefs: Option<serde_json::Value>,
    pub organization_prefs: Option<super::organization::OrganizationPrefs>,
    pub sso_activation_failed: Option<bool>,
    pub id_admins: Option<Vec<TrelloID>>,
    pub enterprise_domains: Option<Vec<serde_json::Value>>,
    pub is_real_enterprise: Option<bool>,
    pub plugin_whitelisting_enabled: Option<Vec<serde_json::Value>>,
    pub id_organizations: Option<Vec<TrelloID>>,
    pub products: Option<Vec<serde_json::Value>>,
    pub licenses: Option<serde_json::Value>,
    pub domains: Option<Vec<String>>,
    pub date_organization_prefs_last_updated: Option<String>,
    pub idp: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnterpriseAdmin {
    pub id: Option<TrelloID>,
    pub full_name: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnterpriseAuditLog {
    pub id_action: Option<TrelloID>,
    #[serde(rename = "type")]
    pub action_type: Option<String>,
    pub date: Option<String>,
    pub member_creator: Option<serde_json::Value>,
    pub organization: Option<serde_json::Value>,
    pub member: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimableOrganization {
    pub organizations: Option<Vec<serde_json::Value>>,
    pub claimable_count: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PendingOrganization {
    pub id: Option<TrelloID>,
    pub id_member: Option<TrelloID>,
    pub member_requestor: Option<serde_json::Value>,
    pub date: Option<String>,
    pub display_name: Option<String>,
    pub membership_count: Option<f64>,
    pub logo_url: Option<String>,
    pub transferability: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferrableOrganization {
    pub transferrable: Option<bool>,
    pub new_billable_members: Option<Vec<serde_json::Value>>,
    pub restricted_members: Option<Vec<serde_json::Value>>,
}
