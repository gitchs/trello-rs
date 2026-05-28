use crate::client::TrelloClient;
use crate::error::Result;

pub struct BatchResource<'a> {
    client: &'a TrelloClient,
}

impl<'a> BatchResource<'a> {
    pub(crate) fn new(client: &'a TrelloClient) -> Self {
        Self { client }
    }

    /// Execute multiple GET requests in a single batch call.
    ///
    /// `urls` should be relative paths (e.g., `["/boards/myBoardId", "/members/me"]`).
    pub async fn get(&self, urls: &[&str]) -> Result<Vec<serde_json::Value>> {
        let url_param = urls.join(",");
        self.client.get("/batch", &[("urls", &url_param)]).await
    }
}
