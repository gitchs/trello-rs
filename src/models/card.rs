use serde::{Deserialize, Serialize};

use super::common::{Limits, TrelloID};
use crate::params::FieldQuery;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub id: Option<TrelloID>,
    pub address: Option<String>,
    pub badges: Option<serde_json::Value>,
    pub card_role: Option<String>,
    pub check_item_states: Option<Vec<serde_json::Value>>,
    pub closed: Option<bool>,
    pub coordinates: Option<serde_json::Value>,
    pub creation_method: Option<String>,
    pub date_last_activity: Option<String>,
    pub desc: Option<String>,
    pub desc_data: Option<serde_json::Value>,
    pub due: Option<String>,
    pub due_reminder: Option<String>,
    pub id_board: Option<TrelloID>,
    pub id_checklists: Option<Vec<TrelloID>>,
    pub id_labels: Option<Vec<TrelloID>>,
    pub id_list: Option<TrelloID>,
    pub id_members: Option<Vec<TrelloID>>,
    pub id_members_voted: Option<Vec<TrelloID>>,
    pub id_short: Option<i64>,
    pub id_attachment_cover: Option<TrelloID>,
    pub labels: Option<Vec<super::label::Label>>,
    pub limits: Option<Limits>,
    pub location_name: Option<String>,
    pub manual_cover_attachment: Option<bool>,
    pub mirror_source_id: Option<TrelloID>,
    pub name: Option<String>,
    pub pos: Option<f64>,
    pub short_link: Option<String>,
    pub short_url: Option<String>,
    pub subscribed: Option<bool>,
    pub url: Option<String>,
    pub cover: Option<CardCover>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardCover {
    pub id_attachment: Option<TrelloID>,
    pub color: Option<String>,
    pub id_uploaded_background: Option<TrelloID>,
    pub size: Option<String>,
    pub brightness: Option<String>,
    pub id_plugin: Option<TrelloID>,
    pub scaled: Option<Vec<super::common::ImageDescriptor>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardBadges {
    pub attachments: Option<i64>,
    pub check_items: Option<i64>,
    pub check_items_checked: Option<i64>,
    pub comments: Option<i64>,
    pub description: Option<bool>,
    pub due: Option<String>,
    pub due_complete: Option<bool>,
    pub start: Option<String>,
    pub subscribed: Option<bool>,
    pub viewing_member_voted: Option<bool>,
    pub votes: Option<i64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CardField {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "address")]
    Address,
    #[serde(rename = "badges")]
    Badges,
    #[serde(rename = "checkItemStates")]
    CheckItemStates,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "coordinates")]
    Coordinates,
    #[serde(rename = "creationMethod")]
    CreationMethod,
    #[serde(rename = "dueComplete")]
    DueComplete,
    #[serde(rename = "dateLastActivity")]
    DateLastActivity,
    #[serde(rename = "desc")]
    Desc,
    #[serde(rename = "descData")]
    DescData,
    #[serde(rename = "due")]
    Due,
    #[serde(rename = "dueReminder")]
    DueReminder,
    #[serde(rename = "idBoard")]
    IdBoard,
    #[serde(rename = "idChecklists")]
    IdChecklists,
    #[serde(rename = "idLabels")]
    IdLabels,
    #[serde(rename = "idList")]
    IdList,
    #[serde(rename = "idMembers")]
    IdMembers,
    #[serde(rename = "idMembersVoted")]
    IdMembersVoted,
    #[serde(rename = "idShort")]
    IdShort,
    #[serde(rename = "idAttachmentCover")]
    IdAttachmentCover,
    #[serde(rename = "labels")]
    Labels,
    #[serde(rename = "limits")]
    Limits,
    #[serde(rename = "locationName")]
    LocationName,
    #[serde(rename = "manualCoverAttachment")]
    ManualCoverAttachment,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "pos")]
    Pos,
    #[serde(rename = "shortLink")]
    ShortLink,
    #[serde(rename = "shortUrl")]
    ShortUrl,
    #[serde(rename = "subscribed")]
    Subscribed,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "cover")]
    Cover,
    #[serde(rename = "isTemplate")]
    IsTemplate,
}

impl FieldQuery for CardField {
    fn as_field_str(&self) -> &'static str {
        match self {
            Self::Id => "id",
            Self::Address => "address",
            Self::Badges => "badges",
            Self::CheckItemStates => "checkItemStates",
            Self::Closed => "closed",
            Self::Coordinates => "coordinates",
            Self::CreationMethod => "creationMethod",
            Self::DueComplete => "dueComplete",
            Self::DateLastActivity => "dateLastActivity",
            Self::Desc => "desc",
            Self::DescData => "descData",
            Self::Due => "due",
            Self::DueReminder => "dueReminder",
            Self::IdBoard => "idBoard",
            Self::IdChecklists => "idChecklists",
            Self::IdLabels => "idLabels",
            Self::IdList => "idList",
            Self::IdMembers => "idMembers",
            Self::IdMembersVoted => "idMembersVoted",
            Self::IdShort => "idShort",
            Self::IdAttachmentCover => "idAttachmentCover",
            Self::Labels => "labels",
            Self::Limits => "limits",
            Self::LocationName => "locationName",
            Self::ManualCoverAttachment => "manualCoverAttachment",
            Self::Name => "name",
            Self::Pos => "pos",
            Self::ShortLink => "shortLink",
            Self::ShortUrl => "shortUrl",
            Self::Subscribed => "subscribed",
            Self::Url => "url",
            Self::Cover => "cover",
            Self::IsTemplate => "isTemplate",
        }
    }
}
