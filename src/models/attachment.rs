use serde::{Deserialize, Serialize};

use super::common::{ImageDescriptor, TrelloID};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub id: Option<TrelloID>,
    pub bytes: Option<String>,
    pub date: Option<String>,
    pub edge_color: Option<crate::params::Color>,
    pub id_member: Option<TrelloID>,
    pub is_upload: Option<bool>,
    pub mime_type: Option<String>,
    pub name: Option<String>,
    pub previews: Option<Vec<ImageDescriptor>>,
    pub url: Option<String>,
    pub pos: Option<f64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AttachmentField {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "bytes")]
    Bytes,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "edgeColor")]
    EdgeColor,
    #[serde(rename = "idMember")]
    IdMember,
    #[serde(rename = "isUpload")]
    IsUpload,
    #[serde(rename = "mimeType")]
    MimeType,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "previews")]
    Previews,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "pos")]
    Pos,
}

impl crate::params::FieldQuery for AttachmentField {
    fn as_field_str(&self) -> &'static str {
        match self {
            Self::Id => "id",
            Self::Bytes => "bytes",
            Self::Date => "date",
            Self::EdgeColor => "edgeColor",
            Self::IdMember => "idMember",
            Self::IsUpload => "isUpload",
            Self::MimeType => "mimeType",
            Self::Name => "name",
            Self::Previews => "previews",
            Self::Url => "url",
            Self::Pos => "pos",
        }
    }
}
