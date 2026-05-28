use serde::{Deserialize, Serialize};

/// Position value for ordering items. Can be "top", "bottom", or a numeric position.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Pos {
    String(PosString),
    Number(f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PosString {
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "bottom")]
    Bottom,
}

/// Filter boards/lists/cards by their open/closed status.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ViewFilter {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "open")]
    Open,
}

/// Card aging style.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CardAging {
    #[serde(rename = "pirate")]
    Pirate,
    #[serde(rename = "regular")]
    Regular,
}

/// Label / attachment edge color.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Color {
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "purple")]
    Purple,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "orange")]
    Orange,
    #[serde(rename = "black")]
    Black,
    #[serde(rename = "sky")]
    Sky,
    #[serde(rename = "pink")]
    Pink,
    #[serde(rename = "lime")]
    Lime,
}

/// Trait for types that can be serialized as comma-separated query values.
pub trait FieldQuery {
    fn as_field_str(&self) -> &'static str;
}

pub fn fields_to_query<T: FieldQuery>(fields: &[T]) -> String {
    fields
        .iter()
        .map(|f| f.as_field_str())
        .collect::<Vec<_>>()
        .join(",")
}

/// Pagination parameters for page-based endpoints.
#[derive(Debug, Clone, Default)]
pub struct PageParams {
    pub limit: Option<u32>,
    pub page: Option<u32>,
    pub before: Option<String>,
    pub since: Option<String>,
}

/// Pagination parameters for cursor-based endpoints.
#[derive(Debug, Clone, Default)]
pub struct CursorParams {
    pub start_index: Option<String>,
    pub count: Option<u32>,
}
