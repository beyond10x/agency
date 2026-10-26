// generated from agency v1
// model digest 71132e90bde806415603d08bb97a71629b2c2f6cda30e1f22e76f407abd9a3eb
// contract digest a96f55e3e1ca68aeb1579b032c3547fffd27e8ab80e76bcb898f9b6c9258792e
// do not edit: regenerate with `ess synthesize --layout crate`

//! The `agency` system, v1: its components assembled, its bindings wired, and its one transport.
//!
//! The transport is derived from the specification, not chosen: `at_least_once` is the only
//! delivery guarantee the model declares, so published events land on an append-only log and a
//! pump delivers each to every binding that reacts to it. The log is the system's observable
//! record, and so is the record of what each binding invoked. What no specification determines
//! — how an escalation event is filled, behaviour behind the ports — stays an obligation; see
//! the `PLAN.md` beside this workspace.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// An event on the system's log: everything any component publishes, and everything a binding
/// escalates into.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemEvent {
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

impl SystemEvent {
    /// The qualified name the specification declares this event under.
    pub fn name(&self) -> &'static str {
        match self {
            Self::AgentDefinitionCreated(_) => "agency.agents.AgentDefinitionCreated",
            Self::AgentDefinitionDeprecated(_) => "agency.agents.AgentDefinitionDeprecated",
            Self::AgentDefinitionPublished(_) => "agency.agents.AgentDefinitionPublished",
            Self::AgentProvisioned(_) => "agency.agents.AgentProvisioned",
            Self::AgentRetired(_) => "agency.agents.AgentRetired",
            Self::AgentSpecChecked(_) => "agency.agents.AgentSpecChecked",
            Self::AgentSpecRefused(_) => "agency.agents.AgentSpecRefused",
            Self::CredentialGranted(_) => "agency.auth.CredentialGranted",
            Self::CredentialRefreshed(_) => "agency.auth.CredentialRefreshed",
            Self::CredentialRevoked(_) => "agency.auth.CredentialRevoked",
            Self::CredentialStored(_) => "agency.auth.CredentialStored",
            Self::GrantRevoked(_) => "agency.auth.GrantRevoked",
            Self::PolicyActivated(_) => "agency.auth.PolicyActivated",
            Self::PolicyCreated(_) => "agency.auth.PolicyCreated",
            Self::PolicySuperseded(_) => "agency.auth.PolicySuperseded",
            Self::PrincipalDisabled(_) => "agency.auth.PrincipalDisabled",
            Self::PrincipalInvited(_) => "agency.auth.PrincipalInvited",
            Self::RoleAssigned(_) => "agency.auth.RoleAssigned",
            Self::TenantCreated(_) => "agency.auth.TenantCreated",
            Self::RunAdmitted(_) => "agency.runs.RunAdmitted",
            Self::RunApprovalDenied(_) => "agency.runs.RunApprovalDenied",
            Self::RunApprovalGranted(_) => "agency.runs.RunApprovalGranted",
            Self::RunApprovalRequested(_) => "agency.runs.RunApprovalRequested",
            Self::RunBudgetSet(_) => "agency.runs.RunBudgetSet",
            Self::RunFailed(_) => "agency.runs.RunFailed",
            Self::RunProposed(_) => "agency.runs.RunProposed",
            Self::RunRequested(_) => "agency.runs.RunRequested",
            Self::RunStarted(_) => "agency.runs.RunStarted",
            Self::RunStopped(_) => "agency.runs.RunStopped",
            Self::SessionClosed(_) => "agency.service.SessionClosed",
            Self::SessionOpened(_) => "agency.service.SessionOpened",
            Self::LeadSet(_) => "agency.teams.LeadSet",
            Self::MemberAdded(_) => "agency.teams.MemberAdded",
            Self::MemberRemoved(_) => "agency.teams.MemberRemoved",
            Self::MessageSent(_) => "agency.teams.MessageSent",
            Self::QueueAttached(_) => "agency.teams.QueueAttached",
            Self::ReportSent(_) => "agency.teams.ReportSent",
            Self::SwarmDeclared(_) => "agency.teams.SwarmDeclared",
            Self::SwarmStopped(_) => "agency.teams.SwarmStopped",
            Self::TeamDisbanded(_) => "agency.teams.TeamDisbanded",
            Self::TeamFormed(_) => "agency.teams.TeamFormed",
            Self::WorkforceDefined(_) => "agency.teams.WorkforceDefined",
            Self::WorkforceRetired(_) => "agency.teams.WorkforceRetired",
            Self::WorkforceStaffed(_) => "agency.teams.WorkforceStaffed",
            Self::BindingRevoked(_) => "agency.tools.BindingRevoked",
            Self::ToolBound(_) => "agency.tools.ToolBound",
            Self::ToolRegistered(_) => "agency.tools.ToolRegistered",
            Self::ToolWithdrawn(_) => "agency.tools.ToolWithdrawn",
            Self::WorkInputConfigured(_) => "agency.work.WorkInputConfigured",
            Self::WorkInputPaused(_) => "agency.work.WorkInputPaused",
            Self::WorkInputRemoved(_) => "agency.work.WorkInputRemoved",
            Self::WorkItemAbandoned(_) => "agency.work.WorkItemAbandoned",
            Self::WorkItemAssigned(_) => "agency.work.WorkItemAssigned",
            Self::WorkItemReturned(_) => "agency.work.WorkItemReturned",
            Self::WorkItemSettled(_) => "agency.work.WorkItemSettled",
            Self::WorkItemSubmitted(_) => "agency.work.WorkItemSubmitted",
        }
    }
}

