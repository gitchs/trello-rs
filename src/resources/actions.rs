use crate::client::TrelloClient;
use crate::error::Result;
use crate::models::action::{Action, ActionField, ReactionSummary};
use crate::models::board::Board;
use crate::models::card::Card;
use crate::models::common::TrelloID;
use crate::models::list::TrelloList;
use crate::models::member::Member;
use crate::models::organization::Organization;
use crate::params::{fields_to_query, FieldQuery};

pub struct ActionsResource<'a> {
    client: &'a TrelloClient,
}

impl<'a> ActionsResource<'a> {
    pub(crate) fn new(client: &'a TrelloClient) -> Self {
        Self { client }
    }

    // ── GET /actions/{id} ───────────────────────────────────────────

    pub fn get(&self, id: impl Into<TrelloID>) -> GetActionRequest<'a> {
        GetActionRequest {
            client: self.client,
            id: id.into(),
            display: None,
            entities: None,
            fields: None,
            member: None,
            member_fields: None,
            member_creator: None,
            member_creator_fields: None,
        }
    }

    // ── PUT /actions/{id} ───────────────────────────────────────────

    pub fn update(&self, id: impl Into<TrelloID>) -> UpdateActionRequest<'a> {
        UpdateActionRequest {
            client: self.client,
            id: id.into(),
            text: None,
        }
    }

    // ── DELETE /actions/{id} ────────────────────────────────────────

    pub async fn delete(&self, id: impl Into<TrelloID>) -> Result<()> {
        let id: TrelloID = id.into();
        self.client
            .delete_no_body(&format!("/actions/{}", id.as_ref()), &[])
            .await
    }

    // ── GET /actions/{id}/{field} ────────────────────────────────────

    pub async fn get_field(&self, id: impl Into<TrelloID>, field: ActionField) -> Result<serde_json::Value> {
        let id: TrelloID = id.into();
        self.client
            .get(&format!("/actions/{}/{}", id.as_ref(), field.as_field_str()), &[])
            .await
    }

    // ── PUT /actions/{id}/text ───────────────────────────────────────

    pub async fn update_text(&self, id: impl Into<TrelloID>, value: &str) -> Result<Action> {
        let id: TrelloID = id.into();
        self.client
            .put(&format!("/actions/{}/text", id.as_ref()), &[("value", value)], None::<&()>)
            .await
    }

    // ── GET /actions/{id}/board ──────────────────────────────────────

    pub fn get_board(&self, id: impl Into<TrelloID>) -> GetActionBoardRequest<'a> {
        GetActionBoardRequest {
            client: self.client,
            id: id.into(),
            fields: None,
        }
    }

    // ── GET /actions/{id}/card ───────────────────────────────────────

    pub fn get_card(&self, id: impl Into<TrelloID>) -> GetActionCardRequest<'a> {
        GetActionCardRequest {
            client: self.client,
            id: id.into(),
            fields: None,
        }
    }

    // ── GET /actions/{id}/list ───────────────────────────────────────

    pub fn get_list(&self, id: impl Into<TrelloID>) -> GetActionListRequest<'a> {
        GetActionListRequest {
            client: self.client,
            id: id.into(),
            fields: None,
        }
    }

    // ── GET /actions/{id}/member ─────────────────────────────────────

    pub fn get_member(&self, id: impl Into<TrelloID>) -> GetActionMemberRequest<'a> {
        GetActionMemberRequest {
            client: self.client,
            id: id.into(),
            fields: None,
        }
    }

    // ── GET /actions/{id}/memberCreator ──────────────────────────────

    pub fn get_member_creator(&self, id: impl Into<TrelloID>) -> GetActionMemberCreatorRequest<'a> {
        GetActionMemberCreatorRequest {
            client: self.client,
            id: id.into(),
            fields: None,
        }
    }

    // ── GET /actions/{id}/organization ───────────────────────────────

    pub fn get_organization(&self, id: impl Into<TrelloID>) -> GetActionOrganizationRequest<'a> {
        GetActionOrganizationRequest {
            client: self.client,
            id: id.into(),
            fields: None,
        }
    }

    // ── GET /actions/{idAction}/reactions ────────────────────────────

    pub fn get_reactions(&self, id: impl Into<TrelloID>) -> GetReactionsRequest<'a> {
        GetReactionsRequest {
            client: self.client,
            id: id.into(),
            member: None,
            emoji: None,
        }
    }

    // ── POST /actions/{idAction}/reactions ───────────────────────────

    pub fn add_reaction(&self, id: impl Into<TrelloID>, short_name: &str) -> AddReactionRequest<'a> {
        AddReactionRequest {
            client: self.client,
            id: id.into(),
            short_name: short_name.to_string(),
        }
    }

    // ── GET /actions/{idAction}/reactions/{id} ───────────────────────

    pub fn get_reaction(&self, action_id: impl Into<TrelloID>, reaction_id: impl Into<TrelloID>) -> GetReactionRequest<'a> {
        GetReactionRequest {
            client: self.client,
            action_id: action_id.into(),
            reaction_id: reaction_id.into(),
            member: None,
            emoji: None,
        }
    }

    // ── DELETE /actions/{idAction}/reactions/{id} ────────────────────

    pub async fn delete_reaction(&self, action_id: impl Into<TrelloID>, reaction_id: impl Into<TrelloID>) -> Result<()> {
        let action_id: TrelloID = action_id.into();
        let reaction_id: TrelloID = reaction_id.into();
        self.client
            .delete_no_body(
                &format!("/actions/{}/reactions/{}", action_id.as_ref(), reaction_id.as_ref()),
                &[],
            )
            .await
    }

    // ── GET /actions/{idAction}/reactionsSummary ─────────────────────

    pub async fn get_reactions_summary(&self, id: impl Into<TrelloID>) -> Result<Vec<ReactionSummary>> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/actions/{}/reactionsSummary", id.as_ref()), &[]).await
    }
}

