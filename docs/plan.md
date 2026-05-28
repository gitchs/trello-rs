# Trello Client Library Implementation Plan

## Overview

Build a type-safe, async Trello REST API client library in Rust using `tokio` and `reqwest`. The API spec defines ~256 operations across 114 paths, organized into ~18 resource groups. The library will model all major resources, provide ergonomic builders for requests with many optional parameters, and support both query-parameter and JSON-body request styles.

---

## 1. Architecture

```
trello-rs/
  src/
    lib.rs              // crate root, re-exports
    client.rs           // TrelloClient struct, authentication, reqwest setup
    error.rs            // Error enum, Result alias
    auth.rs             // ApiKey, ApiToken types
    params.rs           // Shared param types (fields, filter, pagination, pos)

    models/
      mod.rs
      board.rs          // Board, BoardPrefs, BoardMembership, etc.
      card.rs           // Card, CardBadges, CardCover, etc.
      list.rs           // TrelloList
      member.rs         // Member, MemberPrefs
      organization.rs   // Organization, OrganizationPrefs
      action.rs         // Action, ActionData, ActionDisplay
      notification.rs   // Notification
      checklist.rs      // Checklist, CheckItem
      label.rs          // Label
      attachment.rs     // Attachment, ImageDescriptor
      custom_field.rs   // CustomField, CustomFieldItems
      webhook.rs        // Webhook
      token.rs          // Token, TokenPermission
      enterprise.rs     // Enterprise, EnterpriseAuditLog
      search.rs         // Search results
      common.rs         // TrelloID, LimitsObject, posStringOrNumber, etc.
      plugin.rs         // Plugin, PluginData
      tag.rs            // Tag
      emoji.rs          // Emoji, CustomEmoji, CustomSticker
      saved_search.rs   // SavedSearch
      board_star.rs     // BoardStars
      board_background.rs // BoardBackground, CustomBoardBackground

    resources/
      mod.rs
      boards.rs         // /boards/* endpoints
      cards.rs          // /cards/* endpoints
      lists.rs          // /lists/* endpoints
      members.rs        // /members/* endpoints
      organizations.rs  // /organizations/* endpoints
      actions.rs        // /actions/* endpoints
      notifications.rs  // /notifications/* endpoints
      checklists.rs     // /checklists/* endpoints
      labels.rs         // /labels/* endpoints
      custom_fields.rs  // /customFields/* endpoints
      webhooks.rs       // /webhooks/* endpoints
      tokens.rs         // /tokens/* endpoints
      enterprises.rs    // /enterprises/* endpoints
      search.rs         // /search/* endpoints
      batch.rs          // /batch endpoint
      emoji.rs          // /emoji endpoint
      applications.rs   // /applications endpoint
      plugins.rs        // /plugins/* endpoints
```

### Dependency graph
```
resources/ ──→ client.rs ──→ models/
     │              │
     └──────────────┼──→ error.rs
                    └──→ auth.rs
                    └──→ params.rs
```

---

## 2. Dependencies (Cargo.toml)

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", default-features = false, features = ["json", "multipart", "rustls-tls"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"
chrono = { version = "0.4", features = ["serde"] }
url = "2"

[dev-dependencies]
tokio-test = "0.4"
wiremock = "0.6"          # HTTP mocking
```

### Justification
- **reqwest** — native tokio HTTP client with JSON and multipart support
- **serde** — standard Rust serialization for JSON request/response
- **thiserror** — ergonomic error enum derivation
- **chrono** — datetime handling matching Trello's `date-time` format
- **url** — safe URL construction with query parameter encoding
- **wiremock** — test HTTP server for integration tests

---

## 3. Authentication

Two query-parameter-based API keys required on every request:

```rust
// auth.rs
pub struct ApiKey(String);    // 32-char hex, validated on construction
pub struct ApiToken(String);  // 64-char hex

impl TrelloClient {
    pub fn new(key: ApiKey, token: ApiToken) -> Self { ... }
}
```

Credentials are automatically appended to every request's query string. The base URL is `https://api.trello.com/1`.

---

## 4. Core Client

```rust
// client.rs
pub struct TrelloClient {
    http: reqwest::Client,
    key: ApiKey,
    token: ApiToken,
    base_url: url::Url,  // https://api.trello.com/1
}
```

Internal helper methods:
- `get<T>(path, query_params) -> Result<T>` — GET with query params
- `put<T>(path, query_params, body) -> Result<T>` — PUT
- `post<T>(path, query_params, body) -> Result<T>` — POST
- `delete<T>(path, query_params) -> Result<T>` — DELETE
- `upload<T>(path, form) -> Result<T>` — multipart file upload

Each method:
1. Builds the URL from `base_url` + path
2. Appends `key` and `token` to query params
3. Serializes query params into the URL
4. Sends the request, deserializes JSON response
5. Maps HTTP errors to the library's `Error` type

Serialization convention: the Swagger spec uses query parameters for most inputs. The client will use `#[serde(rename_all = "camelCase")]` throughout, with manual mapping for the few snake_case exceptions.

---

