use crate::client::TrelloClient;
use crate::error::Result;
use crate::models::action::Action;
use crate::models::attachment::Attachment;
use crate::models::board::Board;
use crate::models::card::{Card, CardField};
use crate::models::checklist::Checklist;
use crate::models::common::TrelloID;
use crate::models::custom_field::CustomFieldItems;
use crate::models::list::TrelloList;
use crate::models::member::Member;
use crate::models::plugin::PluginData;
use crate::params::{fields_to_query, FieldQuery};

pub struct CardsResource<'a> {
    client: &'a TrelloClient,
}

impl<'a> CardsResource<'a> {
    pub(crate) fn new(client: &'a TrelloClient) -> Self {
        Self { client }
    }

    // ── GET /cards/{id} ────────────────────────────────────────────

    pub fn get(&self, id: impl Into<TrelloID>) -> GetCardRequest<'a> {
        GetCardRequest {
            client: self.client,
            id: id.into(),
            fields: None,
            actions: None,
            attachments: None,
            attachment_fields: None,
            members: None,
            member_fields: None,
            members_voted: None,
            member_voted_fields: None,
            check_item_states: None,
            checklists: None,
            checklist_fields: None,
            board: None,
            board_fields: None,
            list: None,
            plugin_data: None,
            stickers: None,
            sticker_fields: None,
            custom_field_items: None,
        }
    }

    // ── POST /cards ─────────────────────────────────────────────────

    pub fn create(&self) -> CreateCardRequest<'a> {
        CreateCardRequest::new(self.client)
    }

    // ── PUT /cards/{id} ────────────────────────────────────────────

    pub fn update(&self, id: impl Into<TrelloID>) -> UpdateCardRequest<'a> {
        UpdateCardRequest::new(self.client, id.into())
    }

    // ── DELETE /cards/{id} ─────────────────────────────────────────

    pub async fn delete(&self, id: impl Into<TrelloID>) -> Result<()> {
        let id: TrelloID = id.into();
        self.client
            .delete_no_body(&format!("/cards/{}", id.as_ref()), &[])
            .await
    }

    // ── GET /cards/{id}/{field} ─────────────────────────────────────

    pub async fn get_field(&self, id: impl Into<TrelloID>, field: CardField) -> Result<serde_json::Value> {
        let id: TrelloID = id.into();
        self.client
            .get(&format!("/cards/{}/{}", id.as_ref(), field.as_field_str()), &[])
            .await
    }

    // ── GET /cards/{id}/board ───────────────────────────────────────

    pub fn get_board(&self, id: impl Into<TrelloID>) -> GetCardBoardRequest<'a> {
        GetCardBoardRequest {
            client: self.client,
            id: id.into(),
            fields: None,
        }
    }

    // ── GET /cards/{id}/list ────────────────────────────────────────

    pub fn get_list(&self, id: impl Into<TrelloID>) -> GetCardListRequest<'a> {
        GetCardListRequest {
            client: self.client,
            id: id.into(),
            fields: None,
        }
    }

    // ── GET /cards/{id}/actions ─────────────────────────────────────

    pub fn get_actions(&self, id: impl Into<TrelloID>) -> GetCardActionsRequest<'a> {
        GetCardActionsRequest {
            client: self.client,
            id: id.into(),
            filter: None,
            page: None,
        }
    }

    // ── POST /cards/{id}/actions/comments ───────────────────────────

    pub fn add_comment(&self, id: impl Into<TrelloID>, text: &str) -> AddCommentRequest<'a> {
        AddCommentRequest {
            client: self.client,
            id: id.into(),
            text: text.to_string(),
        }
    }

    // ── PUT /cards/{id}/actions/{idAction}/comments ─────────────────

    pub fn update_comment(&self, card_id: impl Into<TrelloID>, action_id: impl Into<TrelloID>) -> UpdateCommentRequest<'a> {
        UpdateCommentRequest {
            client: self.client,
            card_id: card_id.into(),
            action_id: action_id.into(),
            text: None,
        }
    }

    // ── DELETE /cards/{id}/actions/{idAction}/comments ──────────────

    pub async fn delete_comment(&self, card_id: impl Into<TrelloID>, action_id: impl Into<TrelloID>) -> Result<()> {
        let card_id: TrelloID = card_id.into();
        let action_id: TrelloID = action_id.into();
        self.client
            .delete_no_body(&format!("/cards/{}/actions/{}/comments", card_id.as_ref(), action_id.as_ref()), &[])
            .await
    }

    // ── GET /cards/{id}/attachments ─────────────────────────────────

    pub fn get_attachments(&self, id: impl Into<TrelloID>) -> GetCardAttachmentsRequest<'a> {
        GetCardAttachmentsRequest {
            client: self.client,
            id: id.into(),
            fields: None,
            filter: None,
        }
    }

    // ── POST /cards/{id}/attachments ────────────────────────────────

    pub fn create_attachment(&self, id: impl Into<TrelloID>) -> CreateAttachmentRequest<'a> {
        CreateAttachmentRequest {
            client: self.client,
            id: id.into(),
            name: None,
            url: None,
            mime_type: None,
        }
    }

    // ── DELETE /cards/{id}/attachments/{idAttachment} ───────────────

    pub async fn delete_attachment(&self, card_id: impl Into<TrelloID>, attachment_id: impl Into<TrelloID>) -> Result<()> {
        let card_id: TrelloID = card_id.into();
        let attachment_id: TrelloID = attachment_id.into();
        self.client
            .delete_no_body(&format!("/cards/{}/attachments/{}", card_id.as_ref(), attachment_id.as_ref()), &[])
            .await
    }

    // ── GET /cards/{id}/checklists ──────────────────────────────────

    pub fn get_checklists(&self, id: impl Into<TrelloID>) -> GetCardChecklistsRequest<'a> {
        GetCardChecklistsRequest {
            client: self.client,
            id: id.into(),
            check_items: None,
            check_item_fields: None,
            filter: None,
            fields: None,
        }
    }

    // ── POST /cards/{id}/checklists ─────────────────────────────────

    pub fn create_checklist(&self, id: impl Into<TrelloID>, name: &str) -> CreateCardChecklistRequest<'a> {
        CreateCardChecklistRequest {
            client: self.client,
            id: id.into(),
            name: name.to_string(),
            id_checklist_source: None,
        }
    }

    // ── DELETE /cards/{id}/checklists/{idChecklist} ─────────────────

    pub async fn delete_checklist(&self, card_id: impl Into<TrelloID>, checklist_id: impl Into<TrelloID>) -> Result<()> {
        let card_id: TrelloID = card_id.into();
        let checklist_id: TrelloID = checklist_id.into();
        self.client
            .delete_no_body(&format!("/cards/{}/checklists/{}", card_id.as_ref(), checklist_id.as_ref()), &[])
            .await
    }

    // ── GET /cards/{id}/members ─────────────────────────────────────

    pub async fn get_members(&self, id: impl Into<TrelloID>) -> Result<Vec<Member>> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/cards/{}/members", id.as_ref()), &[]).await
    }

    // ── POST /cards/{id}/idMembers ──────────────────────────────────

    pub fn add_member(&self, id: impl Into<TrelloID>, member_id: impl Into<TrelloID>) -> AddMemberToCardRequest<'a> {
        AddMemberToCardRequest {
            client: self.client,
            id: id.into(),
            value: member_id.into(),
        }
    }

    // ── DELETE /cards/{id}/idMembers/{idMember} ─────────────────────

    pub async fn remove_member(&self, card_id: impl Into<TrelloID>, member_id: impl Into<TrelloID>) -> Result<()> {
        let card_id: TrelloID = card_id.into();
        let member_id: TrelloID = member_id.into();
        self.client
            .delete_no_body(&format!("/cards/{}/idMembers/{}", card_id.as_ref(), member_id.as_ref()), &[])
            .await
    }

    // ── POST /cards/{id}/idLabels ───────────────────────────────────

    pub fn add_label(&self, id: impl Into<TrelloID>, label_id: impl Into<TrelloID>) -> AddLabelToCardRequest<'a> {
        AddLabelToCardRequest {
            client: self.client,
            id: id.into(),
            value: label_id.into(),
        }
    }

    // ── DELETE /cards/{id}/idLabels/{idLabel} ───────────────────────

    pub async fn remove_label(&self, card_id: impl Into<TrelloID>, label_id: impl Into<TrelloID>) -> Result<()> {
        let card_id: TrelloID = card_id.into();
        let label_id: TrelloID = label_id.into();
        self.client
            .delete_no_body(&format!("/cards/{}/idLabels/{}", card_id.as_ref(), label_id.as_ref()), &[])
            .await
    }

    // ── GET /cards/{id}/customFieldItems ────────────────────────────

    pub async fn get_custom_field_items(&self, id: impl Into<TrelloID>) -> Result<Vec<CustomFieldItems>> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/cards/{}/customFieldItems", id.as_ref()), &[]).await
    }

    // ── PUT /cards/{idCard}/customField/{idCustomField}/item ────────

    pub async fn set_custom_field(
        &self,
        card_id: impl Into<TrelloID>,
        custom_field_id: impl Into<TrelloID>,
        value: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let card_id: TrelloID = card_id.into();
        let custom_field_id: TrelloID = custom_field_id.into();
        let body = CustomFieldValueBody { value };
        self.client
            .put(
                &format!("/cards/{}/customField/{}/item", card_id.as_ref(), custom_field_id.as_ref()),
                &[],
                Some(&body),
            )
            .await
    }

    // ── GET /cards/{id}/checkItemStates ─────────────────────────────

    pub async fn get_check_item_states(&self, id: impl Into<TrelloID>) -> Result<Vec<serde_json::Value>> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/cards/{}/checkItemStates", id.as_ref()), &[]).await
    }

    // ── GET /cards/{id}/pluginData ──────────────────────────────────

    pub async fn get_plugin_data(&self, id: impl Into<TrelloID>) -> Result<Vec<PluginData>> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/cards/{}/pluginData", id.as_ref()), &[]).await
    }

    // ── POST /cards/{id}/markAssociatedNotificationsRead ────────────

    pub async fn mark_notifications_read(&self, id: impl Into<TrelloID>) -> Result<()> {
        let id: TrelloID = id.into();
        self.client
            .post::<serde_json::Value, ()>(&format!("/cards/{}/markAssociatedNotificationsRead", id.as_ref()), &[], None)
            .await?;
        Ok(())
    }

    // ── MembersVoted ────────────────────────────────────────────────

    pub async fn get_members_voted(&self, id: impl Into<TrelloID>) -> Result<Vec<Member>> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/cards/{}/membersVoted", id.as_ref()), &[]).await
    }

    pub async fn vote(&self, id: impl Into<TrelloID>, member_id: impl Into<TrelloID>) -> Result<()> {
        let id: TrelloID = id.into();
        let member_id: TrelloID = member_id.into();
        self.client
            .post::<serde_json::Value, ()>(
                &format!("/cards/{}/membersVoted", id.as_ref()),
                &[("value", member_id.as_ref())],
                None,
            )
            .await?;
        Ok(())
    }

    pub async fn unvote(&self, id: impl Into<TrelloID>, member_id: impl Into<TrelloID>) -> Result<()> {
        let id: TrelloID = id.into();
        let member_id: TrelloID = member_id.into();
        self.client
            .delete_no_body(&format!("/cards/{}/membersVoted/{}", id.as_ref(), member_id.as_ref()), &[])
            .await
    }

    // ── Stickers ────────────────────────────────────────────────────

    pub async fn get_stickers(&self, id: impl Into<TrelloID>) -> Result<Vec<serde_json::Value>> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/cards/{}/stickers", id.as_ref()), &[]).await
    }

    pub fn create_sticker(&self, card_id: impl Into<TrelloID>) -> CreateStickerRequest<'a> {
        CreateStickerRequest {
            client: self.client,
            card_id: card_id.into(),
            image: None,
            top: None,
            left: None,
            z_index: None,
            rotate: None,
        }
    }

    pub async fn delete_sticker(&self, card_id: impl Into<TrelloID>, sticker_id: impl Into<TrelloID>) -> Result<()> {
        let card_id: TrelloID = card_id.into();
        let sticker_id: TrelloID = sticker_id.into();
        self.client
            .delete_no_body(&format!("/cards/{}/stickers/{}", card_id.as_ref(), sticker_id.as_ref()), &[])
            .await
    }
}

