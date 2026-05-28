use crate::client::TrelloClient;
use crate::error::Result;
use crate::models::action::Action;
use crate::models::board::Board;
use crate::models::board_background::{BoardBackground, CustomBoardBackground};
use crate::models::board_star::BoardStar;
use crate::models::card::Card;
use crate::models::common::TrelloID;
use crate::models::emoji::{CustomEmoji, CustomSticker};
use crate::models::member::{Member, MemberField};
use crate::models::notification::{Notification, NotificationChannelSettings};
use crate::models::organization::Organization;
use crate::models::saved_search::SavedSearch;
use crate::models::token::Token;
use crate::params::FieldQuery;

pub struct MembersResource<'a> {
    client: &'a TrelloClient,
}

impl<'a> MembersResource<'a> {
    pub(crate) fn new(client: &'a TrelloClient) -> Self {
        Self { client }
    }

    // ── GET /members/{id} ──────────────────────────────────────────

    pub fn get(&self, id: impl Into<TrelloID>) -> GetMemberRequest<'a> {
        GetMemberRequest {
            client: self.client,
            id: id.into(),
            actions: None,
            boards: None,
            board_fields: None,
            boards_invited: None,
            cards: None,
            notifications: None,
            organizations: None,
            organization_fields: None,
            tokens: None,
        }
    }

    // ── PUT /members/{id} ──────────────────────────────────────────

    pub fn update(&self, id: impl Into<TrelloID>) -> UpdateMemberRequest<'a> {
        UpdateMemberRequest {
            client: self.client,
            id: id.into(),
            full_name: None,
            initials: None,
            username: None,
            bio: None,
            avatar_source: None,
            prefs_color_blind: None,
            prefs_locale: None,
            prefs_minutes_between_summaries: None,
        }
    }

    // ── GET /members/{id}/{field} ───────────────────────────────────

    pub async fn get_field(&self, id: impl Into<TrelloID>, field: MemberField) -> Result<serde_json::Value> {
        let id: TrelloID = id.into();
        self.client
            .get(&format!("/members/{}/{}", id.as_ref(), field.as_field_str()), &[])
            .await
    }

    // ── GET /members/{id}/boards ────────────────────────────────────

    pub fn get_boards(&self, id: impl Into<TrelloID>) -> GetMemberBoardsRequest<'a> {
        GetMemberBoardsRequest {
            client: self.client,
            id: id.into(),
            filter: None,
            fields: None,
        }
    }

    // ── GET /members/{id}/cards ─────────────────────────────────────

    pub fn get_cards(&self, id: impl Into<TrelloID>) -> GetMemberCardsRequest<'a> {
        GetMemberCardsRequest {
            client: self.client,
            id: id.into(),
            filter: None,
        }
    }

    // ── GET /members/{id}/organizations ─────────────────────────────

    pub fn get_organizations(&self, id: impl Into<TrelloID>) -> GetMemberOrganizationsRequest<'a> {
        GetMemberOrganizationsRequest {
            client: self.client,
            id: id.into(),
            filter: None,
            fields: None,
        }
    }

    // ── GET /members/{id}/notifications ─────────────────────────────

    pub fn get_notifications(&self, id: impl Into<TrelloID>) -> GetMemberNotificationsRequest<'a> {
        GetMemberNotificationsRequest {
            client: self.client,
            id: id.into(),
            entities: None,
            display: None,
            filter: None,
            read_filter: None,
            fields: None,
            limit: None,
            page: None,
            before: None,
            since: None,
            member_creator: None,
            member_creator_fields: None,
        }
    }

    // ── GET /members/{id}/actions ───────────────────────────────────

    pub fn get_actions(&self, id: impl Into<TrelloID>) -> GetMemberActionsRequest<'a> {
        GetMemberActionsRequest {
            client: self.client,
            id: id.into(),
            filter: None,
        }
    }

    // ── GET /members/{id}/tokens ────────────────────────────────────

    pub async fn get_tokens(&self, id: impl Into<TrelloID>) -> Result<Vec<Token>> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/members/{}/tokens", id.as_ref()), &[]).await
    }

    // ── GET /members/{id}/boardsInvited ─────────────────────────────

    pub async fn get_boards_invited(&self, id: impl Into<TrelloID>) -> Result<Vec<Board>> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/members/{}/boardsInvited", id.as_ref()), &[]).await
    }

    // ── GET /members/{id}/organizationsInvited ──────────────────────

    pub async fn get_organizations_invited(&self, id: impl Into<TrelloID>) -> Result<Vec<Organization>> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/members/{}/organizationsInvited", id.as_ref()), &[]).await
    }

    // ── POST /members/{id}/avatar ───────────────────────────────────

    pub async fn upload_avatar(&self, id: impl Into<TrelloID>, file_data: Vec<u8>, filename: &str) -> Result<Member> {
        let id: TrelloID = id.into();
        let part = reqwest::multipart::Part::bytes(file_data)
            .file_name(filename.to_string())
            .mime_str("application/octet-stream")
            .map_err(|e| crate::error::Error::Other(e.to_string()))?;
        let form = reqwest::multipart::Form::new().part("file", part);
        self.client
            .upload(&format!("/members/{}/avatar", id.as_ref()), &[], form)
            .await
    }

    // ── Board backgrounds ───────────────────────────────────────────

    pub async fn get_board_backgrounds(&self, id: impl Into<TrelloID>) -> Result<Vec<BoardBackground>> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/members/{}/boardBackgrounds", id.as_ref()), &[]).await
    }

    pub async fn upload_board_background(&self, member_id: impl Into<TrelloID>, file_data: Vec<u8>, filename: &str) -> Result<BoardBackground> {
        let member_id: TrelloID = member_id.into();
        let part = reqwest::multipart::Part::bytes(file_data)
            .file_name(filename.to_string())
            .mime_str("application/octet-stream")
            .map_err(|e| crate::error::Error::Other(e.to_string()))?;
        let form = reqwest::multipart::Form::new().part("file", part);
        self.client
            .upload(&format!("/members/{}/boardBackgrounds", member_id.as_ref()), &[], form)
            .await
    }

    pub async fn delete_board_background(&self, member_id: impl Into<TrelloID>, bg_id: impl Into<TrelloID>) -> Result<()> {
        let member_id: TrelloID = member_id.into();
        let bg_id: TrelloID = bg_id.into();
        self.client
            .delete_no_body(&format!("/members/{}/boardBackgrounds/{}", member_id.as_ref(), bg_id.as_ref()), &[])
            .await
    }

    // ── Custom board backgrounds ────────────────────────────────────

    pub async fn get_custom_board_backgrounds(&self, id: impl Into<TrelloID>) -> Result<Vec<CustomBoardBackground>> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/members/{}/customBoardBackgrounds", id.as_ref()), &[]).await
    }

    pub async fn upload_custom_board_background(&self, member_id: impl Into<TrelloID>, file_data: Vec<u8>, filename: &str) -> Result<CustomBoardBackground> {
        let member_id: TrelloID = member_id.into();
        let part = reqwest::multipart::Part::bytes(file_data)
            .file_name(filename.to_string())
            .mime_str("application/octet-stream")
            .map_err(|e| crate::error::Error::Other(e.to_string()))?;
        let form = reqwest::multipart::Form::new().part("file", part);
        self.client
            .upload(&format!("/members/{}/customBoardBackgrounds", member_id.as_ref()), &[], form)
            .await
    }

    pub async fn delete_custom_board_background(&self, member_id: impl Into<TrelloID>, bg_id: impl Into<TrelloID>) -> Result<()> {
        let member_id: TrelloID = member_id.into();
        let bg_id: TrelloID = bg_id.into();
        self.client
            .delete_no_body(&format!("/members/{}/customBoardBackgrounds/{}", member_id.as_ref(), bg_id.as_ref()), &[])
            .await
    }

    // ── Board stars ─────────────────────────────────────────────────

    pub async fn get_board_stars(&self, id: impl Into<TrelloID>) -> Result<Vec<BoardStar>> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/members/{}/boardStars", id.as_ref()), &[]).await
    }

    pub fn create_board_star(&self, member_id: impl Into<TrelloID>) -> CreateBoardStarRequest<'a> {
        CreateBoardStarRequest {
            client: self.client,
            member_id: member_id.into(),
            id_board: None,
            pos: None,
        }
    }

    pub async fn delete_board_star(&self, member_id: impl Into<TrelloID>, star_id: impl Into<TrelloID>) -> Result<()> {
        let member_id: TrelloID = member_id.into();
        let star_id: TrelloID = star_id.into();
        self.client
            .delete_no_body(&format!("/members/{}/boardStars/{}", member_id.as_ref(), star_id.as_ref()), &[])
            .await
    }

    // ── Saved searches ──────────────────────────────────────────────

    pub async fn get_saved_searches(&self, id: impl Into<TrelloID>) -> Result<Vec<SavedSearch>> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/members/{}/savedSearches", id.as_ref()), &[]).await
    }

    pub fn create_saved_search(&self, member_id: impl Into<TrelloID>) -> CreateSavedSearchRequest<'a> {
        CreateSavedSearchRequest {
            client: self.client,
            member_id: member_id.into(),
            name: None,
            query: None,
            pos: None,
        }
    }

    pub async fn delete_saved_search(&self, member_id: impl Into<TrelloID>, search_id: impl Into<TrelloID>) -> Result<()> {
        let member_id: TrelloID = member_id.into();
        let search_id: TrelloID = search_id.into();
        self.client
            .delete_no_body(&format!("/members/{}/savedSearches/{}", member_id.as_ref(), search_id.as_ref()), &[])
            .await
    }

    // ── Custom emoji ────────────────────────────────────────────────

    pub async fn get_custom_emoji(&self, id: impl Into<TrelloID>) -> Result<Vec<CustomEmoji>> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/members/{}/customEmoji", id.as_ref()), &[]).await
    }

    pub async fn upload_custom_emoji(&self, member_id: impl Into<TrelloID>, file_data: Vec<u8>, filename: &str, name: &str) -> Result<CustomEmoji> {
        let member_id: TrelloID = member_id.into();
        let part = reqwest::multipart::Part::bytes(file_data)
            .file_name(filename.to_string())
            .mime_str("application/octet-stream")
            .map_err(|e| crate::error::Error::Other(e.to_string()))?;
        let form = reqwest::multipart::Form::new()
            .part("file", part)
            .text("name", name.to_string());
        self.client
            .upload(&format!("/members/{}/customEmoji", member_id.as_ref()), &[], form)
            .await
    }

    // ── Custom stickers ─────────────────────────────────────────────

    pub async fn get_custom_stickers(&self, id: impl Into<TrelloID>) -> Result<Vec<CustomSticker>> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/members/{}/customStickers", id.as_ref()), &[]).await
    }

    pub async fn upload_custom_sticker(&self, member_id: impl Into<TrelloID>, file_data: Vec<u8>, filename: &str) -> Result<CustomSticker> {
        let member_id: TrelloID = member_id.into();
        let part = reqwest::multipart::Part::bytes(file_data)
            .file_name(filename.to_string())
            .mime_str("application/octet-stream")
            .map_err(|e| crate::error::Error::Other(e.to_string()))?;
        let form = reqwest::multipart::Form::new().part("file", part);
        self.client
            .upload(&format!("/members/{}/customStickers", member_id.as_ref()), &[], form)
            .await
    }

    pub async fn delete_custom_sticker(&self, member_id: impl Into<TrelloID>, sticker_id: impl Into<TrelloID>) -> Result<()> {
        let member_id: TrelloID = member_id.into();
        let sticker_id: TrelloID = sticker_id.into();
        self.client
            .delete_no_body(&format!("/members/{}/customStickers/{}", member_id.as_ref(), sticker_id.as_ref()), &[])
            .await
    }

    // ── Notification channel settings ───────────────────────────────

    pub async fn get_notification_channel_settings(&self, id: impl Into<TrelloID>) -> Result<NotificationChannelSettings> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/members/{}/notificationsChannelSettings", id.as_ref()), &[]).await
    }

    pub async fn update_notification_channel_settings(
        &self,
        id: impl Into<TrelloID>,
        channel: &str,
        blocked_keys: Option<Vec<&str>>,
    ) -> Result<NotificationChannelSettings> {
        let id: TrelloID = id.into();
        let mut params = vec![("channel", channel)];
        let bk_s;
        if let Some(ref keys) = blocked_keys {
            bk_s = keys.join(",");
            params.push(("blockedKeys", &bk_s));
        }
        self.client
            .put(&format!("/members/{}/notificationsChannelSettings/{}", id.as_ref(), channel), &params, None::<&()>)
            .await
    }

    // ── POST /members/{id}/oneTimeMessagesDismissed ─────────────────

    pub async fn dismiss_message(&self, id: impl Into<TrelloID>, value: &str) -> Result<()> {
        let id: TrelloID = id.into();
        self.client
            .post::<serde_json::Value, ()>(&format!("/members/{}/oneTimeMessagesDismissed", id.as_ref()), &[("value", value)], None)
            .await?;
        Ok(())
    }
}

