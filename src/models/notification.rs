use serde::{Deserialize, Serialize};

use super::common::TrelloID;
use crate::params::FieldQuery;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: Option<String>,
    pub unread: Option<bool>,
    #[serde(rename = "type")]
    pub notification_type: Option<String>,
    pub date: Option<String>,
    pub date_read: Option<String>,
    pub data: Option<String>,
    pub card: Option<super::card::Card>,
    pub board: Option<super::board::Board>,
    pub id_member_creator: Option<TrelloID>,
    pub id_action: Option<TrelloID>,
    pub reactions: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationChannelSettings {
    pub id: Option<TrelloID>,
    pub id_member: Option<TrelloID>,
    pub blocked_keys: Option<Vec<BlockedKey>>,
    pub channel: Option<Channel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockedKey {
    #[serde(rename = "notification_comment_card")]
    NotificationCommentCard,
    #[serde(rename = "notification_added_a_due_date")]
    NotificationAddedADueDate,
    #[serde(rename = "notification_changed_due_date")]
    NotificationChangedDueDate,
    #[serde(rename = "notification_card_due_soon")]
    NotificationCardDueSoon,
    #[serde(rename = "notification_removed_from_card")]
    NotificationRemovedFromCard,
    #[serde(rename = "notification_added_attachment_to_card")]
    NotificationAddedAttachmentToCard,
    #[serde(rename = "notification_created_card")]
    NotificationCreatedCard,
    #[serde(rename = "notification_moved_card")]
    NotificationMovedCard,
    #[serde(rename = "notification_archived_card")]
    NotificationArchivedCard,
    #[serde(rename = "notification_unarchived_card")]
    NotificationUnarchivedCard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Channel {
    #[serde(rename = "email")]
    Email,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NotificationField {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "unread")]
    Unread,
    #[serde(rename = "type")]
    NotificationType,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "dateRead")]
    DateRead,
    #[serde(rename = "data")]
    Data,
    #[serde(rename = "card")]
    Card,
    #[serde(rename = "board")]
    Board,
    #[serde(rename = "idMemberCreator")]
    IdMemberCreator,
    #[serde(rename = "idAction")]
    IdAction,
    #[serde(rename = "reactions")]
    Reactions,
}

impl FieldQuery for NotificationField {
    fn as_field_str(&self) -> &'static str {
        match self {
            Self::Id => "id",
            Self::Unread => "unread",
            Self::NotificationType => "type",
            Self::Date => "date",
            Self::DateRead => "dateRead",
            Self::Data => "data",
            Self::Card => "card",
            Self::Board => "board",
            Self::IdMemberCreator => "idMemberCreator",
            Self::IdAction => "idAction",
            Self::Reactions => "reactions",
        }
    }
}