// ──────────────────── Builder structs ────────────────────────────────

pub struct GetCardRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    fields: Option<Vec<CardField>>,
    actions: Option<String>,
    attachments: Option<String>,
    attachment_fields: Option<String>,
    members: Option<bool>,
    member_fields: Option<String>,
    members_voted: Option<bool>,
    member_voted_fields: Option<String>,
    check_item_states: Option<bool>,
    checklists: Option<String>,
    checklist_fields: Option<String>,
    board: Option<bool>,
    board_fields: Option<String>,
    list: Option<bool>,
    plugin_data: Option<bool>,
    stickers: Option<bool>,
    sticker_fields: Option<String>,
    custom_field_items: Option<bool>,
}

impl<'a> GetCardRequest<'a> {
    pub fn fields(mut self, f: &[CardField]) -> Self { self.fields = Some(f.to_vec()); self }
    pub fn actions(mut self, v: &str) -> Self { self.actions = Some(v.to_string()); self }
    pub fn attachments(mut self, v: bool) -> Self { self.attachments = Some(v.to_string()); self }
    pub fn members(mut self, v: bool) -> Self { self.members = Some(v); self }
    pub fn member_fields(mut self, v: &str) -> Self { self.member_fields = Some(v.to_string()); self }
    pub fn members_voted(mut self, v: bool) -> Self { self.members_voted = Some(v); self }
    pub fn member_voted_fields(mut self, v: &str) -> Self { self.member_voted_fields = Some(v.to_string()); self }
    pub fn checklists(mut self, v: &str) -> Self { self.checklists = Some(v.to_string()); self }
    pub fn board(mut self, v: bool) -> Self { self.board = Some(v); self }
    pub fn list(mut self, v: bool) -> Self { self.list = Some(v); self }