impl From<crate::ports::agency::PublishedEvent> for SystemEvent {
    fn from(event: crate::ports::agency::PublishedEvent) -> Self {
        match event {
            crate::ports::agency::PublishedEvent::AgentDefinitionCreated(event) => Self::AgentDefinitionCreated(event),
            crate::ports::agency::PublishedEvent::AgentDefinitionDeprecated(event) => Self::AgentDefinitionDeprecated(event),
            crate::ports::agency::PublishedEvent::AgentDefinitionPublished(event) => Self::AgentDefinitionPublished(event),
            crate::ports::agency::PublishedEvent::AgentProvisioned(event) => Self::AgentProvisioned(event),
            crate::ports::agency::PublishedEvent::AgentRetired(event) => Self::AgentRetired(event),
            crate::ports::agency::PublishedEvent::AgentSpecChecked(event) => Self::AgentSpecChecked(event),
            crate::ports::agency::PublishedEvent::AgentSpecRefused(event) => Self::AgentSpecRefused(event),
            crate::ports::agency::PublishedEvent::CredentialGranted(event) => Self::CredentialGranted(event),
            crate::ports::agency::PublishedEvent::CredentialRefreshed(event) => Self::CredentialRefreshed(event),
            crate::ports::agency::PublishedEvent::CredentialRevoked(event) => Self::CredentialRevoked(event),
            crate::ports::agency::PublishedEvent::CredentialStored(event) => Self::CredentialStored(event),
            crate::ports::agency::PublishedEvent::GrantRevoked(event) => Self::GrantRevoked(event),
            crate::ports::agency::PublishedEvent::PolicyActivated(event) => Self::PolicyActivated(event),
            crate::ports::agency::PublishedEvent::PolicyCreated(event) => Self::PolicyCreated(event),
            crate::ports::agency::PublishedEvent::PolicySuperseded(event) => Self::PolicySuperseded(event),
            crate::ports::agency::PublishedEvent::PrincipalDisabled(event) => Self::PrincipalDisabled(event),
            crate::ports::agency::PublishedEvent::PrincipalInvited(event) => Self::PrincipalInvited(event),
            crate::ports::agency::PublishedEvent::RoleAssigned(event) => Self::RoleAssigned(event),
            crate::ports::agency::PublishedEvent::TenantCreated(event) => Self::TenantCreated(event),
            crate::ports::agency::PublishedEvent::RunAdmitted(event) => Self::RunAdmitted(event),
            crate::ports::agency::PublishedEvent::RunApprovalDenied(event) => Self::RunApprovalDenied(event),
            crate::ports::agency::PublishedEvent::RunApprovalGranted(event) => Self::RunApprovalGranted(event),
            crate::ports::agency::PublishedEvent::RunApprovalRequested(event) => Self::RunApprovalRequested(event),
            crate::ports::agency::PublishedEvent::RunBudgetSet(event) => Self::RunBudgetSet(event),
            crate::ports::agency::PublishedEvent::RunFailed(event) => Self::RunFailed(event),
            crate::ports::agency::PublishedEvent::RunProposed(event) => Self::RunProposed(event),
            crate::ports::agency::PublishedEvent::RunRequested(event) => Self::RunRequested(event),
            crate::ports::agency::PublishedEvent::RunStarted(event) => Self::RunStarted(event),
            crate::ports::agency::PublishedEvent::RunStopped(event) => Self::RunStopped(event),
            crate::ports::agency::PublishedEvent::SessionClosed(event) => Self::SessionClosed(event),
            crate::ports::agency::PublishedEvent::SessionOpened(event) => Self::SessionOpened(event),
            crate::ports::agency::PublishedEvent::LeadSet(event) => Self::LeadSet(event),
            crate::ports::agency::PublishedEvent::MemberAdded(event) => Self::MemberAdded(event),
            crate::ports::agency::PublishedEvent::MemberRemoved(event) => Self::MemberRemoved(event),
            crate::ports::agency::PublishedEvent::MessageSent(event) => Self::MessageSent(event),
            crate::ports::agency::PublishedEvent::QueueAttached(event) => Self::QueueAttached(event),
            crate::ports::agency::PublishedEvent::ReportSent(event) => Self::ReportSent(event),
            crate::ports::agency::PublishedEvent::SwarmDeclared(event) => Self::SwarmDeclared(event),
            crate::ports::agency::PublishedEvent::SwarmStopped(event) => Self::SwarmStopped(event),
            crate::ports::agency::PublishedEvent::TeamDisbanded(event) => Self::TeamDisbanded(event),
            crate::ports::agency::PublishedEvent::TeamFormed(event) => Self::TeamFormed(event),
            crate::ports::agency::PublishedEvent::WorkforceDefined(event) => Self::WorkforceDefined(event),
            crate::ports::agency::PublishedEvent::WorkforceRetired(event) => Self::WorkforceRetired(event),
            crate::ports::agency::PublishedEvent::WorkforceStaffed(event) => Self::WorkforceStaffed(event),
            crate::ports::agency::PublishedEvent::BindingRevoked(event) => Self::BindingRevoked(event),
            crate::ports::agency::PublishedEvent::ToolBound(event) => Self::ToolBound(event),
            crate::ports::agency::PublishedEvent::ToolRegistered(event) => Self::ToolRegistered(event),
            crate::ports::agency::PublishedEvent::ToolWithdrawn(event) => Self::ToolWithdrawn(event),
            crate::ports::agency::PublishedEvent::WorkInputConfigured(event) => Self::WorkInputConfigured(event),
            crate::ports::agency::PublishedEvent::WorkInputPaused(event) => Self::WorkInputPaused(event),
            crate::ports::agency::PublishedEvent::WorkInputRemoved(event) => Self::WorkInputRemoved(event),
            crate::ports::agency::PublishedEvent::WorkItemAbandoned(event) => Self::WorkItemAbandoned(event),
            crate::ports::agency::PublishedEvent::WorkItemAssigned(event) => Self::WorkItemAssigned(event),
            crate::ports::agency::PublishedEvent::WorkItemReturned(event) => Self::WorkItemReturned(event),
            crate::ports::agency::PublishedEvent::WorkItemSettled(event) => Self::WorkItemSettled(event),
            crate::ports::agency::PublishedEvent::WorkItemSubmitted(event) => Self::WorkItemSubmitted(event),
        }
    }
}

