use clap::Parser;
use serde::Serialize;
use std::process;
use tracing::Level;
use tracing_subscriber::EnvFilter;

use trello_rs::cli::*;
use trello_rs::client::TrelloClient;
use trello_rs::config::Config;
use trello_rs::{ApiKey, ApiToken};

fn print_json(value: &impl Serialize) {
    println!("{}", serde_json::to_string_pretty(value).unwrap());
}

fn fail(msg: impl std::fmt::Display) -> ! {
    eprintln!("error: {msg}");
    process::exit(1);
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if cli.debug {
        tracing_subscriber::fmt()
            .with_env_filter(
                EnvFilter::builder()
                    .with_default_directive(Level::DEBUG.into())
                    .from_env_lossy(),
            )
            .with_writer(std::io::stderr)
            .init();
    }

    let config = Config::load(&cli.config).unwrap_or_else(|e| {
        fail(format!("loading config from {}: {e}", cli.config));
    });

    let key = ApiKey::new(&config.api_key).unwrap_or_else(|e| fail(e));
    let token = ApiToken::new(&config.api_token).unwrap_or_else(|e| fail(e));
    let client = TrelloClient::new(key, token);

    if let Err(e) = dispatch(&client, &config.api_token, cli.command).await {
        fail(e);
    }
}

async fn dispatch(
    client: &TrelloClient,
    api_token: &str,
    cmd: Commands,
) -> trello_rs::error::Result<()> {
    match cmd {
        Commands::Board { cmd } => handle_board(client, cmd).await,
        Commands::Card { cmd } => handle_card(client, cmd).await,
        Commands::List { cmd } => handle_list(client, cmd).await,
        Commands::Label { cmd } => handle_label(client, cmd).await,
        Commands::Checklist { cmd } => handle_checklist(client, cmd).await,
        Commands::Member { cmd } => handle_member(client, cmd).await,
        Commands::Search(args) => handle_search(client, args).await,
        Commands::Webhook { cmd } => handle_webhook(client, api_token, cmd).await,
        Commands::Organization { cmd } => handle_organization(client, cmd).await,
        Commands::Action { cmd } => handle_action(client, cmd).await,
        Commands::Notification { cmd } => handle_notification(client, cmd).await,
        Commands::CustomField { cmd } => handle_custom_field(client, cmd).await,
        Commands::Enterprise { cmd } => handle_enterprise(client, cmd).await,
        Commands::Emoji => handle_emoji(client).await,
        Commands::Token { cmd } => handle_token(client, api_token, cmd).await,
        Commands::Plugin { cmd } => handle_plugin(client, cmd).await,
        Commands::Batch { urls } => handle_batch(client, &urls).await,
    }
}

// ── Board handlers ────────────────────────────────────────────────────

async fn handle_board(client: &TrelloClient, cmd: BoardCmd) -> trello_rs::error::Result<()> {
    match cmd {
        BoardCmd::Get { id } => {
            let board = client.boards().get(id.as_str()).send().await?;
            print_json(&board);
        }
        BoardCmd::List { member_id, filter } => {
            let mut req = client.members().get_boards(member_id.as_str());
            if let Some(ref f) = filter {
                req = req.filter(f);
            }
            let boards = req.send().await?;
            print_json(&boards);
        }
        BoardCmd::Create { name, desc, id_organization, id_board_source, prefs_permission_level } => {
            let mut req = client.boards().create().name(&name);
            if let Some(ref v) = desc { req = req.desc(v); }
            if let Some(ref v) = id_organization { req = req.id_organization(v.as_str()); }
            if let Some(ref v) = id_board_source { req = req.id_board_source(v.as_str()); }
            if let Some(ref v) = prefs_permission_level { req = req.prefs_permission_level(v); }
            let board = req.send().await?;
            print_json(&board);
        }
        BoardCmd::Update { id, name, desc, closed, subscribed, id_organization } => {
            let mut req = client.boards().update(id.as_str());
            if let Some(ref v) = name { req = req.name(v); }
            if let Some(ref v) = desc { req = req.desc(v); }
            if let Some(v) = closed { req = req.closed(v); }
            if let Some(v) = subscribed { req = req.subscribed(v); }
            if let Some(ref v) = id_organization { req = req.id_organization(v.as_str()); }
            let board = req.send().await?;
            print_json(&board);
        }
        BoardCmd::Delete { id } => {
            client.boards().delete(id.as_str()).await?;
            println!("Board deleted.");
        }
    }
    Ok(())
}

