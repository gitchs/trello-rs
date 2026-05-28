use serde::de::DeserializeOwned;
use tracing::{debug, instrument};
use url::Url;

use crate::auth::{ApiKey, ApiToken};
use crate::error::{ApiErrorBody, Error, Result};
use crate::resources::{
    ActionsResource, ApplicationsResource, BatchResource, BoardsResource, CardsResource,
    ChecklistsResource, CustomFieldsResource, EmojiResource, EnterprisesResource, LabelsResource,
    ListsResource, MembersResource, NotificationsResource, OrganizationsResource, PluginsResource,
    SearchResource, TokensResource, WebhooksResource,
};

const DEFAULT_BASE_URL: &str = "https://api.trello.com/1/";

/// The main Trello API client.
///
/// Create an instance with [`TrelloClient::new`], then use the accessor methods
/// (e.g. `.boards()`, `.cards()`) to interact with the API.
pub struct TrelloClient {
    pub(crate) http: reqwest::Client,
    pub(crate) key: ApiKey,
    pub(crate) token: ApiToken,
    pub(crate) base_url: Url,
}

impl TrelloClient {
    pub fn new(key: ApiKey, token: ApiToken) -> Self {
        let base_url = Url::parse(DEFAULT_BASE_URL).expect("valid base URL");
        Self {
            http: reqwest::Client::new(),
            key,
            token,
            base_url,
        }
    }

    pub fn with_client(key: ApiKey, token: ApiToken, http: reqwest::Client) -> Self {
        let base_url = Url::parse(DEFAULT_BASE_URL).expect("valid base URL");
        Self {
            http,
            key,
            token,
            base_url,
        }
    }

    pub fn with_base_url(key: ApiKey, token: ApiToken, base_url: Url) -> Self {
        Self {
            http: reqwest::Client::new(),
            key,
            token,
            base_url,
        }
    }

    // ── Resource accessors ──────────────────────────────────────────