    pub async fn send(self) -> Result<Card> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let fields_str;
        if let Some(ref f) = self.fields { fields_str = fields_to_query(f); params.push(("fields", &fields_str)); }
        let actions_s; let att_s; let af_s; let chk_s; let chf_s; let bf_s; let sf_s;
        if let Some(ref v) = self.actions { actions_s = v.clone(); params.push(("actions", &actions_s)); }
        if let Some(ref v) = self.attachments { att_s = v.clone(); params.push(("attachments", &att_s)); }
        if let Some(ref v) = self.attachment_fields { af_s = v.clone(); params.push(("attachment_fields", &af_s)); }
        if let Some(v) = self.members { params.push(("members", if v { "true" } else { "false" })); }
        let mf_s;
        if let Some(ref v) = self.member_fields { mf_s = v.clone(); params.push(("member_fields", &mf_s)); }
        if let Some(v) = self.members_voted { params.push(("membersVoted", if v { "true" } else { "false" })); }
        let mvf_s;
        if let Some(ref v) = self.member_voted_fields { mvf_s = v.clone(); params.push(("memberVoted_fields", &mvf_s)); }
        if let Some(v) = self.check_item_states { params.push(("checkItemStates", if v { "true" } else { "false" })); }
        if let Some(ref v) = self.checklists { chk_s = v.clone(); params.push(("checklists", &chk_s)); }
        if let Some(ref v) = self.checklist_fields { chf_s = v.clone(); params.push(("checklist_fields", &chf_s)); }
        if let Some(v) = self.board { params.push(("board", if v { "true" } else { "false" })); }
        if let Some(ref v) = self.board_fields { bf_s = v.clone(); params.push(("board_fields", &bf_s)); }
        if let Some(v) = self.list { params.push(("list", if v { "true" } else { "false" })); }
        if let Some(v) = self.plugin_data { params.push(("pluginData", if v { "true" } else { "false" })); }
        if let Some(v) = self.stickers { params.push(("stickers", if v { "true" } else { "false" })); }
        if let Some(ref v) = self.sticker_fields { sf_s = v.clone(); params.push(("sticker_fields", &sf_s)); }
        if let Some(v) = self.custom_field_items { params.push(("customFieldItems", if v { "true" } else { "false" })); }

