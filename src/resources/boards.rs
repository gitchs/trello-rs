use crate::client::TrelloClient;
use crate::error::Result;
use crate::models::action::Action;
use crate::models::board::{Board, BoardField, BoardMembershipType, BoardStars};
use crate::models::card::Card;
use crate::models::checklist::Checklist;
use crate::models::custom_field::CustomField;
use crate::models::label::Label;
use crate::models::list::TrelloList;
use crate::models::common::{Membership, TrelloID};
use crate::models::plugin::Plugin;
use crate::models::tag::Tag;
use crate::params::{fields_to_query, FieldQuery, ViewFilter};

pub struct BoardsResource<'a> {
    client: &'a TrelloClient,
}

impl<'a> BoardsResource<'a> {
    pub(crate) fn new(client: &'a TrelloClient) -> Self {
        Self { client }
    }

    // ── GET /boards/{id} ───────────────────────────────────────────

    pub fn get(&self, id: impl Into<TrelloID>) -> GetBoardRequest<'a> {
        GetBoardRequest {
            client: self.client,
            id: id.into(),
            actions: None,
            board_stars: None,
            cards: None,
            card_plugin_data: None,
            checklists: None,
            custom_fields: None,
            fields: None,
            labels: None,
            lists: None,
            members: None,
            memberships: None,
            plugin_data: None,
            organization: None,
            organization_plugin_data: None,
            my_prefs: None,
            tags: None,
        }
    }

    // ── POST /boards/ ──────────────────────────────────────────────

    pub fn create(&self) -> CreateBoardRequest<'a> {
        CreateBoardRequest {
            client: self.client,
            name: None,
            default_labels: None,
            default_lists: None,
            desc: None,
            id_organization: None,
            id_board_source: None,
            keep_from_source: None,
            power_ups: None,
            prefs_permission_level: None,
            prefs_voting: None,
            prefs_comments: None,
            prefs_invitations: None,
            prefs_self_join: None,
            prefs_card_covers: None,
            prefs_background: None,
            prefs_card_aging: None,
        }
    }

    // ── PUT /boards/{id} ───────────────────────────────────────────

    pub fn update(&self, id: impl Into<TrelloID>) -> UpdateBoardRequest<'a> {
        UpdateBoardRequest {
            client: self.client,
            id: id.into(),
            name: None,
            desc: None,
            closed: None,
            subscribed: None,
            id_organization: None,
            prefs_permission_level: None,
            prefs_voting: None,
            prefs_comments: None,
            prefs_invitations: None,
            prefs_self_join: None,
            prefs_card_covers: None,
            prefs_background: None,
            prefs_card_aging: None,
        }
    }

    // ── DELETE /boards/{id} ────────────────────────────────────────

    pub async fn delete(&self, id: impl Into<TrelloID>) -> Result<()> {
        let id: TrelloID = id.into();
        self.client
            .delete_no_body(&format!("/boards/{}", id.as_ref()), &[])
            .await
    }

    // ── GET /boards/{id}/{field} ────────────────────────────────────

    pub async fn get_field(&self, id: impl Into<TrelloID>, field: BoardField) -> Result<serde_json::Value> {
        let id: TrelloID = id.into();
        self.client
            .get(&format!("/boards/{}/{}", id.as_ref(), field.as_field_str()), &[])
            .await
    }

    // ── GET /boards/{id}/cards ──────────────────────────────────────

    pub async fn get_cards(&self, id: impl Into<TrelloID>) -> Result<Vec<Card>> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/boards/{}/cards", id.as_ref()), &[]).await
    }

    // ── GET /boards/{id}/cards/{filter} ─────────────────────────────

    pub async fn get_cards_filtered(&self, id: impl Into<TrelloID>, filter: ViewFilter) -> Result<Vec<Card>> {
        let id: TrelloID = id.into();
        self.client
            .get(&format!("/boards/{}/cards/{}", id.as_ref(), serde_json::to_string(&filter).unwrap().trim_matches('"')), &[])
            .await
    }

    // ── GET /boards/{id}/lists ──────────────────────────────────────

    pub fn get_lists(&self, id: impl Into<TrelloID>) -> GetBoardListsRequest<'a> {
        GetBoardListsRequest {
            client: self.client,
            id: id.into(),
            cards: None,
            card_fields: None,
            filter: None,
            fields: None,
        }
    }

    // ── POST /boards/{id}/lists ─────────────────────────────────────

    pub fn create_list(&self, id: impl Into<TrelloID>) -> CreateListRequest<'a> {
        CreateListRequest {
            client: self.client,
            id: id.into(),
            name: None,
            pos: None,
        }
    }

    // ── GET /boards/{id}/labels ─────────────────────────────────────

    pub fn get_labels(&self, id: impl Into<TrelloID>) -> GetBoardLabelsRequest<'a> {
        GetBoardLabelsRequest {
            client: self.client,
            id: id.into(),
            fields: None,
            limit: None,
        }
    }

    // ── POST /boards/{id}/labels ────────────────────────────────────

    pub fn create_label(&self, id: impl Into<TrelloID>) -> CreateLabelRequest<'a> {
        CreateLabelRequest {
            client: self.client,
            id: id.into(),
            name: None,
            color: None,
        }
    }

    // ── GET /boards/{id}/members ────────────────────────────────────

    pub async fn get_members(&self, id: impl Into<TrelloID>) -> Result<Vec<crate::models::member::Member>> {
        let id: TrelloID = id.into();
        self.client
            .get(&format!("/boards/{}/members", id.as_ref()), &[])
            .await
    }

    // ── PUT /boards/{id}/members ────────────────────────────────────

    pub fn invite_member(&self, id: impl Into<TrelloID>) -> InviteMemberToBoardRequest<'a> {
        InviteMemberToBoardRequest {
            client: self.client,
            id: id.into(),
            email: None,
            id_member: None,
            member_type: None,
        }
    }

    // ── PUT /boards/{id}/members/{idMember} ─────────────────────────

    pub fn add_member(&self, board_id: impl Into<TrelloID>, member_id: impl Into<TrelloID>) -> AddMemberRequest<'a> {
        AddMemberRequest {
            client: self.client,
            board_id: board_id.into(),
            member_id: member_id.into(),
            member_type: None,
            allow_billable_guest: None,
        }
    }

    // ── DELETE /boards/{id}/members/{idMember} ──────────────────────

    pub async fn remove_member(&self, board_id: impl Into<TrelloID>, member_id: impl Into<TrelloID>) -> Result<()> {
        let board_id: TrelloID = board_id.into();
        let member_id: TrelloID = member_id.into();
        self.client
            .delete_no_body(&format!("/boards/{}/members/{}", board_id.as_ref(), member_id.as_ref()), &[])
            .await
    }

    // ── GET /boards/{id}/actions ────────────────────────────────────

    pub fn get_actions(&self, board_id: impl Into<TrelloID>) -> GetBoardActionsRequest<'a> {
        GetBoardActionsRequest {
            client: self.client,
            board_id: board_id.into(),
            fields: None,
            filter: None,
            format: None,
            id_models: None,
            limit: None,
            member: None,
            member_fields: None,
            member_creator: None,
            member_creator_fields: None,
            page: None,
            reactions: None,
            before: None,
            since: None,
        }
    }

    // ── GET /boards/{id}/checklists ─────────────────────────────────

    pub async fn get_checklists(&self, id: impl Into<TrelloID>) -> Result<Vec<Checklist>> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/boards/{}/checklists", id.as_ref()), &[]).await
    }

    // ── GET /boards/{id}/customFields ───────────────────────────────

    pub async fn get_custom_fields(&self, id: impl Into<TrelloID>) -> Result<Vec<CustomField>> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/boards/{}/customFields", id.as_ref()), &[]).await
    }

    // ── GET /boards/{id}/memberships ────────────────────────────────

    pub fn get_memberships(&self, id: impl Into<TrelloID>) -> GetMembershipsRequest<'a> {
        GetMembershipsRequest {
            client: self.client,
            id: id.into(),
            filter: None,
            activity: None,
            org_member_type: None,
            member: None,
            member_fields: None,
        }
    }

    // ── PUT /boards/{id}/memberships/{idMembership} ─────────────────

    pub fn update_membership(&self, board_id: impl Into<TrelloID>, membership_id: impl Into<TrelloID>) -> UpdateMembershipRequest<'a> {
        UpdateMembershipRequest {
            client: self.client,
            board_id: board_id.into(),
            membership_id: membership_id.into(),
            member_type: None,
            member_fields: None,
        }
    }

    // ── POST /boards/{id}/markedAsViewed ────────────────────────────

    pub async fn mark_as_viewed(&self, id: impl Into<TrelloID>) -> Result<()> {
        let id: TrelloID = id.into();
        self.client
            .post::<serde_json::Value, ()>(&format!("/boards/{}/markedAsViewed", id.as_ref()), &[], None)
            .await?;
        Ok(())
    }

    // ── Board stars ─────────────────────────────────────────────────

    pub async fn get_board_stars(&self, board_id: impl Into<TrelloID>) -> Result<Vec<BoardStars>> {
        let board_id: TrelloID = board_id.into();
        self.client.get(&format!("/boards/{}/boardStars", board_id.as_ref()), &[]).await
    }

    // ── GET /boards/{id}/plugins ────────────────────────────────────

    pub async fn get_plugins(&self, id: impl Into<TrelloID>) -> Result<Vec<Plugin>> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/boards/{}/plugins", id.as_ref()), &[]).await
    }

    // ── Board email key / calendar key ──────────────────────────────

    pub async fn generate_email_key(&self, id: impl Into<TrelloID>) -> Result<serde_json::Value> {
        let id: TrelloID = id.into();
        self.client
            .post(&format!("/boards/{}/emailKey/generate", id.as_ref()), &[], None::<&()>)
            .await
    }

    pub async fn generate_calendar_key(&self, id: impl Into<TrelloID>) -> Result<serde_json::Value> {
        let id: TrelloID = id.into();
        self.client
            .post(&format!("/boards/{}/calendarKey/generate", id.as_ref()), &[], None::<&()>)
            .await
    }

    // ── Tags ────────────────────────────────────────────────────────

    pub async fn create_tag(&self, id: impl Into<TrelloID>, value: &str) -> Result<Tag> {
        let id: TrelloID = id.into();
        self.client
            .post(&format!("/boards/{}/idTags", id.as_ref()), &[("value", value)], None::<&()>)
            .await
    }

    // ── My preferences ──────────────────────────────────────────────

    pub async fn update_email_position(&self, id: impl Into<TrelloID>, value: &str) -> Result<serde_json::Value> {
        let id: TrelloID = id.into();
        self.client
            .put(&format!("/boards/{}/myPrefs/emailPosition", id.as_ref()), &[("value", value)], None::<&()>)
            .await
    }

    pub async fn update_id_email_list(&self, id: impl Into<TrelloID>, value: &str) -> Result<serde_json::Value> {
        let id: TrelloID = id.into();
        self.client
            .put(&format!("/boards/{}/myPrefs/idEmailList", id.as_ref()), &[("value", value)], None::<&()>)
            .await
    }

    pub async fn update_show_sidebar(&self, id: impl Into<TrelloID>, value: bool) -> Result<serde_json::Value> {
        let id: TrelloID = id.into();
        let v = if value { "true" } else { "false" };
        self.client
            .put(&format!("/boards/{}/myPrefs/showSidebar", id.as_ref()), &[("value", v)], None::<&()>)
            .await
    }

    pub async fn update_show_sidebar_activity(&self, id: impl Into<TrelloID>, value: bool) -> Result<serde_json::Value> {
        let id: TrelloID = id.into();
        let v = if value { "true" } else { "false" };
        self.client
            .put(&format!("/boards/{}/myPrefs/showSidebarActivity", id.as_ref()), &[("value", v)], None::<&()>)
            .await
    }

    pub async fn update_show_sidebar_board_actions(&self, id: impl Into<TrelloID>, value: bool) -> Result<serde_json::Value> {
        let id: TrelloID = id.into();
        let v = if value { "true" } else { "false" };
        self.client
            .put(&format!("/boards/{}/myPrefs/showSidebarBoardActions", id.as_ref()), &[("value", v)], None::<&()>)
            .await
    }

    pub async fn update_show_sidebar_members(&self, id: impl Into<TrelloID>, value: bool) -> Result<serde_json::Value> {
        let id: TrelloID = id.into();
        let v = if value { "true" } else { "false" };
        self.client
            .put(&format!("/boards/{}/myPrefs/showSidebarMembers", id.as_ref()), &[("value", v)], None::<&()>)
            .await
    }
}

