use crate::client::TrelloClient;
use crate::error::Result;
use crate::models::board::Board;
use crate::models::card::Card;
use crate::models::checklist::{CheckItem, Checklist};
use crate::models::common::TrelloID;

pub struct ChecklistsResource<'a> {
    client: &'a TrelloClient,
}

impl<'a> ChecklistsResource<'a> {
    pub(crate) fn new(client: &'a TrelloClient) -> Self {
        Self { client }
    }

    // ── GET /checklists/{id} ────────────────────────────────────────

    pub fn get(&self, id: impl Into<TrelloID>) -> GetChecklistRequest<'a> {
        GetChecklistRequest {
            client: self.client,
            id: id.into(),
            cards: None,
            card_fields: None,
            check_items: None,
            check_item_fields: None,
            fields: None,
        }
    }

    // ── POST /checklists ────────────────────────────────────────────

    pub fn create(&self) -> CreateChecklistRequest<'a> {
        CreateChecklistRequest {
            client: self.client,
            name: None,
            id_board: None,
            id_card: None,
            id_checklist_source: None,
            pos: None,
        }
    }

    // ── PUT /checklists/{id} ────────────────────────────────────────

    pub fn update(&self, id: impl Into<TrelloID>) -> UpdateChecklistRequest<'a> {
        UpdateChecklistRequest {
            client: self.client,
            id: id.into(),
            name: None,
            pos: None,
        }
    }

    // ── DELETE /checklists/{id} ─────────────────────────────────────

    pub async fn delete(&self, id: impl Into<TrelloID>) -> Result<()> {
        let id: TrelloID = id.into();
        self.client
            .delete_no_body(&format!("/checklists/{}", id.as_ref()), &[])
            .await
    }

    // ── GET /checklists/{id}/board ──────────────────────────────────

    pub fn get_board(&self, id: impl Into<TrelloID>) -> GetChecklistBoardRequest<'a> {
        GetChecklistBoardRequest {
            client: self.client,
            id: id.into(),
            fields: None,
        }
    }

    // ── GET /checklists/{id}/cards ──────────────────────────────────

    pub async fn get_cards(&self, id: impl Into<TrelloID>) -> Result<Vec<Card>> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/checklists/{}/cards", id.as_ref()), &[]).await
    }

    // ── GET /checklists/{id}/checkItems ─────────────────────────────

    pub fn get_check_items(&self, id: impl Into<TrelloID>) -> GetCheckItemsRequest<'a> {
        GetCheckItemsRequest {
            client: self.client,
            id: id.into(),
            filter: None,
        }
    }

    // ── POST /checklists/{id}/checkItems ────────────────────────────

    pub fn create_check_item(&self, id: impl Into<TrelloID>, name: &str) -> CreateCheckItemRequest<'a> {
        CreateCheckItemRequest {
            client: self.client,
            id: id.into(),
            name: name.to_string(),
            pos: None,
            checked: None,
        }
    }

    // ── GET /checklists/{id}/checkItems/{idCheckItem} ───────────────

    pub async fn get_check_item(&self, checklist_id: impl Into<TrelloID>, check_item_id: impl Into<TrelloID>) -> Result<CheckItem> {
        let checklist_id: TrelloID = checklist_id.into();
        let check_item_id: TrelloID = check_item_id.into();
        self.client
            .get(
                &format!("/checklists/{}/checkItems/{}", checklist_id.as_ref(), check_item_id.as_ref()),
                &[],
            )
            .await
    }

    // ── DELETE /checklists/{id}/checkItems/{idCheckItem} ────────────

    pub async fn delete_check_item(&self, checklist_id: impl Into<TrelloID>, check_item_id: impl Into<TrelloID>) -> Result<()> {
        let checklist_id: TrelloID = checklist_id.into();
        let check_item_id: TrelloID = check_item_id.into();
        self.client
            .delete_no_body(
                &format!("/checklists/{}/checkItems/{}", checklist_id.as_ref(), check_item_id.as_ref()),
                &[],
            )
            .await
    }

    // ── GET /checklists/{id}/{field} ────────────────────────────────

    pub async fn get_field(&self, id: impl Into<TrelloID>, field: &str) -> Result<serde_json::Value> {
        let id: TrelloID = id.into();
        self.client
            .get(&format!("/checklists/{}/{}", id.as_ref(), field), &[])
            .await
    }

    // ── PUT /checklists/{id}/{field} ────────────────────────────────

    pub async fn update_field(&self, id: impl Into<TrelloID>, field: &str, value: &str) -> Result<Checklist> {
        let id: TrelloID = id.into();
        self.client
            .put(&format!("/checklists/{}/{}", id.as_ref(), field), &[("value", value)], None::<&()>)
            .await
    }
}

// ──────────────────── Builder structs ────────────────────────────────

pub struct GetChecklistRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    cards: Option<String>,
    card_fields: Option<String>,
    check_items: Option<String>,
    check_item_fields: Option<String>,
    fields: Option<String>,
}

impl<'a> GetChecklistRequest<'a> {
    pub fn check_items(mut self, v: &str) -> Self { self.check_items = Some(v.to_string()); self }
    pub fn fields(mut self, v: &str) -> Self { self.fields = Some(v.to_string()); self }

