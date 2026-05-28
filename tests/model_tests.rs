use trello_rs::models::board::{Board, BoardField};
use trello_rs::models::card::{Card, CardField};
use trello_rs::models::common::TrelloID;
use trello_rs::models::label::Label;
use trello_rs::models::list::TrelloList;
use trello_rs::models::member::Member;
use trello_rs::models::webhook::Webhook;
use trello_rs::params::{Color, FieldQuery, ViewFilter};

#[test]
fn test_trello_id_serde() {
    let id = TrelloID::new("5abbe4b7ddc1b351ef961414");
    let json = serde_json::to_string(&id).unwrap();
    assert_eq!(json, r#""5abbe4b7ddc1b351ef961414""#);
    let decoded: TrelloID = serde_json::from_str(&json).unwrap();
    assert_eq!(id, decoded);
}

#[test]
fn test_board_deserialize() {
    let json = r#"{
        "id": "5abbe4b7ddc1b351ef961414",
        "name": "Test Board",
        "desc": "A test board",
        "closed": false,
        "url": "https://trello.com/b/test",
        "shortUrl": "https://trello.com/b/test",
        "starred": false
    }"#;
    let board: Board = serde_json::from_str(json).unwrap();
    assert_eq!(board.name.as_deref(), Some("Test Board"));
    assert_eq!(board.desc.as_deref(), Some("A test board"));
    assert_eq!(board.closed, Some(false));
}

#[test]
fn test_card_deserialize() {
    let json = r#"{
        "id": "5abbe4b7ddc1b351ef961415",
        "name": "Test Card",
        "desc": "Card description",
        "closed": false,
        "idList": "5abbe4b7ddc1b351ef961416",
        "idBoard": "5abbe4b7ddc1b351ef961414",
        "pos": 65535.0,
        "shortLink": "abc123",
        "shortUrl": "https://trello.com/c/abc123",
        "url": "https://trello.com/c/abc123/test-card"
    }"#;
    let card: Card = serde_json::from_str(json).unwrap();
    assert_eq!(card.name.as_deref(), Some("Test Card"));
    assert_eq!(card.pos, Some(65535.0));
}

#[test]
fn test_card_roundtrip() {
    let card = Card {
        id: Some(TrelloID::new("5abbe4b7ddc1b351ef961415")),
        name: Some("Test Card".to_string()),
        desc: Some("Description".to_string()),
        closed: Some(false),
        id_list: Some(TrelloID::new("5abbe4b7ddc1b351ef961416")),
        id_board: Some(TrelloID::new("5abbe4b7ddc1b351ef961414")),
        pos: Some(65535.0),
        short_link: Some("abc123".to_string()),
        short_url: Some("https://trello.com/c/abc123".to_string()),
        url: Some("https://trello.com/c/abc123/test-card".to_string()),
        address: None,
        badges: None,
        card_role: None,
        check_item_states: None,
        coordinates: None,
        creation_method: None,
        date_last_activity: None,
        desc_data: None,
        due: None,
        due_reminder: None,
        id_checklists: None,
        id_labels: None,
        id_members: None,
        id_members_voted: None,
        id_short: None,
        id_attachment_cover: None,
        labels: None,
        limits: None,
        location_name: None,
        manual_cover_attachment: None,
        mirror_source_id: None,
        subscribed: None,
        cover: None,
    };
    let json = serde_json::to_string(&card).unwrap();
    let decoded: Card = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.name.as_deref(), Some("Test Card"));
}

#[test]
fn test_list_deserialize() {
    let json = r#"{
        "id": "5abbe4b7ddc1b351ef961416",
        "name": "To Do",
        "closed": false,
        "pos": 16384.0,
        "idBoard": "5abbe4b7ddc1b351ef961414",
        "subscribed": false
    }"#;
    let list: TrelloList = serde_json::from_str(json).unwrap();
    assert_eq!(list.name.as_deref(), Some("To Do"));
    assert_eq!(list.pos, Some(16384.0));
}

