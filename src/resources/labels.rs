use crate::client::TrelloClient;
use crate::error::Result;
use crate::models::common::TrelloID;
use crate::models::label::Label;

pub struct LabelsResource<'a> {
    client: &'a TrelloClient,
}

impl<'a> LabelsResource<'a> {
    pub(crate) fn new(client: &'a TrelloClient) -> Self {
        Self { client }
    }

    // ── GET /labels/{id} ────────────────────────────────────────────

    pub fn get(&self, id: impl Into<TrelloID>) -> GetLabelRequest<'a> {
        GetLabelRequest {
            client: self.client,
            id: id.into(),
            fields: None,
        }
    }

    // ── POST /labels ────────────────────────────────────────────────

    pub fn create(&self) -> CreateLabelRequest<'a> {
        CreateLabelRequest {
            client: self.client,
            name: None,
            color: None,
            id_board: None,
        }
    }

    // ── PUT /labels/{id} ────────────────────────────────────────────

    pub fn update(&self, id: impl Into<TrelloID>) -> UpdateLabelRequest<'a> {
        UpdateLabelRequest {
            client: self.client,
            id: id.into(),
            name: None,
            color: None,
        }
    }

    // ── DELETE /labels/{id} ─────────────────────────────────────────

    pub async fn delete(&self, id: impl Into<TrelloID>) -> Result<()> {
        let id: TrelloID = id.into();
        self.client
            .delete_no_body(&format!("/labels/{}", id.as_ref()), &[])
            .await
    }

    // ── PUT /labels/{id}/{field} ────────────────────────────────────

    pub async fn update_field(&self, id: impl Into<TrelloID>, field: &str, value: &str) -> Result<Label> {
        let id: TrelloID = id.into();
        self.client
            .put(&format!("/labels/{}/{}", id.as_ref(), field), &[("value", value)], None::<&()>)
            .await
    }
}

// ──────────────────── Builder structs ────────────────────────────────

pub struct GetLabelRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    fields: Option<String>,
}

impl<'a> GetLabelRequest<'a> {
    pub fn fields(mut self, v: &str) -> Self { self.fields = Some(v.to_string()); self }
    pub async fn send(self) -> Result<Label> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let fields_s;
        if let Some(ref v) = self.fields { fields_s = v.clone(); params.push(("fields", &fields_s)); }
        self.client.get(&format!("/labels/{}", self.id.as_ref()), &params).await
    }
}

pub struct CreateLabelRequest<'a> {
    client: &'a TrelloClient,
    name: Option<String>,
    color: Option<String>,
    id_board: Option<TrelloID>,
}

impl<'a> CreateLabelRequest<'a> {
    pub fn name(mut self, v: impl Into<String>) -> Self { self.name = Some(v.into()); self }
    pub fn color(mut self, v: &str) -> Self { self.color = Some(v.to_string()); self }
    pub fn id_board(mut self, v: impl Into<TrelloID>) -> Self { self.id_board = Some(v.into()); self }

    pub async fn send(self) -> Result<Label> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let name_s; let color_s; let ib_s;
        if let Some(ref v) = self.name { name_s = v.clone(); params.push(("name", &name_s)); }
        if let Some(ref v) = self.color { color_s = v.clone(); params.push(("color", &color_s)); }
        if let Some(ref v) = self.id_board { ib_s = v.to_string(); params.push(("idBoard", &ib_s)); }
        self.client.post("/labels", &params, None::<&()>).await
    }
}

pub struct UpdateLabelRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    name: Option<String>,
    color: Option<String>,
}

impl<'a> UpdateLabelRequest<'a> {
    pub fn name(mut self, v: impl Into<String>) -> Self { self.name = Some(v.into()); self }
    pub fn color(mut self, v: &str) -> Self { self.color = Some(v.to_string()); self }

    pub async fn send(self) -> Result<Label> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let name_s; let color_s;
        if let Some(ref v) = self.name { name_s = v.clone(); params.push(("name", &name_s)); }
        if let Some(ref v) = self.color { color_s = v.clone(); params.push(("color", &color_s)); }
        self.client
            .put(&format!("/labels/{}", self.id.as_ref()), &params, None::<&()>)
            .await
    }
}