// ──────────────────── Builder structs ────────────────────────────────

pub struct GetActionRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    display: Option<bool>,
    entities: Option<bool>,
    fields: Option<Vec<ActionField>>,
    member: Option<bool>,
    member_fields: Option<String>,
    member_creator: Option<bool>,
    member_creator_fields: Option<String>,
}

impl<'a> GetActionRequest<'a> {
    pub fn display(mut self, v: bool) -> Self { self.display = Some(v); self }
    pub fn entities(mut self, v: bool) -> Self { self.entities = Some(v); self }
    pub fn fields(mut self, f: &[ActionField]) -> Self { self.fields = Some(f.to_vec()); self }
    pub fn member(mut self, v: bool) -> Self { self.member = Some(v); self }
    pub fn member_creator(mut self, v: bool) -> Self { self.member_creator = Some(v); self }

    pub async fn send(self) -> Result<Action> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let fields_str; let mf_s; let mcf_s;
        if let Some(v) = self.display { params.push(("display", if v { "true" } else { "false" })); }
        if let Some(v) = self.entities { params.push(("entities", if v { "true" } else { "false" })); }
        if let Some(ref f) = self.fields { fields_str = fields_to_query(f); params.push(("fields", &fields_str)); }
        if let Some(v) = self.member { params.push(("member", if v { "true" } else { "false" })); }
        if let Some(ref v) = self.member_fields { mf_s = v.clone(); params.push(("member_fields", &mf_s)); }
        if let Some(v) = self.member_creator { params.push(("memberCreator", if v { "true" } else { "false" })); }
        if let Some(ref v) = self.member_creator_fields { mcf_s = v.clone(); params.push(("memberCreator_fields", &mcf_s)); }
        self.client.get(&format!("/actions/{}", self.id.as_ref()), &params).await
    }
}

pub struct UpdateActionRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    text: Option<String>,
}

impl<'a> UpdateActionRequest<'a> {
    pub fn text(mut self, v: impl Into<String>) -> Self { self.text = Some(v.into()); self }

    pub async fn send(self) -> Result<Action> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let text_s;
        if let Some(ref v) = self.text { text_s = v.clone(); params.push(("text", &text_s)); }
        self.client
            .put(&format!("/actions/{}", self.id.as_ref()), &params, None::<&()>)
            .await
    }
}

pub struct GetActionBoardRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    fields: Option<String>,
}

impl<'a> GetActionBoardRequest<'a> {
    pub fn fields(mut self, v: &str) -> Self { self.fields = Some(v.to_string()); self }
    pub async fn send(self) -> Result<Board> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let fields_s;
        if let Some(ref v) = self.fields { fields_s = v.clone(); params.push(("fields", &fields_s)); }
        self.client.get(&format!("/actions/{}/board", self.id.as_ref()), &params).await
    }
}

