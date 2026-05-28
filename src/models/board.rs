use serde::{Deserialize, Serialize};

use super::common::{ImageDescriptor, Limits, TrelloID};
use crate::params::{CardAging, FieldQuery};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Board {
    pub id: Option<TrelloID>,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub desc_data: Option<serde_json::Value>,
    pub closed: Option<bool>,
    pub id_member_creator: Option<TrelloID>,
    pub id_organization: Option<TrelloID>,
    pub pinned: Option<bool>,
    pub url: Option<String>,
    pub short_url: Option<String>,
    pub prefs: Option<BoardPrefs>,
    pub label_names: Option<serde_json::Value>,
    pub limits: Option<Limits>,
    pub starred: Option<bool>,
    pub memberships: Option<serde_json::Value>,
    pub short_link: Option<String>,
    pub subscribed: Option<bool>,
    pub power_ups: Option<serde_json::Value>,
    pub date_last_activity: Option<String>,
    pub date_last_view: Option<String>,
    pub id_tags: Option<serde_json::Value>,
    pub date_plugin_disable: Option<String>,
    pub creation_method: Option<String>,
    pub ix_update: Option<serde_json::Value>,
    pub template_gallery: Option<serde_json::Value>,
    pub enterprise_owned: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoardPrefs {
    pub permission_level: Option<String>,
    pub hide_votes: Option<bool>,
    pub voting: Option<String>,
    pub comments: Option<String>,
    pub invitations: Option<serde_json::Value>,
    pub self_join: Option<bool>,
    pub card_covers: Option<bool>,
    pub is_template: Option<bool>,
    pub card_aging: Option<CardAging>,
    pub calendar_feed_enabled: Option<bool>,
    pub background: Option<TrelloID>,
    pub background_image: Option<String>,
    pub background_image_scaled: Option<Vec<ImageDescriptor>>,
    pub background_tile: Option<bool>,
    pub background_brightness: Option<String>,
    pub background_bottom_color: Option<String>,
    pub background_top_color: Option<String>,
    pub can_be_public: Option<bool>,
    pub can_be_enterprise: Option<bool>,
    pub can_be_org: Option<bool>,
    pub can_be_private: Option<bool>,
    pub can_invite: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoardStars {
    pub id: Option<TrelloID>,
    pub id_board: Option<TrelloID>,
    pub pos: Option<i64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BoardField {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "desc")]
    Desc,
    #[serde(rename = "descData")]
    DescData,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "idMemberCreator")]
    IdMemberCreator,
    #[serde(rename = "idOrganization")]
    IdOrganization,
    #[serde(rename = "pinned")]
    Pinned,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "shortUrl")]
    ShortUrl,
    #[serde(rename = "prefs")]
    Prefs,
    #[serde(rename = "labelNames")]
    LabelNames,
    #[serde(rename = "starred")]
    Starred,
    #[serde(rename = "limits")]
    Limits,
    #[serde(rename = "memberships")]
    Memberships,
    #[serde(rename = "enterpriseOwned")]
    EnterpriseOwned,
}

impl FieldQuery for BoardField {
    fn as_field_str(&self) -> &'static str {
        match self {
            Self::Id => "id",
            Self::Name => "name",
            Self::Desc => "desc",
            Self::DescData => "descData",
            Self::Closed => "closed",
            Self::IdMemberCreator => "idMemberCreator",
            Self::IdOrganization => "idOrganization",
            Self::Pinned => "pinned",
            Self::Url => "url",
            Self::ShortUrl => "shortUrl",
            Self::Prefs => "prefs",
            Self::LabelNames => "labelNames",
            Self::Starred => "starred",
            Self::Limits => "limits",
            Self::Memberships => "memberships",
            Self::EnterpriseOwned => "enterpriseOwned",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BoardMembershipType {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "observer")]
    Observer,
}