#[test]
fn test_label_deserialize() {
    let json = r#"{
        "id": "5abbe4b7ddc1b351ef961417",
        "idBoard": "5abbe4b7ddc1b351ef961414",
        "name": "Bug",
        "color": "red"
    }"#;
    let label: Label = serde_json::from_str(json).unwrap();
    assert_eq!(label.name.as_deref(), Some("Bug"));
    assert!(matches!(label.color, Some(Color::Red)));
}

#[test]
fn test_member_deserialize() {
    let json = r#"{
        "id": "5abbe4b7ddc1b351ef961418",
        "username": "testuser",
        "fullName": "Test User",
        "initials": "TU",
        "confirmed": true
    }"#;
    let member: Member = serde_json::from_str(json).unwrap();
    assert_eq!(member.username.as_deref(), Some("testuser"));
    assert_eq!(member.full_name.as_deref(), Some("Test User"));
}

#[test]
fn test_webhook_deserialize() {
    let json = r#"{
        "id": "5abbe4b7ddc1b351ef961419",
        "description": "Test webhook",
        "idModel": "5abbe4b7ddc1b351ef961414",
        "callbackURL": "https://example.com/webhook",
        "active": true,
        "consecutiveFailures": 0
    }"#;
    let webhook: Webhook = serde_json::from_str(json).unwrap();
    assert_eq!(webhook.description.as_deref(), Some("Test webhook"));
    assert_eq!(webhook.active, Some(true));
}

#[test]
fn test_board_field_serialization() {
    let fields = vec![BoardField::Name, BoardField::Desc, BoardField::Url];
    let result = trello_rs::params::fields_to_query(&fields);
    assert_eq!(result, "name,desc,url");
}

#[test]
fn test_card_field_serialization() {
    let fields = vec![CardField::Name, CardField::Desc, CardField::Due, CardField::IdList];
    let result = trello_rs::params::fields_to_query(&fields);
    assert_eq!(result, "name,desc,due,idList");
}

#[test]
fn test_board_field_as_str() {
    assert_eq!(BoardField::Name.as_field_str(), "name");
    assert_eq!(BoardField::Desc.as_field_str(), "desc");
    assert_eq!(BoardField::IdOrganization.as_field_str(), "idOrganization");
    assert_eq!(BoardField::EnterpriseOwned.as_field_str(), "enterpriseOwned");
}

#[test]
fn test_view_filter_serialization() {
    assert_eq!(serde_json::to_string(&ViewFilter::All).unwrap(), r#""all""#);
    assert_eq!(serde_json::to_string(&ViewFilter::Open).unwrap(), r#""open""#);
    assert_eq!(serde_json::to_string(&ViewFilter::Closed).unwrap(), r#""closed""#);
}

#[test]
fn test_color_serialization() {
    assert_eq!(serde_json::to_string(&Color::Red).unwrap(), r#""red""#);
    assert_eq!(serde_json::to_string(&Color::Blue).unwrap(), r#""blue""#);
    assert_eq!(serde_json::to_string(&Color::Sky).unwrap(), r#""sky""#);
}

#[test]
fn test_board_with_prefs_deserialize() {
    let json = r#"{
        "id": "5abbe4b7ddc1b351ef961414",
        "name": "Test",
        "prefs": {
            "permissionLevel": "org",
            "cardCovers": true,
            "isTemplate": false,
            "canBePublic": true,
            "canBePrivate": true,
            "canInvite": true
        }
    }"#;
    let board: Board = serde_json::from_str(json).unwrap();
    let prefs = board.prefs.as_ref().unwrap();
    assert_eq!(prefs.permission_level.as_deref(), Some("org"));
    assert_eq!(prefs.card_covers, Some(true));
}

#[test]
fn test_board_empty() {
    let json = r#"{"id": "abc123", "name": "Empty"}"#;
    let board: Board = serde_json::from_str(json).unwrap();
    assert_eq!(board.name.as_deref(), Some("Empty"));
    assert_eq!(board.desc, None);
    assert_eq!(board.closed, None);
}