/// The `agency` system: every component behind its port, and the transport between them.
///
/// The component fields are public because commands enter the system through a component's own
/// port; the log and its delivery cursor are not, because publishing happens by pumping, not by
/// writing history directly.
pub struct System<AgencyBehaviors> {
    /// The `agency` component.
    pub agency: crate::ports::agency::Agency<AgencyBehaviors>,
    published: Vec<SystemEvent>,
    cursor: usize,
}

impl<AgencyBehaviors> System<AgencyBehaviors> {
    /// Assembles the system from its components.
    pub fn new(agency: crate::ports::agency::Agency<AgencyBehaviors>) -> Self {
        Self {
            agency,
            published: Vec::new(),
            cursor: 0,
        }
    }

    /// Everything published so far, in publication order — the system's observable record.
    pub fn published(&self) -> &[SystemEvent] {
        &self.published
    }

    /// Takes every event the pump has already delivered off the log, in publication order.
    ///
    /// A long-running shell calls this after each `pump`, or the log holds every event the
    /// process ever published. A `pump` returns with every logged event delivered: each
    /// reacting binding has had its attempt, and a binding whose attempt stopped holds the event in
    /// its own held-back list, not on the log. Events published since the last `pump` stay on the
    /// log, so the next `pump` still delivers them; taking never skips a binding.
    pub fn take_published(&mut self) -> Vec<SystemEvent> {
        let delivered: Vec<SystemEvent> = self.published.drain(..self.cursor).collect();
        self.cursor = 0;
        delivered
    }
}

impl<AgencyBehaviors> System<AgencyBehaviors>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    /// Delivers until quiescent: collects every component's outbox onto the log. No binding
    /// reacts to anything this specification publishes, so collecting is the whole delivery.
    pub fn pump(&mut self) -> Result<(), crate::obligation::UnmetObligation> {
        loop {
            self.collect();
            if self.cursor == self.published.len() {
                return Ok(());
            }
            self.cursor += 1;
        }
    }

    /// Moves every component's outbox onto the log, in component order.
    fn collect(&mut self) {
        for event in self.agency.drain_outbox() {
            self.published.push(SystemEvent::from(event));
        }
    }
}
