use crate::client::TrelloClient;
use crate::error::Result;
use crate::models::action::Action;
use crate::models::board::Board;
use crate::models::common::TrelloID;
use crate::models::member::Member;
use crate::models::organization::{Organization, OrganizationExport, OrganizationField};
use crate::models::plugin::PluginData;
use crate::models::tag::Tag;
use crate::params::FieldQuery;

pub struct OrganizationsResource<'a> {
    client: &'a TrelloClient,
}

impl<'a> OrganizationsResource<'a> {
    pub(crate) fn new(client: &'a TrelloClient) -> Self {
        Self { client }
    }

    // ── GET /organizations/{id} ─────────────────────────────────────

    pub fn get(&self, id: impl Into<TrelloID>) -> GetOrganizationRequest<'a> {
        GetOrganizationRequest {
            client: self.client,
            id: id.into(),
        }
    }

    // ── POST /organizations ─────────────────────────────────────────

    pub fn create(&self) -> CreateOrganizationRequest<'a> {
        CreateOrganizationRequest {
            client: self.client,
            display_name: None,
            name: None,
            desc: None,
            website: None,
            team_type: None,
        }
    }

    // ── PUT /organizations/{id} ─────────────────────────────────────

    pub fn update(&self, id: impl Into<TrelloID>) -> UpdateOrganizationRequest<'a> {
        UpdateOrganizationRequest {
            client: self.client,
            id: id.into(),
            name: None,
            display_name: None,
            desc: None,
            website: None,
        }
    }

    // ── DELETE /organizations/{id} ──────────────────────────────────

    pub async fn delete(&self, id: impl Into<TrelloID>) -> Result<()> {
        let id: TrelloID = id.into();
        self.client
            .delete_no_body(&format!("/organizations/{}", id.as_ref()), &[])
            .await
    }

    // ── GET /organizations/{id}/{field} ─────────────────────────────

    pub async fn get_field(&self, id: impl Into<TrelloID>, field: OrganizationField) -> Result<serde_json::Value> {
        let id: TrelloID = id.into();
        self.client
            .get(&format!("/organizations/{}/{}", id.as_ref(), field.as_field_str()), &[])
            .await
    }

    // ── GET /organizations/{id}/boards ──────────────────────────────

    pub fn get_boards(&self, id: impl Into<TrelloID>) -> GetOrgBoardsRequest<'a> {
        GetOrgBoardsRequest {
            client: self.client,
            id: id.into(),
            filter: None,
            fields: None,
        }
    }

    // ── GET /organizations/{id}/members ─────────────────────────────

    pub fn get_members(&self, id: impl Into<TrelloID>) -> GetOrgMembersRequest<'a> {
        GetOrgMembersRequest {
            client: self.client,
            id: id.into(),
            filter: None,
            fields: None,
        }
    }

    // ── PUT /organizations/{id}/members ─────────────────────────────

    pub fn invite_member(&self, id: impl Into<TrelloID>, email: &str, full_name: &str, member_type: &str) -> InviteOrgMemberRequest<'a> {
        InviteOrgMemberRequest {
            client: self.client,
            id: id.into(),
            email: Some(email.to_string()),
            full_name: Some(full_name.to_string()),
            member_type: Some(member_type.to_string()),
        }
    }

    // ── PUT /organizations/{id}/members/{idMember} ──────────────────

    pub fn add_member(&self, org_id: impl Into<TrelloID>, member_id: impl Into<TrelloID>, member_type: &str) -> AddOrgMemberRequest<'a> {
        AddOrgMemberRequest {
            client: self.client,
            org_id: org_id.into(),
            member_id: member_id.into(),
            member_type: member_type.to_string(),
        }
    }

    // ── DELETE /organizations/{id}/members/{idMember} ───────────────

    pub async fn remove_member(&self, org_id: impl Into<TrelloID>, member_id: impl Into<TrelloID>) -> Result<()> {
        let org_id: TrelloID = org_id.into();
        let member_id: TrelloID = member_id.into();
        self.client
            .delete_no_body(&format!("/organizations/{}/members/{}", org_id.as_ref(), member_id.as_ref()), &[])
            .await
    }

    // ── GET /organizations/{id}/memberships ─────────────────────────

    pub fn get_memberships(&self, id: impl Into<TrelloID>) -> GetOrgMembershipsRequest<'a> {
        GetOrgMembershipsRequest {
            client: self.client,
            id: id.into(),
            filter: None,
            member: None,
        }
    }

    // ── GET /organizations/{id}/actions ─────────────────────────────

    pub fn get_actions(&self, id: impl Into<TrelloID>) -> GetOrgActionsRequest<'a> {
        GetOrgActionsRequest {
            client: self.client,
            id: id.into(),
            filter: None,
        }
    }

    // ── GET /organizations/{id}/tags ────────────────────────────────

    pub async fn get_tags(&self, id: impl Into<TrelloID>) -> Result<Vec<Tag>> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/organizations/{}/tags", id.as_ref()), &[]).await
    }

    // ── POST /organizations/{id}/tags ───────────────────────────────

    pub fn create_tag(&self, id: impl Into<TrelloID>, name: &str) -> CreateTagRequest<'a> {
        CreateTagRequest {
            client: self.client,
            id: id.into(),
            name: name.to_string(),
        }
    }

    // ── DELETE /organizations/{id}/tags/{idTag} ─────────────────────

    pub async fn delete_tag(&self, org_id: impl Into<TrelloID>, tag_id: impl Into<TrelloID>) -> Result<()> {
        let org_id: TrelloID = org_id.into();
        let tag_id: TrelloID = tag_id.into();
        self.client
            .delete_no_body(&format!("/organizations/{}/tags/{}", org_id.as_ref(), tag_id.as_ref()), &[])
            .await
    }

    // ── POST /organizations/{id}/logo ───────────────────────────────

    pub async fn upload_logo(&self, id: impl Into<TrelloID>, file_data: Vec<u8>, filename: &str) -> Result<Organization> {
        let id: TrelloID = id.into();
        let part = reqwest::multipart::Part::bytes(file_data)
            .file_name(filename.to_string())
            .mime_str("application/octet-stream")
            .map_err(|e| crate::error::Error::Other(e.to_string()))?;
        let form = reqwest::multipart::Form::new().part("file", part);
        self.client
            .upload(&format!("/organizations/{}/logo", id.as_ref()), &[], form)
            .await
    }

    // ── DELETE /organizations/{id}/logo ─────────────────────────────

    pub async fn delete_logo(&self, id: impl Into<TrelloID>) -> Result<()> {
        let id: TrelloID = id.into();
        self.client
            .delete_no_body(&format!("/organizations/{}/logo", id.as_ref()), &[])
            .await
    }

    // ── Exports ─────────────────────────────────────────────────────

    pub fn get_exports(&self, id: impl Into<TrelloID>) -> GetExportsRequest<'a> {
        GetExportsRequest {
            client: self.client,
            id: id.into(),
        }
    }

    pub async fn create_export(&self, id: impl Into<TrelloID>) -> Result<OrganizationExport> {
        let id: TrelloID = id.into();
        self.client
            .post(&format!("/organizations/{}/exports", id.as_ref()), &[], None::<&()>)
            .await
    }

    // ── GET /organizations/{id}/pluginData ──────────────────────────

    pub async fn get_plugin_data(&self, id: impl Into<TrelloID>) -> Result<Vec<PluginData>> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/organizations/{}/pluginData", id.as_ref()), &[]).await
    }

    // ── Prefs ───────────────────────────────────────────────────────

    pub async fn delete_associated_domain(&self, id: impl Into<TrelloID>) -> Result<()> {
        let id: TrelloID = id.into();
        self.client
            .delete_no_body(&format!("/organizations/{}/prefs/associatedDomain", id.as_ref()), &[])
            .await
    }

    pub async fn delete_org_invite_restrict(&self, id: impl Into<TrelloID>) -> Result<()> {
        let id: TrelloID = id.into();
        self.client
            .delete_no_body(&format!("/organizations/{}/prefs/orgInviteRestrict", id.as_ref()), &[])
            .await
    }

    // ── Billable guests ─────────────────────────────────────────────

    pub async fn get_new_billable_guests(&self, org_id: impl Into<TrelloID>, board_id: impl Into<TrelloID>) -> Result<serde_json::Value> {
        let org_id: TrelloID = org_id.into();
        let board_id: TrelloID = board_id.into();
        self.client.get(&format!("/organizations/{}/newBillableGuests/{}", org_id.as_ref(), board_id.as_ref()), &[]).await
    }
}