## 5. Error Handling

```rust
// error.rs
#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error {code}: {message}")]
    Api { code: u16, message: String },

    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;
```

API errors (4xx/5xx JSON bodies with `code` + `message`) are deserialized into `Error::Api`.

---

## 6. Models Design

### Principles
- One struct per OpenAPI schema, with `#[serde(rename_all = "camelCase")]`
- All fields `Option<T>` unless the spec guarantees presence (rare — Trello returns sparse objects)
- Strongly-typed enums for codes/colors/filters, not strings
- `TrelloID` as a newtype around `String` with 24-char hex validation
- `posStringOrNumber` as an enum:

```rust
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Pos {
    String(PosString),
    Number(f64),
}

pub enum PosString {
    Top,
    Bottom,
}
```

### Field selection enums
Each resource type gets a corresponding `*Field` enum, mirroring the `*Fields` enums in the spec (e.g., `BoardField`, `CardField`, `MemberField`). These control which fields the API returns, enabling bandwidth optimization.

```rust
pub enum BoardField {
    Id, Name, Desc, Closed, IdOrganization, Pinned, Url, ShortUrl, Prefs, ...
}

pub enum ViewFilter {
    All, Closed, None, Open,
}
```

These implement a `FieldQuery` trait for serialization to comma-separated query strings.

### Common structs
- `TrelloID` — newtype `String` (24-char hex)
- `LimitsObject` — nested `{ status, disableAt, warnAt }` dicts
- `ImageDescriptor` — `{ url, width, height }`
- `Membership` — `{ id, idMember, memberType, unconfirmed, deactivated }`

---

## 7. Request Builder Pattern

Many endpoints accept 10-20+ optional query parameters. To avoid monstrous function signatures, each resource method uses the builder pattern:

```rust
// Before (bad):
client.get_cards(board_id, filter, fields, limit, before, since, ...)

// After (good):
client.cards()
    .get_cards_on_board(board_id)
    .filter(ViewFilter::Open)
    .fields(&[CardField::Name, CardField::Desc])
    .limit(50)
    .send()
    .await
```

Implementation pattern:
```rust
pub struct GetCardsOnBoardRequest<'a> {
    client: &'a TrelloClient,
    board_id: TrelloID,
    filter: Option<ViewFilter>,
    fields: Option<Vec<CardField>>,
    limit: Option<u32>,
    before: Option<String>,
    since: Option<String>,
    // ... nested field controls
}

impl<'a> GetCardsOnBoardRequest<'a> {
    pub fn filter(mut self, f: ViewFilter) -> Self { self.filter = Some(f); self }
    pub fn fields(mut self, f: &[CardField]) -> Self { self.fields = Some(f.to_vec()); self }
    pub fn limit(mut self, n: u32) -> Self { self.limit = Some(n); self }
    pub async fn send(self) -> Result<Vec<Card>> { ... }
}
```

### Resource modules as accessor objects
Each resource group (boards, cards, etc.) is accessed through a method on `TrelloClient` that returns a lightweight handle:

```rust
impl TrelloClient {
    pub fn boards(&self) -> BoardsResource<'_> { BoardsResource { client: self } }
    pub fn cards(&self) -> CardsResource<'_> { CardsResource { client: self } }
    pub fn lists(&self) -> ListsResource<'_> { ListsResource { client: self } }
    // ...
}
```

---

## 8. Pagination Support

Three pagination strategies exist in the API. The library provides an abstraction over all three:

### Page-based (actions, notifications)
```rust
pub struct PageParams {
    pub limit: Option<u32>,    // 1-1000
    pub page: Option<u32>,
    pub before: Option<String>, // ISO date or TrelloID
    pub since: Option<String>,  // ISO date
}
```

### Cursor-based (enterprise members)
```rust
pub struct CursorParams {
    pub start_index: Option<String>,  // cursor from previous response
    pub count: Option<u32>,           // max 100
}
```

### Convenience: Auto-pagination via `Stream`
Where applicable, provide a method returning a `futures::Stream` that automatically fetches pages:
```rust
pub fn all_actions(&self, resource_id: &TrelloID) -> impl Stream<Item = Result<Action>> { ... }
```
(Requires `futures` or `futures-core` as optional dependency behind a feature flag `stream`.)

---

## 9. File Uploads

Upload endpoints (card attachments, member avatar, board backgrounds, custom emoji/stickers, organization logo) use `reqwest::multipart::Form`:

```rust
// In client.rs
pub async fn upload<T: DeserializeOwned>(
    &self,
    path: &str,
    file: Vec<u8>,
    filename: &str,
    mime: &str,
    extra_params: &[(&str, &str)],
) -> Result<T> {
    let part = reqwest::multipart::Part::bytes(file)
        .file_name(filename.to_string())
        .mime_str(mime)?;
    let mut form = reqwest::multipart::Form::new()
        .part("file", part);
    for (k, v) in extra_params {
        form = form.text(k.to_string(), v.to_string());
    }
    // ...
}
```

---

## 10. Testing Strategy

