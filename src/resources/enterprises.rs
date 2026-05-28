use crate::client::TrelloClient;
use crate::error::Result;
use crate::models::common::TrelloID;
use crate::models::enterprise::{
    ClaimableOrganization, Enterprise, EnterpriseAdmin, EnterpriseAuditLog,
    PendingOrganization, TransferrableOrganization,
};
use crate::models::member::Member;
use crate::models::organization::Organization;
use crate::models::token::Token;

pub struct EnterprisesResource<'a> {
    client: &'a TrelloClient,
}

impl<'a> EnterprisesResource<'a> {
    pub(crate) fn new(client: &'a TrelloClient) -> Self {
        Self { client }
    }

    // ── GET /enterprises/{id} ───────────────────────────────────────

    pub fn get(&self, id: impl Into<TrelloID>) -> GetEnterpriseRequest<'a> {
        GetEnterpriseRequest {
            client: self.client,
            id: id.into(),
            fields: None,
            members: None,
            organizations: None,
        }
    }

    // ── GET /enterprises/{id}/admins ────────────────────────────────

    pub fn get_admins(&self, id: impl Into<TrelloID>) -> GetEnterpriseAdminsRequest<'a> {
        GetEnterpriseAdminsRequest {
            client: self.client,
            id: id.into(),
            fields: None,
        }
    }

    // ── PUT /enterprises/{id}/admins/{idMember} ─────────────────────

    pub fn add_admin(&self, enterprise_id: impl Into<TrelloID>, member_id: impl Into<TrelloID>) -> AddEnterpriseAdminRequest<'a> {
        AddEnterpriseAdminRequest {
            client: self.client,
            enterprise_id: enterprise_id.into(),
            member_id: member_id.into(),
        }
    }

    // ── DELETE /enterprises/{id}/admins/{idMember} ──────────────────

    pub async fn remove_admin(&self, enterprise_id: impl Into<TrelloID>, member_id: impl Into<TrelloID>) -> Result<()> {
        let enterprise_id: TrelloID = enterprise_id.into();
        let member_id: TrelloID = member_id.into();
        self.client
            .delete_no_body(&format!("/enterprises/{}/admins/{}", enterprise_id.as_ref(), member_id.as_ref()), &[])
            .await
    }

    // ── GET /enterprises/{id}/auditlog ──────────────────────────────

    pub async fn get_audit_log(&self, id: impl Into<TrelloID>) -> Result<Vec<EnterpriseAuditLog>> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/enterprises/{}/auditlog", id.as_ref()), &[]).await
    }

    // ── GET /enterprises/{id}/members ───────────────────────────────

    pub fn get_members(&self, id: impl Into<TrelloID>) -> GetEnterpriseMembersRequest<'a> {
        GetEnterpriseMembersRequest {
            client: self.client,
            id: id.into(),
            fields: None,
            filter: None,
            sort: None,
            sort_by: None,
            sort_order: None,
            start_index: None,
            count: None,
        }
    }

    // ── GET /enterprises/{id}/members/{idMember} ────────────────────

    pub async fn get_member(&self, enterprise_id: impl Into<TrelloID>, member_id: impl Into<TrelloID>) -> Result<Member> {
        let enterprise_id: TrelloID = enterprise_id.into();
        let member_id: TrelloID = member_id.into();
        self.client
            .get(
                &format!("/enterprises/{}/members/{}", enterprise_id.as_ref(), member_id.as_ref()),
                &[],
            )
            .await
    }

    // ── PUT /enterprises/{id}/members/{idMember}/deactivated ────────

    pub fn deactivate_member(&self, enterprise_id: impl Into<TrelloID>, member_id: impl Into<TrelloID>) -> DeactivateMemberRequest<'a> {
        DeactivateMemberRequest {
            client: self.client,
            enterprise_id: enterprise_id.into(),
            member_id: member_id.into(),
            value: true,
        }
    }

    // ── GET /enterprises/{id}/organizations ─────────────────────────

    pub fn get_organizations(&self, id: impl Into<TrelloID>) -> GetEnterpriseOrganizationsRequest<'a> {
        GetEnterpriseOrganizationsRequest {
            client: self.client,
            id: id.into(),
            filter: None,
            fields: None,
        }
    }

    // ── PUT /enterprises/{id}/organizations ─────────────────────────

    pub fn add_organization(&self, enterprise_id: impl Into<TrelloID>, org_id: impl Into<TrelloID>) -> AddEnterpriseOrgRequest<'a> {
        AddEnterpriseOrgRequest {
            client: self.client,
            enterprise_id: enterprise_id.into(),
            org_id: org_id.into(),
        }
    }

    // ── DELETE /enterprises/{id}/organizations/{idOrg} ──────────────

    pub async fn remove_organization(&self, enterprise_id: impl Into<TrelloID>, org_id: impl Into<TrelloID>) -> Result<()> {
        let enterprise_id: TrelloID = enterprise_id.into();
        let org_id: TrelloID = org_id.into();
        self.client
            .delete_no_body(
                &format!("/enterprises/{}/organizations/{}", enterprise_id.as_ref(), org_id.as_ref()),
                &[],
            )
            .await
    }

    // ── GET /enterprises/{id}/claimableOrganizations ────────────────

    pub async fn get_claimable_organizations(&self, id: impl Into<TrelloID>) -> Result<ClaimableOrganization> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/enterprises/{}/claimableOrganizations", id.as_ref()), &[]).await
    }

    // ── GET /enterprises/{id}/pendingOrganizations ──────────────────

    pub async fn get_pending_organizations(&self, id: impl Into<TrelloID>) -> Result<Vec<PendingOrganization>> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/enterprises/{}/pendingOrganizations", id.as_ref()), &[]).await
    }

    // ── POST /enterprises/{id}/tokens ───────────────────────────────

    pub async fn create_token(&self, id: impl Into<TrelloID>) -> Result<Token> {
        let id: TrelloID = id.into();
        self.client
            .post(&format!("/enterprises/{}/tokens", id.as_ref()), &[], None::<&()>)
            .await
    }

    // ── GET /enterprises/{id}/signupUrl ─────────────────────────────

    pub async fn get_signup_url(&self, id: impl Into<TrelloID>) -> Result<serde_json::Value> {
        let id: TrelloID = id.into();
        self.client.get(&format!("/enterprises/{}/signupUrl", id.as_ref()), &[]).await
    }

    // ── Transferrable organizations ─────────────────────────────────

    pub async fn get_transferrable_bulk(&self, enterprise_id: impl Into<TrelloID>, org_ids: &str) -> Result<Vec<TransferrableOrganization>> {
        let enterprise_id: TrelloID = enterprise_id.into();
        self.client
            .get(&format!("/enterprises/{}/transferrable/bulk/{}", enterprise_id.as_ref(), org_ids), &[])
            .await
    }

    pub async fn get_transferrable_org(&self, enterprise_id: impl Into<TrelloID>, org_id: impl Into<TrelloID>) -> Result<TransferrableOrganization> {
        let enterprise_id: TrelloID = enterprise_id.into();
        let org_id: TrelloID = org_id.into();
        self.client
            .get(&format!("/enterprises/{}/transferrable/organization/{}", enterprise_id.as_ref(), org_id.as_ref()), &[])
            .await
    }
}

