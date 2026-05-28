use crate::client::TrelloClient;
use crate::error::Result;
use crate::models::emoji::Emoji;

pub struct EmojiResource<'a> {
    client: &'a TrelloClient,
}

impl<'a> EmojiResource<'a> {
    pub(crate) fn new(client: &'a TrelloClient) -> Self {
        Self { client }
    }

    /// Get all available emoji.
    pub async fn get(&self) -> Result<Emoji> {
        self.client.get("/emoji", &[]).await
    }
}