// ── Card handlers ─────────────────────────────────────────────────────

async fn handle_card(client: &TrelloClient, cmd: CardCmd) -> trello_rs::error::Result<()> {
    match cmd {
        CardCmd::Get { id } => {
            let card = client.cards().get(id.as_str()).send().await?;
            print_json(&card);
        }
        CardCmd::List { list_id, board_id } => {
            let cards = match (list_id, board_id) {
                (Some(lid), _) => client.lists().get_cards(lid.as_str()).await?,
                (_, Some(bid)) => client.boards().get_cards(bid.as_str()).await?,
                _ => fail("card list requires --list-id or --board-id"),
            };
            print_json(&cards);
        }
        CardCmd::Create { name, list_id, desc, pos, due } => {
            let mut req = client.cards().create().name(&name);
            if let Some(ref v) = list_id { req = req.id_list(v.as_str()); }
            if let Some(ref v) = desc { req = req.desc(v); }
            if let Some(ref v) = pos { req = req.pos(v); }
            if let Some(ref v) = due { req = req.due(v); }
            let card = req.send().await?;
            print_json(&card);
        }
        CardCmd::Update { id, name, desc, closed, id_list } => {
            let mut req = client.cards().update(id.as_str());
            if let Some(ref v) = name { req = req.name(v); }
            if let Some(ref v) = desc { req = req.desc(v); }
            if let Some(v) = closed { req = req.closed(v); }
            if let Some(ref v) = id_list { req = req.id_list(v.as_str()); }
            let card = req.send().await?;
            print_json(&card);
        }
        CardCmd::Delete { id } => {
            client.cards().delete(id.as_str()).await?;
            println!("Card deleted.");
        }
    }
    Ok(())
}

// ── List handlers ─────────────────────────────────────────────────────

async fn handle_list(client: &TrelloClient, cmd: ListCmd) -> trello_rs::error::Result<()> {
    match cmd {
        ListCmd::Get { id } => {
            let list = client.lists().get(id.as_str()).send().await?;
            print_json(&list);
        }
        ListCmd::List { board_id } => {
            let lists = client.boards().get_lists(board_id.as_str()).send().await?;
            print_json(&lists);
        }
        ListCmd::Create { name, board_id, pos } => {
            let mut req = client.lists().create().name(&name).id_board(board_id.as_str());
            if let Some(ref v) = pos { req = req.pos(v); }
            let list = req.send().await?;
            print_json(&list);
        }
        ListCmd::Update { id, name, closed, id_board, pos } => {
            let mut req = client.lists().update(id.as_str());
            if let Some(ref v) = name { req = req.name(v); }
            if let Some(v) = closed { req = req.closed(v); }
            if let Some(ref v) = id_board { req = req.id_board(v.as_str()); }
            if let Some(ref v) = pos { req = req.pos(v); }
            let list = req.send().await?;
            print_json(&list);
        }
        ListCmd::Close { id } => {
            client.lists().close(id.as_str(), true).await?;
            println!("List archived.");
        }
    }
    Ok(())
}

// ── Label handlers ────────────────────────────────────────────────────

async fn handle_label(client: &TrelloClient, cmd: LabelCmd) -> trello_rs::error::Result<()> {
    match cmd {
        LabelCmd::Get { id } => {
            let label = client.labels().get(id.as_str()).send().await?;
            print_json(&label);
        }
        LabelCmd::List { board_id } => {
            let labels = client.boards().get_labels(board_id.as_str()).send().await?;
            print_json(&labels);
        }
        LabelCmd::Create { name, color, board_id } => {
            let label = client
                .labels()
                .create()
                .name(&name)
                .color(&color)
                .id_board(board_id.as_str())
                .send()
                .await?;
            print_json(&label);
        }
        LabelCmd::Update { id, name, color } => {
            let mut req = client.labels().update(id.as_str());
            if let Some(ref v) = name { req = req.name(v); }
            if let Some(ref v) = color { req = req.color(v); }
            let label = req.send().await?;
            print_json(&label);
        }
        LabelCmd::Delete { id } => {
            client.labels().delete(id.as_str()).await?;
            println!("Label deleted.");
        }
    }
    Ok(())
}

// ── Checklist handlers ────────────────────────────────────────────────