pub struct GetActionCardRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    fields: Option<String>,
}

impl<'a> GetActionCardRequest<'a> {
    pub fn fields(mut self, v: &str) -> Self { self.fields = Some(v.to_string()); self }
    pub async fn send(self) -> Result<Card> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let fields_s;
        if let Some(ref v) = self.fields { fields_s = v.clone(); params.push(("fields", &fields_s)); }
        self.client.get(&format!("/actions/{}/card", self.id.as_ref()), &params).await
    }
}

pub struct GetActionListRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    fields: Option<String>,
}

impl<'a> GetActionListRequest<'a> {
    pub fn fields(mut self, v: &str) -> Self { self.fields = Some(v.to_string()); self }
    pub async fn send(self) -> Result<TrelloList> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let fields_s;
        if let Some(ref v) = self.fields { fields_s = v.clone(); params.push(("fields", &fields_s)); }
        self.client.get(&format!("/actions/{}/list", self.id.as_ref()), &params).await
    }
}

pub struct GetActionMemberRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    fields: Option<String>,
}

impl<'a> GetActionMemberRequest<'a> {
    pub fn fields(mut self, v: &str) -> Self { self.fields = Some(v.to_string()); self }
    pub async fn send(self) -> Result<Member> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let fields_s;
        if let Some(ref v) = self.fields { fields_s = v.clone(); params.push(("fields", &fields_s)); }
        self.client.get(&format!("/actions/{}/member", self.id.as_ref()), &params).await
    }
}

pub struct GetActionMemberCreatorRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    fields: Option<String>,
}

impl<'a> GetActionMemberCreatorRequest<'a> {
    pub fn fields(mut self, v: &str) -> Self { self.fields = Some(v.to_string()); self }
    pub async fn send(self) -> Result<Member> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let fields_s;
        if let Some(ref v) = self.fields { fields_s = v.clone(); params.push(("fields", &fields_s)); }
        self.client.get(&format!("/actions/{}/memberCreator", self.id.as_ref()), &params).await
    }
}

pub struct GetActionOrganizationRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    fields: Option<String>,
}

impl<'a> GetActionOrganizationRequest<'a> {
    pub fn fields(mut self, v: &str) -> Self { self.fields = Some(v.to_string()); self }
    pub async fn send(self) -> Result<Organization> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let fields_s;
        if let Some(ref v) = self.fields { fields_s = v.clone(); params.push(("fields", &fields_s)); }
        self.client.get(&format!("/actions/{}/organization", self.id.as_ref()), &params).await
    }
}

pub struct GetReactionsRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    member: Option<bool>,
    emoji: Option<bool>,
}

impl<'a> GetReactionsRequest<'a> {
    pub fn member(mut self, v: bool) -> Self { self.member = Some(v); self }
    pub fn emoji(mut self, v: bool) -> Self { self.emoji = Some(v); self }
    pub async fn send(self) -> Result<Vec<Action>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        if let Some(v) = self.member { params.push(("member", if v { "true" } else { "false" })); }
        if let Some(v) = self.emoji { params.push(("emoji", if v { "true" } else { "false" })); }
        self.client.get(&format!("/actions/{}/reactions", self.id.as_ref()), &params).await
    }
}

pub struct AddReactionRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    short_name: String,
}

impl<'a> AddReactionRequest<'a> {
    pub async fn send(self) -> Result<Action> {
        self.client
            .post(&format!("/actions/{}/reactions", self.id.as_ref()), &[("shortName", &self.short_name)], None::<&()>)
            .await
    }
}

pub struct GetReactionRequest<'a> {
    client: &'a TrelloClient,
    action_id: TrelloID,
    reaction_id: TrelloID,
    member: Option<bool>,
    emoji: Option<bool>,
}

impl<'a> GetReactionRequest<'a> {
    pub fn member(mut self, v: bool) -> Self { self.member = Some(v); self }
    pub fn emoji(mut self, v: bool) -> Self { self.emoji = Some(v); self }
    pub async fn send(self) -> Result<Action> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        if let Some(v) = self.member { params.push(("member", if v { "true" } else { "false" })); }
        if let Some(v) = self.emoji { params.push(("emoji", if v { "true" } else { "false" })); }
        self.client
            .get(&format!("/actions/{}/reactions/{}", self.action_id.as_ref(), self.reaction_id.as_ref()), &params)
            .await
    }
}
