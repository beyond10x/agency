// generated from agency v1
// model digest 71132e90bde806415603d08bb97a71629b2c2f6cda30e1f22e76f407abd9a3eb
// contract digest a96f55e3e1ca68aeb1579b032c3547fffd27e8ab80e76bcb898f9b6c9258792e
// do not edit: regenerate with `ess synthesize --layout crate`

//! agency — the `agency` component of `agency` v1.
//!
//! The Agency service and cockpit API (docs/design/agency.md §5.1-§5.2).
//!
//! The component's outer surface exactly as the specification declares it: accepted commands as
//! handlers, declared views as queries, published events as a typed outbox. The behaviour behind
//! every handler is an implementation obligation — see the `PLAN.md` beside this workspace — and
//! until one is satisfied, its stub answers with a typed refusal naming what is owed.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// An event this component declares it publishes, on its way to the system's transport.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublishedEvent {
    /// `agency.agents.AgentDefinitionCreated`.
    AgentDefinitionCreated(crate::agents::AgentDefinitionCreated),
    /// `agency.agents.AgentDefinitionDeprecated`.
    AgentDefinitionDeprecated(crate::agents::AgentDefinitionDeprecated),
    /// `agency.agents.AgentDefinitionPublished`.
    AgentDefinitionPublished(crate::agents::AgentDefinitionPublished),
    /// `agency.agents.AgentProvisioned`.
    AgentProvisioned(crate::agents::AgentProvisioned),
    /// `agency.agents.AgentRetired`.
    AgentRetired(crate::agents::AgentRetired),
    /// `agency.agents.AgentSpecChecked`.
    AgentSpecChecked(crate::agents::AgentSpecChecked),
    /// `agency.agents.AgentSpecRefused`.
    AgentSpecRefused(crate::agents::AgentSpecRefused),
    /// `agency.auth.CredentialGranted`.
    CredentialGranted(crate::auth::CredentialGranted),
    /// `agency.auth.CredentialRefreshed`.
    CredentialRefreshed(crate::auth::CredentialRefreshed),
    /// `agency.auth.CredentialRevoked`.
    CredentialRevoked(crate::auth::CredentialRevoked),
    /// `agency.auth.CredentialStored`.
    CredentialStored(crate::auth::CredentialStored),
    /// `agency.auth.GrantRevoked`.
    GrantRevoked(crate::auth::GrantRevoked),
    /// `agency.auth.PolicyActivated`.
    PolicyActivated(crate::auth::PolicyActivated),
    /// `agency.auth.PolicyCreated`.
    PolicyCreated(crate::auth::PolicyCreated),
    /// `agency.auth.PolicySuperseded`.
    PolicySuperseded(crate::auth::PolicySuperseded),
    /// `agency.auth.PrincipalDisabled`.
    PrincipalDisabled(crate::auth::PrincipalDisabled),
    /// `agency.auth.PrincipalInvited`.
    PrincipalInvited(crate::auth::PrincipalInvited),
    /// `agency.auth.RoleAssigned`.
    RoleAssigned(crate::auth::RoleAssigned),
    /// `agency.auth.TenantCreated`.
    TenantCreated(crate::auth::TenantCreated),
    /// `agency.runs.RunAdmitted`.
    RunAdmitted(crate::runs::RunAdmitted),
    /// `agency.runs.RunApprovalDenied`.
    RunApprovalDenied(crate::runs::RunApprovalDenied),
    /// `agency.runs.RunApprovalGranted`.
    RunApprovalGranted(crate::runs::RunApprovalGranted),
    /// `agency.runs.RunApprovalRequested`.
    RunApprovalRequested(crate::runs::RunApprovalRequested),
    /// `agency.runs.RunBudgetSet`.
    RunBudgetSet(crate::runs::RunBudgetSet),
    /// `agency.runs.RunFailed`.
    RunFailed(crate::runs::RunFailed),
    /// `agency.runs.RunProposed`.
    RunProposed(crate::runs::RunProposed),
    /// `agency.runs.RunRequested`.
    RunRequested(crate::runs::RunRequested),
    /// `agency.runs.RunStarted`.
    RunStarted(crate::runs::RunStarted),
    /// `agency.runs.RunStopped`.
    RunStopped(crate::runs::RunStopped),
    /// `agency.service.SessionClosed`.
    SessionClosed(crate::service::SessionClosed),
    /// `agency.service.SessionOpened`.
    SessionOpened(crate::service::SessionOpened),
    /// `agency.teams.LeadSet`.
    LeadSet(crate::teams::LeadSet),
    /// `agency.teams.MemberAdded`.
    MemberAdded(crate::teams::MemberAdded),
    /// `agency.teams.MemberRemoved`.
    MemberRemoved(crate::teams::MemberRemoved),
    /// `agency.teams.MessageSent`.
    MessageSent(crate::teams::MessageSent),
    /// `agency.teams.QueueAttached`.
    QueueAttached(crate::teams::QueueAttached),
    /// `agency.teams.ReportSent`.
    ReportSent(crate::teams::ReportSent),
    /// `agency.teams.SwarmDeclared`.
    SwarmDeclared(crate::teams::SwarmDeclared),
    /// `agency.teams.SwarmStopped`.
    SwarmStopped(crate::teams::SwarmStopped),
    /// `agency.teams.TeamDisbanded`.
    TeamDisbanded(crate::teams::TeamDisbanded),
    /// `agency.teams.TeamFormed`.
    TeamFormed(crate::teams::TeamFormed),
    /// `agency.teams.WorkforceDefined`.
    WorkforceDefined(crate::teams::WorkforceDefined),
    /// `agency.teams.WorkforceRetired`.
    WorkforceRetired(crate::teams::WorkforceRetired),
    /// `agency.teams.WorkforceStaffed`.
    WorkforceStaffed(crate::teams::WorkforceStaffed),
    /// `agency.tools.BindingRevoked`.
    BindingRevoked(crate::tools::BindingRevoked),
    /// `agency.tools.ToolBound`.
    ToolBound(crate::tools::ToolBound),
    /// `agency.tools.ToolRegistered`.
    ToolRegistered(crate::tools::ToolRegistered),
    /// `agency.tools.ToolWithdrawn`.
    ToolWithdrawn(crate::tools::ToolWithdrawn),
    /// `agency.work.WorkInputConfigured`.
    WorkInputConfigured(crate::work::WorkInputConfigured),
    /// `agency.work.WorkInputPaused`.
    WorkInputPaused(crate::work::WorkInputPaused),
    /// `agency.work.WorkInputRemoved`.
    WorkInputRemoved(crate::work::WorkInputRemoved),
    /// `agency.work.WorkItemAbandoned`.
    WorkItemAbandoned(crate::work::WorkItemAbandoned),
    /// `agency.work.WorkItemAssigned`.
    WorkItemAssigned(crate::work::WorkItemAssigned),
    /// `agency.work.WorkItemReturned`.
    WorkItemReturned(crate::work::WorkItemReturned),
    /// `agency.work.WorkItemSettled`.
    WorkItemSettled(crate::work::WorkItemSettled),
    /// `agency.work.WorkItemSubmitted`.
    WorkItemSubmitted(crate::work::WorkItemSubmitted),
}