async fn handle_checklist(client: &TrelloClient, cmd: ChecklistCmd) -> trello_rs::error::Result<()> {
    match cmd {
        ChecklistCmd::Get { id } => {
            let checklist = client.checklists().get(id.as_str()).send().await?;
            print_json(&checklist);
        }
        ChecklistCmd::Create { name, card_id } => {
            let checklist = client
                .checklists()
                .create()
                .name(&name)
                .id_card(card_id.as_str())
                .send()
                .await?;
            print_json(&checklist);
        }
        ChecklistCmd::Update { id, name } => {
            let mut req = client.checklists().update(id.as_str());
            if let Some(ref v) = name { req = req.name(v); }
            let checklist = req.send().await?;
            print_json(&checklist);
        }
        ChecklistCmd::Delete { id } => {
            client.checklists().delete(id.as_str()).await?;
            println!("Checklist deleted.");
        }
    }
    Ok(())
}

// ── Member handlers ───────────────────────────────────────────────────

async fn handle_member(client: &TrelloClient, cmd: MemberCmd) -> trello_rs::error::Result<()> {
    match cmd {
        MemberCmd::Get { id } => {
            let member = client.members().get(id.as_str()).send().await?;
            print_json(&member);
        }
        MemberCmd::Boards { member_id, filter } => {
            let mut req = client.members().get_boards(member_id.as_str());
            if let Some(ref f) = filter {
                req = req.filter(f);
            }
            let boards = req.send().await?;
            print_json(&boards);
        }
        MemberCmd::Update { id, full_name, username, bio, initials } => {
            let mut req = client.members().update(id.as_str());
            if let Some(ref v) = full_name { req = req.full_name(v); }
            if let Some(ref v) = username { req = req.username(v); }
            if let Some(ref v) = bio { req = req.bio(v); }
            if let Some(ref v) = initials { req = req.initials(v); }
            let member = req.send().await?;
            print_json(&member);
        }
    }
    Ok(())
}

// ── Search handlers ───────────────────────────────────────────────────

async fn handle_search(client: &TrelloClient, args: SearchArgs) -> trello_rs::error::Result<()> {
    if args.members {
        let mut req = client.search().search_members(&args.query);
        if let Some(ref v) = args.id_boards { req = req.id_board(v); }
        if let Some(ref v) = args.id_organizations { req = req.id_organization(v); }
        if let Some(v) = args.limit { req = req.limit(v); }
        let results = req.send().await?;
        print_json(&results);
    } else {
        let mut req = client.search().search(&args.query);
        if let Some(ref v) = args.model_types { req = req.model_types(v); }
        if let Some(ref v) = args.id_boards { req = req.id_boards(v); }
        if let Some(ref v) = args.id_organizations { req = req.id_organizations(v); }
        let results = req.send().await?;
        print_json(&results);
    }
    Ok(())
}

// ── Webhook handlers ──────────────────────────────────────────────────

async fn handle_webhook(
    client: &TrelloClient,
    api_token: &str,
    cmd: WebhookCmd,
) -> trello_rs::error::Result<()> {
    match cmd {
        WebhookCmd::Get { id } => {
            let webhook = client.webhooks().get(id.as_str()).await?;
            print_json(&webhook);
        }
        WebhookCmd::List => {
            let webhooks = client.tokens().get_webhooks(api_token).await?;
            print_json(&webhooks);
        }
        WebhookCmd::Create { callback_url, id_model, description } => {
            let mut req = client
                .webhooks()
                .create()
                .callback_url(&callback_url)
                .id_model(id_model.as_str());
            if let Some(ref v) = description { req = req.description(v); }
            let webhook = req.send().await?;
            print_json(&webhook);
        }
        WebhookCmd::Update { id, callback_url, id_model, description } => {
            let mut req = client.webhooks().update(id.as_str());
            if let Some(ref v) = callback_url { req = req.callback_url(v); }
            if let Some(ref v) = id_model { req = req.id_model(v.as_str()); }
            if let Some(ref v) = description { req = req.description(v); }
            let webhook = req.send().await?;
            print_json(&webhook);
        }
        WebhookCmd::Delete { id } => {
            client.webhooks().delete(id.as_str()).await?;
            println!("Webhook deleted.");
        }
    }
    Ok(())
}

// ── Organization handlers ─────────────────────────────────────────────

