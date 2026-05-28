use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "trello", about = "Trello CLI client", version)]
pub struct Cli {
    /// Path to config file
    #[arg(long, default_value_t = crate::config::default_config_path())]
    pub config: String,

    /// Enable debug logging (dump request/response details)
    #[arg(long, global = true)]
    pub debug: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage boards
    Board {
        #[command(subcommand)]
        cmd: BoardCmd,
    },
    /// Manage cards
    Card {
        #[command(subcommand)]
        cmd: CardCmd,
    },
    /// Manage lists
    List {
        #[command(subcommand)]
        cmd: ListCmd,
    },
    /// Manage labels
    Label {
        #[command(subcommand)]
        cmd: LabelCmd,
    },
    /// Manage checklists
    Checklist {
        #[command(subcommand)]
        cmd: ChecklistCmd,
    },
    /// Manage members
    Member {
        #[command(subcommand)]
        cmd: MemberCmd,
    },
    /// Search Trello
    Search(SearchArgs),
    /// Manage webhooks
    Webhook {
        #[command(subcommand)]
        cmd: WebhookCmd,
    },
    /// Manage organizations
    Organization {
        #[command(subcommand)]
        cmd: OrganizationCmd,
    },
    /// Manage actions
    #[command(name = "action")]
    Action {
        #[command(subcommand)]
        cmd: ActionCmd,
    },
    /// Manage notifications
    #[command(name = "notification")]
    Notification {
        #[command(subcommand)]
        cmd: NotificationCmd,
    },
    /// Manage custom fields
    #[command(name = "custom-field")]
    CustomField {
        #[command(subcommand)]
        cmd: CustomFieldCmd,
    },
    /// Manage enterprises
    Enterprise {
        #[command(subcommand)]
        cmd: EnterpriseCmd,
    },
    /// List available emoji
    Emoji,
    /// Manage tokens
    Token {
        #[command(subcommand)]
        cmd: TokenCmd,
    },
    /// Get plugin info
    #[command(name = "plugin")]
    Plugin {
        #[command(subcommand)]
        cmd: PluginCmd,
    },
    /// Send batch requests
    Batch {
        /// Comma-separated list of URLs to batch (e.g. "/boards/123,/boards/456")
        urls: String,
    },
    /// Manage the local cache
    Cache {
        #[command(subcommand)]
        cmd: CacheCmd,
    },
}

// ── Board ──────────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum BoardCmd {
    /// Get a board by ID
    Get {
        /// Board ID
        id: String,
    },
    /// List boards for a member
    List {
        /// Member ID (default: "me")
        #[arg(long, default_value = "me")]
        member_id: String,
        /// Filter: all, open, closed, etc.
        #[arg(long)]
        filter: Option<String>,
    },
    /// Create a new board
    Create {
        /// Board name
        #[arg(long, short)]
        name: String,
        /// Board description
        #[arg(long)]
        desc: Option<String>,
        /// Organization ID
        #[arg(long)]
        id_organization: Option<String>,
        /// Source board ID to copy from
        #[arg(long)]
        id_board_source: Option<String>,
        /// Permission level: private, org, public
        #[arg(long)]
        prefs_permission_level: Option<String>,
    },
    /// Update a board
    Update {
        /// Board ID
        id: String,
        /// New board name
        #[arg(long)]
        name: Option<String>,
        /// New board description
        #[arg(long)]
        desc: Option<String>,
        /// Close the board
        #[arg(long)]
        closed: Option<bool>,
        /// Subscribe to the board
        #[arg(long)]
        subscribed: Option<bool>,
        /// Organization ID
        #[arg(long)]
        id_organization: Option<String>,
    },
    /// Delete a board
    Delete {
        /// Board ID
        id: String,
    },
}

// ── Card ───────────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum CardCmd {
    /// Get a card by ID
    Get {
        /// Card ID
        id: String,
    },
    /// List cards in a list or on a board
    List {
        /// List ID (mutually exclusive with --board-id)
        #[arg(long)]
        list_id: Option<String>,
        /// Board ID (mutually exclusive with --list-id)
        #[arg(long)]
        board_id: Option<String>,
        /// Resolve board and list information for each card, using local cache
        #[arg(long)]
        full: bool,
    },
    /// Create a new card
    Create {
        /// Card name
        #[arg(long, short)]
        name: String,
        /// List ID
        #[arg(long)]
        list_id: Option<String>,
        /// Card description
        #[arg(long)]
        desc: Option<String>,
        /// Position: top, bottom, or numeric
        #[arg(long)]
        pos: Option<String>,
        /// Due date
        #[arg(long)]
        due: Option<String>,
        /// Label names to attach (auto-creates on the board if not found)
        #[arg(long = "label")]
        labels: Vec<String>,
    },
    /// Update a card
    Update {
        /// Card ID
        id: String,
        /// New card name
        #[arg(long)]
        name: Option<String>,
        /// New description
        #[arg(long)]
        desc: Option<String>,
        /// Close/archive the card
        #[arg(long)]
        closed: Option<bool>,
        /// Move to a different list
        #[arg(long)]
        id_list: Option<String>,
    },
    /// Delete a card
    Delete {
        /// Card ID
        id: String,
    },
    /// Manage comments on a card
    Comment {
        #[command(subcommand)]
        cmd: CardCommentCmd,
    },
    /// Manage labels on a card
    Label {
        #[command(subcommand)]
        cmd: CardLabelCmd,
    },
}