/// agency — the port over the component's obligations.
///
/// `B` bundles every behaviour and query this component owes; constructing it over the domain's
/// `obligations::Unimplemented` yields a component that compiles and refuses, in the type system,
/// everything not yet implemented.
pub struct Agency<B> {
    behaviors: B,
    outbox: Vec<PublishedEvent>,
}

impl<B> Agency<B> {
    /// A new port over the given obligation implementations.
    pub fn new(behaviors: B) -> Self {
        Self {
            behaviors,
            outbox: Vec::new(),
        }
    }

    /// Hands over everything published since the last drain, in publication order.
    ///
    /// The system's transport calls this; anything else reading it is taking events the transport
    /// will then never deliver.
    pub fn drain_outbox(&mut self) -> Vec<PublishedEvent> {
        core::mem::take(&mut self.outbox)
    }
}

impl<B> Agency<B>
where
    B: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    /// Accepts `agency.agents.CreateAgentDefinition`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn create_agent_definition(&mut self, input: crate::agents::CreateAgentDefinition) -> Result<crate::agents::CreateAgentDefinitionOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.create_agent_definition(input)?;
        match &outcome {
            crate::agents::CreateAgentDefinitionOutcome::Created { agent_definition_created, .. } => {
                self.outbox.push(PublishedEvent::AgentDefinitionCreated(agent_definition_created.clone()));
            }
        }
        Ok(outcome)
    }

    /// Accepts `agency.agents.DeprecateAgentDefinition`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn deprecate_agent_definition(&mut self, input: crate::agents::DeprecateAgentDefinition) -> Result<crate::agents::DeprecateAgentDefinitionOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.deprecate_agent_definition(input)?;
        match &outcome {
            crate::agents::DeprecateAgentDefinitionOutcome::NoSuchDefinition { .. } => {}
            crate::agents::DeprecateAgentDefinitionOutcome::Deprecated { agent_definition_deprecated, .. } => {
                self.outbox.push(PublishedEvent::AgentDefinitionDeprecated(agent_definition_deprecated.clone()));
            }
            crate::agents::DeprecateAgentDefinitionOutcome::WrongState { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.agents.ProvisionAgent`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn provision_agent(&mut self, input: crate::agents::ProvisionAgent) -> Result<crate::agents::ProvisionAgentOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.provision_agent(input)?;
        match &outcome {
            crate::agents::ProvisionAgentOutcome::Provisioned { agent_provisioned, .. } => {
                self.outbox.push(PublishedEvent::AgentProvisioned(agent_provisioned.clone()));
            }
        }
        Ok(outcome)
    }

    /// Accepts `agency.agents.PublishAgentDefinition`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn publish_agent_definition(&mut self, input: crate::agents::PublishAgentDefinition) -> Result<crate::agents::PublishAgentDefinitionOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.publish_agent_definition(input)?;
        match &outcome {
            crate::agents::PublishAgentDefinitionOutcome::NoSuchDefinition { .. } => {}
            crate::agents::PublishAgentDefinitionOutcome::Published { agent_definition_published, .. } => {
                self.outbox.push(PublishedEvent::AgentDefinitionPublished(agent_definition_published.clone()));
            }
            crate::agents::PublishAgentDefinitionOutcome::WrongState { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.agents.RecordSpecCheck`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn record_spec_check(&mut self, input: crate::agents::RecordSpecCheck) -> Result<crate::agents::RecordSpecCheckOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.record_spec_check(input)?;
        match &outcome {
            crate::agents::RecordSpecCheckOutcome::NoSuchDefinition { .. } => {}
            crate::agents::RecordSpecCheckOutcome::Refused { agent_spec_refused, .. } => {
                self.outbox.push(PublishedEvent::AgentSpecRefused(agent_spec_refused.clone()));
            }
            crate::agents::RecordSpecCheckOutcome::Checked { agent_spec_checked, .. } => {
                self.outbox.push(PublishedEvent::AgentSpecChecked(agent_spec_checked.clone()));
            }
            crate::agents::RecordSpecCheckOutcome::WrongState { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.agents.RetireAgent`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn retire_agent(&mut self, input: crate::agents::RetireAgent) -> Result<crate::agents::RetireAgentOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.retire_agent(input)?;
        match &outcome {
            crate::agents::RetireAgentOutcome::NoSuchAgent { .. } => {}
            crate::agents::RetireAgentOutcome::Retired { agent_retired, .. } => {
                self.outbox.push(PublishedEvent::AgentRetired(agent_retired.clone()));
            }
            crate::agents::RetireAgentOutcome::WrongState { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.auth.ActivatePolicy`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn activate_policy(&mut self, input: crate::auth::ActivatePolicy) -> Result<crate::auth::ActivatePolicyOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.activate_policy(input)?;
        match &outcome {
            crate::auth::ActivatePolicyOutcome::NoSuchPolicy { .. } => {}
            crate::auth::ActivatePolicyOutcome::Activated { policy_activated, .. } => {
                self.outbox.push(PublishedEvent::PolicyActivated(policy_activated.clone()));
            }
            crate::auth::ActivatePolicyOutcome::WrongState { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.auth.AssignRole`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn assign_role(&mut self, input: crate::auth::AssignRole) -> Result<crate::auth::AssignRoleOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.assign_role(input)?;
        match &outcome {
            crate::auth::AssignRoleOutcome::NoSuchPrincipal { .. } => {}
            crate::auth::AssignRoleOutcome::Assigned { role_assigned, .. } => {
                self.outbox.push(PublishedEvent::RoleAssigned(role_assigned.clone()));
            }
        }
        Ok(outcome)
    }

    /// Accepts `agency.auth.CreatePolicy`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn create_policy(&mut self, input: crate::auth::CreatePolicy) -> Result<crate::auth::CreatePolicyOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.create_policy(input)?;
        match &outcome {
            crate::auth::CreatePolicyOutcome::Created { policy_created, .. } => {
                self.outbox.push(PublishedEvent::PolicyCreated(policy_created.clone()));
            }
        }
        Ok(outcome)
    }

    /// Accepts `agency.auth.CreateTenant`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn create_tenant(&mut self, input: crate::auth::CreateTenant) -> Result<crate::auth::CreateTenantOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.create_tenant(input)?;
        match &outcome {
            crate::auth::CreateTenantOutcome::Created { tenant_created, .. } => {
                self.outbox.push(PublishedEvent::TenantCreated(tenant_created.clone()));
            }
        }
        Ok(outcome)
    }

    /// Accepts `agency.auth.DisablePrincipal`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn disable_principal(&mut self, input: crate::auth::DisablePrincipal) -> Result<crate::auth::DisablePrincipalOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.disable_principal(input)?;
        match &outcome {
            crate::auth::DisablePrincipalOutcome::NoSuchPrincipal { .. } => {}
            crate::auth::DisablePrincipalOutcome::Disabled { principal_disabled, .. } => {
                self.outbox.push(PublishedEvent::PrincipalDisabled(principal_disabled.clone()));
            }
            crate::auth::DisablePrincipalOutcome::WrongState { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.auth.GrantCredential`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn grant_credential(&mut self, input: crate::auth::GrantCredential) -> Result<crate::auth::GrantCredentialOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.grant_credential(input)?;
        match &outcome {
            crate::auth::GrantCredentialOutcome::GrantedToAgent { credential_granted, .. } => {
                self.outbox.push(PublishedEvent::CredentialGranted(credential_granted.clone()));
            }
            crate::auth::GrantCredentialOutcome::GrantedToTeam { credential_granted, .. } => {
                self.outbox.push(PublishedEvent::CredentialGranted(credential_granted.clone()));
            }
            crate::auth::GrantCredentialOutcome::GrantedToWorkforce { credential_granted, .. } => {
                self.outbox.push(PublishedEvent::CredentialGranted(credential_granted.clone()));
            }
            crate::auth::GrantCredentialOutcome::GranteeInvalid { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.auth.InvitePrincipal`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn invite_principal(&mut self, input: crate::auth::InvitePrincipal) -> Result<crate::auth::InvitePrincipalOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.invite_principal(input)?;
        match &outcome {
            crate::auth::InvitePrincipalOutcome::Invited { principal_invited, .. } => {
                self.outbox.push(PublishedEvent::PrincipalInvited(principal_invited.clone()));
            }
        }
        Ok(outcome)
    }

    /// Accepts `agency.auth.RefreshCredential`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn refresh_credential(&mut self, input: crate::auth::RefreshCredential) -> Result<crate::auth::RefreshCredentialOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.refresh_credential(input)?;
        match &outcome {
            crate::auth::RefreshCredentialOutcome::NoSuchCredential { .. } => {}
            crate::auth::RefreshCredentialOutcome::Revoked { .. } => {}
            crate::auth::RefreshCredentialOutcome::Refreshed { credential_refreshed, .. } => {
                self.outbox.push(PublishedEvent::CredentialRefreshed(credential_refreshed.clone()));
            }
        }
        Ok(outcome)
    }

    /// Accepts `agency.auth.RevokeCredential`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn revoke_credential(&mut self, input: crate::auth::RevokeCredential) -> Result<crate::auth::RevokeCredentialOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.revoke_credential(input)?;
        match &outcome {
            crate::auth::RevokeCredentialOutcome::NoSuchCredential { .. } => {}
            crate::auth::RevokeCredentialOutcome::Revoked { credential_revoked, .. } => {
                self.outbox.push(PublishedEvent::CredentialRevoked(credential_revoked.clone()));
            }
            crate::auth::RevokeCredentialOutcome::WrongState { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.auth.RevokeGrant`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn revoke_grant(&mut self, input: crate::auth::RevokeGrant) -> Result<crate::auth::RevokeGrantOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.revoke_grant(input)?;
        match &outcome {
            crate::auth::RevokeGrantOutcome::NoSuchGrant { .. } => {}
            crate::auth::RevokeGrantOutcome::Revoked { grant_revoked, .. } => {
                self.outbox.push(PublishedEvent::GrantRevoked(grant_revoked.clone()));
            }
            crate::auth::RevokeGrantOutcome::WrongState { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.auth.StoreCredential`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn store_credential(&mut self, input: crate::auth::StoreCredential) -> Result<crate::auth::StoreCredentialOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.store_credential(input)?;
        match &outcome {
            crate::auth::StoreCredentialOutcome::Stored { credential_stored, .. } => {
                self.outbox.push(PublishedEvent::CredentialStored(credential_stored.clone()));
            }
        }
        Ok(outcome)
    }

    /// Accepts `agency.auth.SupersedePolicy`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn supersede_policy(&mut self, input: crate::auth::SupersedePolicy) -> Result<crate::auth::SupersedePolicyOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.supersede_policy(input)?;
        match &outcome {
            crate::auth::SupersedePolicyOutcome::NoSuchPolicy { .. } => {}
            crate::auth::SupersedePolicyOutcome::Superseded { policy_superseded, .. } => {
                self.outbox.push(PublishedEvent::PolicySuperseded(policy_superseded.clone()));
            }
            crate::auth::SupersedePolicyOutcome::WrongState { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.runs.AdmitRun`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn admit_run(&mut self, input: crate::runs::AdmitRun) -> Result<crate::runs::AdmitRunOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.admit_run(input)?;
        match &outcome {
            crate::runs::AdmitRunOutcome::NoSuchRun { .. } => {}
            crate::runs::AdmitRunOutcome::Admitted { run_admitted, .. } => {
                self.outbox.push(PublishedEvent::RunAdmitted(run_admitted.clone()));
            }
            crate::runs::AdmitRunOutcome::WrongState { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.runs.DenyRunApproval`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn deny_run_approval(&mut self, input: crate::runs::DenyRunApproval) -> Result<crate::runs::DenyRunApprovalOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.deny_run_approval(input)?;
        match &outcome {
            crate::runs::DenyRunApprovalOutcome::NoSuchApproval { .. } => {}
            crate::runs::DenyRunApprovalOutcome::Denied { run_approval_denied, .. } => {
                self.outbox.push(PublishedEvent::RunApprovalDenied(run_approval_denied.clone()));
            }
            crate::runs::DenyRunApprovalOutcome::WrongState { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.runs.GrantRunApproval`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn grant_run_approval(&mut self, input: crate::runs::GrantRunApproval) -> Result<crate::runs::GrantRunApprovalOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.grant_run_approval(input)?;
        match &outcome {
            crate::runs::GrantRunApprovalOutcome::NoSuchApproval { .. } => {}
            crate::runs::GrantRunApprovalOutcome::Granted { run_approval_granted, .. } => {
                self.outbox.push(PublishedEvent::RunApprovalGranted(run_approval_granted.clone()));
            }
            crate::runs::GrantRunApprovalOutcome::WrongState { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.runs.RecordRunFailure`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn record_run_failure(&mut self, input: crate::runs::RecordRunFailure) -> Result<crate::runs::RecordRunFailureOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.record_run_failure(input)?;
        match &outcome {
            crate::runs::RecordRunFailureOutcome::NoSuchRun { .. } => {}
            crate::runs::RecordRunFailureOutcome::Failed { run_failed, .. } => {
                self.outbox.push(PublishedEvent::RunFailed(run_failed.clone()));
            }
            crate::runs::RecordRunFailureOutcome::WrongState { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.runs.RecordRunProposal`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn record_run_proposal(&mut self, input: crate::runs::RecordRunProposal) -> Result<crate::runs::RecordRunProposalOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.record_run_proposal(input)?;
        match &outcome {
            crate::runs::RecordRunProposalOutcome::NoSuchRun { .. } => {}
            crate::runs::RecordRunProposalOutcome::Proposed { run_proposed, .. } => {
                self.outbox.push(PublishedEvent::RunProposed(run_proposed.clone()));
            }
            crate::runs::RecordRunProposalOutcome::WrongState { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.runs.RecordRunStarted`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn record_run_started(&mut self, input: crate::runs::RecordRunStarted) -> Result<crate::runs::RecordRunStartedOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.record_run_started(input)?;
        match &outcome {
            crate::runs::RecordRunStartedOutcome::NoSuchRun { .. } => {}
            crate::runs::RecordRunStartedOutcome::Started { run_started, .. } => {
                self.outbox.push(PublishedEvent::RunStarted(run_started.clone()));
            }
            crate::runs::RecordRunStartedOutcome::WrongState { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.runs.RequestRunApproval`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn request_run_approval(&mut self, input: crate::runs::RequestRunApproval) -> Result<crate::runs::RequestRunApprovalOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.request_run_approval(input)?;
        match &outcome {
            crate::runs::RequestRunApprovalOutcome::Requested { run_approval_requested, .. } => {
                self.outbox.push(PublishedEvent::RunApprovalRequested(run_approval_requested.clone()));
            }
        }
        Ok(outcome)
    }

    /// Accepts `agency.runs.SetRunBudget`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn set_run_budget(&mut self, input: crate::runs::SetRunBudget) -> Result<crate::runs::SetRunBudgetOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.set_run_budget(input)?;
        match &outcome {
            crate::runs::SetRunBudgetOutcome::NoSuchRun { .. } => {}
            crate::runs::SetRunBudgetOutcome::Ended { .. } => {}
            crate::runs::SetRunBudgetOutcome::Set { run_budget_set, .. } => {
                self.outbox.push(PublishedEvent::RunBudgetSet(run_budget_set.clone()));
            }
        }
        Ok(outcome)
    }

    /// Accepts `agency.runs.StartRun`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn start_run(&mut self, input: crate::runs::StartRun) -> Result<crate::runs::StartRunOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.start_run(input)?;
        match &outcome {
            crate::runs::StartRunOutcome::Requested { run_requested, .. } => {
                self.outbox.push(PublishedEvent::RunRequested(run_requested.clone()));
            }
        }
        Ok(outcome)
    }

    /// Accepts `agency.runs.StopRun`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn stop_run(&mut self, input: crate::runs::StopRun) -> Result<crate::runs::StopRunOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.stop_run(input)?;
        match &outcome {
            crate::runs::StopRunOutcome::NoSuchRun { .. } => {}
            crate::runs::StopRunOutcome::Stopped { run_stopped, .. } => {
                self.outbox.push(PublishedEvent::RunStopped(run_stopped.clone()));
            }
            crate::runs::StopRunOutcome::WrongState { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.service.CloseSession`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn close_session(&mut self, input: crate::service::CloseSession) -> Result<crate::service::CloseSessionOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.close_session(input)?;
        match &outcome {
            crate::service::CloseSessionOutcome::NoSuchSession { .. } => {}
            crate::service::CloseSessionOutcome::Closed { session_closed, .. } => {
                self.outbox.push(PublishedEvent::SessionClosed(session_closed.clone()));
            }
            crate::service::CloseSessionOutcome::WrongState { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.service.OpenSession`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn open_session(&mut self, input: crate::service::OpenSession) -> Result<crate::service::OpenSessionOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.open_session(input)?;
        match &outcome {
            crate::service::OpenSessionOutcome::Opened { session_opened, .. } => {
                self.outbox.push(PublishedEvent::SessionOpened(session_opened.clone()));
            }
        }
        Ok(outcome)
    }

    /// Accepts `agency.teams.AddMember`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn add_member(&mut self, input: crate::teams::AddMember) -> Result<crate::teams::AddMemberOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.add_member(input)?;
        match &outcome {
            crate::teams::AddMemberOutcome::NoSuchTeam { .. } => {}
            crate::teams::AddMemberOutcome::Added { member_added, .. } => {
                self.outbox.push(PublishedEvent::MemberAdded(member_added.clone()));
            }
        }
        Ok(outcome)
    }

    /// Accepts `agency.teams.AttachQueue`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn attach_queue(&mut self, input: crate::teams::AttachQueue) -> Result<crate::teams::AttachQueueOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.attach_queue(input)?;
        match &outcome {
            crate::teams::AttachQueueOutcome::NoSuchInput { .. } => {}
            crate::teams::AttachQueueOutcome::Attached { queue_attached, .. } => {
                self.outbox.push(PublishedEvent::QueueAttached(queue_attached.clone()));
            }
        }
        Ok(outcome)
    }

    /// Accepts `agency.teams.DeclareSwarm`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn declare_swarm(&mut self, input: crate::teams::DeclareSwarm) -> Result<crate::teams::DeclareSwarmOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.declare_swarm(input)?;
        match &outcome {
            crate::teams::DeclareSwarmOutcome::Declared { swarm_declared, .. } => {
                self.outbox.push(PublishedEvent::SwarmDeclared(swarm_declared.clone()));
            }
        }
        Ok(outcome)
    }

    /// Accepts `agency.teams.DefineWorkforce`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn define_workforce(&mut self, input: crate::teams::DefineWorkforce) -> Result<crate::teams::DefineWorkforceOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.define_workforce(input)?;
        match &outcome {
            crate::teams::DefineWorkforceOutcome::Defined { workforce_defined, .. } => {
                self.outbox.push(PublishedEvent::WorkforceDefined(workforce_defined.clone()));
            }
        }
        Ok(outcome)
    }

    /// Accepts `agency.teams.DisbandTeam`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn disband_team(&mut self, input: crate::teams::DisbandTeam) -> Result<crate::teams::DisbandTeamOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.disband_team(input)?;
        match &outcome {
            crate::teams::DisbandTeamOutcome::NoSuchTeam { .. } => {}
            crate::teams::DisbandTeamOutcome::Disbanded { team_disbanded, .. } => {
                self.outbox.push(PublishedEvent::TeamDisbanded(team_disbanded.clone()));
            }
            crate::teams::DisbandTeamOutcome::WrongState { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.teams.FormTeam`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn form_team(&mut self, input: crate::teams::FormTeam) -> Result<crate::teams::FormTeamOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.form_team(input)?;
        match &outcome {
            crate::teams::FormTeamOutcome::Formed { team_formed, .. } => {
                self.outbox.push(PublishedEvent::TeamFormed(team_formed.clone()));
            }
        }
        Ok(outcome)
    }

    /// Accepts `agency.teams.RemoveMember`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn remove_member(&mut self, input: crate::teams::RemoveMember) -> Result<crate::teams::RemoveMemberOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.remove_member(input)?;
        match &outcome {
            crate::teams::RemoveMemberOutcome::NoSuchTeam { .. } => {}
            crate::teams::RemoveMemberOutcome::Removed { member_removed, .. } => {
                self.outbox.push(PublishedEvent::MemberRemoved(member_removed.clone()));
            }
        }
        Ok(outcome)
    }

    /// Accepts `agency.teams.Report`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn report(&mut self, input: crate::teams::Report) -> Result<crate::teams::ReportOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.report(input)?;
        match &outcome {
            crate::teams::ReportOutcome::NoSuchTeam { .. } => {}
            crate::teams::ReportOutcome::NotOneLevelUp { .. } => {}
            crate::teams::ReportOutcome::Reported { report_sent, .. } => {
                self.outbox.push(PublishedEvent::ReportSent(report_sent.clone()));
            }
        }
        Ok(outcome)
    }

    /// Accepts `agency.teams.RetireWorkforce`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn retire_workforce(&mut self, input: crate::teams::RetireWorkforce) -> Result<crate::teams::RetireWorkforceOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.retire_workforce(input)?;
        match &outcome {
            crate::teams::RetireWorkforceOutcome::NoSuchWorkforce { .. } => {}
            crate::teams::RetireWorkforceOutcome::Retired { workforce_retired, .. } => {
                self.outbox.push(PublishedEvent::WorkforceRetired(workforce_retired.clone()));
            }
            crate::teams::RetireWorkforceOutcome::WrongState { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.teams.Send`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn send(&mut self, input: crate::teams::Send) -> Result<crate::teams::SendOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.send(input)?;
        match &outcome {
            crate::teams::SendOutcome::NoSuchTeam { .. } => {}
            crate::teams::SendOutcome::ForeignInstance { .. } => {}
            crate::teams::SendOutcome::OutsideTeam { .. } => {}
            crate::teams::SendOutcome::Sent { message_sent, .. } => {
                self.outbox.push(PublishedEvent::MessageSent(message_sent.clone()));
            }
        }
        Ok(outcome)
    }

    /// Accepts `agency.teams.SetLead`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn set_lead(&mut self, input: crate::teams::SetLead) -> Result<crate::teams::SetLeadOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.set_lead(input)?;
        match &outcome {
            crate::teams::SetLeadOutcome::NoSuchTeam { .. } => {}
            crate::teams::SetLeadOutcome::Set { lead_set, .. } => {
                self.outbox.push(PublishedEvent::LeadSet(lead_set.clone()));
            }
        }
        Ok(outcome)
    }

    /// Accepts `agency.teams.StaffWorkforce`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn staff_workforce(&mut self, input: crate::teams::StaffWorkforce) -> Result<crate::teams::StaffWorkforceOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.staff_workforce(input)?;
        match &outcome {
            crate::teams::StaffWorkforceOutcome::NoSuchWorkforce { .. } => {}
            crate::teams::StaffWorkforceOutcome::Staffed { workforce_staffed, .. } => {
                self.outbox.push(PublishedEvent::WorkforceStaffed(workforce_staffed.clone()));
            }
            crate::teams::StaffWorkforceOutcome::WrongState { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.teams.StopSwarm`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn stop_swarm(&mut self, input: crate::teams::StopSwarm) -> Result<crate::teams::StopSwarmOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.stop_swarm(input)?;
        match &outcome {
            crate::teams::StopSwarmOutcome::NoSuchSwarm { .. } => {}
            crate::teams::StopSwarmOutcome::Stopped { swarm_stopped, .. } => {
                self.outbox.push(PublishedEvent::SwarmStopped(swarm_stopped.clone()));
            }
            crate::teams::StopSwarmOutcome::WrongState { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.tools.BindTool`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn bind_tool(&mut self, input: crate::tools::BindTool) -> Result<crate::tools::BindToolOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.bind_tool(input)?;
        match &outcome {
            crate::tools::BindToolOutcome::BoundToDefinition { tool_bound, .. } => {
                self.outbox.push(PublishedEvent::ToolBound(tool_bound.clone()));
            }
            crate::tools::BindToolOutcome::BoundToTeam { tool_bound, .. } => {
                self.outbox.push(PublishedEvent::ToolBound(tool_bound.clone()));
            }
            crate::tools::BindToolOutcome::TargetInvalid { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.tools.RegisterTool`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn register_tool(&mut self, input: crate::tools::RegisterTool) -> Result<crate::tools::RegisterToolOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.register_tool(input)?;
        match &outcome {
            crate::tools::RegisterToolOutcome::Registered { tool_registered, .. } => {
                self.outbox.push(PublishedEvent::ToolRegistered(tool_registered.clone()));
            }
        }
        Ok(outcome)
    }

    /// Accepts `agency.tools.RevokeBinding`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn revoke_binding(&mut self, input: crate::tools::RevokeBinding) -> Result<crate::tools::RevokeBindingOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.revoke_binding(input)?;
        match &outcome {
            crate::tools::RevokeBindingOutcome::NoSuchBinding { .. } => {}
            crate::tools::RevokeBindingOutcome::Revoked { binding_revoked, .. } => {
                self.outbox.push(PublishedEvent::BindingRevoked(binding_revoked.clone()));
            }
            crate::tools::RevokeBindingOutcome::WrongState { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.tools.WithdrawTool`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn withdraw_tool(&mut self, input: crate::tools::WithdrawTool) -> Result<crate::tools::WithdrawToolOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.withdraw_tool(input)?;
        match &outcome {
            crate::tools::WithdrawToolOutcome::NoSuchTool { .. } => {}
            crate::tools::WithdrawToolOutcome::Withdrawn { tool_withdrawn, .. } => {
                self.outbox.push(PublishedEvent::ToolWithdrawn(tool_withdrawn.clone()));
            }
            crate::tools::WithdrawToolOutcome::WrongState { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.work.AbandonWorkItem`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn abandon_work_item(&mut self, input: crate::work::AbandonWorkItem) -> Result<crate::work::AbandonWorkItemOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.abandon_work_item(input)?;
        match &outcome {
            crate::work::AbandonWorkItemOutcome::NoSuchItem { .. } => {}
            crate::work::AbandonWorkItemOutcome::Abandoned { work_item_abandoned, .. } => {
                self.outbox.push(PublishedEvent::WorkItemAbandoned(work_item_abandoned.clone()));
            }
            crate::work::AbandonWorkItemOutcome::WrongState { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.work.AssignWorkItem`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn assign_work_item(&mut self, input: crate::work::AssignWorkItem) -> Result<crate::work::AssignWorkItemOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.assign_work_item(input)?;
        match &outcome {
            crate::work::AssignWorkItemOutcome::NoSuchItem { .. } => {}
            crate::work::AssignWorkItemOutcome::Assigned { work_item_assigned, .. } => {
                self.outbox.push(PublishedEvent::WorkItemAssigned(work_item_assigned.clone()));
            }
            crate::work::AssignWorkItemOutcome::WrongState { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.work.ConfigureWorkInput`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn configure_work_input(&mut self, input: crate::work::ConfigureWorkInput) -> Result<crate::work::ConfigureWorkInputOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.configure_work_input(input)?;
        match &outcome {
            crate::work::ConfigureWorkInputOutcome::Configured { work_input_configured, .. } => {
                self.outbox.push(PublishedEvent::WorkInputConfigured(work_input_configured.clone()));
            }
        }
        Ok(outcome)
    }

    /// Accepts `agency.work.PauseWorkInput`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn pause_work_input(&mut self, input: crate::work::PauseWorkInput) -> Result<crate::work::PauseWorkInputOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.pause_work_input(input)?;
        match &outcome {
            crate::work::PauseWorkInputOutcome::NoSuchInput { .. } => {}
            crate::work::PauseWorkInputOutcome::Paused { work_input_paused, .. } => {
                self.outbox.push(PublishedEvent::WorkInputPaused(work_input_paused.clone()));
            }
            crate::work::PauseWorkInputOutcome::WrongState { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.work.RemoveWorkInput`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn remove_work_input(&mut self, input: crate::work::RemoveWorkInput) -> Result<crate::work::RemoveWorkInputOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.remove_work_input(input)?;
        match &outcome {
            crate::work::RemoveWorkInputOutcome::NoSuchInput { .. } => {}
            crate::work::RemoveWorkInputOutcome::Removed { work_input_removed, .. } => {
                self.outbox.push(PublishedEvent::WorkInputRemoved(work_input_removed.clone()));
            }
            crate::work::RemoveWorkInputOutcome::WrongState { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.work.ReturnWorkItem`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn return_work_item(&mut self, input: crate::work::ReturnWorkItem) -> Result<crate::work::ReturnWorkItemOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.return_work_item(input)?;
        match &outcome {
            crate::work::ReturnWorkItemOutcome::NoSuchItem { .. } => {}
            crate::work::ReturnWorkItemOutcome::Returned { work_item_returned, .. } => {
                self.outbox.push(PublishedEvent::WorkItemReturned(work_item_returned.clone()));
            }
            crate::work::ReturnWorkItemOutcome::WrongState { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.work.SettleWorkItem`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn settle_work_item(&mut self, input: crate::work::SettleWorkItem) -> Result<crate::work::SettleWorkItemOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.settle_work_item(input)?;
        match &outcome {
            crate::work::SettleWorkItemOutcome::NoSuchItem { .. } => {}
            crate::work::SettleWorkItemOutcome::Settled { work_item_settled, .. } => {
                self.outbox.push(PublishedEvent::WorkItemSettled(work_item_settled.clone()));
            }
            crate::work::SettleWorkItemOutcome::WrongState { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agency.work.SubmitWorkItem`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn submit_work_item(&mut self, input: crate::work::SubmitWorkItem) -> Result<crate::work::SubmitWorkItemOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.submit_work_item(input)?;
        match &outcome {
            crate::work::SubmitWorkItemOutcome::Received { work_item_submitted, .. } => {
                self.outbox.push(PublishedEvent::WorkItemSubmitted(work_item_submitted.clone()));
            }
        }
        Ok(outcome)
    }

    /// Serves `agency.agents.AgentDefinitionList` at `read_your_writes` consistency, from the owed projection.
    pub fn agent_definition_list(&self) -> Result<Vec<crate::agents::AgentDefinitionList>, crate::obligation::UnmetObligation> {
        self.behaviors.agent_definition_list()
    }

    /// Serves `agency.agents.AgentList` at `read_your_writes` consistency, from the owed projection.
    pub fn agent_list(&self) -> Result<Vec<crate::agents::AgentList>, crate::obligation::UnmetObligation> {
        self.behaviors.agent_list()
    }

    /// Serves `agency.auth.CredentialList` at `read_your_writes` consistency, from the owed projection.
    pub fn credential_list(&self) -> Result<Vec<crate::auth::CredentialList>, crate::obligation::UnmetObligation> {
        self.behaviors.credential_list()
    }

    /// Serves `agency.auth.EffectivePolicy` at `read_your_writes` consistency, from the owed projection.
    pub fn effective_policy(&self, definition_id: crate::agents::DefinitionId) -> Result<Vec<crate::auth::EffectivePolicy>, crate::obligation::UnmetObligation> {
        self.behaviors.effective_policy(definition_id)
    }

    /// Serves `agency.auth.GrantMatrix` at `read_your_writes` consistency, from the owed projection.
    pub fn grant_matrix(&self) -> Result<Vec<crate::auth::GrantMatrix>, crate::obligation::UnmetObligation> {
        self.behaviors.grant_matrix()
    }

    /// Serves `agency.auth.PolicyList` at `read_your_writes` consistency, from the owed projection.
    pub fn policy_list(&self) -> Result<Vec<crate::auth::PolicyList>, crate::obligation::UnmetObligation> {
        self.behaviors.policy_list()
    }

    /// Serves `agency.auth.PrincipalList` at `read_your_writes` consistency, from the owed projection.
    pub fn principal_list(&self) -> Result<Vec<crate::auth::PrincipalList>, crate::obligation::UnmetObligation> {
        self.behaviors.principal_list()
    }

    /// Serves `agency.runs.ApprovalInbox` at `read_your_writes` consistency, from the owed projection.
    pub fn approval_inbox(&self) -> Result<Vec<crate::runs::ApprovalInbox>, crate::obligation::UnmetObligation> {
        self.behaviors.approval_inbox()
    }

    /// Serves `agency.runs.RunDetail` at `read_your_writes` consistency, from the owed projection.
    pub fn run_detail(&self, run_id: crate::runs::RunId) -> Result<Vec<crate::runs::RunDetail>, crate::obligation::UnmetObligation> {
        self.behaviors.run_detail(run_id)
    }

    /// Serves `agency.runs.RunList` at `read_your_writes` consistency, from the owed projection.
    pub fn run_list(&self) -> Result<Vec<crate::runs::RunList>, crate::obligation::UnmetObligation> {
        self.behaviors.run_list()
    }

    /// Serves `agency.runs.UsageByAgent` at `read_your_writes` consistency, from the owed projection.
    pub fn usage_by_agent(&self) -> Result<Vec<crate::runs::UsageByAgent>, crate::obligation::UnmetObligation> {
        self.behaviors.usage_by_agent()
    }

    /// Serves `agency.service.SessionList` at `read_your_writes` consistency, from the owed projection.
    pub fn session_list(&self) -> Result<Vec<crate::service::SessionList>, crate::obligation::UnmetObligation> {
        self.behaviors.session_list()
    }

    /// Serves `agency.teams.SwarmList` at `read_your_writes` consistency, from the owed projection.
    pub fn swarm_list(&self) -> Result<Vec<crate::teams::SwarmList>, crate::obligation::UnmetObligation> {
        self.behaviors.swarm_list()
    }

    /// Serves `agency.teams.TeamDetail` at `read_your_writes` consistency, from the owed projection.
    pub fn team_detail(&self, team_id: crate::teams::TeamId) -> Result<Vec<crate::teams::TeamDetail>, crate::obligation::UnmetObligation> {
        self.behaviors.team_detail(team_id)
    }

    /// Serves `agency.teams.TeamList` at `read_your_writes` consistency, from the owed projection.
    pub fn team_list(&self) -> Result<Vec<crate::teams::TeamList>, crate::obligation::UnmetObligation> {
        self.behaviors.team_list()
    }

    /// Serves `agency.teams.WorkforceBoard` at `read_your_writes` consistency, from the owed projection.
    pub fn workforce_board(&self) -> Result<Vec<crate::teams::WorkforceBoard>, crate::obligation::UnmetObligation> {
        self.behaviors.workforce_board()
    }

    /// Serves `agency.tools.BindingList` at `read_your_writes` consistency, from the owed projection.
    pub fn binding_list(&self) -> Result<Vec<crate::tools::BindingList>, crate::obligation::UnmetObligation> {
        self.behaviors.binding_list()
    }

    /// Serves `agency.tools.ToolCatalogue` at `read_your_writes` consistency, from the owed projection.
    pub fn tool_catalogue(&self) -> Result<Vec<crate::tools::ToolCatalogue>, crate::obligation::UnmetObligation> {
        self.behaviors.tool_catalogue()
    }

    /// Serves `agency.work.WorkInputList` at `read_your_writes` consistency, from the owed projection.
    pub fn work_input_list(&self) -> Result<Vec<crate::work::WorkInputList>, crate::obligation::UnmetObligation> {
        self.behaviors.work_input_list()
    }

    /// Serves `agency.work.WorkItemDetail` at `read_your_writes` consistency, from the owed projection.
    pub fn work_item_detail(&self, item_ref: crate::work::WorkItemRef) -> Result<Vec<crate::work::WorkItemDetail>, crate::obligation::UnmetObligation> {
        self.behaviors.work_item_detail(item_ref)
    }

    /// Serves `agency.work.WorkQueue` at `read_your_writes` consistency, from the owed projection.
    pub fn work_queue(&self) -> Result<Vec<crate::work::WorkQueue>, crate::obligation::UnmetObligation> {
        self.behaviors.work_queue()
    }
}
