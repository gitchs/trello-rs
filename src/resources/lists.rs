use crate::client::TrelloClient;
use crate::error::Result;
use crate::models::action::Action;
use crate::models::board::Board;
use crate::models::card::Card;
use crate::models::common::TrelloID;
use crate::models::list::{ListField, TrelloList};
use crate::params::FieldQuery;

pub struct ListsResource<'a> {
    client: &'a TrelloClient,
}

impl<'a> ListsResource<'a> {
    pub(crate) fn new(client: &'a TrelloClient) -> Self {
        Self { client }
    }

    // ── GET /lists/{id} ────────────────────────────────────────────

    pub fn get(&self, id: impl Into<TrelloID>) -> GetListRequest<'a> {
        GetListRequest {
            client: self.client,
            id: id.into(),
            fields: None,
        }
    }

    // ── POST /lists ─────────────────────────────────────────────────

    pub fn create(&self) -> CreateListRequest<'a> {
        CreateListRequest {
            client: self.client,
            name: None,
            id_board: None,
            id_list_source: None,
            pos: None,
        }
    }

    // ── PUT /lists/{id} ────────────────────────────────────────────

    pub fn update(&self, id: impl Into<TrelloID>) -> UpdateListRequest<'a> {
        UpdateListRequest {
            client: self.client,
            id: id.into(),
            name: None,
            closed: None,
            id_board: None,
            pos: None,
            subscribed: None,
        }
    }

    // ── PUT /lists/{id}/closed ──────────────────────────────────────

    pub async fn close(&self, id: impl Into<TrelloID>, value: bool) -> Result<TrelloList> {
        let id: TrelloID = id.into();
        let v = if value { "true" } else { "false" };
        self.client
            .put(&format!("/lists/{}/closed", id.as_ref()), &[("value", v)], None::<&()>)
            .await
    }

    // ── PUT /lists/{id}/idBoard ─────────────────────────────────────

    pub async fn move_to_board(&self, id: impl Into<TrelloID>, board_id: impl Into<TrelloID>) -> Result<TrelloList> {
        let id: TrelloID = id.into();
        let board_id: TrelloID = board_id.into();
        self.client
            .put(&format!("/lists/{}/idBoard", id.as_ref()), &[("value", board_id.as_ref())], None::<&()>)
            .await
    }

    // ── POST /lists/{id}/archiveAllCards ────────────────────────────

    pub async fn archive_all_cards(&self, id: impl Into<TrelloID>) -> Result<()> {
        let id: TrelloID = id.into();
        self.client
            .post::<serde_json::Value, ()>(&format!("/lists/{}/archiveAllCards", id.as_ref()), &[], None)
            .await?;
        Ok(())
    }

    // ── POST /lists/{id}/moveAllCards ───────────────────────────────

    pub async fn move_all_cards(
        &self,
        id: impl Into<TrelloID>,
        target_board_id: impl Into<TrelloID>,
        target_list_id: impl Into<TrelloID>,
    ) -> Result<()> {
        let id: TrelloID = id.into();
        let target_board_id: TrelloID = target_board_id.into();
        let target_list_id: TrelloID = target_list_id.into();
        self.client
            .post::<serde_json::Value, ()>(
                &format!("/lists/{}/moveAllCards", id.as_ref()),
                &[("idBoard", target_board_id.as_ref()), ("idList", target_list_id.as_ref())],
                None,
            )
            .await?;
        Ok(())
    }

    // ── GET /lists/{id}/actions ─────────────────────────────────────

    pub fn get_actions(&self, id: impl Into<TrelloID>) -> GetListActionsRequest<'a> {
        GetListActionsRequest {
            client: self.client,
            id: id.into(),
            filter: None,
        }
    }

    // ── GET /lists/{id}/board ───────────────────────────────────────

    pub fn get_board(&self, id: impl Into<TrelloID>) -> GetListBoardRequest<'a> {
        GetListBoardRequest {
            client: self.client,
            id: id.into(),
            fields: None,
        }
    }

    // ── GET /lists/{id}/cards ───────────────────────────────────────

    pub async fn get_cards(&self, id: impl Into<TrelloID>) -> Result<Vec<Card>> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/lists/{}/cards", id.as_ref()), &[]).await
    }

    // ── GET /lists/{id}/{field} ─────────────────────────────────────

    pub async fn get_field(&self, id: impl Into<TrelloID>, field: ListField) -> Result<serde_json::Value> {
        let id: TrelloID = id.into();
        self.client
            .get(&format!("/lists/{}/{}", id.as_ref(), field.as_field_str()), &[])
            .await
    }

    // ── PUT /lists/{id}/{field} ─────────────────────────────────────

    pub async fn update_field(&self, id: impl Into<TrelloID>, field: &str, value: &str) -> Result<TrelloList> {
        let id: TrelloID = id.into();
        self.client
            .put(&format!("/lists/{}/{}", id.as_ref(), field), &[("value", value)], None::<&()>)
            .await
    }
}

