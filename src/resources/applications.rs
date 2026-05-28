use crate::client::TrelloClient;
use crate::error::Result;

pub struct ApplicationsResource<'a> {
    client: &'a TrelloClient,
}

impl<'a> ApplicationsResource<'a> {
    pub(crate) fn new(client: &'a TrelloClient) -> Self {
        Self { client }
    }

    /// Get compliance data for an application.
    pub async fn get_compliance(&self, key: &str) -> Result<serde_json::Value> {
        self.client.get(&format!("/applications/{}/compliance", key), &[]).await
    }
}