// ──────────────────── Builder structs ────────────────────────────────

pub struct GetMemberRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    actions: Option<String>,
    boards: Option<String>,
    board_fields: Option<String>,
    boards_invited: Option<String>,
    cards: Option<String>,
    notifications: Option<String>,
    organizations: Option<String>,
    organization_fields: Option<String>,
    tokens: Option<String>,
}

impl<'a> GetMemberRequest<'a> {
    pub fn actions(mut self, v: &str) -> Self { self.actions = Some(v.to_string()); self }
    pub fn boards(mut self, v: &str) -> Self { self.boards = Some(v.to_string()); self }
    pub fn cards(mut self, v: &str) -> Self { self.cards = Some(v.to_string()); self }
    pub fn notifications(mut self, v: &str) -> Self { self.notifications = Some(v.to_string()); self }
    pub fn organizations(mut self, v: &str) -> Self { self.organizations = Some(v.to_string()); self }
    pub fn tokens(mut self, v: &str) -> Self { self.tokens = Some(v.to_string()); self }

    pub async fn send(self) -> Result<Member> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let actions_s; let boards_s; let bf_s; let bi_s; let cards_s;
        let notifs_s; let orgs_s; let of_s; let tokens_s;
        if let Some(ref v) = self.actions { actions_s = v.clone(); params.push(("actions", &actions_s)); }
        if let Some(ref v) = self.boards { boards_s = v.clone(); params.push(("boards", &boards_s)); }
        if let Some(ref v) = self.board_fields { bf_s = v.clone(); params.push(("board_fields", &bf_s)); }
        if let Some(ref v) = self.boards_invited { bi_s = v.clone(); params.push(("boardsInvited", &bi_s)); }
        if let Some(ref v) = self.cards { cards_s = v.clone(); params.push(("cards", &cards_s)); }
        if let Some(ref v) = self.notifications { notifs_s = v.clone(); params.push(("notifications", &notifs_s)); }
        if let Some(ref v) = self.organizations { orgs_s = v.clone(); params.push(("organizations", &orgs_s)); }
        if let Some(ref v) = self.organization_fields { of_s = v.clone(); params.push(("organization_fields", &of_s)); }
        if let Some(ref v) = self.tokens { tokens_s = v.clone(); params.push(("tokens", &tokens_s)); }
        self.client.get(&format!("/members/{}", self.id.as_ref()), &params).await
    }
}