        self.client
            .get(&format!("/cards/{}", self.id.as_ref()), &params)
            .await
    }
}

pub struct CreateCardRequest<'a> {
    client: &'a TrelloClient,
    name: Option<String>,
    desc: Option<String>,
    pos: Option<String>,
    due: Option<String>,
    start: Option<String>,
    due_complete: Option<bool>,
    id_list: Option<TrelloID>,
    id_members: Option<Vec<TrelloID>>,
    id_labels: Option<Vec<TrelloID>>,
    url_source: Option<String>,
    file_source: Option<String>,
    mime_type: Option<String>,
    id_card_source: Option<TrelloID>,
    keep_from_source: Option<String>,
    address: Option<String>,
    location_name: Option<String>,
    coordinates: Option<String>,
    card_role: Option<String>,
}

impl<'a> CreateCardRequest<'a> {
    pub(crate) fn new(client: &'a TrelloClient) -> Self {
        Self {
            client,
            name: None,
            desc: None,
            pos: None,
            due: None,
            start: None,
            due_complete: None,
            id_list: None,
            id_members: None,
            id_labels: None,
            url_source: None,
            file_source: None,
            mime_type: None,
            id_card_source: None,
            keep_from_source: None,
            address: None,
            location_name: None,
            coordinates: None,
            card_role: None,
        }
    }