### Unit tests
- **Model serialization**: round-trip serde tests with JSON fixtures captured from the API
- **Field enum formatting**: verify `BoardField::Name` serializes to `"name"`, join to `"name,desc"`
- **Auth types**: verify ApiKey rejects invalid hex strings
- **Builder correctness**: verify query params are constructed correctly

### Integration tests
- Use `wiremock` to simulate Trello API responses
- Each resource module gets test coverage for:
  - Successful GET/POST/PUT/DELETE
  - 404 handling
  - 401 handling
  - Parameter encoding (special characters, empty values)
  - Pagination (first page, last page, empty page)

### Test fixtures
Store sample JSON responses in `tests/fixtures/` organized by resource.

---

## 11. Implementation Phases

### Phase 1: Foundation (must ship first)
1. Project setup: `lib.rs`, `Cargo.toml` dependencies
2. `auth.rs` — ApiKey, ApiToken types with validation
3. `error.rs` — Error enum and Result alias
4. `client.rs` — TrelloClient with get/put/post/delete helpers
5. `params.rs` — shared types (ViewFilter, field selection, Pos, pagination params)
6. `models/common.rs` — TrelloID, LimitsObject, Membership, ImageDescriptor, posStringOrNumber
7. Builder pattern infrastructure (generic `RequestBuilder` trait or macro)

### Phase 2: Core Resources
8. `models/board.rs` + `resources/boards.rs` — Board CRUD, preferences, power-ups, memberships
9. `models/card.rs` + `resources/cards.rs` — Card CRUD, attachments, stickers, custom fields
10. `models/list.rs` + `resources/lists.rs` — List CRUD, archive/move, card nesting
11. `models/member.rs` + `resources/members.rs` — Profile, boards, organizations, notifications

### Phase 3: Extended Resources
12. `models/organization.rs` + `resources/organizations.rs`
13. `models/action.rs` + `resources/actions.rs`
14. `models/checklist.rs` + `resources/checklists.rs`
15. `models/label.rs` + `resources/labels.rs`
16. `models/custom_field.rs` + `resources/custom_fields.rs`

### Phase 4: Specialized Resources
17. `models/notification.rs` + `resources/notifications.rs`
18. `models/webhook.rs` + `resources/webhooks.rs`
19. `models/token.rs` + `resources/tokens.rs`
20. `models/enterprise.rs` + `resources/enterprises.rs`
21. `models/search.rs` + `resources/search.rs`

### Phase 5: Utilities & Polish
22. `resources/batch.rs` — batch GET endpoint
23. `resources/emoji.rs` — emoji listings
24. `resources/plugins.rs` — plugin management
25. File upload support (multipart helpers + attachment/avatar/background endpoints)
26. Auto-pagination streams (behind feature flag)
27. Comprehensive integration tests with wiremock

---

## 12. Open Questions / Design Decisions

| Decision | Options | Recommendation |
|---|---|---|
| Field enums vs `&str` slices | Type-safe enums vs flexible strings | **Type-safe enums** — the API spec has stable field names; enums prevent typos |
| Builder methods return `&mut Self` or `Self` | Chained `Self` (consuming) or `&mut Self` (mutable ref) | **Consuming `Self`** — builder is single-use, prevents reuse after send |
| `TrelloID` newtype vs `String` | Newtype with validation or plain String | **Newtype** — 24-char hex invariant, impl `From<String>` for flexibility |
| Optional params as `Option<Vec<T>>` vs empty vec default | `Option` for distinguish "not set" vs "empty" | **`Option`** — "not set" (don't send param) differs from "send empty list" |
| `chrono` vs `time` crate | Two major Rust datetime libraries | **`chrono`** — wider ecosystem support, built-in serde feature |
| Feature flags for resource groups | Gate enterprise/batch behind features | **No feature flags initially** — keep simple, add if compile times become an issue |
| JSON error type | `anyhow` vs `thiserror` | **`thiserror`** — library code should expose structured errors |

---

## 13. Example Usage (Target API)

```rust
use trello_rs::{TrelloClient, ApiKey, ApiToken};
use trello_rs::models::{board::{Board, BoardField}, card::ViewFilter};

#[tokio::main]
async fn main() -> trello_rs::Result<()> {
    let client = TrelloClient::new(
        ApiKey::new("0471642aefef5fa1fa76530ce1ba4c85")?,
        ApiToken::new("9eb76d9a9d02b8dd40c2f3e5df18556c831d4d1fadbe2c45f8310e6c93b5c548")?,
    );

    // Get a board
    let board = client.boards()
        .get("5abbe4b7ddc1b351ef961414")
        .fields(&[BoardField::Name, BoardField::Desc, BoardField::Url])
        .send()
        .await?;

    // List open cards
    let cards = client.cards()
        .get_cards_on_board("5abbe4b7ddc1b351ef961414")
        .filter(ViewFilter::Open)
        .send()
        .await?;

    // Create a card with attachment
    client.cards()
        .create()
        .name("New task")
        .id_list("5abbe4b7ddc1b351ef961415")
        .desc("Task description")
        .send()
        .await?;

    Ok(())
}
```