// ──────────────────── Builder structs ────────────────────────────────

pub struct GetBoardRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    actions: Option<String>,
    board_stars: Option<String>,
    cards: Option<String>,
    card_plugin_data: Option<bool>,
    checklists: Option<String>,
    custom_fields: Option<bool>,
    fields: Option<Vec<BoardField>>,
    labels: Option<String>,
    lists: Option<String>,
    members: Option<String>,
    memberships: Option<String>,
    plugin_data: Option<bool>,
    organization: Option<bool>,
    organization_plugin_data: Option<bool>,
    my_prefs: Option<bool>,
    tags: Option<bool>,
}

impl<'a> GetBoardRequest<'a> {
    pub fn fields(mut self, f: &[BoardField]) -> Self {
        self.fields = Some(f.to_vec());
        self
    }
    pub fn actions(mut self, v: &str) -> Self { self.actions = Some(v.to_string()); self }
    pub fn cards(mut self, v: &str) -> Self { self.cards = Some(v.to_string()); self }
    pub fn card_plugin_data(mut self, v: bool) -> Self { self.card_plugin_data = Some(v); self }
    pub fn checklists(mut self, v: &str) -> Self { self.checklists = Some(v.to_string()); self }
    pub fn custom_fields(mut self, v: bool) -> Self { self.custom_fields = Some(v); self }
    pub fn labels(mut self, v: &str) -> Self { self.labels = Some(v.to_string()); self }
    pub fn lists(mut self, v: &str) -> Self { self.lists = Some(v.to_string()); self }
    pub fn members(mut self, v: &str) -> Self { self.members = Some(v.to_string()); self }
    pub fn memberships(mut self, v: &str) -> Self { self.memberships = Some(v.to_string()); self }
    pub fn organization(mut self, v: bool) -> Self { self.organization = Some(v); self }
    pub fn organization_plugin_data(mut self, v: bool) -> Self { self.organization_plugin_data = Some(v); self }
    pub fn board_stars(mut self, v: &str) -> Self { self.board_stars = Some(v.to_string()); self }
    pub fn plugin_data(mut self, v: bool) -> Self { self.plugin_data = Some(v); self }
    pub fn my_prefs(mut self, v: bool) -> Self { self.my_prefs = Some(v); self }
    pub fn tags(mut self, v: bool) -> Self { self.tags = Some(v); self }