// ──────────────────── Builder structs ────────────────────────────────

pub struct GetEnterpriseRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    fields: Option<String>,
    members: Option<String>,
    organizations: Option<String>,
}

impl<'a> GetEnterpriseRequest<'a> {
    pub fn fields(mut self, v: &str) -> Self { self.fields = Some(v.to_string()); self }
    pub fn members(mut self, v: &str) -> Self { self.members = Some(v.to_string()); self }
    pub fn organizations(mut self, v: &str) -> Self { self.organizations = Some(v.to_string()); self }

    pub async fn send(self) -> Result<Enterprise> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let fields_s; let members_s; let orgs_s;
        if let Some(ref v) = self.fields { fields_s = v.clone(); params.push(("fields", &fields_s)); }
        if let Some(ref v) = self.members { members_s = v.clone(); params.push(("members", &members_s)); }
        if let Some(ref v) = self.organizations { orgs_s = v.clone(); params.push(("organizations", &orgs_s)); }
        self.client.get(&format!("/enterprises/{}", self.id.as_ref()), &params).await
    }
}

pub struct GetEnterpriseAdminsRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    fields: Option<String>,
}

impl<'a> GetEnterpriseAdminsRequest<'a> {
    pub fn fields(mut self, v: &str) -> Self { self.fields = Some(v.to_string()); self }
    pub async fn send(self) -> Result<Vec<EnterpriseAdmin>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let fields_s;
        if let Some(ref v) = self.fields { fields_s = v.clone(); params.push(("fields", &fields_s)); }
        self.client.get(&format!("/enterprises/{}/admins", self.id.as_ref()), &params).await
    }
}

