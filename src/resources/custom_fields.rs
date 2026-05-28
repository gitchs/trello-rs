use crate::client::TrelloClient;
use crate::error::Result;
use crate::models::common::TrelloID;
use crate::models::custom_field::{CustomField, CustomFieldOption};

pub struct CustomFieldsResource<'a> {
    client: &'a TrelloClient,
}

impl<'a> CustomFieldsResource<'a> {
    pub(crate) fn new(client: &'a TrelloClient) -> Self {
        Self { client }
    }

    // ── GET /customFields/{id} ──────────────────────────────────────

    pub async fn get(&self, id: impl Into<TrelloID>) -> Result<CustomField> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/customFields/{}", id.as_ref()), &[]).await
    }

    // ── POST /customFields ──────────────────────────────────────────

    pub fn create(&self) -> CreateCustomFieldRequest<'a> {
        CreateCustomFieldRequest {
            client: self.client,
            id_model: None,
            model_type: None,
            name: None,
            field_type: None,
            options: None,
            pos: None,
            display_card_front: None,
        }
    }

    // ── PUT /customFields/{id} ──────────────────────────────────────

    pub fn update(&self, id: impl Into<TrelloID>) -> UpdateCustomFieldRequest<'a> {
        UpdateCustomFieldRequest {
            client: self.client,
            id: id.into(),
            name: None,
            pos: None,
            display_card_front: None,
        }
    }

    // ── DELETE /customFields/{id} ───────────────────────────────────

    pub async fn delete(&self, id: impl Into<TrelloID>) -> Result<()> {
        let id: TrelloID = id.into();
        self.client
            .delete_no_body(&format!("/customFields/{}", id.as_ref()), &[])
            .await
    }

    // ── GET /customFields/{id}/options ──────────────────────────────

    pub async fn get_options(&self, id: impl Into<TrelloID>) -> Result<Vec<CustomFieldOption>> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/customFields/{}/options", id.as_ref()), &[]).await
    }

    // ── POST /customFields/{id}/options ─────────────────────────────

    pub fn create_option(&self, id: impl Into<TrelloID>) -> CreateCustomFieldOptionRequest<'a> {
        CreateCustomFieldOptionRequest {
            client: self.client,
            id: id.into(),
            value: None,
            color: None,
            pos: None,
        }
    }

    // ── DELETE /customFields/{id}/options/{idCustomFieldOption} ─────

    pub async fn delete_option(&self, field_id: impl Into<TrelloID>, option_id: impl Into<TrelloID>) -> Result<()> {
        let field_id: TrelloID = field_id.into();
        let option_id: TrelloID = option_id.into();
        self.client
            .delete_no_body(
                &format!("/customFields/{}/options/{}", field_id.as_ref(), option_id.as_ref()),
                &[],
            )
            .await
    }
}

// ──────────────────── Builder structs ────────────────────────────────

pub struct CreateCustomFieldRequest<'a> {
    client: &'a TrelloClient,
    id_model: Option<TrelloID>,
    model_type: Option<String>,
    name: Option<String>,
    field_type: Option<String>,
    options: Option<Vec<serde_json::Value>>,
    pos: Option<String>,
    display_card_front: Option<bool>,
}

impl<'a> CreateCustomFieldRequest<'a> {
    pub fn id_model(mut self, v: impl Into<TrelloID>) -> Self { self.id_model = Some(v.into()); self }
    pub fn model_type(mut self, v: &str) -> Self { self.model_type = Some(v.to_string()); self }
    pub fn name(mut self, v: impl Into<String>) -> Self { self.name = Some(v.into()); self }
    pub fn field_type(mut self, v: &str) -> Self { self.field_type = Some(v.to_string()); self }
    pub fn options(mut self, v: serde_json::Value) -> Self { self.options = Some(vec![v]); self }
    pub fn pos(mut self, v: &str) -> Self { self.pos = Some(v.to_string()); self }
    pub fn display_card_front(mut self, v: bool) -> Self { self.display_card_front = Some(v); self }

    pub async fn send(self) -> Result<CustomField> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let im_s; let mt_s; let name_s; let ft_s; let pos_s; let dcf_s; let opt_s;
        if let Some(ref v) = self.id_model { im_s = v.to_string(); params.push(("idModel", &im_s)); }
        if let Some(ref v) = self.model_type { mt_s = v.clone(); params.push(("modelType", &mt_s)); }
        if let Some(ref v) = self.name { name_s = v.clone(); params.push(("name", &name_s)); }
        if let Some(ref v) = self.field_type { ft_s = v.clone(); params.push(("type", &ft_s)); }
        if let Some(ref v) = self.options { opt_s = serde_json::to_string(v).unwrap(); params.push(("options", &opt_s)); }
        if let Some(ref v) = self.pos { pos_s = v.clone(); params.push(("pos", &pos_s)); }
        if let Some(v) = self.display_card_front { dcf_s = if v { "true" } else { "false" }; params.push(("display/cardFront", dcf_s)); }
        self.client.post("/customFields", &params, None::<&()>).await
    }
}

pub struct UpdateCustomFieldRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    name: Option<String>,
    pos: Option<String>,
    display_card_front: Option<bool>,
}

impl<'a> UpdateCustomFieldRequest<'a> {
    pub fn name(mut self, v: impl Into<String>) -> Self { self.name = Some(v.into()); self }
    pub fn pos(mut self, v: &str) -> Self { self.pos = Some(v.to_string()); self }
    pub fn display_card_front(mut self, v: bool) -> Self { self.display_card_front = Some(v); self }

    pub async fn send(self) -> Result<CustomField> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let name_s; let pos_s; let dcf_s;
        if let Some(ref v) = self.name { name_s = v.clone(); params.push(("name", &name_s)); }
        if let Some(ref v) = self.pos { pos_s = v.clone(); params.push(("pos", &pos_s)); }
        if let Some(v) = self.display_card_front { dcf_s = if v { "true" } else { "false" }; params.push(("display/cardFront", dcf_s)); }
        self.client
            .put(&format!("/customFields/{}", self.id.as_ref()), &params, None::<&()>)
            .await
    }
}

pub struct CreateCustomFieldOptionRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    value: Option<serde_json::Value>,
    color: Option<String>,
    pos: Option<String>,
}

impl<'a> CreateCustomFieldOptionRequest<'a> {
    pub fn value(mut self, v: serde_json::Value) -> Self { self.value = Some(v); self }
    pub fn color(mut self, v: &str) -> Self { self.color = Some(v.to_string()); self }
    pub fn pos(mut self, v: &str) -> Self { self.pos = Some(v.to_string()); self }

    pub async fn send(self) -> Result<CustomFieldOption> {
        let body = serde_json::json!({
            "value": self.value,
            "color": self.color,
            "pos": self.pos,
        });
        let body_val: serde_json::Value = body;
        self.client
            .post(&format!("/customFields/{}/options", self.id.as_ref()), &[], Some(&body_val))
            .await
    }
}