    pub async fn send(self) -> Result<Board> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let fields_str;
        if let Some(ref f) = self.fields {
            fields_str = fields_to_query(f);
            params.push(("fields", &fields_str));
        }
        let actions_s; let cards_s; let checklists_s; let labels_s; let lists_s;
        let members_s; let memberships_s;
        if let Some(ref v) = self.actions { actions_s = v.clone(); params.push(("actions", &actions_s)); }
        if let Some(ref v) = self.cards { cards_s = v.clone(); params.push(("cards", &cards_s)); }
        let cp = if self.card_plugin_data.unwrap_or(false) { "true" } else { "" };
        if self.card_plugin_data.is_some() { params.push(("card_pluginData", cp)); }
        if let Some(ref v) = self.checklists { checklists_s = v.clone(); params.push(("checklists", &checklists_s)); }
        let cf = if self.custom_fields.unwrap_or(false) { "true" } else { "" };
        if self.custom_fields.is_some() { params.push(("customFields", cf)); }
        let bs_s;
        if let Some(ref v) = self.board_stars { bs_s = v.clone(); params.push(("boardStars", &bs_s)); }
        if let Some(ref v) = self.labels { labels_s = v.clone(); params.push(("labels", &labels_s)); }
        if let Some(ref v) = self.lists { lists_s = v.clone(); params.push(("lists", &lists_s)); }
        if let Some(ref v) = self.members { members_s = v.clone(); params.push(("members", &members_s)); }
        if let Some(ref v) = self.memberships { memberships_s = v.clone(); params.push(("memberships", &memberships_s)); }
        let pd = if self.plugin_data.unwrap_or(false) { "true" } else { "" };
        if self.plugin_data.is_some() { params.push(("pluginData", pd)); }
        let org = if self.organization.unwrap_or(false) { "true" } else { "" };
        if self.organization.is_some() { params.push(("organization", org)); }
        let opd = if self.organization_plugin_data.unwrap_or(false) { "true" } else { "" };
        if self.organization_plugin_data.is_some() { params.push(("organization_pluginData", opd)); }
        let mp = if self.my_prefs.unwrap_or(false) { "true" } else { "" };
        if self.my_prefs.is_some() { params.push(("myPrefs", mp)); }
        let tags = if self.tags.unwrap_or(false) { "true" } else { "" };
        if self.tags.is_some() { params.push(("tags", tags)); }

