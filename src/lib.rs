//! A type-safe, async Trello REST API client library for Rust.
//!
//! Built on `tokio` and `reqwest`, this library provides complete coverage
//! of the Trello API with ergonomic builder patterns for requests with many
//! optional parameters.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use trello_rs::{TrelloClient, ApiKey, ApiToken};
//! use trello_rs::models::board::BoardField;
//!
//! #[tokio::main]
//! async fn main() -> trello_rs::Result<()> {
//!     let client = TrelloClient::new(
//!         ApiKey::new("0471642aefef5fa1fa76530ce1ba4c85")?,
//!         ApiToken::new("9eb76d9a9d02b8dd40c2f3e5df18556c831d4d1fadbe2c45f8310e6c93b5c548")?,
//!     );
//!
//!     let board = client.boards()
//!         .get("5abbe4b7ddc1b351ef961414")
//!         .fields(&[BoardField::Name, BoardField::Desc])
//!         .send()
//!         .await?;
//!
//!     Ok(())
//! }
//! ```

pub mod auth;
pub mod cache;
pub mod cli;
pub mod client;
pub mod config;
pub mod error;
pub mod models;
pub mod params;
pub mod resources;

pub use auth::{ApiKey, ApiToken, AuthError};
pub use client::TrelloClient;
pub use config::Config;
pub use error::{Error, Result};