    pub fn name(mut self, v: impl Into<String>) -> Self { self.name = Some(v.into()); self }
    pub fn desc(mut self, v: impl Into<String>) -> Self { self.desc = Some(v.into()); self }
    pub fn pos(mut self, v: &str) -> Self { self.pos = Some(v.to_string()); self }
    pub fn due(mut self, v: &str) -> Self { self.due = Some(v.to_string()); self }
    pub fn start(mut self, v: &str) -> Self { self.start = Some(v.to_string()); self }
    pub fn due_complete(mut self, v: bool) -> Self { self.due_complete = Some(v); self }
    pub fn id_list(mut self, v: impl Into<TrelloID>) -> Self { self.id_list = Some(v.into()); self }
    pub fn id_members(mut self, v: Vec<TrelloID>) -> Self { self.id_members = Some(v); self }
    pub fn id_labels(mut self, v: Vec<TrelloID>) -> Self { self.id_labels = Some(v); self }
    pub fn url_source(mut self, v: &str) -> Self { self.url_source = Some(v.to_string()); self }
    pub fn file_source(mut self, v: &str) -> Self { self.file_source = Some(v.to_string()); self }
    pub fn mime_type(mut self, v: &str) -> Self { self.mime_type = Some(v.to_string()); self }
    pub fn id_card_source(mut self, v: impl Into<TrelloID>) -> Self { self.id_card_source = Some(v.into()); self }
    pub fn keep_from_source(mut self, v: &str) -> Self { self.keep_from_source = Some(v.to_string()); self }
    pub fn address(mut self, v: &str) -> Self { self.address = Some(v.to_string()); self }
    pub fn location_name(mut self, v: &str) -> Self { self.location_name = Some(v.to_string()); self }
    pub fn coordinates(mut self, v: &str) -> Self { self.coordinates = Some(v.to_string()); self }

    pub async fn send(self) -> Result<Card> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let name_s; let desc_s; let pos_s; let due_s; let start_s; let dc_s; let il_s; let im_s; let ilb_s;
        let us_s; let fs_s; let mt_s; let ics_s; let kfs_s; let addr_s; let ln_s; let coord_s; let cr_s;
        if let Some(ref v) = self.name { name_s = v.clone(); params.push(("name", &name_s)); }
        if let Some(ref v) = self.desc { desc_s = v.clone(); params.push(("desc", &desc_s)); }
        if let Some(ref v) = self.pos { pos_s = v.clone(); params.push(("pos", &pos_s)); }
        if let Some(ref v) = self.due { due_s = v.clone(); params.push(("due", &due_s)); }
        if let Some(ref v) = self.start { start_s = v.clone(); params.push(("start", &start_s)); }
        if let Some(v) = self.due_complete { dc_s = if v { "true" } else { "false" }; params.push(("dueComplete", dc_s)); }
        if let Some(ref v) = self.id_list { il_s = v.to_string(); params.push(("idList", &il_s)); }
        if let Some(ref v) = self.id_members { im_s = v.iter().map(|m| m.as_ref()).collect::<Vec<_>>().join(","); params.push(("idMembers", &im_s)); }
        if let Some(ref v) = self.id_labels { ilb_s = v.iter().map(|m| m.as_ref()).collect::<Vec<_>>().join(","); params.push(("idLabels", &ilb_s)); }
        if let Some(ref v) = self.url_source { us_s = v.clone(); params.push(("urlSource", &us_s)); }
        if let Some(ref v) = self.file_source { fs_s = v.clone(); params.push(("fileSource", &fs_s)); }
        if let Some(ref v) = self.mime_type { mt_s = v.clone(); params.push(("mimeType", &mt_s)); }
        if let Some(ref v) = self.id_card_source { ics_s = v.to_string(); params.push(("idCardSource", &ics_s)); }
        if let Some(ref v) = self.keep_from_source { kfs_s = v.clone(); params.push(("keepFromSource", &kfs_s)); }
        if let Some(ref v) = self.address { addr_s = v.clone(); params.push(("address", &addr_s)); }
        if let Some(ref v) = self.location_name { ln_s = v.clone(); params.push(("locationName", &ln_s)); }
        if let Some(ref v) = self.coordinates { coord_s = v.clone(); params.push(("coordinates", &coord_s)); }
        if let Some(ref v) = self.card_role { cr_s = v.clone(); params.push(("cardRole", &cr_s)); }
        self.client.post("/cards", &params, None::<&()>).await
    }
}