// ──────────────────── Builder structs ────────────────────────────────

pub struct GetOrganizationRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
}

impl<'a> GetOrganizationRequest<'a> {
    pub async fn send(self) -> Result<Organization> {
        self.client
            .get(&format!("/organizations/{}", self.id.as_ref()), &[])
            .await
    }
}

pub struct CreateOrganizationRequest<'a> {
    client: &'a TrelloClient,
    display_name: Option<String>,
    name: Option<String>,
    desc: Option<String>,
    website: Option<String>,
    team_type: Option<String>,
}

impl<'a> CreateOrganizationRequest<'a> {
    pub fn display_name(mut self, v: impl Into<String>) -> Self { self.display_name = Some(v.into()); self }
    pub fn name(mut self, v: impl Into<String>) -> Self { self.name = Some(v.into()); self }
    pub fn desc(mut self, v: impl Into<String>) -> Self { self.desc = Some(v.into()); self }
    pub fn website(mut self, v: impl Into<String>) -> Self { self.website = Some(v.into()); self }

    pub async fn send(self) -> Result<Organization> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let dn_s; let name_s; let desc_s; let web_s; let tt_s;
        if let Some(ref v) = self.display_name { dn_s = v.clone(); params.push(("displayName", &dn_s)); }
        if let Some(ref v) = self.name { name_s = v.clone(); params.push(("name", &name_s)); }
        if let Some(ref v) = self.desc { desc_s = v.clone(); params.push(("desc", &desc_s)); }
        if let Some(ref v) = self.website { web_s = v.clone(); params.push(("website", &web_s)); }
        if let Some(ref v) = self.team_type { tt_s = v.clone(); params.push(("teamType", &tt_s)); }
        self.client.post("/organizations", &params, None::<&()>).await
    }
}