pub struct AddEnterpriseAdminRequest<'a> {
    client: &'a TrelloClient,
    enterprise_id: TrelloID,
    member_id: TrelloID,
}

impl<'a> AddEnterpriseAdminRequest<'a> {
    pub async fn send(self) -> Result<()> {
        self.client
            .put::<serde_json::Value, ()>(
                &format!("/enterprises/{}/admins/{}", self.enterprise_id.as_ref(), self.member_id.as_ref()),
                &[],
                None,
            )
            .await?;
        Ok(())
    }
}

pub struct DeactivateMemberRequest<'a> {
    client: &'a TrelloClient,
    enterprise_id: TrelloID,
    member_id: TrelloID,
    value: bool,
}

impl<'a> DeactivateMemberRequest<'a> {
    pub async fn send(self) -> Result<Member> {
        let v = if self.value { "true" } else { "false" };
        self.client
            .put(
                &format!("/enterprises/{}/members/{}/deactivated", self.enterprise_id.as_ref(), self.member_id.as_ref()),
                &[("value", v)],
                None::<&()>,
            )
            .await
    }
}

pub struct GetEnterpriseMembersRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    fields: Option<String>,
    filter: Option<String>,
    sort: Option<String>,
    sort_by: Option<String>,
    sort_order: Option<String>,
    start_index: Option<String>,
    count: Option<u32>,
}

impl<'a> GetEnterpriseMembersRequest<'a> {
    pub fn fields(mut self, v: &str) -> Self { self.fields = Some(v.to_string()); self }
    pub fn filter(mut self, v: &str) -> Self { self.filter = Some(v.to_string()); self }
    pub fn start_index(mut self, v: &str) -> Self { self.start_index = Some(v.to_string()); self }
    pub fn count(mut self, v: u32) -> Self { self.count = Some(v); self }

    pub async fn send(self) -> Result<Vec<Member>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let fields_s; let filter_s; let sort_s; let sb_s; let so_s; let si_s; let count_s;
        if let Some(ref v) = self.fields { fields_s = v.clone(); params.push(("fields", &fields_s)); }
        if let Some(ref v) = self.filter { filter_s = v.clone(); params.push(("filter", &filter_s)); }
        if let Some(ref v) = self.sort { sort_s = v.clone(); params.push(("sort", &sort_s)); }
        if let Some(ref v) = self.sort_by { sb_s = v.clone(); params.push(("sortBy", &sb_s)); }
        if let Some(ref v) = self.sort_order { so_s = v.clone(); params.push(("sortOrder", &so_s)); }
        if let Some(ref v) = self.start_index { si_s = v.clone(); params.push(("startIndex", &si_s)); }
        if let Some(v) = self.count { count_s = v.to_string(); params.push(("count", &count_s)); }
        self.client.get(&format!("/enterprises/{}/members", self.id.as_ref()), &params).await
    }
}

pub struct GetEnterpriseOrganizationsRequest<'a> {
    client: &'a TrelloClient,
    id: TrelloID,
    filter: Option<String>,
    fields: Option<String>,
}

impl<'a> GetEnterpriseOrganizationsRequest<'a> {
    pub fn filter(mut self, v: &str) -> Self { self.filter = Some(v.to_string()); self }
    pub fn fields(mut self, v: &str) -> Self { self.fields = Some(v.to_string()); self }

    pub async fn send(self) -> Result<Vec<Organization>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let filter_s; let fields_s;
        if let Some(ref v) = self.filter { filter_s = v.clone(); params.push(("filter", &filter_s)); }
        if let Some(ref v) = self.fields { fields_s = v.clone(); params.push(("fields", &fields_s)); }
        self.client.get(&format!("/enterprises/{}/organizations", self.id.as_ref()), &params).await
    }
}

pub struct AddEnterpriseOrgRequest<'a> {
    client: &'a TrelloClient,
    enterprise_id: TrelloID,
    org_id: TrelloID,
}

impl<'a> AddEnterpriseOrgRequest<'a> {
    pub async fn send(self) -> Result<()> {
        self.client
            .put::<serde_json::Value, ()>(
                &format!("/enterprises/{}/organizations", self.enterprise_id.as_ref()),
                &[("idOrganization", self.org_id.as_ref())],
                None,
            )
            .await?;
        Ok(())
    }
}
