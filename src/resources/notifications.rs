use crate::client::TrelloClient;
use crate::error::Result;
use crate::models::board::Board;
use crate::models::card::Card;
use crate::models::common::TrelloID;
use crate::models::list::TrelloList;
use crate::models::member::Member;
use crate::models::notification::Notification;
use crate::models::organization::Organization;

pub struct NotificationsResource<'a> {
    client: &'a TrelloClient,
}

impl<'a> NotificationsResource<'a> {
    pub(crate) fn new(client: &'a TrelloClient) -> Self {
        Self { client }
    }

    // ── GET /notifications/{id} ─────────────────────────────────────

    pub fn get(&self, id: impl Into<TrelloID>) -> GetNotificationRequest<'a> {
        GetNotificationRequest {
            client: self.client,
            id: id.into(),
            display: None,
            entities: None,
            fields: None,
            member_creator: None,
            member_creator_fields: None,
        }
    }

    // ── PUT /notifications/{id} ─────────────────────────────────────

    pub async fn update(&self, id: impl Into<TrelloID>, unread: Option<bool>) -> Result<Notification> {
        let id: TrelloID = id.into();
        let mut params: Vec<(&str, &str)> = Vec::new();
        if let Some(v) = unread {
            params.push(("unread", if v { "true" } else { "false" }));
        }
        self.client
            .put(&format!("/notifications/{}", id.as_ref()), &params, None::<&()>)
            .await
    }

    // ── GET /notifications/{id}/{field} ─────────────────────────────

    pub async fn get_field(&self, id: impl Into<TrelloID>, field: &str) -> Result<serde_json::Value> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/notifications/{}/{}", id.as_ref(), field), &[]).await
    }

    // ── POST /notifications/all/read ────────────────────────────────

    pub async fn mark_all_read(&self) -> Result<()> {
        self.client
            .post::<serde_json::Value, ()>("/notifications/all/read", &[], None)
            .await?;
        Ok(())
    }

    // ── PUT /notifications/{id}/unread ──────────────────────────────

    pub async fn mark_unread(&self, id: impl Into<TrelloID>) -> Result<Notification> {
        let id: TrelloID = id.into();
        self.client
            .put(&format!("/notifications/{}/unread", id.as_ref()), &[("value", "true")], None::<&()>)
            .await
    }

    // ── GET /notifications/{id}/board ───────────────────────────────

    pub fn get_board(&self, id: impl Into<TrelloID>) -> GetNotificationBoardRequest<'a> {
        GetNotificationBoardRequest {
            client: self.client,
            id: id.into(),
            fields: None,
        }
    }

    // ── GET /notifications/{id}/card ────────────────────────────────

    pub fn get_card(&self, id: impl Into<TrelloID>) -> GetNotificationCardRequest<'a> {
        GetNotificationCardRequest {
            client: self.client,
            id: id.into(),
            fields: None,
        }
    }

    // ── GET /notifications/{id}/list ────────────────────────────────

    pub fn get_list(&self, id: impl Into<TrelloID>) -> GetNotificationListRequest<'a> {
        GetNotificationListRequest {
            client: self.client,
            id: id.into(),
            fields: None,
        }
    }

    // ── GET /notifications/{id}/member ──────────────────────────────

    pub fn get_member(&self, id: impl Into<TrelloID>) -> GetNotificationMemberRequest<'a> {
        GetNotificationMemberRequest {
            client: self.client,
            id: id.into(),
            fields: None,
        }
    }

    // ── GET /notifications/{id}/memberCreator ───────────────────────

    pub fn get_member_creator(&self, id: impl Into<TrelloID>) -> GetNotificationMemberCreatorRequest<'a> {
        GetNotificationMemberCreatorRequest {
            client: self.client,
            id: id.into(),
            fields: None,
        }
    }

    // ── GET /notifications/{id}/organization ────────────────────────

    pub fn get_organization(&self, id: impl Into<TrelloID>) -> GetNotificationOrganizationRequest<'a> {
        GetNotificationOrganizationRequest {
            client: self.client,
            id: id.into(),
            fields: None,
        }
    }
}

// ──────────────────── Builder structs ────────────────────────────────

pub struct GetNotificationRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    display: Option<bool>,
    entities: Option<bool>,
    fields: Option<String>,
    member_creator: Option<bool>,
    member_creator_fields: Option<String>,
}

impl<'a> GetNotificationRequest<'a> {
    pub fn display(mut self, v: bool) -> Self { self.display = Some(v); self }
    pub fn entities(mut self, v: bool) -> Self { self.entities = Some(v); self }
    pub fn fields(mut self, v: &str) -> Self { self.fields = Some(v.to_string()); self }
    pub fn member_creator(mut self, v: bool) -> Self { self.member_creator = Some(v); self }

    pub async fn send(self) -> Result<Notification> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let fields_s; let mcf_s;
        if let Some(v) = self.display { params.push(("display", if v { "true" } else { "false" })); }
        if let Some(v) = self.entities { params.push(("entities", if v { "true" } else { "false" })); }
        if let Some(ref v) = self.fields { fields_s = v.clone(); params.push(("fields", &fields_s)); }
        if let Some(v) = self.member_creator { params.push(("memberCreator", if v { "true" } else { "false" })); }
        if let Some(ref v) = self.member_creator_fields { mcf_s = v.clone(); params.push(("memberCreator_fields", &mcf_s)); }
        self.client.get(&format!("/notifications/{}", self.id.as_ref()), &params).await
    }
}

macro_rules! notification_getter {
    ($name:ident, $resource:ty, $path:literal) => {
        pub struct $name<'a> {
            client: &'a TrelloClient,
            id: TrelloID,
            fields: Option<String>,
        }

        impl<'a> $name<'a> {
            pub fn fields(mut self, v: &str) -> Self { self.fields = Some(v.to_string()); self }
            pub async fn send(self) -> Result<$resource> {
                let mut params: Vec<(&str, &str)> = Vec::new();
                let fields_s;
                if let Some(ref v) = self.fields { fields_s = v.clone(); params.push(("fields", &fields_s)); }
                self.client.get(&format!($path, self.id.as_ref()), &params).await
            }
        }
    };
}

notification_getter!(GetNotificationBoardRequest, Board, "/notifications/{}/board");
notification_getter!(GetNotificationCardRequest, Card, "/notifications/{}/card");
notification_getter!(GetNotificationListRequest, TrelloList, "/notifications/{}/list");
notification_getter!(GetNotificationMemberRequest, Member, "/notifications/{}/member");
notification_getter!(GetNotificationMemberCreatorRequest, Member, "/notifications/{}/memberCreator");
notification_getter!(GetNotificationOrganizationRequest, Organization, "/notifications/{}/organization");