pub struct UpdateMemberRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    full_name: Option<String>,
    initials: Option<String>,
    username: Option<String>,
    bio: Option<String>,
    avatar_source: Option<String>,
    prefs_color_blind: Option<bool>,
    prefs_locale: Option<String>,
    prefs_minutes_between_summaries: Option<i32>,
}

impl<'a> UpdateMemberRequest<'a> {
    pub fn full_name(mut self, v: impl Into<String>) -> Self { self.full_name = Some(v.into()); self }
    pub fn initials(mut self, v: impl Into<String>) -> Self { self.initials = Some(v.into()); self }
    pub fn username(mut self, v: impl Into<String>) -> Self { self.username = Some(v.into()); self }
    pub fn bio(mut self, v: impl Into<String>) -> Self { self.bio = Some(v.into()); self }
    pub fn avatar_source(mut self, v: &str) -> Self { self.avatar_source = Some(v.to_string()); self }
    pub fn prefs_color_blind(mut self, v: bool) -> Self { self.prefs_color_blind = Some(v); self }
    pub fn prefs_locale(mut self, v: &str) -> Self { self.prefs_locale = Some(v.to_string()); self }
    pub fn prefs_minutes_between_summaries(mut self, v: i32) -> Self { self.prefs_minutes_between_summaries = Some(v); self }