pub struct UpdateCardRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    name: Option<String>,
    desc: Option<String>,
    closed: Option<bool>,
    id_list: Option<TrelloID>,
    id_board: Option<TrelloID>,
    pos: Option<String>,
    due: Option<String>,
    start: Option<String>,
    due_complete: Option<bool>,
    subscribed: Option<bool>,
    address: Option<String>,
    location_name: Option<String>,
    coordinates: Option<String>,
    cover_color: Option<String>,
    id_attachment_cover: Option<TrelloID>,
}

impl<'a> UpdateCardRequest<'a> {
    pub(crate) fn new(client: &'a TrelloClient, id: TrelloID) -> Self {
        Self {
            client,
            id,
            name: None,
            desc: None,
            closed: None,
            id_list: None,
            id_board: None,
            pos: None,
            due: None,
            start: None,
            due_complete: None,
            subscribed: None,
            address: None,
            location_name: None,
            coordinates: None,
            cover_color: None,
            id_attachment_cover: None,
        }
    }

    pub fn name(mut self, v: impl Into<String>) -> Self { self.name = Some(v.into()); self }
    pub fn desc(mut self, v: impl Into<String>) -> Self { self.desc = Some(v.into()); self }
    pub fn closed(mut self, v: bool) -> Self { self.closed = Some(v); self }
    pub fn id_list(mut self, v: impl Into<TrelloID>) -> Self { self.id_list = Some(v.into()); self }
    pub fn id_board(mut self, v: impl Into<TrelloID>) -> Self { self.id_board = Some(v.into()); self }
    pub fn pos(mut self, v: &str) -> Self { self.pos = Some(v.to_string()); self }
    pub fn due(mut self, v: &str) -> Self { self.due = Some(v.to_string()); self }
    pub fn start(mut self, v: &str) -> Self { self.start = Some(v.to_string()); self }
    pub fn due_complete(mut self, v: bool) -> Self { self.due_complete = Some(v); self }
    pub fn subscribed(mut self, v: bool) -> Self { self.subscribed = Some(v); self }
    pub fn cover_color(mut self, v: &str) -> Self { self.cover_color = Some(v.to_string()); self }

    pub async fn send(self) -> Result<Card> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let name_s; let desc_s; let cl_s; let il_s; let ib_s; let pos_s; let due_s; let start_s;
        let dc_s; let sub_s; let addr_s; let ln_s; let coord_s; let cc_s; let iac_s;
        if let Some(ref v) = self.name { name_s = v.clone(); params.push(("name", &name_s)); }
        if let Some(ref v) = self.desc { desc_s = v.clone(); params.push(("desc", &desc_s)); }
        if let Some(v) = self.closed { cl_s = if v { "true" } else { "false" }; params.push(("closed", cl_s)); }
        if let Some(ref v) = self.id_list { il_s = v.to_string(); params.push(("idList", &il_s)); }
        if let Some(ref v) = self.id_board { ib_s = v.to_string(); params.push(("idBoard", &ib_s)); }
        if let Some(ref v) = self.pos { pos_s = v.clone(); params.push(("pos", &pos_s)); }
        if let Some(ref v) = self.due { due_s = v.clone(); params.push(("due", &due_s)); }
        if let Some(ref v) = self.start { start_s = v.clone(); params.push(("start", &start_s)); }
        if let Some(v) = self.due_complete { dc_s = if v { "true" } else { "false" }; params.push(("dueComplete", dc_s)); }
        if let Some(v) = self.subscribed { sub_s = if v { "true" } else { "false" }; params.push(("subscribed", sub_s)); }
        if let Some(ref v) = self.address { addr_s = v.clone(); params.push(("address", &addr_s)); }
        if let Some(ref v) = self.location_name { ln_s = v.clone(); params.push(("locationName", &ln_s)); }
        if let Some(ref v) = self.coordinates { coord_s = v.clone(); params.push(("coordinates", &coord_s)); }
        if let Some(ref v) = self.cover_color { cc_s = v.clone(); params.push(("cover_color", &cc_s)); }
        if let Some(ref v) = self.id_attachment_cover { iac_s = v.to_string(); params.push(("idAttachmentCover", &iac_s)); }
        self.client
            .put(&format!("/cards/{}", self.id.as_ref()), &params, None::<&()>)
            .await
    }
}

