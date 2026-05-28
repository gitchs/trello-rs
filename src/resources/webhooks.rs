use crate::client::TrelloClient;
use crate::error::Result;
use crate::models::common::TrelloID;
use crate::models::webhook::Webhook;

pub struct WebhooksResource<'a> {
    client: &'a TrelloClient,
}

impl<'a> WebhooksResource<'a> {
    pub(crate) fn new(client: &'a TrelloClient) -> Self {
        Self { client }
    }

    // ── GET /webhooks/{id} ──────────────────────────────────────────

    pub async fn get(&self, id: impl Into<TrelloID>) -> Result<Webhook> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/webhooks/{}", id.as_ref()), &[]).await
    }

    // ── POST /webhooks/ ─────────────────────────────────────────────

    pub fn create(&self) -> CreateWebhookRequest<'a> {
        CreateWebhookRequest {
            client: self.client,
            callback_url: None,
            id_model: None,
            description: None,
            active: None,
        }
    }

    // ── PUT /webhooks/{id} ──────────────────────────────────────────

    pub fn update(&self, id: impl Into<TrelloID>) -> UpdateWebhookRequest<'a> {
        UpdateWebhookRequest {
            client: self.client,
            id: id.into(),
            callback_url: None,
            id_model: None,
            description: None,
            active: None,
        }
    }

    // ── DELETE /webhooks/{id} ───────────────────────────────────────

    pub async fn delete(&self, id: impl Into<TrelloID>) -> Result<()> {
        let id: TrelloID = id.into();
        self.client
            .delete_no_body(&format!("/webhooks/{}", id.as_ref()), &[])
            .await
    }

    // ── GET /webhooks/{id}/{field} ──────────────────────────────────

    pub async fn get_field(&self, id: impl Into<TrelloID>, field: &str) -> Result<serde_json::Value> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/webhooks/{}/{}", id.as_ref(), field), &[]).await
    }
}

// ──────────────────── Builder structs ────────────────────────────────

pub struct CreateWebhookRequest<'a> {
    client: &'a TrelloClient,
    callback_url: Option<String>,
    id_model: Option<TrelloID>,
    description: Option<String>,
    active: Option<bool>,
}

impl<'a> CreateWebhookRequest<'a> {
    pub fn callback_url(mut self, v: impl Into<String>) -> Self { self.callback_url = Some(v.into()); self }
    pub fn id_model(mut self, v: impl Into<TrelloID>) -> Self { self.id_model = Some(v.into()); self }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.description = Some(v.into()); self }
    pub fn active(mut self, v: bool) -> Self { self.active = Some(v); self }

    pub async fn send(self) -> Result<Webhook> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let cb_s; let im_s; let desc_s; let active_s;
        if let Some(ref v) = self.callback_url { cb_s = v.clone(); params.push(("callbackURL", &cb_s)); }
        if let Some(ref v) = self.id_model { im_s = v.to_string(); params.push(("idModel", &im_s)); }
        if let Some(ref v) = self.description { desc_s = v.clone(); params.push(("description", &desc_s)); }
        if let Some(v) = self.active { active_s = if v { "true" } else { "false" }; params.push(("active", active_s)); }
        self.client.post("/webhooks/", &params, None::<&()>).await
    }
}

pub struct UpdateWebhookRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    callback_url: Option<String>,
    id_model: Option<TrelloID>,
    description: Option<String>,
    active: Option<bool>,
}

impl<'a> UpdateWebhookRequest<'a> {
    pub fn callback_url(mut self, v: impl Into<String>) -> Self { self.callback_url = Some(v.into()); self }
    pub fn id_model(mut self, v: impl Into<TrelloID>) -> Self { self.id_model = Some(v.into()); self }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.description = Some(v.into()); self }
    pub fn active(mut self, v: bool) -> Self { self.active = Some(v); self }

    pub async fn send(self) -> Result<Webhook> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let cb_s; let im_s; let desc_s; let active_s;
        if let Some(ref v) = self.callback_url { cb_s = v.clone(); params.push(("callbackURL", &cb_s)); }
        if let Some(ref v) = self.id_model { im_s = v.to_string(); params.push(("idModel", &im_s)); }
        if let Some(ref v) = self.description { desc_s = v.clone(); params.push(("description", &desc_s)); }
        if let Some(v) = self.active { active_s = if v { "true" } else { "false" }; params.push(("active", active_s)); }
        self.client
            .put(&format!("/webhooks/{}", self.id.as_ref()), &params, None::<&()>)
            .await
    }
}