    pub async fn send(self) -> Result<Member> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let fn_s; let init_s; let uname_s; let bio_s; let as_s; let pcb_s; let pl_s; let pmbs_s;
        if let Some(ref v) = self.full_name { fn_s = v.clone(); params.push(("fullName", &fn_s)); }
        if let Some(ref v) = self.initials { init_s = v.clone(); params.push(("initials", &init_s)); }
        if let Some(ref v) = self.username { uname_s = v.clone(); params.push(("username", &uname_s)); }
        if let Some(ref v) = self.bio { bio_s = v.clone(); params.push(("bio", &bio_s)); }
        if let Some(ref v) = self.avatar_source { as_s = v.clone(); params.push(("avatarSource", &as_s)); }
        if let Some(v) = self.prefs_color_blind { pcb_s = if v { "true" } else { "false" }; params.push(("prefs/colorBlind", pcb_s)); }
        if let Some(ref v) = self.prefs_locale { pl_s = v.clone(); params.push(("prefs/locale", &pl_s)); }
        if let Some(v) = self.prefs_minutes_between_summaries { pmbs_s = v.to_string(); params.push(("prefs/minutesBetweenSummaries", &pmbs_s)); }
        self.client
            .put(&format!("/members/{}", self.id.as_ref()), &params, None::<&()>)
            .await
    }
}

