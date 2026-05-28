use serde::{Deserialize, Serialize};

/// A Trello resource identifier (24-character hex string).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TrelloID(pub String);

impl TrelloID {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl std::fmt::Display for TrelloID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for TrelloID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for TrelloID {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for TrelloID {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// A limit threshold object: `{ status, disableAt, warnAt }`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitsObject {
    pub status: Option<String>,
    pub disable_at: Option<f64>,
    pub warn_at: Option<f64>,
}

/// Common limits object for boards/cards.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Limits {
    pub attachments: Option<AttachmentsLimits>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentsLimits {
    pub per_board: Option<LimitsObject>,
    pub per_card: Option<LimitsObject>,
}

/// An image descriptor: `{ url, width, height }`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageDescriptor {
    pub url: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
}

/// Board/card membership entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Membership {
    #[serde(default)]
    pub managed: bool,
    #[serde(default)]
    pub licensed: bool,
    #[serde(default)]
    pub admin: bool,
    #[serde(default)]
    pub deactivated: bool,
    #[serde(default)]
    pub collaborator: bool,
    pub member: Option<MemberRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberRef {
    pub id: Option<TrelloID>,
    pub full_name: Option<String>,
    pub username: Option<String>,
}

/// Simple membership id wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Memberships {
    pub id: Option<TrelloID>,
}

/// Field enums for API response filtering.
pub trait FieldQuery {
    fn as_field_str(&self) -> &'static str;
}