pub struct UpdateOrganizationRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    name: Option<String>,
    display_name: Option<String>,
    desc: Option<String>,
    website: Option<String>,
}

impl<'a> UpdateOrganizationRequest<'a> {
    pub fn name(mut self, v: impl Into<String>) -> Self { self.name = Some(v.into()); self }
    pub fn display_name(mut self, v: impl Into<String>) -> Self { self.display_name = Some(v.into()); self }
    pub fn desc(mut self, v: impl Into<String>) -> Self { self.desc = Some(v.into()); self }
    pub fn website(mut self, v: impl Into<String>) -> Self { self.website = Some(v.into()); self }

    pub async fn send(self) -> Result<Organization> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let name_s; let dn_s; let desc_s; let web_s;
        if let Some(ref v) = self.name { name_s = v.clone(); params.push(("name", &name_s)); }
        if let Some(ref v) = self.display_name { dn_s = v.clone(); params.push(("displayName", &dn_s)); }
        if let Some(ref v) = self.desc { desc_s = v.clone(); params.push(("desc", &desc_s)); }
        if let Some(ref v) = self.website { web_s = v.clone(); params.push(("website", &web_s)); }
        self.client
            .put(&format!("/organizations/{}", self.id.as_ref()), &params, None::<&()>)
            .await
    }
}

pub struct GetOrgBoardsRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    filter: Option<String>,
    fields: Option<String>,
}

impl<'a> GetOrgBoardsRequest<'a> {
    pub fn filter(mut self, v: &str) -> Self { self.filter = Some(v.to_string()); self }
    pub fn fields(mut self, v: &str) -> Self { self.fields = Some(v.to_string()); self }

    pub async fn send(self) -> Result<Vec<Board>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let filter_s; let fields_s;
        if let Some(ref v) = self.filter { filter_s = v.clone(); params.push(("filter", &filter_s)); }
        if let Some(ref v) = self.fields { fields_s = v.clone(); params.push(("fields", &fields_s)); }
        self.client.get(&format!("/organizations/{}/boards", self.id.as_ref()), &params).await
    }
}