pub struct GetMemberBoardsRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    filter: Option<String>,
    fields: Option<String>,
}

impl<'a> GetMemberBoardsRequest<'a> {
    pub fn filter(mut self, v: &str) -> Self { self.filter = Some(v.to_string()); self }
    pub fn fields(mut self, v: &str) -> Self { self.fields = Some(v.to_string()); self }
    pub async fn send(self) -> Result<Vec<Board>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let filter_s; let fields_s;
        if let Some(ref v) = self.filter { filter_s = v.clone(); params.push(("filter", &filter_s)); }
        if let Some(ref v) = self.fields { fields_s = v.clone(); params.push(("fields", &fields_s)); }
        self.client.get(&format!("/members/{}/boards", self.id.as_ref()), &params).await
    }
}

pub struct GetMemberCardsRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    filter: Option<String>,
}

impl<'a> GetMemberCardsRequest<'a> {
    pub fn filter(mut self, v: &str) -> Self { self.filter = Some(v.to_string()); self }
    pub async fn send(self) -> Result<Vec<Card>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let filter_s;
        if let Some(ref v) = self.filter { filter_s = v.clone(); params.push(("filter", &filter_s)); }
        self.client.get(&format!("/members/{}/cards", self.id.as_ref()), &params).await
    }
}

pub struct GetMemberOrganizationsRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    filter: Option<String>,
    fields: Option<String>,
}

impl<'a> GetMemberOrganizationsRequest<'a> {
    pub fn filter(mut self, v: &str) -> Self { self.filter = Some(v.to_string()); self }
    pub fn fields(mut self, v: &str) -> Self { self.fields = Some(v.to_string()); self }
    pub async fn send(self) -> Result<Vec<Organization>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let filter_s; let fields_s;
        if let Some(ref v) = self.filter { filter_s = v.clone(); params.push(("filter", &filter_s)); }
        if let Some(ref v) = self.fields { fields_s = v.clone(); params.push(("fields", &fields_s)); }
        self.client.get(&format!("/members/{}/organizations", self.id.as_ref()), &params).await
    }
}

pub struct GetMemberNotificationsRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    entities: Option<bool>,
    display: Option<bool>,
    filter: Option<String>,
    read_filter: Option<String>,
    fields: Option<String>,
    limit: Option<u32>,
    page: Option<u32>,
    before: Option<String>,
    since: Option<String>,
    member_creator: Option<bool>,
    member_creator_fields: Option<String>,
}

impl<'a> GetMemberNotificationsRequest<'a> {
    pub fn filter(mut self, v: &str) -> Self { self.filter = Some(v.to_string()); self }
    pub fn limit(mut self, v: u32) -> Self { self.limit = Some(v); self }
    pub fn page(mut self, v: u32) -> Self { self.page = Some(v); self }
    pub fn before(mut self, v: &str) -> Self { self.before = Some(v.to_string()); self }
    pub fn since(mut self, v: &str) -> Self { self.since = Some(v.to_string()); self }

