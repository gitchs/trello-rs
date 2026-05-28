use crate::client::TrelloClient;
use crate::error::Result;
use crate::models::common::TrelloID;
use crate::models::member::Member;
use crate::models::token::Token;
use crate::models::webhook::Webhook;

pub struct TokensResource<'a> {
    client: &'a TrelloClient,
}

impl<'a> TokensResource<'a> {
    pub(crate) fn new(client: &'a TrelloClient) -> Self {
        Self { client }
    }

    // ── GET /tokens/{token} ─────────────────────────────────────────

    pub fn get(&self, token: impl Into<String>) -> GetTokenRequest<'a> {
        GetTokenRequest {
            client: self.client,
            token: token.into(),
            fields: None,
        }
    }

    // ── DELETE /tokens/{token} ──────────────────────────────────────

    pub async fn delete(&self, token: impl Into<String>) -> Result<()> {
        let token: String = token.into();
        self.client
            .delete_no_body(&format!("/tokens/{}", token.as_str()), &[])
            .await
    }

    // ── GET /tokens/{token}/member ──────────────────────────────────

    pub async fn get_member(&self, token: impl Into<String>) -> Result<Member> {
        let token: String = token.into();
        self.client.get(&format!("/tokens/{}/member", token), &[]).await
    }

    // ── GET /tokens/{token}/webhooks ────────────────────────────────

    pub async fn get_webhooks(&self, token: impl Into<String>) -> Result<Vec<Webhook>> {
        let token: String = token.into();
        self.client.get(&format!("/tokens/{}/webhooks", token), &[]).await
    }

    // ── POST /tokens/{token}/webhooks ───────────────────────────────

    pub fn create_webhook(&self, token: impl Into<String>) -> CreateTokenWebhookRequest<'a> {
        CreateTokenWebhookRequest {
            client: self.client,
            token: token.into(),
            callback_url: None,
            id_model: None,
            description: None,
        }
    }

    // ── GET /tokens/{token}/webhooks/{idWebhook} ────────────────────

    pub async fn get_webhook(&self, token: impl Into<String>, webhook_id: impl Into<TrelloID>) -> Result<Webhook> {
        let token: String = token.into();
        let webhook_id: TrelloID = webhook_id.into();
        self.client
            .get(
                &format!("/tokens/{}/webhooks/{}", token, webhook_id.as_ref()),
                &[],
            )
            .await
    }

    // ── DELETE /tokens/{token}/webhooks/{idWebhook} ─────────────────

    pub async fn delete_webhook(&self, token: impl Into<String>, webhook_id: impl Into<TrelloID>) -> Result<()> {
        let token: String = token.into();
        let webhook_id: TrelloID = webhook_id.into();
        self.client
            .delete_no_body(&format!("/tokens/{}/webhooks/{}", token, webhook_id.as_ref()), &[])
            .await
    }

    // ── PUT /tokens/{token}/webhooks/{idWebhook} ────────────────────

    pub fn update_webhook(&self, token: impl Into<String>, webhook_id: impl Into<TrelloID>) -> UpdateTokenWebhookRequest<'a> {
        UpdateTokenWebhookRequest {
            client: self.client,
            token: token.into(),
            webhook_id: webhook_id.into(),
            callback_url: None,
            id_model: None,
            description: None,
        }
    }
}

// ──────────────────── Builder structs ────────────────────────────────

pub struct GetTokenRequest<'a> {
    client: &'a TrelloClient,
    token: String,
    fields: Option<String>,
}

impl<'a> GetTokenRequest<'a> {
    pub fn fields(mut self, v: &str) -> Self { self.fields = Some(v.to_string()); self }
    pub async fn send(self) -> Result<Token> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let fields_s;
        if let Some(ref v) = self.fields { fields_s = v.clone(); params.push(("fields", &fields_s)); }
        self.client.get(&format!("/tokens/{}", self.token), &params).await
    }
}

pub struct CreateTokenWebhookRequest<'a> {
    client: &'a TrelloClient,
    token: String,
    callback_url: Option<String>,
    id_model: Option<TrelloID>,
    description: Option<String>,
}

impl<'a> CreateTokenWebhookRequest<'a> {
    pub fn callback_url(mut self, v: impl Into<String>) -> Self { self.callback_url = Some(v.into()); self }
    pub fn id_model(mut self, v: impl Into<TrelloID>) -> Self { self.id_model = Some(v.into()); self }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.description = Some(v.into()); self }

    pub async fn send(self) -> Result<Webhook> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let cb_s; let im_s; let desc_s;
        if let Some(ref v) = self.callback_url { cb_s = v.clone(); params.push(("callbackURL", &cb_s)); }
        if let Some(ref v) = self.id_model { im_s = v.to_string(); params.push(("idModel", &im_s)); }
        if let Some(ref v) = self.description { desc_s = v.clone(); params.push(("description", &desc_s)); }
        self.client
            .post(&format!("/tokens/{}/webhooks", self.token), &params, None::<&()>)
            .await
    }
}

pub struct UpdateTokenWebhookRequest<'a> {
    client: &'a TrelloClient,
    token: String,
    webhook_id: TrelloID,
    callback_url: Option<String>,
    id_model: Option<TrelloID>,
    description: Option<String>,
}

impl<'a> UpdateTokenWebhookRequest<'a> {
    pub fn callback_url(mut self, v: impl Into<String>) -> Self { self.callback_url = Some(v.into()); self }
    pub fn id_model(mut self, v: impl Into<TrelloID>) -> Self { self.id_model = Some(v.into()); self }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.description = Some(v.into()); self }

    pub async fn send(self) -> Result<Webhook> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let cb_s; let im_s; let desc_s;
        if let Some(ref v) = self.callback_url { cb_s = v.clone(); params.push(("callbackURL", &cb_s)); }
        if let Some(ref v) = self.id_model { im_s = v.to_string(); params.push(("idModel", &im_s)); }
        if let Some(ref v) = self.description { desc_s = v.clone(); params.push(("description", &desc_s)); }
        self.client
            .put(&format!("/tokens/{}/webhooks/{}", self.token, self.webhook_id.as_ref()), &params, None::<&()>)
            .await
    }
}