pub struct GetOrgMembersRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    filter: Option<String>,
    fields: Option<String>,
}

impl<'a> GetOrgMembersRequest<'a> {
    pub fn filter(mut self, v: &str) -> Self { self.filter = Some(v.to_string()); self }
    pub fn fields(mut self, v: &str) -> Self { self.fields = Some(v.to_string()); self }

    pub async fn send(self) -> Result<Vec<Member>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let filter_s; let fields_s;
        if let Some(ref v) = self.filter { filter_s = v.clone(); params.push(("filter", &filter_s)); }
        if let Some(ref v) = self.fields { fields_s = v.clone(); params.push(("fields", &fields_s)); }
        self.client.get(&format!("/organizations/{}/members", self.id.as_ref()), &params).await
    }
}

pub struct InviteOrgMemberRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    email: Option<String>,
    full_name: Option<String>,
    member_type: Option<String>,
}

impl<'a> InviteOrgMemberRequest<'a> {
    pub fn send(self) -> impl std::future::Future<Output = Result<()>> + use<'a> {
        async move {
            let mut params: Vec<(&str, &str)> = Vec::new();
            let email_s; let fn_s; let mt_s;
            if let Some(ref v) = self.email { email_s = v.clone(); params.push(("email", &email_s)); }
            if let Some(ref v) = self.full_name { fn_s = v.clone(); params.push(("fullName", &fn_s)); }
            if let Some(ref v) = self.member_type { mt_s = v.clone(); params.push(("type", &mt_s)); }
            self.client
                .put::<serde_json::Value, ()>(&format!("/organizations/{}/members", self.id.as_ref()), &params, None)
                .await?;
            Ok(())
        }
    }
}

pub struct AddOrgMemberRequest<'a> {
    client: &'a TrelloClient,
    org_id: TrelloID,
    member_id: TrelloID,
    member_type: String,
}

impl<'a> AddOrgMemberRequest<'a> {
    pub async fn send(self) -> Result<Member> {
        self.client
            .put(
                &format!("/organizations/{}/members/{}", self.org_id.as_ref(), self.member_id.as_ref()),
                &[("type", self.member_type.as_str())],
                None::<&()>,
            )
            .await
    }
}

pub struct GetOrgMembershipsRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    filter: Option<String>,
    member: Option<bool>,
}

impl<'a> GetOrgMembershipsRequest<'a> {
    pub fn filter(mut self, v: &str) -> Self { self.filter = Some(v.to_string()); self }
    pub fn member(mut self, v: bool) -> Self { self.member = Some(v); self }

    pub async fn send(self) -> Result<Vec<crate::models::common::Membership>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let filter_s;
        if let Some(ref v) = self.filter { filter_s = v.clone(); params.push(("filter", &filter_s)); }
        if let Some(v) = self.member { params.push(("member", if v { "true" } else { "false" })); }
        self.client.get(&format!("/organizations/{}/memberships", self.id.as_ref()), &params).await
    }
}

pub struct GetOrgActionsRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    filter: Option<String>,
}

impl<'a> GetOrgActionsRequest<'a> {
    pub fn filter(mut self, v: &str) -> Self { self.filter = Some(v.to_string()); self }

    pub async fn send(self) -> Result<Vec<Action>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let filter_s;
        if let Some(ref v) = self.filter { filter_s = v.clone(); params.push(("filter", &filter_s)); }
        self.client.get(&format!("/organizations/{}/actions", self.id.as_ref()), &params).await
    }
}

pub struct CreateTagRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    name: String,
}

impl<'a> CreateTagRequest<'a> {
    pub async fn send(self) -> Result<Tag> {
        self.client
            .post(&format!("/organizations/{}/tags", self.id.as_ref()), &[("name", &self.name)], None::<&()>)
            .await
    }
}

pub struct GetExportsRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
}

impl<'a> GetExportsRequest<'a> {
    pub async fn send(self) -> Result<Vec<OrganizationExport>> {
        self.client.get(&format!("/organizations/{}/exports", self.id.as_ref()), &[]).await
    }
}