pub struct GetCardBoardRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    fields: Option<String>,
}

impl<'a> GetCardBoardRequest<'a> {
    pub fn fields(mut self, v: &str) -> Self { self.fields = Some(v.to_string()); self }
    pub async fn send(self) -> Result<Board> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let fields_s;
        if let Some(ref v) = self.fields { fields_s = v.clone(); params.push(("fields", &fields_s)); }
        self.client.get(&format!("/cards/{}/board", self.id.as_ref()), &params).await
    }
}

pub struct GetCardListRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    fields: Option<String>,
}

impl<'a> GetCardListRequest<'a> {
    pub fn fields(mut self, v: &str) -> Self { self.fields = Some(v.to_string()); self }
    pub async fn send(self) -> Result<TrelloList> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let fields_s;
        if let Some(ref v) = self.fields { fields_s = v.clone(); params.push(("fields", &fields_s)); }
        self.client.get(&format!("/cards/{}/list", self.id.as_ref()), &params).await
    }
}

pub struct GetCardActionsRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    filter: Option<String>,
    page: Option<u32>,
}

impl<'a> GetCardActionsRequest<'a> {
    pub fn filter(mut self, v: &str) -> Self { self.filter = Some(v.to_string()); self }
    pub fn page(mut self, v: u32) -> Self { self.page = Some(v); self }
    pub async fn send(self) -> Result<Vec<Action>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let filter_s; let page_s;
        if let Some(ref v) = self.filter { filter_s = v.clone(); params.push(("filter", &filter_s)); }
        if let Some(v) = self.page { page_s = v.to_string(); params.push(("page", &page_s)); }
        self.client
            .get(&format!("/cards/{}/actions", self.id.as_ref()), &params)
            .await
    }
}

pub struct AddCommentRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    text: String,
}

impl<'a> AddCommentRequest<'a> {
    pub async fn send(self) -> Result<Action> {
        self.client
            .post(&format!("/cards/{}/actions/comments", self.id.as_ref()), &[("text", &self.text)], None::<&()>)
            .await
    }
}

pub struct UpdateCommentRequest<'a> {
    client: &'a TrelloClient,
    card_id: TrelloID,
    action_id: TrelloID,
    text: Option<String>,
}

impl<'a> UpdateCommentRequest<'a> {
    pub fn text(mut self, v: impl Into<String>) -> Self { self.text = Some(v.into()); self }
    pub async fn send(self) -> Result<Action> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let text_s;
        if let Some(ref v) = self.text { text_s = v.clone(); params.push(("text", &text_s)); }
        self.client
            .put(&format!("/cards/{}/actions/{}/comments", self.card_id.as_ref(), self.action_id.as_ref()), &params, None::<&()>)
            .await
    }
}

pub struct GetCardAttachmentsRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    fields: Option<String>,
    filter: Option<String>,
}

impl<'a> GetCardAttachmentsRequest<'a> {
    pub fn fields(mut self, v: &str) -> Self { self.fields = Some(v.to_string()); self }
    pub fn filter(mut self, v: &str) -> Self { self.filter = Some(v.to_string()); self }
    pub async fn send(self) -> Result<Vec<Attachment>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let fields_s; let filter_s;
        if let Some(ref v) = self.fields { fields_s = v.clone(); params.push(("fields", &fields_s)); }
        if let Some(ref v) = self.filter { filter_s = v.clone(); params.push(("filter", &filter_s)); }
        self.client.get(&format!("/cards/{}/attachments", self.id.as_ref()), &params).await
    }
}

pub struct CreateAttachmentRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    name: Option<String>,
    url: Option<String>,
    mime_type: Option<String>,
}

impl<'a> CreateAttachmentRequest<'a> {
    pub fn name(mut self, v: &str) -> Self { self.name = Some(v.to_string()); self }
    pub fn url(mut self, v: &str) -> Self { self.url = Some(v.to_string()); self }
    pub fn mime_type(mut self, v: &str) -> Self { self.mime_type = Some(v.to_string()); self }

    pub async fn send(self) -> Result<Attachment> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let name_s; let url_s; let mt_s;
        if let Some(ref v) = self.name { name_s = v.clone(); params.push(("name", &name_s)); }
        if let Some(ref v) = self.url { url_s = v.clone(); params.push(("url", &url_s)); }
        if let Some(ref v) = self.mime_type { mt_s = v.clone(); params.push(("mimeType", &mt_s)); }
        self.client
            .post(&format!("/cards/{}/attachments", self.id.as_ref()), &params, None::<&()>)
            .await
    }
}