// ──────────────────── Builder structs ────────────────────────────────

pub struct GetListRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    fields: Option<String>,
}

impl<'a> GetListRequest<'a> {
    pub fn fields(mut self, v: &str) -> Self { self.fields = Some(v.to_string()); self }

    pub async fn send(self) -> Result<TrelloList> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let fields_s;
        if let Some(ref v) = self.fields { fields_s = v.clone(); params.push(("fields", &fields_s)); }
        self.client
            .get(&format!("/lists/{}", self.id.as_ref()), &params)
            .await
    }
}

pub struct CreateListRequest<'a> {
    client: &'a TrelloClient,
    name: Option<String>,
    id_board: Option<TrelloID>,
    id_list_source: Option<TrelloID>,
    pos: Option<String>,
}

impl<'a> CreateListRequest<'a> {
    pub fn name(mut self, v: impl Into<String>) -> Self { self.name = Some(v.into()); self }
    pub fn id_board(mut self, v: impl Into<TrelloID>) -> Self { self.id_board = Some(v.into()); self }
    pub fn id_list_source(mut self, v: impl Into<TrelloID>) -> Self { self.id_list_source = Some(v.into()); self }
    pub fn pos(mut self, v: &str) -> Self { self.pos = Some(v.to_string()); self }

    pub async fn send(self) -> Result<TrelloList> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let name_s; let ib_s; let ils_s; let pos_s;
        if let Some(ref v) = self.name { name_s = v.clone(); params.push(("name", &name_s)); }
        if let Some(ref v) = self.id_board { ib_s = v.to_string(); params.push(("idBoard", &ib_s)); }
        if let Some(ref v) = self.id_list_source { ils_s = v.to_string(); params.push(("idListSource", &ils_s)); }
        if let Some(ref v) = self.pos { pos_s = v.clone(); params.push(("pos", &pos_s)); }
        self.client.post("/lists", &params, None::<&()>).await
    }
}

pub struct UpdateListRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    name: Option<String>,
    closed: Option<bool>,
    id_board: Option<TrelloID>,
    pos: Option<String>,
    subscribed: Option<bool>,
}

impl<'a> UpdateListRequest<'a> {
    pub fn name(mut self, v: impl Into<String>) -> Self { self.name = Some(v.into()); self }
    pub fn closed(mut self, v: bool) -> Self { self.closed = Some(v); self }
    pub fn id_board(mut self, v: impl Into<TrelloID>) -> Self { self.id_board = Some(v.into()); self }
    pub fn pos(mut self, v: &str) -> Self { self.pos = Some(v.to_string()); self }
    pub fn subscribed(mut self, v: bool) -> Self { self.subscribed = Some(v); self }

    pub async fn send(self) -> Result<TrelloList> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let name_s; let cl_s; let ib_s; let pos_s; let sub_s;
        if let Some(ref v) = self.name { name_s = v.clone(); params.push(("name", &name_s)); }
        if let Some(v) = self.closed { cl_s = if v { "true" } else { "false" }; params.push(("closed", cl_s)); }
        if let Some(ref v) = self.id_board { ib_s = v.to_string(); params.push(("idBoard", &ib_s)); }
        if let Some(ref v) = self.pos { pos_s = v.clone(); params.push(("pos", &pos_s)); }
        if let Some(v) = self.subscribed { sub_s = if v { "true" } else { "false" }; params.push(("subscribed", sub_s)); }
        self.client
            .put(&format!("/lists/{}", self.id.as_ref()), &params, None::<&()>)
            .await
    }
}

pub struct GetListBoardRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    fields: Option<String>,
}

impl<'a> GetListBoardRequest<'a> {
    pub fn fields(mut self, v: &str) -> Self { self.fields = Some(v.to_string()); self }
    pub async fn send(self) -> Result<Board> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let fields_s;
        if let Some(ref v) = self.fields { fields_s = v.clone(); params.push(("fields", &fields_s)); }
        self.client.get(&format!("/lists/{}/board", self.id.as_ref()), &params).await
    }
}

pub struct GetListActionsRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    filter: Option<String>,
}

impl<'a> GetListActionsRequest<'a> {
    pub fn filter(mut self, v: &str) -> Self { self.filter = Some(v.to_string()); self }
    pub async fn send(self) -> Result<Vec<Action>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let filter_s;
        if let Some(ref v) = self.filter { filter_s = v.clone(); params.push(("filter", &filter_s)); }
        self.client.get(&format!("/lists/{}/actions", self.id.as_ref()), &params).await
    }
}