        self.client
            .get(&format!("/boards/{}", self.id.as_ref()), &params)
            .await
    }
}

pub struct CreateBoardRequest<'a> {
    client: &'a TrelloClient,
    name: Option<String>,
    default_labels: Option<bool>,
    default_lists: Option<bool>,
    desc: Option<String>,
    id_organization: Option<TrelloID>,
    id_board_source: Option<TrelloID>,
    keep_from_source: Option<String>,
    power_ups: Option<String>,
    prefs_permission_level: Option<String>,
    prefs_voting: Option<String>,
    prefs_comments: Option<String>,
    prefs_invitations: Option<String>,
    prefs_self_join: Option<bool>,
    prefs_card_covers: Option<bool>,
    prefs_background: Option<String>,
    prefs_card_aging: Option<String>,
}

impl<'a> CreateBoardRequest<'a> {
    pub fn name(mut self, v: impl Into<String>) -> Self { self.name = Some(v.into()); self }
    pub fn desc(mut self, v: impl Into<String>) -> Self { self.desc = Some(v.into()); self }
    pub fn default_labels(mut self, v: bool) -> Self { self.default_labels = Some(v); self }
    pub fn default_lists(mut self, v: bool) -> Self { self.default_lists = Some(v); self }
    pub fn id_organization(mut self, v: impl Into<TrelloID>) -> Self { self.id_organization = Some(v.into()); self }
    pub fn id_board_source(mut self, v: impl Into<TrelloID>) -> Self { self.id_board_source = Some(v.into()); self }
    pub fn keep_from_source(mut self, v: &str) -> Self { self.keep_from_source = Some(v.to_string()); self }
    pub fn power_ups(mut self, v: &str) -> Self { self.power_ups = Some(v.to_string()); self }
    pub fn prefs_permission_level(mut self, v: &str) -> Self { self.prefs_permission_level = Some(v.to_string()); self }
    pub fn prefs_voting(mut self, v: &str) -> Self { self.prefs_voting = Some(v.to_string()); self }
    pub fn prefs_comments(mut self, v: &str) -> Self { self.prefs_comments = Some(v.to_string()); self }
    pub fn prefs_invitations(mut self, v: &str) -> Self { self.prefs_invitations = Some(v.to_string()); self }
    pub fn prefs_self_join(mut self, v: bool) -> Self { self.prefs_self_join = Some(v); self }
    pub fn prefs_card_covers(mut self, v: bool) -> Self { self.prefs_card_covers = Some(v); self }
    pub fn prefs_background(mut self, v: &str) -> Self { self.prefs_background = Some(v.to_string()); self }
    pub fn prefs_card_aging(mut self, v: &str) -> Self { self.prefs_card_aging = Some(v.to_string()); self }