    pub async fn send(self) -> Result<Checklist> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let cards_s; let cf_s; let ci_s; let cif_s; let fields_s;
        if let Some(ref v) = self.cards { cards_s = v.clone(); params.push(("cards", &cards_s)); }
        if let Some(ref v) = self.card_fields { cf_s = v.clone(); params.push(("card_fields", &cf_s)); }
        if let Some(ref v) = self.check_items { ci_s = v.clone(); params.push(("checkItems", &ci_s)); }
        if let Some(ref v) = self.check_item_fields { cif_s = v.clone(); params.push(("checkItem_fields", &cif_s)); }
        if let Some(ref v) = self.fields { fields_s = v.clone(); params.push(("fields", &fields_s)); }
        self.client.get(&format!("/checklists/{}", self.id.as_ref()), &params).await
    }
}

pub struct CreateChecklistRequest<'a> {
    client: &'a TrelloClient,
    name: Option<String>,
    id_board: Option<TrelloID>,
    id_card: Option<TrelloID>,
    id_checklist_source: Option<TrelloID>,
    pos: Option<String>,
}

impl<'a> CreateChecklistRequest<'a> {
    pub fn name(mut self, v: impl Into<String>) -> Self { self.name = Some(v.into()); self }
    pub fn id_board(mut self, v: impl Into<TrelloID>) -> Self { self.id_board = Some(v.into()); self }
    pub fn id_card(mut self, v: impl Into<TrelloID>) -> Self { self.id_card = Some(v.into()); self }
    pub fn id_checklist_source(mut self, v: impl Into<TrelloID>) -> Self { self.id_checklist_source = Some(v.into()); self }
    pub fn pos(mut self, v: &str) -> Self { self.pos = Some(v.to_string()); self }

    pub async fn send(self) -> Result<Checklist> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let name_s; let ib_s; let ic_s; let ics_s; let pos_s;
        if let Some(ref v) = self.name { name_s = v.clone(); params.push(("name", &name_s)); }
        if let Some(ref v) = self.id_board { ib_s = v.to_string(); params.push(("idBoard", &ib_s)); }
        if let Some(ref v) = self.id_card { ic_s = v.to_string(); params.push(("idCard", &ic_s)); }
        if let Some(ref v) = self.id_checklist_source { ics_s = v.to_string(); params.push(("idChecklistSource", &ics_s)); }
        if let Some(ref v) = self.pos { pos_s = v.clone(); params.push(("pos", &pos_s)); }
        self.client.post("/checklists", &params, None::<&()>).await
    }
}

pub struct UpdateChecklistRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    name: Option<String>,
    pos: Option<String>,
}

impl<'a> UpdateChecklistRequest<'a> {
    pub fn name(mut self, v: impl Into<String>) -> Self { self.name = Some(v.into()); self }
    pub fn pos(mut self, v: &str) -> Self { self.pos = Some(v.to_string()); self }

    pub async fn send(self) -> Result<Checklist> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let name_s; let pos_s;
        if let Some(ref v) = self.name { name_s = v.clone(); params.push(("name", &name_s)); }
        if let Some(ref v) = self.pos { pos_s = v.clone(); params.push(("pos", &pos_s)); }
        self.client
            .put(&format!("/checklists/{}", self.id.as_ref()), &params, None::<&()>)
            .await
    }
}

pub struct GetChecklistBoardRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    fields: Option<String>,
}

impl<'a> GetChecklistBoardRequest<'a> {
    pub fn fields(mut self, v: &str) -> Self { self.fields = Some(v.to_string()); self }
    pub async fn send(self) -> Result<Board> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let fields_s;
        if let Some(ref v) = self.fields { fields_s = v.clone(); params.push(("fields", &fields_s)); }
        self.client.get(&format!("/checklists/{}/board", self.id.as_ref()), &params).await
    }
}

pub struct GetCheckItemsRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    filter: Option<String>,
}

impl<'a> GetCheckItemsRequest<'a> {
    pub fn filter(mut self, v: &str) -> Self { self.filter = Some(v.to_string()); self }
    pub async fn send(self) -> Result<Vec<CheckItem>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let filter_s;
        if let Some(ref v) = self.filter { filter_s = v.clone(); params.push(("filter", &filter_s)); }
        self.client.get(&format!("/checklists/{}/checkItems", self.id.as_ref()), &params).await
    }
}

pub struct CreateCheckItemRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    name: String,
    pos: Option<String>,
    checked: Option<bool>,
}

impl<'a> CreateCheckItemRequest<'a> {
    pub fn pos(mut self, v: &str) -> Self { self.pos = Some(v.to_string()); self }
    pub fn checked(mut self, v: bool) -> Self { self.checked = Some(v); self }

    pub async fn send(self) -> Result<CheckItem> {
        let mut params = vec![("name", self.name.as_str())];
        let pos_s; let checked_s;
        if let Some(ref v) = self.pos { pos_s = v.clone(); params.push(("pos", &pos_s)); }
        if let Some(v) = self.checked { checked_s = if v { "true" } else { "false" }; params.push(("checked", checked_s)); }
        self.client
            .post(&format!("/checklists/{}/checkItems", self.id.as_ref()), &params, None::<&()>)
            .await
    }
}