pub struct GetCardChecklistsRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    check_items: Option<String>,
    check_item_fields: Option<String>,
    filter: Option<String>,
    fields: Option<String>,
}

impl<'a> GetCardChecklistsRequest<'a> {
    pub fn check_items(mut self, v: &str) -> Self { self.check_items = Some(v.to_string()); self }
    pub fn fields(mut self, v: &str) -> Self { self.fields = Some(v.to_string()); self }

    pub async fn send(self) -> Result<Vec<Checklist>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let ci_s; let cif_s; let filter_s; let fields_s;
        if let Some(ref v) = self.check_items { ci_s = v.clone(); params.push(("checkItems", &ci_s)); }
        if let Some(ref v) = self.check_item_fields { cif_s = v.clone(); params.push(("checkItem_fields", &cif_s)); }
        if let Some(ref v) = self.filter { filter_s = v.clone(); params.push(("filter", &filter_s)); }
        if let Some(ref v) = self.fields { fields_s = v.clone(); params.push(("fields", &fields_s)); }
        self.client.get(&format!("/cards/{}/checklists", self.id.as_ref()), &params).await
    }
}

pub struct CreateCardChecklistRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    name: String,
    id_checklist_source: Option<TrelloID>,
}

impl<'a> CreateCardChecklistRequest<'a> {
    pub fn id_checklist_source(mut self, v: impl Into<TrelloID>) -> Self { self.id_checklist_source = Some(v.into()); self }

    pub async fn send(self) -> Result<Checklist> {
        let mut params: Vec<(&str, &str)> = vec![("name", self.name.as_str())];
        let idcs_s;
        if let Some(ref v) = self.id_checklist_source { idcs_s = v.to_string(); params.push(("idChecklistSource", &idcs_s)); }
        self.client
            .post(&format!("/cards/{}/checklists", self.id.as_ref()), &params, None::<&()>)
            .await
    }
}

pub struct AddMemberToCardRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    value: TrelloID,
}

impl<'a> AddMemberToCardRequest<'a> {
    pub async fn send(self) -> Result<()> {
        self.client
            .post::<serde_json::Value, ()>(
                &format!("/cards/{}/idMembers", self.id.as_ref()),
                &[("value", self.value.as_ref())],
                None,
            )
            .await?;
        Ok(())
    }
}

pub struct AddLabelToCardRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    value: TrelloID,
}

impl<'a> AddLabelToCardRequest<'a> {
    pub async fn send(self) -> Result<()> {
        self.client
            .post::<serde_json::Value, ()>(
                &format!("/cards/{}/idLabels", self.id.as_ref()),
                &[("value", self.value.as_ref())],
                None,
            )
            .await?;
        Ok(())
    }
}

pub struct CreateStickerRequest<'a> {
    client: &'a TrelloClient,
    card_id: TrelloID,
    image: Option<String>,
    top: Option<f64>,
    left: Option<f64>,
    z_index: Option<i64>,
    rotate: Option<i64>,
}

impl<'a> CreateStickerRequest<'a> {
    pub fn image(mut self, v: &str) -> Self { self.image = Some(v.to_string()); self }
    pub fn top(mut self, v: f64) -> Self { self.top = Some(v); self }
    pub fn left(mut self, v: f64) -> Self { self.left = Some(v); self }
    pub fn z_index(mut self, v: i64) -> Self { self.z_index = Some(v); self }
    pub fn rotate(mut self, v: i64) -> Self { self.rotate = Some(v); self }

    pub async fn send(self) -> Result<serde_json::Value> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let image_s; let top_s; let left_s; let zi_s; let rotate_s;
        if let Some(ref v) = self.image { image_s = v.clone(); params.push(("image", &image_s)); }
        if let Some(v) = self.top { top_s = v.to_string(); params.push(("top", &top_s)); }
        if let Some(v) = self.left { left_s = v.to_string(); params.push(("left", &left_s)); }
        if let Some(v) = self.z_index { zi_s = v.to_string(); params.push(("zIndex", &zi_s)); }
        if let Some(v) = self.rotate { rotate_s = v.to_string(); params.push(("rotate", &rotate_s)); }
        self.client
            .post(&format!("/cards/{}/stickers", self.card_id.as_ref()), &params, None::<&()>)
            .await
    }
}

#[derive(serde::Serialize)]
struct CustomFieldValueBody {
    value: serde_json::Value,
}