async fn handle_organization(client: &TrelloClient, cmd: OrganizationCmd) -> trello_rs::error::Result<()> {
    match cmd {
        OrganizationCmd::Get { id } => {
            let org = client.organizations().get(id.as_str()).send().await?;
            print_json(&org);
        }
        OrganizationCmd::Create { display_name, name, desc, website } => {
            let mut req = client.organizations().create().display_name(&display_name);
            if let Some(ref v) = name { req = req.name(v); }
            if let Some(ref v) = desc { req = req.desc(v); }
            if let Some(ref v) = website { req = req.website(v); }
            let org = req.send().await?;
            print_json(&org);
        }
        OrganizationCmd::Update { id, display_name, desc, website } => {
            let mut req = client.organizations().update(id.as_str());
            if let Some(ref v) = display_name { req = req.display_name(v); }
            if let Some(ref v) = desc { req = req.desc(v); }
            if let Some(ref v) = website { req = req.website(v); }
            let org = req.send().await?;
            print_json(&org);
        }
        OrganizationCmd::Delete { id } => {
            client.organizations().delete(id.as_str()).await?;
            println!("Organization deleted.");
        }
    }
    Ok(())
}

// ── Action handlers ───────────────────────────────────────────────────

async fn handle_action(client: &TrelloClient, cmd: ActionCmd) -> trello_rs::error::Result<()> {
    match cmd {
        ActionCmd::Get { id } => {
            let action = client.actions().get(id.as_str()).send().await?;
            print_json(&action);
        }
        ActionCmd::ListBoard { board_id, filter } => {
            let mut req = client.boards().get_actions(board_id.as_str());
            if let Some(ref v) = filter { req = req.filter(v); }
            let actions = req.send().await?;
            print_json(&actions);
        }
        ActionCmd::ListCard { card_id, filter } => {
            let mut req = client.cards().get_actions(card_id.as_str());
            if let Some(ref v) = filter { req = req.filter(v); }
            let actions = req.send().await?;
            print_json(&actions);
        }
    }
    Ok(())
}

// ── Notification handlers ─────────────────────────────────────────────

async fn handle_notification(client: &TrelloClient, cmd: NotificationCmd) -> trello_rs::error::Result<()> {
    match cmd {
        NotificationCmd::Get { id } => {
            let notification = client.notifications().get(id.as_str()).send().await?;
            print_json(&notification);
        }
    }
    Ok(())
}

// ── Custom field handlers ─────────────────────────────────────────────

async fn handle_custom_field(client: &TrelloClient, cmd: CustomFieldCmd) -> trello_rs::error::Result<()> {
    match cmd {
        CustomFieldCmd::Get { id } => {
            let field = client.custom_fields().get(id.as_str()).await?;
            print_json(&field);
        }
    }
    Ok(())
}

// ── Enterprise handlers ───────────────────────────────────────────────

async fn handle_enterprise(client: &TrelloClient, cmd: EnterpriseCmd) -> trello_rs::error::Result<()> {
    match cmd {
        EnterpriseCmd::Get { id } => {
            let enterprise = client.enterprises().get(id.as_str()).send().await?;
            print_json(&enterprise);
        }
    }
    Ok(())
}

// ── Emoji handler ─────────────────────────────────────────────────────

async fn handle_emoji(client: &TrelloClient) -> trello_rs::error::Result<()> {
    let emoji = client.emoji().get().await?;
    print_json(&emoji);
    Ok(())
}

// ── Token handlers ────────────────────────────────────────────────────

async fn handle_token(
    client: &TrelloClient,
    api_token: &str,
    cmd: TokenCmd,
) -> trello_rs::error::Result<()> {
    match cmd {
        TokenCmd::Get { id } => {
            let token_value = if id == "self" { api_token } else { &id };
            let token = client.tokens().get(token_value).send().await?;
            print_json(&token);
        }
    }
    Ok(())
}

// ── Plugin handlers ───────────────────────────────────────────────────

async fn handle_plugin(client: &TrelloClient, cmd: PluginCmd) -> trello_rs::error::Result<()> {
    match cmd {
        PluginCmd::Get { id } => {
            let plugin = client.plugins().get(id.as_str()).await?;
            print_json(&plugin);
        }
    }
    Ok(())
}

// ── Batch handler ─────────────────────────────────────────────────────

async fn handle_batch(client: &TrelloClient, urls: &str) -> trello_rs::error::Result<()> {
    let urls: Vec<&str> = urls.split(',').map(|s| s.trim()).collect();
    let results = client.batch().get(&urls).await?;
    print_json(&results);
    Ok(())
}