    pub async fn send(self) -> Result<Board> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let name_s; let desc_s; let dl_s; let dls_s; let org_s; let src_s; let kfs_s; let pu_s;
        let ppl_s; let pv_s; let pc_s; let pi_s; let psj_s; let pcc_s; let pb_s; let pca_s;
        if let Some(ref v) = self.name { name_s = v.clone(); params.push(("name", &name_s)); }
        if let Some(ref v) = self.desc { desc_s = v.clone(); params.push(("desc", &desc_s)); }
        if let Some(v) = self.default_labels { dl_s = if v { "true" } else { "false" }; params.push(("defaultLabels", dl_s)); }
        if let Some(v) = self.default_lists { dls_s = if v { "true" } else { "false" }; params.push(("defaultLists", dls_s)); }
        if let Some(ref v) = self.id_organization { org_s = v.to_string(); params.push(("idOrganization", &org_s)); }
        if let Some(ref v) = self.id_board_source { src_s = v.to_string(); params.push(("idBoardSource", &src_s)); }
        if let Some(ref v) = self.keep_from_source { kfs_s = v.clone(); params.push(("keepFromSource", &kfs_s)); }
        if let Some(ref v) = self.power_ups { pu_s = v.clone(); params.push(("powerUps", &pu_s)); }
        if let Some(ref v) = self.prefs_permission_level { ppl_s = v.clone(); params.push(("prefs_permissionLevel", &ppl_s)); }
        if let Some(ref v) = self.prefs_voting { pv_s = v.clone(); params.push(("prefs_voting", &pv_s)); }
        if let Some(ref v) = self.prefs_comments { pc_s = v.clone(); params.push(("prefs_comments", &pc_s)); }
        if let Some(ref v) = self.prefs_invitations { pi_s = v.clone(); params.push(("prefs_invitations", &pi_s)); }
        if let Some(v) = self.prefs_self_join { psj_s = if v { "true" } else { "false" }; params.push(("prefs_selfJoin", psj_s)); }
        if let Some(v) = self.prefs_card_covers { pcc_s = if v { "true" } else { "false" }; params.push(("prefs_cardCovers", pcc_s)); }
        if let Some(ref v) = self.prefs_background { pb_s = v.clone(); params.push(("prefs_background", &pb_s)); }
        if let Some(ref v) = self.prefs_card_aging { pca_s = v.clone(); params.push(("prefs_cardAging", &pca_s)); }
        self.client.post("/boards/", &params, None::<&()>).await
    }
}

pub struct UpdateBoardRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    name: Option<String>,
    desc: Option<String>,
    closed: Option<bool>,
    subscribed: Option<bool>,
    id_organization: Option<TrelloID>,
    prefs_permission_level: Option<String>,
    prefs_voting: Option<String>,
    prefs_comments: Option<String>,
    prefs_invitations: Option<String>,
    prefs_self_join: Option<bool>,
    prefs_card_covers: Option<bool>,
    prefs_background: Option<String>,
    prefs_card_aging: Option<String>,
}