    pub fn actions(&self) -> ActionsResource<'_> {
        ActionsResource::new(self)
    }

    pub fn applications(&self) -> ApplicationsResource<'_> {
        ApplicationsResource::new(self)
    }

    pub fn batch(&self) -> BatchResource<'_> {
        BatchResource::new(self)
    }

    pub fn boards(&self) -> BoardsResource<'_> {
        BoardsResource::new(self)
    }

    pub fn cards(&self) -> CardsResource<'_> {
        CardsResource::new(self)
    }

    pub fn checklists(&self) -> ChecklistsResource<'_> {
        ChecklistsResource::new(self)
    }

    pub fn custom_fields(&self) -> CustomFieldsResource<'_> {
        CustomFieldsResource::new(self)
    }

    pub fn emoji(&self) -> EmojiResource<'_> {
        EmojiResource::new(self)
    }

    pub fn enterprises(&self) -> EnterprisesResource<'_> {
        EnterprisesResource::new(self)
    }

    pub fn labels(&self) -> LabelsResource<'_> {
        LabelsResource::new(self)
    }

    pub fn lists(&self) -> ListsResource<'_> {
        ListsResource::new(self)
    }

    pub fn members(&self) -> MembersResource<'_> {
        MembersResource::new(self)
    }

    pub fn notifications(&self) -> NotificationsResource<'_> {
        NotificationsResource::new(self)
    }

    pub fn organizations(&self) -> OrganizationsResource<'_> {
        OrganizationsResource::new(self)
    }

    pub fn plugins(&self) -> PluginsResource<'_> {
        PluginsResource::new(self)
    }

    pub fn search(&self) -> SearchResource<'_> {
        SearchResource::new(self)
    }

    pub fn tokens(&self) -> TokensResource<'_> {
        TokensResource::new(self)
    }

    pub fn webhooks(&self) -> WebhooksResource<'_> {
        WebhooksResource::new(self)
    }

    // ── Internal HTTP helpers ───────────────────────────────────────

    #[instrument(skip(self), fields(method = "GET", path = %path))]
    pub(crate) async fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<T> {
        debug!(?query, "sending request");
        let url = self.build_url(path, query)?;
        let resp = self.http.get(url).send().await?;
        debug!(status = %resp.status(), "received response");
        self.handle_response(resp).await
    }

    #[allow(dead_code)]
    pub(crate) async fn get_opt<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, Option<&str>)],
    ) -> Result<T> {
        let filtered: Vec<(&str, &str)> = query
            .iter()
            .filter_map(|(k, v)| v.map(|v| (*k, v)))
            .collect();
        let pairs: Vec<(&str, &str)> = filtered.iter().map(|(k, v)| (*k, *v)).collect();
        self.get(path, &pairs).await
    }

    #[instrument(skip(self, body), fields(method = "PUT", path = %path))]
    pub(crate) async fn put<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        query: &[(&str, &str)],
        body: Option<&B>,
    ) -> Result<T> {
        debug!(?query, "sending request");
        let url = self.build_url(path, query)?;
        let mut req = self.http.put(url);
        if let Some(b) = body {
            req = req.json(b);
        }
        let resp = req.send().await?;
        debug!(status = %resp.status(), "received response");
        self.handle_response(resp).await
    }

    #[instrument(skip(self, body), fields(method = "POST", path = %path))]
    pub(crate) async fn post<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        query: &[(&str, &str)],
        body: Option<&B>,
    ) -> Result<T> {
        debug!(?query, "sending request");
        let url = self.build_url(path, query)?;
        let mut req = self.http.post(url);
        if let Some(b) = body {
            req = req.json(b);
        }
        let resp = req.send().await?;
        debug!(status = %resp.status(), "received response");
        self.handle_response(resp).await
    }

    #[allow(dead_code)]
    #[instrument(skip(self), fields(method = "DELETE", path = %path))]
    pub(crate) async fn delete<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<T> {
        debug!(?query, "sending request");
        let url = self.build_url(path, query)?;
        let resp = self.http.delete(url).send().await?;
        debug!(status = %resp.status(), "received response");
        self.handle_response(resp).await
    }

    #[instrument(skip(self), fields(method = "DELETE", path = %path))]
    pub(crate) async fn delete_no_body(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<()> {
        debug!(?query, "sending request");
        let url = self.build_url(path, query)?;
        let resp = self.http.delete(url).send().await?;
        debug!(status = %resp.status(), "received response");
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(self.api_error(resp).await)
        }
    }

    #[instrument(skip(self, form), fields(method = "POST", path = %path))]
    pub(crate) async fn upload<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, &str)],
        form: reqwest::multipart::Form,
    ) -> Result<T> {
        debug!(?query, "sending multipart upload");
        let url = self.build_url(path, query)?;
        let resp = self
            .http
            .post(url)
            .multipart(form)
            .send()
            .await?;
        debug!(status = %resp.status(), "received response");
        self.handle_response(resp).await
    }

    // ── URL construction ────────────────────────────────────────────

    fn build_url(&self, path: &str, query: &[(&str, &str)]) -> Result<Url> {
        let path = path.strip_prefix('/').unwrap_or(path);
        let mut url = self.base_url.join(path)?;
        {
            let mut pairs = url.query_pairs_mut();
            pairs.append_pair("key", self.key.as_ref());
            pairs.append_pair("token", self.token.as_ref());
            for (k, v) in query {
                pairs.append_pair(k, v);
            }
        }
        Ok(url)
    }

    async fn handle_response<T: DeserializeOwned>(&self, resp: reqwest::Response) -> Result<T> {
        let status = resp.status();
        if status.is_success() {
            let bytes = resp.bytes().await?;
            debug!(body_len = bytes.len(), "reading response body");
            if bytes.is_empty() {
                debug!("response body is empty");
                return Err(Error::Other(
                    "API returned empty response body".into(),
                ));
            }
            serde_json::from_slice(&bytes).map_err(|e| {
                let preview = String::from_utf8_lossy(&bytes[..bytes.len().min(500)]);
                debug!(body = %preview, "deserialization failed");
                Error::SerdeBody {
                    error: e,
                    body_preview: preview.into_owned(),
                }
            })
        } else {
            Err(self.api_error(resp).await)
        }
    }

    async fn api_error(&self, resp: reqwest::Response) -> Error {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        debug!(%status, body = %body, "API error response");
        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Error::Api {
                status,
                message: "authentication failed – check your API key and token".into(),
            };
        }
        if let Ok(api_err) = serde_json::from_str::<ApiErrorBody>(&body) {
            let msg = api_err
                .message
                .or(api_err.error)
                .unwrap_or_else(|| format!("unknown API error: {body}"));
            Error::Api {
                status,
                message: msg,
            }
        } else if !body.is_empty() {
            Error::Api {
                status,
                message: body,
            }
        } else {
            Error::Api {
                status,
                message: status.to_string(),
            }
        }
    }
}