    pub async fn send(self) -> Result<Vec<Notification>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let filter_s; let rf_s; let fields_s; let limit_s;
        let page_s; let before_s; let since_s; let mcf_s;
        if let Some(v) = self.entities { params.push(("entities", if v { "true" } else { "false" })); }
        if let Some(v) = self.display { params.push(("display", if v { "true" } else { "false" })); }
        if let Some(ref v) = self.filter { filter_s = v.clone(); params.push(("filter", &filter_s)); }
        if let Some(ref v) = self.read_filter { rf_s = v.clone(); params.push(("read_filter", &rf_s)); }
        if let Some(ref v) = self.fields { fields_s = v.clone(); params.push(("fields", &fields_s)); }
        if let Some(v) = self.limit { limit_s = v.to_string(); params.push(("limit", &limit_s)); }
        if let Some(v) = self.page { page_s = v.to_string(); params.push(("page", &page_s)); }
        if let Some(ref v) = self.before { before_s = v.clone(); params.push(("before", &before_s)); }
        if let Some(ref v) = self.since { since_s = v.clone(); params.push(("since", &since_s)); }
        if let Some(v) = self.member_creator { params.push(("memberCreator", if v { "true" } else { "false" })); }
        if let Some(ref v) = self.member_creator_fields { mcf_s = v.clone(); params.push(("memberCreator_fields", &mcf_s)); }
        self.client.get(&format!("/members/{}/notifications", self.id.as_ref()), &params).await
    }
}

pub struct GetMemberActionsRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    filter: Option<String>,
}

impl<'a> GetMemberActionsRequest<'a> {
    pub fn filter(mut self, v: &str) -> Self { self.filter = Some(v.to_string()); self }
    pub async fn send(self) -> Result<Vec<Action>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let filter_s;
        if let Some(ref v) = self.filter { filter_s = v.clone(); params.push(("filter", &filter_s)); }
        self.client.get(&format!("/members/{}/actions", self.id.as_ref()), &params).await
    }
}

pub struct CreateBoardStarRequest<'a> {
    client: &'a TrelloClient,
    member_id: TrelloID,
    id_board: Option<TrelloID>,
    pos: Option<String>,
}

impl<'a> CreateBoardStarRequest<'a> {
    pub fn id_board(mut self, v: impl Into<TrelloID>) -> Self { self.id_board = Some(v.into()); self }
    pub fn pos(mut self, v: &str) -> Self { self.pos = Some(v.to_string()); self }

    pub async fn send(self) -> Result<BoardStar> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let ib_s; let pos_s;
        if let Some(ref v) = self.id_board { ib_s = v.to_string(); params.push(("idBoard", &ib_s)); }
        if let Some(ref v) = self.pos { pos_s = v.clone(); params.push(("pos", &pos_s)); }
        self.client
            .post(&format!("/members/{}/boardStars", self.member_id.as_ref()), &params, None::<&()>)
            .await
    }
}

pub struct CreateSavedSearchRequest<'a> {
    client: &'a TrelloClient,
    member_id: TrelloID,
    name: Option<String>,
    query: Option<String>,
    pos: Option<String>,
}

impl<'a> CreateSavedSearchRequest<'a> {
    pub fn name(mut self, v: impl Into<String>) -> Self { self.name = Some(v.into()); self }
    pub fn query(mut self, v: impl Into<String>) -> Self { self.query = Some(v.into()); self }
    pub fn pos(mut self, v: &str) -> Self { self.pos = Some(v.to_string()); self }

    pub async fn send(self) -> Result<SavedSearch> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let name_s; let query_s; let pos_s;
        if let Some(ref v) = self.name { name_s = v.clone(); params.push(("name", &name_s)); }
        if let Some(ref v) = self.query { query_s = v.clone(); params.push(("query", &query_s)); }
        if let Some(ref v) = self.pos { pos_s = v.clone(); params.push(("pos", &pos_s)); }
        self.client
            .post(&format!("/members/{}/savedSearches", self.member_id.as_ref()), &params, None::<&()>)
            .await
    }
}
