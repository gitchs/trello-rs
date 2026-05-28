use crate::client::TrelloClient;
use crate::error::Result;
use crate::models::common::TrelloID;
use crate::models::plugin::{Plugin, PluginListing};

pub struct PluginsResource<'a> {
    client: &'a TrelloClient,
}

impl<'a> PluginsResource<'a> {
    pub(crate) fn new(client: &'a TrelloClient) -> Self {
        Self { client }
    }

    // ── GET /plugins/{id}/ ──────────────────────────────────────────

    pub async fn get(&self, id: impl Into<TrelloID>) -> Result<Plugin> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/plugins/{}/", id.as_ref()), &[]).await
    }

    // ── PUT /plugins/{id}/ ──────────────────────────────────────────

    pub fn update(&self, id: impl Into<TrelloID>) -> UpdatePluginRequest<'a> {
        UpdatePluginRequest {
            client: self.client,
            id: id.into(),
            name: None,
            locale: None,
            description: None,
            overview: None,
        }
    }

    // ── POST /plugins/{idPlugin}/listing ────────────────────────────

    pub fn create_listing(&self, plugin_id: impl Into<TrelloID>) -> CreateListingRequest<'a> {
        CreateListingRequest {
            client: self.client,
            plugin_id: plugin_id.into(),
            name: None,
            locale: None,
            description: None,
            overview: None,
        }
    }

    // ── PUT /plugins/{idPlugin}/listings/{idListing} ─────────────────

    pub fn update_listing(&self, plugin_id: impl Into<TrelloID>, listing_id: impl Into<TrelloID>) -> UpdateListingRequest<'a> {
        UpdateListingRequest {
            client: self.client,
            plugin_id: plugin_id.into(),
            listing_id: listing_id.into(),
            name: None,
            locale: None,
            description: None,
            overview: None,
        }
    }

    // ── GET /plugins/{id}/compliance/memberPrivacy ──────────────────

    pub async fn get_compliance(&self, id: impl Into<TrelloID>) -> Result<serde_json::Value> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/plugins/{}/compliance/memberPrivacy", id.as_ref()), &[]).await
    }
}

// ──────────────────── Builder structs ────────────────────────────────

pub struct UpdatePluginRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    name: Option<String>,
    locale: Option<String>,
    description: Option<String>,
    overview: Option<String>,
}

impl<'a> UpdatePluginRequest<'a> {
    pub fn name(mut self, v: impl Into<String>) -> Self { self.name = Some(v.into()); self }
    pub fn locale(mut self, v: impl Into<String>) -> Self { self.locale = Some(v.into()); self }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.description = Some(v.into()); self }
    pub fn overview(mut self, v: impl Into<String>) -> Self { self.overview = Some(v.into()); self }

    pub async fn send(self) -> Result<Plugin> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let name_s; let loc_s; let desc_s; let ov_s;
        if let Some(ref v) = self.name { name_s = v.clone(); params.push(("name", &name_s)); }
        if let Some(ref v) = self.locale { loc_s = v.clone(); params.push(("locale", &loc_s)); }
        if let Some(ref v) = self.description { desc_s = v.clone(); params.push(("description", &desc_s)); }
        if let Some(ref v) = self.overview { ov_s = v.clone(); params.push(("overview", &ov_s)); }
        self.client
            .put(&format!("/plugins/{}/", self.id.as_ref()), &params, None::<&()>)
            .await
    }
}

pub struct CreateListingRequest<'a> {
    client: &'a TrelloClient,
    plugin_id: TrelloID,
    name: Option<String>,
    locale: Option<String>,
    description: Option<String>,
    overview: Option<String>,
}

impl<'a> CreateListingRequest<'a> {
    pub fn name(mut self, v: impl Into<String>) -> Self { self.name = Some(v.into()); self }
    pub fn locale(mut self, v: impl Into<String>) -> Self { self.locale = Some(v.into()); self }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.description = Some(v.into()); self }
    pub fn overview(mut self, v: impl Into<String>) -> Self { self.overview = Some(v.into()); self }

    pub async fn send(self) -> Result<PluginListing> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let name_s; let loc_s; let desc_s; let ov_s;
        if let Some(ref v) = self.name { name_s = v.clone(); params.push(("name", &name_s)); }
        if let Some(ref v) = self.locale { loc_s = v.clone(); params.push(("locale", &loc_s)); }
        if let Some(ref v) = self.description { desc_s = v.clone(); params.push(("description", &desc_s)); }
        if let Some(ref v) = self.overview { ov_s = v.clone(); params.push(("overview", &ov_s)); }
        self.client
            .post(&format!("/plugins/{}/listing", self.plugin_id.as_ref()), &params, None::<&()>)
            .await
    }
}

pub struct UpdateListingRequest<'a> {
    client: &'a TrelloClient,
    plugin_id: TrelloID,
    listing_id: TrelloID,
    name: Option<String>,
    locale: Option<String>,
    description: Option<String>,
    overview: Option<String>,
}

impl<'a> UpdateListingRequest<'a> {
    pub fn name(mut self, v: impl Into<String>) -> Self { self.name = Some(v.into()); self }
    pub fn locale(mut self, v: impl Into<String>) -> Self { self.locale = Some(v.into()); self }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.description = Some(v.into()); self }
    pub fn overview(mut self, v: impl Into<String>) -> Self { self.overview = Some(v.into()); self }

    pub async fn send(self) -> Result<PluginListing> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let name_s; let loc_s; let desc_s; let ov_s;
        if let Some(ref v) = self.name { name_s = v.clone(); params.push(("name", &name_s)); }
        if let Some(ref v) = self.locale { loc_s = v.clone(); params.push(("locale", &loc_s)); }
        if let Some(ref v) = self.description { desc_s = v.clone(); params.push(("description", &desc_s)); }
        if let Some(ref v) = self.overview { ov_s = v.clone(); params.push(("overview", &ov_s)); }
        self.client
            .put(&format!("/plugins/{}/listings/{}", self.plugin_id.as_ref(), self.listing_id.as_ref()), &params, None::<&()>)
            .await
    }
}