impl<'a> UpdateBoardRequest<'a> {
    pub fn name(mut self, v: impl Into<String>) -> Self { self.name = Some(v.into()); self }
    pub fn desc(mut self, v: impl Into<String>) -> Self { self.desc = Some(v.into()); self }
    pub fn closed(mut self, v: bool) -> Self { self.closed = Some(v); self }
    pub fn subscribed(mut self, v: bool) -> Self { self.subscribed = Some(v); self }
    pub fn id_organization(mut self, v: impl Into<TrelloID>) -> Self { self.id_organization = Some(v.into()); self }
    pub fn prefs_permission_level(mut self, v: &str) -> Self { self.prefs_permission_level = Some(v.to_string()); self }
    pub fn prefs_voting(mut self, v: &str) -> Self { self.prefs_voting = Some(v.to_string()); self }
    pub fn prefs_comments(mut self, v: &str) -> Self { self.prefs_comments = Some(v.to_string()); self }
    pub fn prefs_invitations(mut self, v: &str) -> Self { self.prefs_invitations = Some(v.to_string()); self }
    pub fn prefs_self_join(mut self, v: bool) -> Self { self.prefs_self_join = Some(v); self }
    pub fn prefs_card_covers(mut self, v: bool) -> Self { self.prefs_card_covers = Some(v); self }
    pub fn prefs_background(mut self, v: &str) -> Self { self.prefs_background = Some(v.to_string()); self }
    pub fn prefs_card_aging(mut self, v: &str) -> Self { self.prefs_card_aging = Some(v.to_string()); self }

    pub async fn send(self) -> Result<Board> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let name_s; let desc_s; let cl_s; let sub_s; let org_s;
        let ppl_s; let pv_s; let pc_s; let pi_s; let psj_s; let pcc_s; let pb_s; let pca_s;
        if let Some(ref v) = self.name { name_s = v.clone(); params.push(("name", &name_s)); }
        if let Some(ref v) = self.desc { desc_s = v.clone(); params.push(("desc", &desc_s)); }
        if let Some(v) = self.closed { cl_s = if v { "true" } else { "false" }; params.push(("closed", cl_s)); }
        if let Some(v) = self.subscribed { sub_s = if v { "true" } else { "false" }; params.push(("subscribed", sub_s)); }
        if let Some(ref v) = self.id_organization { org_s = v.to_string(); params.push(("idOrganization", &org_s)); }
        if let Some(ref v) = self.prefs_permission_level { ppl_s = v.clone(); params.push(("prefs/permissionLevel", &ppl_s)); }
        if let Some(ref v) = self.prefs_voting { pv_s = v.clone(); params.push(("prefs/voting", &pv_s)); }
        if let Some(ref v) = self.prefs_comments { pc_s = v.clone(); params.push(("prefs/comments", &pc_s)); }
        if let Some(ref v) = self.prefs_invitations { pi_s = v.clone(); params.push(("prefs/invitations", &pi_s)); }
        if let Some(v) = self.prefs_self_join { psj_s = if v { "true" } else { "false" }; params.push(("prefs/selfJoin", psj_s)); }
        if let Some(v) = self.prefs_card_covers { pcc_s = if v { "true" } else { "false" }; params.push(("prefs/cardCovers", pcc_s)); }
        if let Some(ref v) = self.prefs_background { pb_s = v.clone(); params.push(("prefs/background", &pb_s)); }
        if let Some(ref v) = self.prefs_card_aging { pca_s = v.clone(); params.push(("prefs/cardAging", &pca_s)); }
        self.client
            .put(&format!("/boards/{}", self.id.as_ref()), &params, None::<&()>)
            .await
    }
}

pub struct GetBoardListsRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    cards: Option<String>,
    card_fields: Option<String>,
    filter: Option<ViewFilter>,
    fields: Option<String>,
}

impl<'a> GetBoardListsRequest<'a> {
    pub fn cards(mut self, v: &str) -> Self { self.cards = Some(v.to_string()); self }
    pub fn card_fields(mut self, v: &str) -> Self { self.card_fields = Some(v.to_string()); self }
    pub fn filter(mut self, v: ViewFilter) -> Self { self.filter = Some(v); self }
    pub fn fields(mut self, v: &str) -> Self { self.fields = Some(v.to_string()); self }

    pub async fn send(self) -> Result<Vec<TrelloList>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let cards_s; let cf_s; let filter_s; let fields_s;
        if let Some(ref v) = self.cards { cards_s = v.clone(); params.push(("cards", &cards_s)); }
        if let Some(ref v) = self.card_fields { cf_s = v.clone(); params.push(("card_fields", &cf_s)); }
        if let Some(v) = self.filter { filter_s = serde_json::to_string(&v).unwrap(); params.push(("filter", &filter_s)); }
        if let Some(ref v) = self.fields { fields_s = v.clone(); params.push(("fields", &fields_s)); }
        self.client
            .get(&format!("/boards/{}/lists", self.id.as_ref()), &params)
            .await
    }
}

pub struct CreateListRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    name: Option<String>,
    pos: Option<String>,
}

impl<'a> CreateListRequest<'a> {
    pub fn name(mut self, v: impl Into<String>) -> Self { self.name = Some(v.into()); self }
    pub fn pos(mut self, v: &str) -> Self { self.pos = Some(v.to_string()); self }

