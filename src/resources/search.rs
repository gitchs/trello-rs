use crate::client::TrelloClient;
use crate::error::Result;
use crate::models::member::Member;
use crate::models::search::SearchResults;

pub struct SearchResource<'a> {
    client: &'a TrelloClient,
}

impl<'a> SearchResource<'a> {
    pub(crate) fn new(client: &'a TrelloClient) -> Self {
        Self { client }
    }

    // ── GET /search ─────────────────────────────────────────────────

    pub fn search(&self, query: &str) -> SearchRequest<'a> {
        SearchRequest {
            client: self.client,
            query: query.to_string(),
            id_boards: None,
            id_organizations: None,
            id_cards: None,
            model_types: None,
            board_fields: None,
            board_organization: None,
            card_board: None,
            card_list: None,
            card_members: None,
            card_stickers: None,
            card_attachments: None,
            organization_fields: None,
            member_fields: None,
            partial: None,
        }
    }

    // ── GET /search/members/ ────────────────────────────────────────

    pub fn search_members(&self, query: &str) -> SearchMembersRequest<'a> {
        SearchMembersRequest {
            client: self.client,
            query: query.to_string(),
            limit: None,
            id_board: None,
            id_organization: None,
            only_org_members: None,
        }
    }
}

// ──────────────────── Builder structs ────────────────────────────────

pub struct SearchRequest<'a> {
    client: &'a TrelloClient,
    query: String,
    id_boards: Option<String>,
    id_organizations: Option<String>,
    id_cards: Option<String>,
    model_types: Option<String>,
    board_fields: Option<String>,
    board_organization: Option<bool>,
    card_board: Option<bool>,
    card_list: Option<bool>,
    card_members: Option<bool>,
    card_stickers: Option<bool>,
    card_attachments: Option<bool>,
    organization_fields: Option<String>,
    member_fields: Option<String>,
    partial: Option<bool>,
}

impl<'a> SearchRequest<'a> {
    pub fn id_boards(mut self, v: &str) -> Self { self.id_boards = Some(v.to_string()); self }
    pub fn id_organizations(mut self, v: &str) -> Self { self.id_organizations = Some(v.to_string()); self }
    pub fn id_cards(mut self, v: &str) -> Self { self.id_cards = Some(v.to_string()); self }
    pub fn model_types(mut self, v: &str) -> Self { self.model_types = Some(v.to_string()); self }
    pub fn board_fields(mut self, v: &str) -> Self { self.board_fields = Some(v.to_string()); self }
    pub fn board_organization(mut self, v: bool) -> Self { self.board_organization = Some(v); self }
    pub fn card_board(mut self, v: bool) -> Self { self.card_board = Some(v); self }
    pub fn card_list(mut self, v: bool) -> Self { self.card_list = Some(v); self }
    pub fn card_members(mut self, v: bool) -> Self { self.card_members = Some(v); self }
    pub fn partial(mut self, v: bool) -> Self { self.partial = Some(v); self }

    pub async fn send(self) -> Result<SearchResults> {
        let mut params: Vec<(&str, &str)> = vec![("query", &self.query)];
        let ib_s; let io_s; let ic_s; let mt_s; let bf_s; let of_s; let mf_s;
        if let Some(ref v) = self.id_boards { ib_s = v.clone(); params.push(("idBoards", &ib_s)); }
        if let Some(ref v) = self.id_organizations { io_s = v.clone(); params.push(("idOrganizations", &io_s)); }
        if let Some(ref v) = self.id_cards { ic_s = v.clone(); params.push(("idCards", &ic_s)); }
        if let Some(ref v) = self.model_types { mt_s = v.clone(); params.push(("modelTypes", &mt_s)); }
        if let Some(ref v) = self.board_fields { bf_s = v.clone(); params.push(("board_fields", &bf_s)); }
        if let Some(v) = self.board_organization { params.push(("board_organization", if v { "true" } else { "false" })); }
        if let Some(v) = self.card_board { params.push(("card_board", if v { "true" } else { "false" })); }
        if let Some(v) = self.card_list { params.push(("card_list", if v { "true" } else { "false" })); }
        if let Some(v) = self.card_members { params.push(("card_members", if v { "true" } else { "false" })); }
        if let Some(v) = self.card_stickers { params.push(("card_stickers", if v { "true" } else { "false" })); }
        if let Some(v) = self.card_attachments { params.push(("card_attachments", if v { "true" } else { "false" })); }
        if let Some(ref v) = self.organization_fields { of_s = v.clone(); params.push(("organization_fields", &of_s)); }
        if let Some(ref v) = self.member_fields { mf_s = v.clone(); params.push(("member_fields", &mf_s)); }
        if let Some(v) = self.partial { params.push(("partial", if v { "true" } else { "false" })); }
        self.client.get("/search", &params).await
    }
}

pub struct SearchMembersRequest<'a> {
    client: &'a TrelloClient,
    query: String,
    limit: Option<u32>,
    id_board: Option<String>,
    id_organization: Option<String>,
    only_org_members: Option<bool>,
}

impl<'a> SearchMembersRequest<'a> {
    pub fn limit(mut self, v: u32) -> Self { self.limit = Some(v); self }
    pub fn id_board(mut self, v: &str) -> Self { self.id_board = Some(v.to_string()); self }
    pub fn id_organization(mut self, v: &str) -> Self { self.id_organization = Some(v.to_string()); self }
    pub fn only_org_members(mut self, v: bool) -> Self { self.only_org_members = Some(v); self }

    pub async fn send(self) -> Result<Vec<Member>> {
        let mut params: Vec<(&str, &str)> = vec![("query", &self.query)];
        let limit_s; let ib_s; let io_s;
        if let Some(v) = self.limit { limit_s = v.to_string(); params.push(("limit", &limit_s)); }
        if let Some(ref v) = self.id_board { ib_s = v.clone(); params.push(("idBoard", &ib_s)); }
        if let Some(ref v) = self.id_organization { io_s = v.clone(); params.push(("idOrganization", &io_s)); }
        if let Some(v) = self.only_org_members { params.push(("onlyOrgMembers", if v { "true" } else { "false" })); }
        self.client.get("/search/members/", &params).await
    }
}