// ── Card Comment ─────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum CardCommentCmd {
    /// List comments on a card
    List {
        /// Card ID
        #[arg(long)]
        card_id: String,
    },
    /// Add a comment to a card
    Add {
        /// Card ID
        #[arg(long)]
        card_id: String,
        /// Comment text
        #[arg(long, short)]
        text: String,
    },
    /// Update a comment on a card
    Update {
        /// Card ID
        #[arg(long)]
        card_id: String,
        /// Comment action ID
        #[arg(long)]
        action_id: String,
        /// New comment text
        #[arg(long, short)]
        text: String,
    },
    /// Delete a comment from a card
    Delete {
        /// Card ID
        #[arg(long)]
        card_id: String,
        /// Comment action ID
        #[arg(long)]
        action_id: String,
    },
}

// ── Card Label ───────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum CardLabelCmd {
    /// Add a label to a card
    Add {
        /// Card ID
        #[arg(long)]
        card_id: String,
        /// Label ID
        #[arg(long)]
        label_id: String,
    },
    /// Remove a label from a card
    Remove {
        /// Card ID
        #[arg(long)]
        card_id: String,
        /// Label ID
        #[arg(long)]
        label_id: String,
    },
}

// ── List ───────────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum ListCmd {
    /// Get a list by ID
    Get {
        /// List ID
        id: String,
    },
    /// List lists on a board
    List {
        /// Board ID (falls back to config default_board_id)
        #[arg(long)]
        board_id: Option<String>,
    },
    /// Create a new list
    Create {
        /// List name
        #[arg(long, short)]
        name: String,
        /// Board ID (falls back to config default_board_id)
        #[arg(long)]
        board_id: Option<String>,
        /// Position: top, bottom, or numeric
        #[arg(long)]
        pos: Option<String>,
    },
    /// Update a list
    Update {
        /// List ID
        id: String,
        /// New list name
        #[arg(long)]
        name: Option<String>,
        /// Close/archive
        #[arg(long)]
        closed: Option<bool>,
        /// Move to different board
        #[arg(long)]
        id_board: Option<String>,
        /// Position
        #[arg(long)]
        pos: Option<String>,
    },
    /// Archive/close a list
    Close {
        /// List ID
        id: String,
    },
}

// ── Label ──────────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum LabelCmd {
    /// Get a label by ID
    Get {
        /// Label ID
        id: String,
    },
    /// List labels on a board
    List {
        /// Board ID (falls back to config default_board_id)
        #[arg(long)]
        board_id: Option<String>,
    },
    /// Create a new label
    Create {
        /// Label name
        #[arg(long, short)]
        name: String,
        /// Color: yellow, purple, blue, red, green, orange, black, sky, pink, lime
        #[arg(long)]
        color: String,
        /// Board ID (falls back to config default_board_id)
        #[arg(long)]
        board_id: Option<String>,
    },
    /// Update a label
    Update {
        /// Label ID
        id: String,
        /// New label name
        #[arg(long)]
        name: Option<String>,
        /// New color
        #[arg(long)]
        color: Option<String>,
    },
    /// Delete a label
    Delete {
        /// Label ID
        id: String,
    },
}

// ── Checklist ──────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum ChecklistCmd {
    /// Get a checklist by ID
    Get {
        /// Checklist ID
        id: String,
    },
    /// Create a new checklist
    Create {
        /// Checklist name
        #[arg(long, short)]
        name: String,
        /// Card ID
        #[arg(long)]
        card_id: String,
    },
    /// Update a checklist
    Update {
        /// Checklist ID
        id: String,
        /// New checklist name
        #[arg(long)]
        name: Option<String>,
    },
    /// Delete a checklist
    Delete {
        /// Checklist ID
        id: String,
    },
}