    pub async fn send(self) -> Result<TrelloList> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let name_s; let pos_s;
        if let Some(ref v) = self.name { name_s = v.clone(); params.push(("name", &name_s)); }
        if let Some(ref v) = self.pos { pos_s = v.clone(); params.push(("pos", &pos_s)); }
        self.client
            .post(&format!("/boards/{}/lists", self.id.as_ref()), &params, None::<&()>)
            .await
    }
}

pub struct GetBoardLabelsRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    fields: Option<String>,
    limit: Option<u32>,
}

impl<'a> GetBoardLabelsRequest<'a> {
    pub fn fields(mut self, v: &str) -> Self { self.fields = Some(v.to_string()); self }
    pub fn limit(mut self, v: u32) -> Self { self.limit = Some(v); self }

    pub async fn send(self) -> Result<Vec<Label>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let fields_s; let limit_s;
        if let Some(ref v) = self.fields { fields_s = v.clone(); params.push(("fields", &fields_s)); }
        if let Some(v) = self.limit { limit_s = v.to_string(); params.push(("limit", &limit_s)); }
        self.client
            .get(&format!("/boards/{}/labels", self.id.as_ref()), &params)
            .await
    }
}

pub struct CreateLabelRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    name: Option<String>,
    color: Option<String>,
}

impl<'a> CreateLabelRequest<'a> {
    pub fn name(mut self, v: impl Into<String>) -> Self { self.name = Some(v.into()); self }
    pub fn color(mut self, v: &str) -> Self { self.color = Some(v.to_string()); self }

    pub async fn send(self) -> Result<Label> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let name_s; let color_s;
        if let Some(ref v) = self.name { name_s = v.clone(); params.push(("name", &name_s)); }
        if let Some(ref v) = self.color { color_s = v.clone(); params.push(("color", &color_s)); }
        self.client
            .post(&format!("/boards/{}/labels", self.id.as_ref()), &params, None::<&()>)
            .await
    }
}

pub struct InviteMemberToBoardRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    email: Option<String>,
    id_member: Option<TrelloID>,
    member_type: Option<BoardMembershipType>,
}

impl<'a> InviteMemberToBoardRequest<'a> {
    pub fn email(mut self, v: impl Into<String>) -> Self { self.email = Some(v.into()); self }
    pub fn id_member(mut self, v: impl Into<TrelloID>) -> Self { self.id_member = Some(v.into()); self }
    pub fn member_type(mut self, v: BoardMembershipType) -> Self { self.member_type = Some(v); self }

    pub async fn send(self) -> Result<()> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let email_s; let idm_s; let mt_s;
        if let Some(ref v) = self.email { email_s = v.clone(); params.push(("email", &email_s)); }
        if let Some(ref v) = self.id_member { idm_s = v.to_string(); params.push(("idMember", &idm_s)); }
        if let Some(ref v) = self.member_type { mt_s = serde_json::to_string(v).unwrap(); params.push(("type", &mt_s)); }
        self.client
            .put::<serde_json::Value, ()>(&format!("/boards/{}/members", self.id.as_ref()), &params, None)
            .await?;
        Ok(())
    }
}

pub struct AddMemberRequest<'a> {
    client: &'a TrelloClient,
    board_id: TrelloID,
    member_id: TrelloID,
    member_type: Option<BoardMembershipType>,
    allow_billable_guest: Option<bool>,
}

impl<'a> AddMemberRequest<'a> {
    pub fn member_type(mut self, v: BoardMembershipType) -> Self { self.member_type = Some(v); self }
    pub fn allow_billable_guest(mut self, v: bool) -> Self { self.allow_billable_guest = Some(v); self }

    pub async fn send(self) -> Result<crate::models::member::Member> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let mt_s; let abg_s;
        if let Some(ref v) = self.member_type { mt_s = serde_json::to_string(v).unwrap(); params.push(("type", &mt_s)); }
        if let Some(v) = self.allow_billable_guest { abg_s = if v { "true" } else { "false" }; params.push(("allowBillableGuest", abg_s)); }
        self.client
            .put(&format!("/boards/{}/members/{}", self.board_id.as_ref(), self.member_id.as_ref()), &params, None::<&()>)
            .await
    }
}

pub struct GetBoardActionsRequest<'a> {
    client: &'a TrelloClient,
    board_id: TrelloID,
    fields: Option<Vec<crate::models::action::ActionField>>,
    filter: Option<String>,
    format: Option<String>,
    id_models: Option<String>,
    limit: Option<u32>,
    member: Option<bool>,
    member_fields: Option<String>,
    member_creator: Option<bool>,
    member_creator_fields: Option<String>,
    page: Option<u32>,
    reactions: Option<bool>,
    before: Option<String>,
    since: Option<String>,
}