// ── Member ─────────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum MemberCmd {
    /// Get a member by ID (use "me" for current user)
    Get {
        /// Member ID or "me"
        #[arg(default_value = "me")]
        id: String,
    },
    /// List boards for a member
    Boards {
        /// Member ID (default: "me")
        #[arg(long, default_value = "me")]
        member_id: String,
        /// Filter: all, open, closed, etc.
        #[arg(long)]
        filter: Option<String>,
    },
    /// Update a member
    Update {
        /// Member ID
        id: String,
        /// New full name
        #[arg(long)]
        full_name: Option<String>,
        /// New username
        #[arg(long)]
        username: Option<String>,
        /// New bio
        #[arg(long)]
        bio: Option<String>,
        /// New initials
        #[arg(long)]
        initials: Option<String>,
    },
}

// ── Search ─────────────────────────────────────────────────────────────

#[derive(Parser)]
pub struct SearchArgs {
    /// Search query
    pub query: String,
    /// Comma-separated model types (boards, cards, members, organizations)
    #[arg(long)]
    pub model_types: Option<String>,
    /// Comma-separated board IDs to limit search
    #[arg(long)]
    pub id_boards: Option<String>,
    /// Comma-separated organization IDs to limit search
    #[arg(long)]
    pub id_organizations: Option<String>,
    /// Limit results
    #[arg(long)]
    pub limit: Option<u32>,
    /// Search members only
    #[arg(long)]
    pub members: bool,
}

// ── Webhook ────────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum WebhookCmd {
    /// Get a webhook by ID
    Get {
        /// Webhook ID
        id: String,
    },
    /// List webhooks for a token
    List,
    /// Create a new webhook (requires a callback URL reachable by Trello)
    Create {
        /// Callback URL
        #[arg(long)]
        callback_url: String,
        /// Model ID to watch (board, list, card, etc.)
        #[arg(long)]
        id_model: String,
        /// Description
        #[arg(long)]
        description: Option<String>,
    },
    /// Update a webhook
    Update {
        /// Webhook ID
        id: String,
        /// New callback URL
        #[arg(long)]
        callback_url: Option<String>,
        /// New model ID
        #[arg(long)]
        id_model: Option<String>,
        /// New description
        #[arg(long)]
        description: Option<String>,
    },
    /// Delete a webhook
    Delete {
        /// Webhook ID
        id: String,
    },
}

// ── Organization ───────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum OrganizationCmd {
    /// Get an organization by ID
    Get {
        /// Organization ID
        id: String,
    },
    /// Create a new organization
    Create {
        /// Organization display name
        #[arg(long, short)]
        display_name: String,
        /// Organization name (lowercase, no spaces)
        #[arg(long)]
        name: Option<String>,
        /// Description
        #[arg(long)]
        desc: Option<String>,
        /// Website URL
        #[arg(long)]
        website: Option<String>,
    },
    /// Update an organization
    Update {
        /// Organization ID
        id: String,
        /// New display name
        #[arg(long)]
        display_name: Option<String>,
        /// New description
        #[arg(long)]
        desc: Option<String>,
        /// New website
        #[arg(long)]
        website: Option<String>,
    },
    /// Delete an organization
    Delete {
        /// Organization ID
        id: String,
    },
}

// ── Action ─────────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum ActionCmd {
    /// Get an action by ID
    Get {
        /// Action ID
        id: String,
    },
    /// List actions on a board
    #[command(name = "board")]
    ListBoard {
        /// Board ID (falls back to config default_board_id)
        #[arg(long)]
        board_id: Option<String>,
        /// Filter type
        #[arg(long)]
        filter: Option<String>,
    },
    /// List actions on a card
    #[command(name = "card")]
    ListCard {
        /// Card ID
        #[arg(long)]
        card_id: String,
        /// Filter type
        #[arg(long)]
        filter: Option<String>,
    },
}

// ── Notification ───────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum NotificationCmd {
    /// Get a notification by ID
    Get {
        /// Notification ID
        id: String,
    },
}

// ── Custom Field ───────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum CustomFieldCmd {
    /// Get a custom field by ID
    Get {
        /// Custom field ID
        id: String,
    },
}

// ── Enterprise ─────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum EnterpriseCmd {
    /// Get an enterprise by ID
    Get {
        /// Enterprise ID
        id: String,
    },
}

// ── Token ──────────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum TokenCmd {
    /// Get token info
    Get {
        /// Token value (default: use configured token)
        #[arg(default_value = "self")]
        id: String,
    },
}

// ── Plugin ─────────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum PluginCmd {
    /// Get a plugin by ID
    Get {
        /// Plugin ID
        id: String,
    },
}

// ── Cache ──────────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum CacheCmd {
    /// Clear the local RocksDB cache
    Refresh,
}