impl<'a> GetBoardActionsRequest<'a> {
    pub fn fields(mut self, f: &[crate::models::action::ActionField]) -> Self { self.fields = Some(f.to_vec()); self }
    pub fn filter(mut self, v: &str) -> Self { self.filter = Some(v.to_string()); self }
    pub fn limit(mut self, v: u32) -> Self { self.limit = Some(v); self }
    pub fn page(mut self, v: u32) -> Self { self.page = Some(v); self }
    pub fn before(mut self, v: &str) -> Self { self.before = Some(v.to_string()); self }
    pub fn since(mut self, v: &str) -> Self { self.since = Some(v.to_string()); self }
    pub fn member(mut self, v: bool) -> Self { self.member = Some(v); self }
    pub fn member_creator(mut self, v: bool) -> Self { self.member_creator = Some(v); self }
    pub fn reactions(mut self, v: bool) -> Self { self.reactions = Some(v); self }

    pub async fn send(self) -> Result<Vec<Action>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let fields_str;
        if let Some(ref f) = self.fields { fields_str = fields_to_query(f); params.push(("fields", &fields_str)); }
        let filter_s; let format_s; let idm_s; let limit_s; let mf_s; let mcf_s; let page_s; let before_s; let since_s;
        if let Some(ref v) = self.filter { filter_s = v.clone(); params.push(("filter", &filter_s)); }
        if let Some(ref v) = self.format { format_s = v.clone(); params.push(("format", &format_s)); }
        if let Some(ref v) = self.id_models { idm_s = v.clone(); params.push(("idModels", &idm_s)); }
        if let Some(v) = self.limit { limit_s = v.to_string(); params.push(("limit", &limit_s)); }
        if let Some(v) = self.member { params.push(("member", if v { "true" } else { "false" })); }
        if let Some(ref v) = self.member_fields { mf_s = v.clone(); params.push(("member_fields", &mf_s)); }
        if let Some(v) = self.member_creator { params.push(("memberCreator", if v { "true" } else { "false" })); }
        if let Some(ref v) = self.member_creator_fields { mcf_s = v.clone(); params.push(("memberCreator_fields", &mcf_s)); }
        if let Some(v) = self.page { page_s = v.to_string(); params.push(("page", &page_s)); }
        if let Some(v) = self.reactions { params.push(("reactions", if v { "true" } else { "false" })); }
        if let Some(ref v) = self.before { before_s = v.clone(); params.push(("before", &before_s)); }
        if let Some(ref v) = self.since { since_s = v.clone(); params.push(("since", &since_s)); }
        self.client
            .get(&format!("/boards/{}/actions", self.board_id.as_ref()), &params)
            .await
    }
}

pub struct GetMembershipsRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    filter: Option<String>,
    activity: Option<bool>,
    org_member_type: Option<bool>,
    member: Option<bool>,
    member_fields: Option<String>,
}

impl<'a> GetMembershipsRequest<'a> {
    pub fn filter(mut self, v: &str) -> Self { self.filter = Some(v.to_string()); self }
    pub fn member(mut self, v: bool) -> Self { self.member = Some(v); self }

    pub async fn send(self) -> Result<Vec<Membership>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let filter_s; let mf_s;
        if let Some(ref v) = self.filter { filter_s = v.clone(); params.push(("filter", &filter_s)); }
        if let Some(v) = self.activity { params.push(("activity", if v { "true" } else { "false" })); }
        if let Some(v) = self.org_member_type { params.push(("orgMemberType", if v { "true" } else { "false" })); }
        if let Some(v) = self.member { params.push(("member", if v { "true" } else { "false" })); }
        if let Some(ref v) = self.member_fields { mf_s = v.clone(); params.push(("member_fields", &mf_s)); }
        self.client
            .get(&format!("/boards/{}/memberships", self.id.as_ref()), &params)
            .await
    }
}

pub struct UpdateMembershipRequest<'a> {
    client: &'a TrelloClient,
    board_id: TrelloID,
    membership_id: TrelloID,
    member_type: Option<BoardMembershipType>,
    member_fields: Option<String>,
}

impl<'a> UpdateMembershipRequest<'a> {
    pub fn member_type(mut self, v: BoardMembershipType) -> Self { self.member_type = Some(v); self }
    pub fn member_fields(mut self, v: &str) -> Self { self.member_fields = Some(v.to_string()); self }

    pub async fn send(self) -> Result<Membership> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let mt_s; let mf_s;
        if let Some(ref v) = self.member_type { mt_s = serde_json::to_string(v).unwrap(); params.push(("type", &mt_s)); }
        if let Some(ref v) = self.member_fields { mf_s = v.clone(); params.push(("member_fields", &mf_s)); }
        self.client
            .put(&format!("/boards/{}/memberships/{}", self.board_id.as_ref(), self.membership_id.as_ref()), &params, None::<&()>)
            .await
    }
}
