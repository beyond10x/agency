// generated from agency v1
// model digest 71132e90bde806415603d08bb97a71629b2c2f6cda30e1f22e76f407abd9a3eb
// contract digest a96f55e3e1ca68aeb1579b032c3547fffd27e8ab80e76bcb898f9b6c9258792e
// do not edit: regenerate with `ess synthesize --layout crate`

//! The `agency` component of `agency` v1, on the wire.
//!
//! The specification says this component's callers are not deployed with it, so its surface
//! exists on a wire. Which wire is derived rather than chosen: the one contract this model
//! projects for a command surface is the `OpenAPI` document, and an `OpenAPI` document is an
//! HTTP contract. The document is beside this file, served verbatim at `/openapi.json`.

use crate::server::{entry, http, json, wire};

/// The contract this surface answers, byte for byte as `generated/` commits it.
///
/// Embedded rather than rebuilt at run time: a server that regenerated its own contract could
/// publish one the repository never reviewed.
pub const OPENAPI: &str = include_str!("agency.openapi.json");

/// The prose the same model produced, byte for byte as the documentation projection wrote it.
pub const DOCS: &str = include_str!("agency.docs.md");

/// Every route this surface answers, in path order.
///
/// The same set the `OpenAPI` document declares, plus the two documents about the surface
/// itself, which no specification construct names and nothing can therefore derive. A path
/// absent from this table is answered with `404`, including one the document declares and this
/// table forgot — which is the failure a table computed twice would hide.
pub const ROUTES: &[(&str, &str)] = &[
    ("POST", "/agency/commands/AbandonWorkItem"),
    ("POST", "/agency/commands/ActivatePolicy"),
    ("POST", "/agency/commands/AddMember"),
    ("POST", "/agency/commands/AdmitRun"),
    ("POST", "/agency/commands/AssignRole"),
    ("POST", "/agency/commands/AssignWorkItem"),
    ("POST", "/agency/commands/AttachQueue"),
    ("POST", "/agency/commands/BindTool"),
    ("POST", "/agency/commands/CloseSession"),
    ("POST", "/agency/commands/ConfigureWorkInput"),
    ("POST", "/agency/commands/CreateAgentDefinition"),
    ("POST", "/agency/commands/CreatePolicy"),
    ("POST", "/agency/commands/CreateTenant"),
    ("POST", "/agency/commands/DeclareSwarm"),
    ("POST", "/agency/commands/DefineWorkforce"),
    ("POST", "/agency/commands/DenyRunApproval"),
    ("POST", "/agency/commands/DeprecateAgentDefinition"),
    ("POST", "/agency/commands/DisablePrincipal"),
    ("POST", "/agency/commands/DisbandTeam"),
    ("POST", "/agency/commands/FormTeam"),
    ("POST", "/agency/commands/GrantCredential"),
    ("POST", "/agency/commands/GrantRunApproval"),
    ("POST", "/agency/commands/InvitePrincipal"),
    ("POST", "/agency/commands/OpenSession"),
    ("POST", "/agency/commands/PauseWorkInput"),
    ("POST", "/agency/commands/ProvisionAgent"),
    ("POST", "/agency/commands/PublishAgentDefinition"),
    ("POST", "/agency/commands/RecordRunFailure"),
    ("POST", "/agency/commands/RecordRunProposal"),
    ("POST", "/agency/commands/RecordRunStarted"),
    ("POST", "/agency/commands/RecordSpecCheck"),
    ("POST", "/agency/commands/RefreshCredential"),
    ("POST", "/agency/commands/RegisterTool"),
    ("POST", "/agency/commands/RemoveMember"),
    ("POST", "/agency/commands/RemoveWorkInput"),
    ("POST", "/agency/commands/Report"),
    ("POST", "/agency/commands/RequestRunApproval"),
    ("POST", "/agency/commands/RetireAgent"),
    ("POST", "/agency/commands/RetireWorkforce"),
    ("POST", "/agency/commands/ReturnWorkItem"),
    ("POST", "/agency/commands/RevokeBinding"),
    ("POST", "/agency/commands/RevokeCredential"),
    ("POST", "/agency/commands/RevokeGrant"),
    ("POST", "/agency/commands/Send"),
    ("POST", "/agency/commands/SetLead"),
    ("POST", "/agency/commands/SetRunBudget"),
    ("POST", "/agency/commands/SettleWorkItem"),
    ("POST", "/agency/commands/StaffWorkforce"),
    ("POST", "/agency/commands/StartRun"),
    ("POST", "/agency/commands/StopRun"),
    ("POST", "/agency/commands/StopSwarm"),
    ("POST", "/agency/commands/StoreCredential"),
    ("POST", "/agency/commands/SubmitWorkItem"),
    ("POST", "/agency/commands/SupersedePolicy"),
    ("POST", "/agency/commands/WithdrawTool"),
    ("GET", "/agency/views/AgentDefinitionList"),
    ("GET", "/agency/views/AgentList"),
    ("GET", "/agency/views/ApprovalInbox"),
    ("GET", "/agency/views/BindingList"),
    ("GET", "/agency/views/CredentialList"),
    ("GET", "/agency/views/EffectivePolicy"),
    ("GET", "/agency/views/GrantMatrix"),
    ("GET", "/agency/views/PolicyList"),
    ("GET", "/agency/views/PrincipalList"),
    ("GET", "/agency/views/RunDetail"),
    ("GET", "/agency/views/RunList"),
    ("GET", "/agency/views/SessionList"),
    ("GET", "/agency/views/SwarmList"),
    ("GET", "/agency/views/TeamDetail"),
    ("GET", "/agency/views/TeamList"),
    ("GET", "/agency/views/ToolCatalogue"),
    ("GET", "/agency/views/UsageByAgent"),
    ("GET", "/agency/views/WorkInputList"),
    ("GET", "/agency/views/WorkItemDetail"),
    ("GET", "/agency/views/WorkQueue"),
    ("GET", "/agency/views/WorkforceBoard"),
    ("GET", "/docs"),
    ("GET", "/openapi.json"),
];

/// What this process says about itself as it starts, before it answers anything.
///
/// Three lines of JSON on standard output, in this order, every member of them derived from the
/// specification — except `runtime`, which is appended by the emitted code below and holds what
/// is true of *this process*: the language it was synthesised into, and the address it bound.
/// Everything outside `runtime` is the same in every language this plan is emitted into, and
/// `cargo xtask synth --check` starts both and compares them.
pub const STARTUP: &[&str] = &[
    "{\"log\":\"ess/1\",\"event\":\"system.starting\",\"system\":\"agency\",\"version\":\"v1\",\"model_digest\":\"71132e90bde806415603d08bb97a71629b2c2f6cda30e1f22e76f407abd9a3eb\",\"contract_digest\":\"a96f55e3e1ca68aeb1579b032c3547fffd27e8ab80e76bcb898f9b6c9258792e\",\"components\":[\"agency\"],\"capabilities\":{\"generated\":323,\"obligations\":5,\"refused\":0}",
    "{\"log\":\"ess/1\",\"event\":\"surface.serving\",\"component\":\"agency\",\"reached_by\":\"network\",\"transport\":\"http/1.1\",\"routes\":78,\"paths\":[{\"method\":\"POST\",\"path\":\"/agency/commands/AbandonWorkItem\",\"serves\":\"command\",\"name\":\"agency.work.AbandonWorkItem\"},{\"method\":\"POST\",\"path\":\"/agency/commands/ActivatePolicy\",\"serves\":\"command\",\"name\":\"agency.auth.ActivatePolicy\"},{\"method\":\"POST\",\"path\":\"/agency/commands/AddMember\",\"serves\":\"command\",\"name\":\"agency.teams.AddMember\"},{\"method\":\"POST\",\"path\":\"/agency/commands/AdmitRun\",\"serves\":\"command\",\"name\":\"agency.runs.AdmitRun\"},{\"method\":\"POST\",\"path\":\"/agency/commands/AssignRole\",\"serves\":\"command\",\"name\":\"agency.auth.AssignRole\"},{\"method\":\"POST\",\"path\":\"/agency/commands/AssignWorkItem\",\"serves\":\"command\",\"name\":\"agency.work.AssignWorkItem\"},{\"method\":\"POST\",\"path\":\"/agency/commands/AttachQueue\",\"serves\":\"command\",\"name\":\"agency.teams.AttachQueue\"},{\"method\":\"POST\",\"path\":\"/agency/commands/BindTool\",\"serves\":\"command\",\"name\":\"agency.tools.BindTool\"},{\"method\":\"POST\",\"path\":\"/agency/commands/CloseSession\",\"serves\":\"command\",\"name\":\"agency.service.CloseSession\"},{\"method\":\"POST\",\"path\":\"/agency/commands/ConfigureWorkInput\",\"serves\":\"command\",\"name\":\"agency.work.ConfigureWorkInput\"},{\"method\":\"POST\",\"path\":\"/agency/commands/CreateAgentDefinition\",\"serves\":\"command\",\"name\":\"agency.agents.CreateAgentDefinition\"},{\"method\":\"POST\",\"path\":\"/agency/commands/CreatePolicy\",\"serves\":\"command\",\"name\":\"agency.auth.CreatePolicy\"},{\"method\":\"POST\",\"path\":\"/agency/commands/CreateTenant\",\"serves\":\"command\",\"name\":\"agency.auth.CreateTenant\"},{\"method\":\"POST\",\"path\":\"/agency/commands/DeclareSwarm\",\"serves\":\"command\",\"name\":\"agency.teams.DeclareSwarm\"},{\"method\":\"POST\",\"path\":\"/agency/commands/DefineWorkforce\",\"serves\":\"command\",\"name\":\"agency.teams.DefineWorkforce\"},{\"method\":\"POST\",\"path\":\"/agency/commands/DenyRunApproval\",\"serves\":\"command\",\"name\":\"agency.runs.DenyRunApproval\"},{\"method\":\"POST\",\"path\":\"/agency/commands/DeprecateAgentDefinition\",\"serves\":\"command\",\"name\":\"agency.agents.DeprecateAgentDefinition\"},{\"method\":\"POST\",\"path\":\"/agency/commands/DisablePrincipal\",\"serves\":\"command\",\"name\":\"agency.auth.DisablePrincipal\"},{\"method\":\"POST\",\"path\":\"/agency/commands/DisbandTeam\",\"serves\":\"command\",\"name\":\"agency.teams.DisbandTeam\"},{\"method\":\"POST\",\"path\":\"/agency/commands/FormTeam\",\"serves\":\"command\",\"name\":\"agency.teams.FormTeam\"},{\"method\":\"POST\",\"path\":\"/agency/commands/GrantCredential\",\"serves\":\"command\",\"name\":\"agency.auth.GrantCredential\"},{\"method\":\"POST\",\"path\":\"/agency/commands/GrantRunApproval\",\"serves\":\"command\",\"name\":\"agency.runs.GrantRunApproval\"},{\"method\":\"POST\",\"path\":\"/agency/commands/InvitePrincipal\",\"serves\":\"command\",\"name\":\"agency.auth.InvitePrincipal\"},{\"method\":\"POST\",\"path\":\"/agency/commands/OpenSession\",\"serves\":\"command\",\"name\":\"agency.service.OpenSession\"},{\"method\":\"POST\",\"path\":\"/agency/commands/PauseWorkInput\",\"serves\":\"command\",\"name\":\"agency.work.PauseWorkInput\"},{\"method\":\"POST\",\"path\":\"/agency/commands/ProvisionAgent\",\"serves\":\"command\",\"name\":\"agency.agents.ProvisionAgent\"},{\"method\":\"POST\",\"path\":\"/agency/commands/PublishAgentDefinition\",\"serves\":\"command\",\"name\":\"agency.agents.PublishAgentDefinition\"},{\"method\":\"POST\",\"path\":\"/agency/commands/RecordRunFailure\",\"serves\":\"command\",\"name\":\"agency.runs.RecordRunFailure\"},{\"method\":\"POST\",\"path\":\"/agency/commands/RecordRunProposal\",\"serves\":\"command\",\"name\":\"agency.runs.RecordRunProposal\"},{\"method\":\"POST\",\"path\":\"/agency/commands/RecordRunStarted\",\"serves\":\"command\",\"name\":\"agency.runs.RecordRunStarted\"},{\"method\":\"POST\",\"path\":\"/agency/commands/RecordSpecCheck\",\"serves\":\"command\",\"name\":\"agency.agents.RecordSpecCheck\"},{\"method\":\"POST\",\"path\":\"/agency/commands/RefreshCredential\",\"serves\":\"command\",\"name\":\"agency.auth.RefreshCredential\"},{\"method\":\"POST\",\"path\":\"/agency/commands/RegisterTool\",\"serves\":\"command\",\"name\":\"agency.tools.RegisterTool\"},{\"method\":\"POST\",\"path\":\"/agency/commands/RemoveMember\",\"serves\":\"command\",\"name\":\"agency.teams.RemoveMember\"},{\"method\":\"POST\",\"path\":\"/agency/commands/RemoveWorkInput\",\"serves\":\"command\",\"name\":\"agency.work.RemoveWorkInput\"},{\"method\":\"POST\",\"path\":\"/agency/commands/Report\",\"serves\":\"command\",\"name\":\"agency.teams.Report\"},{\"method\":\"POST\",\"path\":\"/agency/commands/RequestRunApproval\",\"serves\":\"command\",\"name\":\"agency.runs.RequestRunApproval\"},{\"method\":\"POST\",\"path\":\"/agency/commands/RetireAgent\",\"serves\":\"command\",\"name\":\"agency.agents.RetireAgent\"},{\"method\":\"POST\",\"path\":\"/agency/commands/RetireWorkforce\",\"serves\":\"command\",\"name\":\"agency.teams.RetireWorkforce\"},{\"method\":\"POST\",\"path\":\"/agency/commands/ReturnWorkItem\",\"serves\":\"command\",\"name\":\"agency.work.ReturnWorkItem\"},{\"method\":\"POST\",\"path\":\"/agency/commands/RevokeBinding\",\"serves\":\"command\",\"name\":\"agency.tools.RevokeBinding\"},{\"method\":\"POST\",\"path\":\"/agency/commands/RevokeCredential\",\"serves\":\"command\",\"name\":\"agency.auth.RevokeCredential\"},{\"method\":\"POST\",\"path\":\"/agency/commands/RevokeGrant\",\"serves\":\"command\",\"name\":\"agency.auth.RevokeGrant\"},{\"method\":\"POST\",\"path\":\"/agency/commands/Send\",\"serves\":\"command\",\"name\":\"agency.teams.Send\"},{\"method\":\"POST\",\"path\":\"/agency/commands/SetLead\",\"serves\":\"command\",\"name\":\"agency.teams.SetLead\"},{\"method\":\"POST\",\"path\":\"/agency/commands/SetRunBudget\",\"serves\":\"command\",\"name\":\"agency.runs.SetRunBudget\"},{\"method\":\"POST\",\"path\":\"/agency/commands/SettleWorkItem\",\"serves\":\"command\",\"name\":\"agency.work.SettleWorkItem\"},{\"method\":\"POST\",\"path\":\"/agency/commands/StaffWorkforce\",\"serves\":\"command\",\"name\":\"agency.teams.StaffWorkforce\"},{\"method\":\"POST\",\"path\":\"/agency/commands/StartRun\",\"serves\":\"command\",\"name\":\"agency.runs.StartRun\"},{\"method\":\"POST\",\"path\":\"/agency/commands/StopRun\",\"serves\":\"command\",\"name\":\"agency.runs.StopRun\"},{\"method\":\"POST\",\"path\":\"/agency/commands/StopSwarm\",\"serves\":\"command\",\"name\":\"agency.teams.StopSwarm\"},{\"method\":\"POST\",\"path\":\"/agency/commands/StoreCredential\",\"serves\":\"command\",\"name\":\"agency.auth.StoreCredential\"},{\"method\":\"POST\",\"path\":\"/agency/commands/SubmitWorkItem\",\"serves\":\"command\",\"name\":\"agency.work.SubmitWorkItem\"},{\"method\":\"POST\",\"path\":\"/agency/commands/SupersedePolicy\",\"serves\":\"command\",\"name\":\"agency.auth.SupersedePolicy\"},{\"method\":\"POST\",\"path\":\"/agency/commands/WithdrawTool\",\"serves\":\"command\",\"name\":\"agency.tools.WithdrawTool\"},{\"method\":\"GET\",\"path\":\"/agency/views/AgentDefinitionList\",\"serves\":\"view\",\"name\":\"agency.agents.AgentDefinitionList\"},{\"method\":\"GET\",\"path\":\"/agency/views/AgentList\",\"serves\":\"view\",\"name\":\"agency.agents.AgentList\"},{\"method\":\"GET\",\"path\":\"/agency/views/ApprovalInbox\",\"serves\":\"view\",\"name\":\"agency.runs.ApprovalInbox\"},{\"method\":\"GET\",\"path\":\"/agency/views/BindingList\",\"serves\":\"view\",\"name\":\"agency.tools.BindingList\"},{\"method\":\"GET\",\"path\":\"/agency/views/CredentialList\",\"serves\":\"view\",\"name\":\"agency.auth.CredentialList\"},{\"method\":\"GET\",\"path\":\"/agency/views/EffectivePolicy\",\"serves\":\"view\",\"name\":\"agency.auth.EffectivePolicy\"},{\"method\":\"GET\",\"path\":\"/agency/views/GrantMatrix\",\"serves\":\"view\",\"name\":\"agency.auth.GrantMatrix\"},{\"method\":\"GET\",\"path\":\"/agency/views/PolicyList\",\"serves\":\"view\",\"name\":\"agency.auth.PolicyList\"},{\"method\":\"GET\",\"path\":\"/agency/views/PrincipalList\",\"serves\":\"view\",\"name\":\"agency.auth.PrincipalList\"},{\"method\":\"GET\",\"path\":\"/agency/views/RunDetail\",\"serves\":\"view\",\"name\":\"agency.runs.RunDetail\"},{\"method\":\"GET\",\"path\":\"/agency/views/RunList\",\"serves\":\"view\",\"name\":\"agency.runs.RunList\"},{\"method\":\"GET\",\"path\":\"/agency/views/SessionList\",\"serves\":\"view\",\"name\":\"agency.service.SessionList\"},{\"method\":\"GET\",\"path\":\"/agency/views/SwarmList\",\"serves\":\"view\",\"name\":\"agency.teams.SwarmList\"},{\"method\":\"GET\",\"path\":\"/agency/views/TeamDetail\",\"serves\":\"view\",\"name\":\"agency.teams.TeamDetail\"},{\"method\":\"GET\",\"path\":\"/agency/views/TeamList\",\"serves\":\"view\",\"name\":\"agency.teams.TeamList\"},{\"method\":\"GET\",\"path\":\"/agency/views/ToolCatalogue\",\"serves\":\"view\",\"name\":\"agency.tools.ToolCatalogue\"},{\"method\":\"GET\",\"path\":\"/agency/views/UsageByAgent\",\"serves\":\"view\",\"name\":\"agency.runs.UsageByAgent\"},{\"method\":\"GET\",\"path\":\"/agency/views/WorkInputList\",\"serves\":\"view\",\"name\":\"agency.work.WorkInputList\"},{\"method\":\"GET\",\"path\":\"/agency/views/WorkItemDetail\",\"serves\":\"view\",\"name\":\"agency.work.WorkItemDetail\"},{\"method\":\"GET\",\"path\":\"/agency/views/WorkQueue\",\"serves\":\"view\",\"name\":\"agency.work.WorkQueue\"},{\"method\":\"GET\",\"path\":\"/agency/views/WorkforceBoard\",\"serves\":\"view\",\"name\":\"agency.teams.WorkforceBoard\"},{\"method\":\"GET\",\"path\":\"/docs\",\"serves\":\"documentation\",\"name\":\"docs\"},{\"method\":\"GET\",\"path\":\"/openapi.json\",\"serves\":\"contract\",\"name\":\"openapi\"}]",
    "{\"log\":\"ess/1\",\"event\":\"system.ready\",\"system\":\"agency\",\"surfaces\":1",
];

/// Writes the startup record, with this process's own facts closing each line.
fn announce(address: &std::net::SocketAddr) {
    for facts in STARTUP {
        let mut line = String::from(*facts);
        line.push_str(",\"runtime\":{\"address\":");
        json::push_text(&mut line, &address.to_string());
        line.push_str(",\"language\":\"rust\",\"port\":");
        json::push_integer(&mut line, i64::from(address.port()));
        line.push_str("}}");
        println!("{line}");
    }
}

/// Serves `agency` at `address`, and does not return while it can answer.
///
/// `address` may name port `0`, which binds an ephemeral port; the startup record says which one
/// was taken, because a caller that cannot learn the port cannot make a request.
///
/// It chooses no realization. Every command reaches the port, and a port over unimplemented
/// obligations answers the typed refusal this surface reports as `501` — the honest empty
/// state rather than a server that pretends.
///
/// `authenticate` is the realization's: it says who each request was sent by, or `None`, and
/// [`dispatch`] checks that caller's grant before the command runs. Nothing here reads an
/// actor from the request itself.
///
/// One connection at a time: each is dropped after [`http::READ_TIMEOUT`] without a byte, or
/// [`http::WRITE_TIMEOUT`] of a stalled write, and whatever fails on one connection — a caller
/// that hung up before reading its answer, a reset, a failed accept — ends that connection only.
///
/// # Errors
///
/// What binding the address refuses: the address is taken, or the port is privileged.
pub fn serve<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, address: &str, authenticate: impl Fn(&http::Request) -> Option<crate::actor::Caller>) -> std::io::Result<()>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{ serve_with_static(system, address, authenticate, None) }

/// Serves the declared surface, with files only for paths outside its route table.
///
/// # Errors
/// Returns a listener or static-root error before announcing readiness.
pub fn serve_with_static<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, address: &str, authenticate: impl Fn(&http::Request) -> Option<crate::actor::Caller>, static_root: Option<&std::path::Path>) -> std::io::Result<()>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let static_root = static_root.map(std::fs::canonicalize).transpose()?;
    if static_root.as_ref().is_some_and(|root| !root.is_dir()) { return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "static root is not a directory")); }
    let listener = std::net::TcpListener::bind(address)?;
    announce(&listener.local_addr()?);
    for connection in listener.incoming() {
        // What fails on one connection ends that connection, never this loop: a caller that
        // gave up before it was accepted, or before it read its answer, is that caller's affair.
        let Ok(connection) = connection else {
            // An accept the listener itself failed (no descriptor left) would fail again at
            // once: pause before the next.
            std::thread::sleep(std::time::Duration::from_millis(10));
            continue;
        };
        let _ = connection.set_read_timeout(Some(http::READ_TIMEOUT));
        let _ = connection.set_write_timeout(Some(http::WRITE_TIMEOUT));
        let mut reader = std::io::BufReader::new(connection);
        let (answer, refused) = match http::read(&mut reader) {
            Ok(request) => {
                if !ROUTES.iter().any(|(_, path)| *path == request.path) {
                    if let Some(root) = &static_root {
                        let _ = crate::server::static_assets::answer(reader.get_mut(), root, &request);
                        continue;
                    }
                }
                (dispatch(system, authenticate(&request).as_ref(), &request), false)
            },
            Err(refusal) => (refusal, true),
        };
        let mut stream = reader.into_inner();
        if http::write(&mut stream, &answer).is_ok() && refused {
            http::linger(&mut stream);
        }
    }
    Ok(())
}

/// Answers one request.
///
/// A path this table does not hold is a `404` naming where the whole table is published; a
/// path it holds under a different method is a `405` naming the one it answers. Neither is a
/// status the contract declares, and neither should be: both are facts about a transport rather
/// than about any command.
///
/// Public so a caller can hand it a request it built itself: [`serve`] is this function behind a
/// socket, and nothing else.
///
/// `caller` is who the realization authenticated the request as, or `None`. Every command
/// checks its grant before it runs, and answers the standard refusal when the caller is none
/// or is an actor the specification does not grant the command.
/// A view some actor's grant names checks the reader's grant the same way before it is read.
pub fn dispatch<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, caller: Option<&crate::actor::Caller>, request: &http::Request) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    dispatch_with_events(system, caller, request).0
}

/// Answers one request as [`dispatch`] does, and hands back every event published while it
/// ran, in log order: what the command published, what the bindings that reacted to it
/// published in turn, and what a held-back delivery retried during the call published. An
/// event already on the system's log when the call started is not the call's and is never
/// in the list. The list is empty where no command ran — a refusal before the port, a view,
/// a document or a path no route matches — and holds what was published where delivering
/// it failed after the command took effect. An event a component's port published through a
/// direct call is in that component's outbox until a pump collects it, so one no pump has
/// collected yet is listed as this call's: call `pump` after calling a port directly.
pub fn dispatch_with_events<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, caller: Option<&crate::actor::Caller>, request: &http::Request) -> (http::Response, Vec<crate::system::SystemEvent>)
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let mut taken = Vec::new();
    let answer = route_request(system, caller, request, &mut taken);
    (answer, taken)
}

/// The route match behind [`dispatch_with_events`]: each command pushes what it took off the
/// log onto `taken`.
fn route_request<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, caller: Option<&crate::actor::Caller>, request: &http::Request, taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    match request.path.as_str() {
        "/agency/commands/AbandonWorkItem" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.work.AbandonWorkItem") {
                return not_granted(actor);
            }
            serve_agency_work_abandon_work_item(system, &request.body, taken)
        }
        "/agency/commands/ActivatePolicy" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.auth.ActivatePolicy") {
                return not_granted(actor);
            }
            serve_agency_auth_activate_policy(system, &request.body, taken)
        }
        "/agency/commands/AddMember" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.teams.AddMember") {
                return not_granted(actor);
            }
            serve_agency_teams_add_member(system, &request.body, taken)
        }
        "/agency/commands/AdmitRun" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.runs.AdmitRun") {
                return not_granted(actor);
            }
            serve_agency_runs_admit_run(system, &request.body, taken)
        }
        "/agency/commands/AssignRole" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.auth.AssignRole") {
                return not_granted(actor);
            }
            serve_agency_auth_assign_role(system, &request.body, taken)
        }
        "/agency/commands/AssignWorkItem" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.work.AssignWorkItem") {
                return not_granted(actor);
            }
            serve_agency_work_assign_work_item(system, &request.body, taken)
        }
        "/agency/commands/AttachQueue" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.teams.AttachQueue") {
                return not_granted(actor);
            }
            serve_agency_teams_attach_queue(system, &request.body, taken)
        }
        "/agency/commands/BindTool" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.tools.BindTool") {
                return not_granted(actor);
            }
            serve_agency_tools_bind_tool(system, &request.body, taken)
        }
        "/agency/commands/CloseSession" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.service.CloseSession") {
                return not_granted(actor);
            }
            serve_agency_service_close_session(system, &request.body, taken)
        }
        "/agency/commands/ConfigureWorkInput" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.work.ConfigureWorkInput") {
                return not_granted(actor);
            }
            serve_agency_work_configure_work_input(system, &request.body, taken)
        }
        "/agency/commands/CreateAgentDefinition" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.agents.CreateAgentDefinition") {
                return not_granted(actor);
            }
            serve_agency_agents_create_agent_definition(system, &request.body, taken)
        }
        "/agency/commands/CreatePolicy" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.auth.CreatePolicy") {
                return not_granted(actor);
            }
            serve_agency_auth_create_policy(system, &request.body, taken)
        }
        "/agency/commands/CreateTenant" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.auth.CreateTenant") {
                return not_granted(actor);
            }
            serve_agency_auth_create_tenant(system, &request.body, taken)
        }
        "/agency/commands/DeclareSwarm" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.teams.DeclareSwarm") {
                return not_granted(actor);
            }
            serve_agency_teams_declare_swarm(system, &request.body, taken)
        }
        "/agency/commands/DefineWorkforce" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.teams.DefineWorkforce") {
                return not_granted(actor);
            }
            serve_agency_teams_define_workforce(system, &request.body, taken)
        }
        "/agency/commands/DenyRunApproval" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.runs.DenyRunApproval") {
                return not_granted(actor);
            }
            serve_agency_runs_deny_run_approval(system, &request.body, taken)
        }
        "/agency/commands/DeprecateAgentDefinition" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.agents.DeprecateAgentDefinition") {
                return not_granted(actor);
            }
            serve_agency_agents_deprecate_agent_definition(system, &request.body, taken)
        }
        "/agency/commands/DisablePrincipal" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.auth.DisablePrincipal") {
                return not_granted(actor);
            }
            serve_agency_auth_disable_principal(system, &request.body, taken)
        }
        "/agency/commands/DisbandTeam" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.teams.DisbandTeam") {
                return not_granted(actor);
            }
            serve_agency_teams_disband_team(system, &request.body, taken)
        }
        "/agency/commands/FormTeam" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.teams.FormTeam") {
                return not_granted(actor);
            }
            serve_agency_teams_form_team(system, &request.body, taken)
        }
        "/agency/commands/GrantCredential" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.auth.GrantCredential") {
                return not_granted(actor);
            }
            serve_agency_auth_grant_credential(system, &request.body, taken)
        }
        "/agency/commands/GrantRunApproval" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.runs.GrantRunApproval") {
                return not_granted(actor);
            }
            serve_agency_runs_grant_run_approval(system, &request.body, taken)
        }
        "/agency/commands/InvitePrincipal" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.auth.InvitePrincipal") {
                return not_granted(actor);
            }
            serve_agency_auth_invite_principal(system, &request.body, taken)
        }
        "/agency/commands/OpenSession" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.service.OpenSession") {
                return not_granted(actor);
            }
            serve_agency_service_open_session(system, &request.body, taken)
        }
        "/agency/commands/PauseWorkInput" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.work.PauseWorkInput") {
                return not_granted(actor);
            }
            serve_agency_work_pause_work_input(system, &request.body, taken)
        }
        "/agency/commands/ProvisionAgent" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.agents.ProvisionAgent") {
                return not_granted(actor);
            }
            serve_agency_agents_provision_agent(system, &request.body, taken)
        }
        "/agency/commands/PublishAgentDefinition" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.agents.PublishAgentDefinition") {
                return not_granted(actor);
            }
            serve_agency_agents_publish_agent_definition(system, &request.body, taken)
        }
        "/agency/commands/RecordRunFailure" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.runs.RecordRunFailure") {
                return not_granted(actor);
            }
            serve_agency_runs_record_run_failure(system, &request.body, taken)
        }
        "/agency/commands/RecordRunProposal" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.runs.RecordRunProposal") {
                return not_granted(actor);
            }
            serve_agency_runs_record_run_proposal(system, &request.body, taken)
        }
        "/agency/commands/RecordRunStarted" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.runs.RecordRunStarted") {
                return not_granted(actor);
            }
            serve_agency_runs_record_run_started(system, &request.body, taken)
        }
        "/agency/commands/RecordSpecCheck" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.agents.RecordSpecCheck") {
                return not_granted(actor);
            }
            serve_agency_agents_record_spec_check(system, &request.body, taken)
        }
        "/agency/commands/RefreshCredential" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.auth.RefreshCredential") {
                return not_granted(actor);
            }
            serve_agency_auth_refresh_credential(system, &request.body, taken)
        }
        "/agency/commands/RegisterTool" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.tools.RegisterTool") {
                return not_granted(actor);
            }
            serve_agency_tools_register_tool(system, &request.body, taken)
        }
        "/agency/commands/RemoveMember" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.teams.RemoveMember") {
                return not_granted(actor);
            }
            serve_agency_teams_remove_member(system, &request.body, taken)
        }
        "/agency/commands/RemoveWorkInput" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.work.RemoveWorkInput") {
                return not_granted(actor);
            }
            serve_agency_work_remove_work_input(system, &request.body, taken)
        }
        "/agency/commands/Report" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.teams.Report") {
                return not_granted(actor);
            }
            serve_agency_teams_report(system, &request.body, taken)
        }
        "/agency/commands/RequestRunApproval" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.runs.RequestRunApproval") {
                return not_granted(actor);
            }
            serve_agency_runs_request_run_approval(system, &request.body, taken)
        }
        "/agency/commands/RetireAgent" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.agents.RetireAgent") {
                return not_granted(actor);
            }
            serve_agency_agents_retire_agent(system, &request.body, taken)
        }
        "/agency/commands/RetireWorkforce" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.teams.RetireWorkforce") {
                return not_granted(actor);
            }
            serve_agency_teams_retire_workforce(system, &request.body, taken)
        }
        "/agency/commands/ReturnWorkItem" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.work.ReturnWorkItem") {
                return not_granted(actor);
            }
            serve_agency_work_return_work_item(system, &request.body, taken)
        }
        "/agency/commands/RevokeBinding" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.tools.RevokeBinding") {
                return not_granted(actor);
            }
            serve_agency_tools_revoke_binding(system, &request.body, taken)
        }
        "/agency/commands/RevokeCredential" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.auth.RevokeCredential") {
                return not_granted(actor);
            }
            serve_agency_auth_revoke_credential(system, &request.body, taken)
        }
        "/agency/commands/RevokeGrant" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.auth.RevokeGrant") {
                return not_granted(actor);
            }
            serve_agency_auth_revoke_grant(system, &request.body, taken)
        }
        "/agency/commands/Send" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.teams.Send") {
                return not_granted(actor);
            }
            serve_agency_teams_send(system, &request.body, taken)
        }
        "/agency/commands/SetLead" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.teams.SetLead") {
                return not_granted(actor);
            }
            serve_agency_teams_set_lead(system, &request.body, taken)
        }
        "/agency/commands/SetRunBudget" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.runs.SetRunBudget") {
                return not_granted(actor);
            }
            serve_agency_runs_set_run_budget(system, &request.body, taken)
        }
        "/agency/commands/SettleWorkItem" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.work.SettleWorkItem") {
                return not_granted(actor);
            }
            serve_agency_work_settle_work_item(system, &request.body, taken)
        }
        "/agency/commands/StaffWorkforce" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.teams.StaffWorkforce") {
                return not_granted(actor);
            }
            serve_agency_teams_staff_workforce(system, &request.body, taken)
        }
        "/agency/commands/StartRun" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.runs.StartRun") {
                return not_granted(actor);
            }
            serve_agency_runs_start_run(system, &request.body, taken)
        }
        "/agency/commands/StopRun" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.runs.StopRun") {
                return not_granted(actor);
            }
            serve_agency_runs_stop_run(system, &request.body, taken)
        }
        "/agency/commands/StopSwarm" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.teams.StopSwarm") {
                return not_granted(actor);
            }
            serve_agency_teams_stop_swarm(system, &request.body, taken)
        }
        "/agency/commands/StoreCredential" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.auth.StoreCredential") {
                return not_granted(actor);
            }
            serve_agency_auth_store_credential(system, &request.body, taken)
        }
        "/agency/commands/SubmitWorkItem" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.work.SubmitWorkItem") {
                return not_granted(actor);
            }
            serve_agency_work_submit_work_item(system, &request.body, taken)
        }
        "/agency/commands/SupersedePolicy" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.auth.SupersedePolicy") {
                return not_granted(actor);
            }
            serve_agency_auth_supersede_policy(system, &request.body, taken)
        }
        "/agency/commands/WithdrawTool" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agency.tools.WithdrawTool") {
                return not_granted(actor);
            }
            serve_agency_tools_withdraw_tool(system, &request.body, taken)
        }
        "/agency/views/AgentDefinitionList" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agency.agents.AgentDefinitionList") {
                return not_granted(actor);
            }
            http::answer(run_agency_agents_agent_definition_list(system))
        }
        "/agency/views/AgentList" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agency.agents.AgentList") {
                return not_granted(actor);
            }
            http::answer(run_agency_agents_agent_list(system))
        }
        "/agency/views/ApprovalInbox" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agency.runs.ApprovalInbox") {
                return not_granted(actor);
            }
            http::answer(run_agency_runs_approval_inbox(system))
        }
        "/agency/views/BindingList" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agency.tools.BindingList") {
                return not_granted(actor);
            }
            http::answer(run_agency_tools_binding_list(system))
        }
        "/agency/views/CredentialList" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agency.auth.CredentialList") {
                return not_granted(actor);
            }
            http::answer(run_agency_auth_credential_list(system))
        }
        "/agency/views/EffectivePolicy" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agency.auth.EffectivePolicy") {
                return not_granted(actor);
            }
            serve_agency_auth_effective_policy(system, &request.query)
        }
        "/agency/views/GrantMatrix" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agency.auth.GrantMatrix") {
                return not_granted(actor);
            }
            http::answer(run_agency_auth_grant_matrix(system))
        }
        "/agency/views/PolicyList" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agency.auth.PolicyList") {
                return not_granted(actor);
            }
            http::answer(run_agency_auth_policy_list(system))
        }
        "/agency/views/PrincipalList" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agency.auth.PrincipalList") {
                return not_granted(actor);
            }
            http::answer(run_agency_auth_principal_list(system))
        }
        "/agency/views/RunDetail" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agency.runs.RunDetail") {
                return not_granted(actor);
            }
            serve_agency_runs_run_detail(system, &request.query)
        }
        "/agency/views/RunList" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agency.runs.RunList") {
                return not_granted(actor);
            }
            http::answer(run_agency_runs_run_list(system))
        }
        "/agency/views/SessionList" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agency.service.SessionList") {
                return not_granted(actor);
            }
            http::answer(run_agency_service_session_list(system))
        }
        "/agency/views/SwarmList" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agency.teams.SwarmList") {
                return not_granted(actor);
            }
            http::answer(run_agency_teams_swarm_list(system))
        }
        "/agency/views/TeamDetail" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agency.teams.TeamDetail") {
                return not_granted(actor);
            }
            serve_agency_teams_team_detail(system, &request.query)
        }
        "/agency/views/TeamList" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agency.teams.TeamList") {
                return not_granted(actor);
            }
            http::answer(run_agency_teams_team_list(system))
        }
        "/agency/views/ToolCatalogue" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agency.tools.ToolCatalogue") {
                return not_granted(actor);
            }
            http::answer(run_agency_tools_tool_catalogue(system))
        }
        "/agency/views/UsageByAgent" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agency.runs.UsageByAgent") {
                return not_granted(actor);
            }
            http::answer(run_agency_runs_usage_by_agent(system))
        }
        "/agency/views/WorkInputList" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agency.work.WorkInputList") {
                return not_granted(actor);
            }
            http::answer(run_agency_work_work_input_list(system))
        }
        "/agency/views/WorkItemDetail" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agency.work.WorkItemDetail") {
                return not_granted(actor);
            }
            serve_agency_work_work_item_detail(system, &request.query)
        }
        "/agency/views/WorkQueue" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agency.work.WorkQueue") {
                return not_granted(actor);
            }
            http::answer(run_agency_work_work_queue(system))
        }
        "/agency/views/WorkforceBoard" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agency.teams.WorkforceBoard") {
                return not_granted(actor);
            }
            http::answer(run_agency_teams_workforce_board(system))
        }
        "/docs" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            http::Response::new(200, http::MARKDOWN, DOCS)
        }
        "/openapi.json" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            http::Response::new(200, http::JSON, OPENAPI)
        }
        other => http::Response::refusal(
            404,
            &format!("`{other}` is not a path this surface declares; `GET /openapi.json` publishes every one that is"),
        ),
    }
}

/// Runs one command or view of `agency` by its qualified name, with no transport.
///
/// The same decoding, refusals and rendering the HTTP routes use — each route and this function call
/// one `run_*` function — so a conformance runner or an in-process caller drives the system
/// without a socket and without a dispatch table of its own. `Ok` is the declared outcome, as the
/// route's body renders it; a view without parameters ignores `input`, as its `GET` route ignores a body, and a view with
/// parameters reads them from `input`, an object keyed by their wire names, as its route reads
/// them from the query string.
///
/// # Errors
///
/// [`entry::Refused::Unknown`] naming `name` when this surface declares no command or view
/// by it; [`entry::Refused::Input`] when `input` is not the command's declared input (the
/// route's `400`); [`entry::Refused::Unmet`] when the port reports an unmet obligation, and
/// [`entry::Refused::Undelivered`] when the command took effect and delivering what it published
/// failed (the route's `501`, with `committed` `false` and `true`).
/// [`entry::Refused::NotGranted`] when `caller` — who the realization authenticated the call
/// as — is none, or is an actor the specification does not grant the command; checked before
/// the command runs (the route's `403`).
/// A view some actor's grant names answers the same refusal, checked before it is read.
///
/// `input` is held to what the route's reader admits: nested no deeper, every number spelled
/// as JSON spells one, or [`entry::Refused::Input`]. The answer is read back at any depth; one holding a
/// `Json` value the realization built with a number JSON does not spell is not JSON, and is
/// [`entry::Refused::Undelivered`] for a command, whose effect stands, and [`entry::Refused::Unmet`]
/// for a view.
pub fn handle<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, caller: Option<&crate::actor::Caller>, name: &str, input: json::Value) -> Result<json::Value, entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    handle_with_events(system, caller, name, input).0
}

/// Runs one command or view as [`handle`] does, and hands back every event published while it
/// ran, in log order: what the command published, what the bindings that reacted to it
/// published in turn, and what a held-back delivery retried during the call published. An
/// event already on the system's log when the call started is not the call's and is never
/// in the list. The list is empty where no command ran — an unknown name, a refusal before
/// the port, a view — and holds what was published where delivering it failed after the
/// command took effect. An event a component's port published through a direct call is in
/// that component's outbox until a pump collects it, so one no pump has collected yet is
/// listed as this call's: call `pump` after calling a port directly.
pub fn handle_with_events<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, caller: Option<&crate::actor::Caller>, name: &str, input: json::Value) -> (Result<json::Value, entry::Refused>, Vec<crate::system::SystemEvent>)
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let mut taken = Vec::new();
    let checked = |input: &json::Value| {
        json::admitted(input).map_err(|expected| {
            entry::Refused::Input(format!("the input is not JSON: expected {expected}"))
        })
    };
    let answered = match name {
        "agency.agents.AgentDefinitionList" => match admit_read(caller, "agency.agents.AgentDefinitionList") {
            Ok(()) => run_agency_agents_agent_definition_list(system),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.agents.AgentList" => match admit_read(caller, "agency.agents.AgentList") {
            Ok(()) => run_agency_agents_agent_list(system),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.agents.CreateAgentDefinition" => match admit(caller, "agency.agents.CreateAgentDefinition") {
            Ok(()) => checked(&input).and_then(|()| run_agency_agents_create_agent_definition(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.agents.DeprecateAgentDefinition" => match admit(caller, "agency.agents.DeprecateAgentDefinition") {
            Ok(()) => checked(&input).and_then(|()| run_agency_agents_deprecate_agent_definition(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.agents.ProvisionAgent" => match admit(caller, "agency.agents.ProvisionAgent") {
            Ok(()) => checked(&input).and_then(|()| run_agency_agents_provision_agent(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.agents.PublishAgentDefinition" => match admit(caller, "agency.agents.PublishAgentDefinition") {
            Ok(()) => checked(&input).and_then(|()| run_agency_agents_publish_agent_definition(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.agents.RecordSpecCheck" => match admit(caller, "agency.agents.RecordSpecCheck") {
            Ok(()) => checked(&input).and_then(|()| run_agency_agents_record_spec_check(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.agents.RetireAgent" => match admit(caller, "agency.agents.RetireAgent") {
            Ok(()) => checked(&input).and_then(|()| run_agency_agents_retire_agent(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.auth.ActivatePolicy" => match admit(caller, "agency.auth.ActivatePolicy") {
            Ok(()) => checked(&input).and_then(|()| run_agency_auth_activate_policy(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.auth.AssignRole" => match admit(caller, "agency.auth.AssignRole") {
            Ok(()) => checked(&input).and_then(|()| run_agency_auth_assign_role(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.auth.CreatePolicy" => match admit(caller, "agency.auth.CreatePolicy") {
            Ok(()) => checked(&input).and_then(|()| run_agency_auth_create_policy(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.auth.CreateTenant" => match admit(caller, "agency.auth.CreateTenant") {
            Ok(()) => checked(&input).and_then(|()| run_agency_auth_create_tenant(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.auth.CredentialList" => match admit_read(caller, "agency.auth.CredentialList") {
            Ok(()) => run_agency_auth_credential_list(system),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.auth.DisablePrincipal" => match admit(caller, "agency.auth.DisablePrincipal") {
            Ok(()) => checked(&input).and_then(|()| run_agency_auth_disable_principal(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.auth.EffectivePolicy" => match admit_read(caller, "agency.auth.EffectivePolicy") {
            Ok(()) => checked(&input).and_then(|()| run_agency_auth_effective_policy(system, &input)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.auth.GrantCredential" => match admit(caller, "agency.auth.GrantCredential") {
            Ok(()) => checked(&input).and_then(|()| run_agency_auth_grant_credential(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.auth.GrantMatrix" => match admit_read(caller, "agency.auth.GrantMatrix") {
            Ok(()) => run_agency_auth_grant_matrix(system),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.auth.InvitePrincipal" => match admit(caller, "agency.auth.InvitePrincipal") {
            Ok(()) => checked(&input).and_then(|()| run_agency_auth_invite_principal(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.auth.PolicyList" => match admit_read(caller, "agency.auth.PolicyList") {
            Ok(()) => run_agency_auth_policy_list(system),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.auth.PrincipalList" => match admit_read(caller, "agency.auth.PrincipalList") {
            Ok(()) => run_agency_auth_principal_list(system),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.auth.RefreshCredential" => match admit(caller, "agency.auth.RefreshCredential") {
            Ok(()) => checked(&input).and_then(|()| run_agency_auth_refresh_credential(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.auth.RevokeCredential" => match admit(caller, "agency.auth.RevokeCredential") {
            Ok(()) => checked(&input).and_then(|()| run_agency_auth_revoke_credential(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.auth.RevokeGrant" => match admit(caller, "agency.auth.RevokeGrant") {
            Ok(()) => checked(&input).and_then(|()| run_agency_auth_revoke_grant(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.auth.StoreCredential" => match admit(caller, "agency.auth.StoreCredential") {
            Ok(()) => checked(&input).and_then(|()| run_agency_auth_store_credential(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.auth.SupersedePolicy" => match admit(caller, "agency.auth.SupersedePolicy") {
            Ok(()) => checked(&input).and_then(|()| run_agency_auth_supersede_policy(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.runs.AdmitRun" => match admit(caller, "agency.runs.AdmitRun") {
            Ok(()) => checked(&input).and_then(|()| run_agency_runs_admit_run(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.runs.ApprovalInbox" => match admit_read(caller, "agency.runs.ApprovalInbox") {
            Ok(()) => run_agency_runs_approval_inbox(system),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.runs.DenyRunApproval" => match admit(caller, "agency.runs.DenyRunApproval") {
            Ok(()) => checked(&input).and_then(|()| run_agency_runs_deny_run_approval(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.runs.GrantRunApproval" => match admit(caller, "agency.runs.GrantRunApproval") {
            Ok(()) => checked(&input).and_then(|()| run_agency_runs_grant_run_approval(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.runs.RecordRunFailure" => match admit(caller, "agency.runs.RecordRunFailure") {
            Ok(()) => checked(&input).and_then(|()| run_agency_runs_record_run_failure(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.runs.RecordRunProposal" => match admit(caller, "agency.runs.RecordRunProposal") {
            Ok(()) => checked(&input).and_then(|()| run_agency_runs_record_run_proposal(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.runs.RecordRunStarted" => match admit(caller, "agency.runs.RecordRunStarted") {
            Ok(()) => checked(&input).and_then(|()| run_agency_runs_record_run_started(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.runs.RequestRunApproval" => match admit(caller, "agency.runs.RequestRunApproval") {
            Ok(()) => checked(&input).and_then(|()| run_agency_runs_request_run_approval(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.runs.RunDetail" => match admit_read(caller, "agency.runs.RunDetail") {
            Ok(()) => checked(&input).and_then(|()| run_agency_runs_run_detail(system, &input)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.runs.RunList" => match admit_read(caller, "agency.runs.RunList") {
            Ok(()) => run_agency_runs_run_list(system),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.runs.SetRunBudget" => match admit(caller, "agency.runs.SetRunBudget") {
            Ok(()) => checked(&input).and_then(|()| run_agency_runs_set_run_budget(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.runs.StartRun" => match admit(caller, "agency.runs.StartRun") {
            Ok(()) => checked(&input).and_then(|()| run_agency_runs_start_run(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.runs.StopRun" => match admit(caller, "agency.runs.StopRun") {
            Ok(()) => checked(&input).and_then(|()| run_agency_runs_stop_run(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.runs.UsageByAgent" => match admit_read(caller, "agency.runs.UsageByAgent") {
            Ok(()) => run_agency_runs_usage_by_agent(system),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.service.CloseSession" => match admit(caller, "agency.service.CloseSession") {
            Ok(()) => checked(&input).and_then(|()| run_agency_service_close_session(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.service.OpenSession" => match admit(caller, "agency.service.OpenSession") {
            Ok(()) => checked(&input).and_then(|()| run_agency_service_open_session(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.service.SessionList" => match admit_read(caller, "agency.service.SessionList") {
            Ok(()) => run_agency_service_session_list(system),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.teams.AddMember" => match admit(caller, "agency.teams.AddMember") {
            Ok(()) => checked(&input).and_then(|()| run_agency_teams_add_member(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.teams.AttachQueue" => match admit(caller, "agency.teams.AttachQueue") {
            Ok(()) => checked(&input).and_then(|()| run_agency_teams_attach_queue(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.teams.DeclareSwarm" => match admit(caller, "agency.teams.DeclareSwarm") {
            Ok(()) => checked(&input).and_then(|()| run_agency_teams_declare_swarm(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.teams.DefineWorkforce" => match admit(caller, "agency.teams.DefineWorkforce") {
            Ok(()) => checked(&input).and_then(|()| run_agency_teams_define_workforce(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.teams.DisbandTeam" => match admit(caller, "agency.teams.DisbandTeam") {
            Ok(()) => checked(&input).and_then(|()| run_agency_teams_disband_team(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.teams.FormTeam" => match admit(caller, "agency.teams.FormTeam") {
            Ok(()) => checked(&input).and_then(|()| run_agency_teams_form_team(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.teams.RemoveMember" => match admit(caller, "agency.teams.RemoveMember") {
            Ok(()) => checked(&input).and_then(|()| run_agency_teams_remove_member(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.teams.Report" => match admit(caller, "agency.teams.Report") {
            Ok(()) => checked(&input).and_then(|()| run_agency_teams_report(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.teams.RetireWorkforce" => match admit(caller, "agency.teams.RetireWorkforce") {
            Ok(()) => checked(&input).and_then(|()| run_agency_teams_retire_workforce(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.teams.Send" => match admit(caller, "agency.teams.Send") {
            Ok(()) => checked(&input).and_then(|()| run_agency_teams_send(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.teams.SetLead" => match admit(caller, "agency.teams.SetLead") {
            Ok(()) => checked(&input).and_then(|()| run_agency_teams_set_lead(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.teams.StaffWorkforce" => match admit(caller, "agency.teams.StaffWorkforce") {
            Ok(()) => checked(&input).and_then(|()| run_agency_teams_staff_workforce(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.teams.StopSwarm" => match admit(caller, "agency.teams.StopSwarm") {
            Ok(()) => checked(&input).and_then(|()| run_agency_teams_stop_swarm(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.teams.SwarmList" => match admit_read(caller, "agency.teams.SwarmList") {
            Ok(()) => run_agency_teams_swarm_list(system),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.teams.TeamDetail" => match admit_read(caller, "agency.teams.TeamDetail") {
            Ok(()) => checked(&input).and_then(|()| run_agency_teams_team_detail(system, &input)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.teams.TeamList" => match admit_read(caller, "agency.teams.TeamList") {
            Ok(()) => run_agency_teams_team_list(system),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.teams.WorkforceBoard" => match admit_read(caller, "agency.teams.WorkforceBoard") {
            Ok(()) => run_agency_teams_workforce_board(system),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.tools.BindTool" => match admit(caller, "agency.tools.BindTool") {
            Ok(()) => checked(&input).and_then(|()| run_agency_tools_bind_tool(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.tools.BindingList" => match admit_read(caller, "agency.tools.BindingList") {
            Ok(()) => run_agency_tools_binding_list(system),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.tools.RegisterTool" => match admit(caller, "agency.tools.RegisterTool") {
            Ok(()) => checked(&input).and_then(|()| run_agency_tools_register_tool(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.tools.RevokeBinding" => match admit(caller, "agency.tools.RevokeBinding") {
            Ok(()) => checked(&input).and_then(|()| run_agency_tools_revoke_binding(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.tools.ToolCatalogue" => match admit_read(caller, "agency.tools.ToolCatalogue") {
            Ok(()) => run_agency_tools_tool_catalogue(system),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.tools.WithdrawTool" => match admit(caller, "agency.tools.WithdrawTool") {
            Ok(()) => checked(&input).and_then(|()| run_agency_tools_withdraw_tool(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.work.AbandonWorkItem" => match admit(caller, "agency.work.AbandonWorkItem") {
            Ok(()) => checked(&input).and_then(|()| run_agency_work_abandon_work_item(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.work.AssignWorkItem" => match admit(caller, "agency.work.AssignWorkItem") {
            Ok(()) => checked(&input).and_then(|()| run_agency_work_assign_work_item(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.work.ConfigureWorkInput" => match admit(caller, "agency.work.ConfigureWorkInput") {
            Ok(()) => checked(&input).and_then(|()| run_agency_work_configure_work_input(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.work.PauseWorkInput" => match admit(caller, "agency.work.PauseWorkInput") {
            Ok(()) => checked(&input).and_then(|()| run_agency_work_pause_work_input(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.work.RemoveWorkInput" => match admit(caller, "agency.work.RemoveWorkInput") {
            Ok(()) => checked(&input).and_then(|()| run_agency_work_remove_work_input(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.work.ReturnWorkItem" => match admit(caller, "agency.work.ReturnWorkItem") {
            Ok(()) => checked(&input).and_then(|()| run_agency_work_return_work_item(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.work.SettleWorkItem" => match admit(caller, "agency.work.SettleWorkItem") {
            Ok(()) => checked(&input).and_then(|()| run_agency_work_settle_work_item(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.work.SubmitWorkItem" => match admit(caller, "agency.work.SubmitWorkItem") {
            Ok(()) => checked(&input).and_then(|()| run_agency_work_submit_work_item(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.work.WorkInputList" => match admit_read(caller, "agency.work.WorkInputList") {
            Ok(()) => run_agency_work_work_input_list(system),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.work.WorkItemDetail" => match admit_read(caller, "agency.work.WorkItemDetail") {
            Ok(()) => checked(&input).and_then(|()| run_agency_work_work_item_detail(system, &input)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agency.work.WorkQueue" => match admit_read(caller, "agency.work.WorkQueue") {
            Ok(()) => run_agency_work_work_queue(system),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        other => return (Err(entry::Refused::Unknown(other.to_owned())), taken),
    };
    let body = match answered {
        Ok((_, body)) => body,
        Err(refused) => return (Err(refused), taken),
    };
    let read = entry::read(&body).map_err(|error| {
        if matches!(name, "agency.agents.CreateAgentDefinition" | "agency.agents.DeprecateAgentDefinition" | "agency.agents.ProvisionAgent" | "agency.agents.PublishAgentDefinition" | "agency.agents.RecordSpecCheck" | "agency.agents.RetireAgent" | "agency.auth.ActivatePolicy" | "agency.auth.AssignRole" | "agency.auth.CreatePolicy" | "agency.auth.CreateTenant" | "agency.auth.DisablePrincipal" | "agency.auth.GrantCredential" | "agency.auth.InvitePrincipal" | "agency.auth.RefreshCredential" | "agency.auth.RevokeCredential" | "agency.auth.RevokeGrant" | "agency.auth.StoreCredential" | "agency.auth.SupersedePolicy" | "agency.runs.AdmitRun" | "agency.runs.DenyRunApproval" | "agency.runs.GrantRunApproval" | "agency.runs.RecordRunFailure" | "agency.runs.RecordRunProposal" | "agency.runs.RecordRunStarted" | "agency.runs.RequestRunApproval" | "agency.runs.SetRunBudget" | "agency.runs.StartRun" | "agency.runs.StopRun" | "agency.service.CloseSession" | "agency.service.OpenSession" | "agency.teams.AddMember" | "agency.teams.AttachQueue" | "agency.teams.DeclareSwarm" | "agency.teams.DefineWorkforce" | "agency.teams.DisbandTeam" | "agency.teams.FormTeam" | "agency.teams.RemoveMember" | "agency.teams.Report" | "agency.teams.RetireWorkforce" | "agency.teams.Send" | "agency.teams.SetLead" | "agency.teams.StaffWorkforce" | "agency.teams.StopSwarm" | "agency.tools.BindTool" | "agency.tools.RegisterTool" | "agency.tools.RevokeBinding" | "agency.tools.WithdrawTool" | "agency.work.AbandonWorkItem" | "agency.work.AssignWorkItem" | "agency.work.ConfigureWorkInput" | "agency.work.PauseWorkInput" | "agency.work.RemoveWorkInput" | "agency.work.ReturnWorkItem" | "agency.work.SettleWorkItem" | "agency.work.SubmitWorkItem") {
            entry::Refused::Undelivered(format!(
                "delivering what the command published: its answer is not JSON this surface reads back: {error}"
            ))
        } else {
            entry::Refused::Unmet(format!(
                "unmet obligation: view_query `{name}` answered rows this surface cannot read back: {error}"
            ))
        }
    });
    (read, taken)
}

/// Nothing, where `caller` may invoke `command`; otherwise the actor the standard refusal names,
/// `None` where the request was authenticated as no actor.
///
/// Checked before the command runs, on the caller the realization authenticated the request
/// as and never on anything the request says about itself. Public, so code that drives the system
/// in process checks the grant exactly as every route does.
pub fn admit(caller: Option<&crate::actor::Caller>, command: &str) -> Result<(), Option<&'static str>> {
    match caller {
        Some(caller) if caller.may(command) => Ok(()),
        Some(caller) => Err(Some(caller.actor.name())),
        None => Err(None),
    }
}

/// The standard refusal for an actor no grant admits, as the contract declares it: `403`,
/// `{"refused": "not granted", "actor": <name or null>}`.
fn not_granted(actor: Option<&str>) -> http::Response {
    let mut body = String::from("{");
    json::member(&mut body, "refused");
    json::push_text(&mut body, "not granted");
    json::member(&mut body, "actor");
    match actor {
        Some(actor) => json::push_text(&mut body, actor),
        None => body.push_str("null"),
    }
    body.push('}');
    http::Response::new(403, http::JSON, body)
}

/// Nothing, where `caller` may read `view`, a view some actor's grant names; otherwise the actor
/// the standard refusal names, `None` where the request was authenticated as no actor.
///
/// Checked before the view is read, on the caller the realization authenticated the request as.
/// Public, so code that reads the system in process checks the grant exactly as every route does.
pub fn admit_read(caller: Option<&crate::actor::Caller>, view: &str) -> Result<(), Option<&'static str>> {
    match caller {
        Some(caller) if caller.may_read(view) => Ok(()),
        Some(caller) => Err(Some(caller.actor.name())),
        None => Err(None),
    }
}

/// `POST` `agency.work.AbandonWorkItem`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_work_abandon_work_item<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_work_abandon_work_item(system, &value, taken))
}

/// `agency.work.AbandonWorkItem` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_work_abandon_work_item<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_work_abandon_work_item(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.abandon_work_item(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_work_abandon_work_item(&outcome))
}

/// One declared outcome of `agency.work.AbandonWorkItem`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_work_abandon_work_item(outcome: &crate::work::AbandonWorkItemOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::work::AbandonWorkItemOutcome::NoSuchItem { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-item");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.work.NotFound");
            404
        }
        crate::work::AbandonWorkItemOutcome::Abandoned { work_item_abandoned, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "abandoned");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.work.WorkItemAbandoned");
            json::member(&mut body, "payload");
            wire::encode_event_agency_work_work_item_abandoned(work_item_abandoned, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::work::AbandonWorkItemOutcome::WrongState { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.work.StateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.auth.ActivatePolicy`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_auth_activate_policy<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_auth_activate_policy(system, &value, taken))
}

/// `agency.auth.ActivatePolicy` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_auth_activate_policy<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_auth_activate_policy(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.activate_policy(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_auth_activate_policy(&outcome))
}

/// One declared outcome of `agency.auth.ActivatePolicy`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_auth_activate_policy(outcome: &crate::auth::ActivatePolicyOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::auth::ActivatePolicyOutcome::NoSuchPolicy { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-policy");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.auth.NotFound");
            404
        }
        crate::auth::ActivatePolicyOutcome::Activated { policy_activated, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "activated");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.auth.PolicyActivated");
            json::member(&mut body, "payload");
            wire::encode_event_agency_auth_policy_activated(policy_activated, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::auth::ActivatePolicyOutcome::WrongState { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.auth.StateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.teams.AddMember`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_teams_add_member<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_teams_add_member(system, &value, taken))
}

/// `agency.teams.AddMember` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_teams_add_member<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_teams_add_member(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.add_member(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_teams_add_member(&outcome))
}

/// One declared outcome of `agency.teams.AddMember`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_teams_add_member(outcome: &crate::teams::AddMemberOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::teams::AddMemberOutcome::NoSuchTeam { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-team");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.teams.NotFound");
            404
        }
        crate::teams::AddMemberOutcome::Added { member_added, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "added");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.teams.MemberAdded");
            json::member(&mut body, "payload");
            wire::encode_event_agency_teams_member_added(member_added, &mut body);
            body.push('}');
            body.push(']');
            202
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.runs.AdmitRun`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_runs_admit_run<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_runs_admit_run(system, &value, taken))
}

/// `agency.runs.AdmitRun` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_runs_admit_run<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_runs_admit_run(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.admit_run(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_runs_admit_run(&outcome))
}

/// One declared outcome of `agency.runs.AdmitRun`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_runs_admit_run(outcome: &crate::runs::AdmitRunOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::runs::AdmitRunOutcome::NoSuchRun { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-run");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.runs.NotFound");
            404
        }
        crate::runs::AdmitRunOutcome::Admitted { run_admitted, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "admitted");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.runs.RunAdmitted");
            json::member(&mut body, "payload");
            wire::encode_event_agency_runs_run_admitted(run_admitted, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::runs::AdmitRunOutcome::WrongState { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.runs.StateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.auth.AssignRole`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_auth_assign_role<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_auth_assign_role(system, &value, taken))
}

/// `agency.auth.AssignRole` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_auth_assign_role<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_auth_assign_role(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.assign_role(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_auth_assign_role(&outcome))
}

/// One declared outcome of `agency.auth.AssignRole`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_auth_assign_role(outcome: &crate::auth::AssignRoleOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::auth::AssignRoleOutcome::NoSuchPrincipal { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-principal");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.auth.NotFound");
            404
        }
        crate::auth::AssignRoleOutcome::Assigned { role_assigned, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "assigned");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.auth.RoleAssigned");
            json::member(&mut body, "payload");
            wire::encode_event_agency_auth_role_assigned(role_assigned, &mut body);
            body.push('}');
            body.push(']');
            202
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.work.AssignWorkItem`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_work_assign_work_item<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_work_assign_work_item(system, &value, taken))
}

/// `agency.work.AssignWorkItem` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_work_assign_work_item<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_work_assign_work_item(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.assign_work_item(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_work_assign_work_item(&outcome))
}

/// One declared outcome of `agency.work.AssignWorkItem`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_work_assign_work_item(outcome: &crate::work::AssignWorkItemOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::work::AssignWorkItemOutcome::NoSuchItem { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-item");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.work.NotFound");
            404
        }
        crate::work::AssignWorkItemOutcome::Assigned { work_item_assigned, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "assigned");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.work.WorkItemAssigned");
            json::member(&mut body, "payload");
            wire::encode_event_agency_work_work_item_assigned(work_item_assigned, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::work::AssignWorkItemOutcome::WrongState { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.work.StateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.teams.AttachQueue`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_teams_attach_queue<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_teams_attach_queue(system, &value, taken))
}

/// `agency.teams.AttachQueue` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_teams_attach_queue<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_teams_attach_queue(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.attach_queue(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_teams_attach_queue(&outcome))
}

/// One declared outcome of `agency.teams.AttachQueue`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_teams_attach_queue(outcome: &crate::teams::AttachQueueOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::teams::AttachQueueOutcome::NoSuchInput { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-input");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.teams.NotFound");
            404
        }
        crate::teams::AttachQueueOutcome::Attached { queue_attached, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "attached");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.teams.QueueAttached");
            json::member(&mut body, "payload");
            wire::encode_event_agency_teams_queue_attached(queue_attached, &mut body);
            body.push('}');
            body.push(']');
            202
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.tools.BindTool`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_tools_bind_tool<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_tools_bind_tool(system, &value, taken))
}

/// `agency.tools.BindTool` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_tools_bind_tool<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_tools_bind_tool(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.bind_tool(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_tools_bind_tool(&outcome))
}

/// One declared outcome of `agency.tools.BindTool`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_tools_bind_tool(outcome: &crate::tools::BindToolOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::tools::BindToolOutcome::BoundToDefinition { tool_bound, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "bound-to-definition");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.tools.ToolBound");
            json::member(&mut body, "payload");
            wire::encode_event_agency_tools_tool_bound(tool_bound, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::tools::BindToolOutcome::BoundToTeam { tool_bound, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "bound-to-team");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.tools.ToolBound");
            json::member(&mut body, "payload");
            wire::encode_event_agency_tools_tool_bound(tool_bound, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::tools::BindToolOutcome::TargetInvalid { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "target-invalid");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.tools.BindingTargetInvalid");
            422
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.service.CloseSession`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_service_close_session<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_service_close_session(system, &value, taken))
}

/// `agency.service.CloseSession` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_service_close_session<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_service_close_session(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.close_session(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_service_close_session(&outcome))
}

/// One declared outcome of `agency.service.CloseSession`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_service_close_session(outcome: &crate::service::CloseSessionOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::service::CloseSessionOutcome::NoSuchSession { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-session");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.service.NotFound");
            404
        }
        crate::service::CloseSessionOutcome::Closed { session_closed, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "closed");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.service.SessionClosed");
            json::member(&mut body, "payload");
            wire::encode_event_agency_service_session_closed(session_closed, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::service::CloseSessionOutcome::WrongState { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.service.StateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.work.ConfigureWorkInput`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_work_configure_work_input<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_work_configure_work_input(system, &value, taken))
}

/// `agency.work.ConfigureWorkInput` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_work_configure_work_input<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_work_configure_work_input(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.configure_work_input(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_work_configure_work_input(&outcome))
}

/// One declared outcome of `agency.work.ConfigureWorkInput`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_work_configure_work_input(outcome: &crate::work::ConfigureWorkInputOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::work::ConfigureWorkInputOutcome::Configured { work_input_configured, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "configured");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.work.WorkInputConfigured");
            json::member(&mut body, "payload");
            wire::encode_event_agency_work_work_input_configured(work_input_configured, &mut body);
            body.push('}');
            body.push(']');
            202
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.agents.CreateAgentDefinition`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_agents_create_agent_definition<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_agents_create_agent_definition(system, &value, taken))
}

/// `agency.agents.CreateAgentDefinition` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_agents_create_agent_definition<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_agents_create_agent_definition(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.create_agent_definition(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_agents_create_agent_definition(&outcome))
}

/// One declared outcome of `agency.agents.CreateAgentDefinition`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_agents_create_agent_definition(outcome: &crate::agents::CreateAgentDefinitionOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::agents::CreateAgentDefinitionOutcome::Created { agent_definition_created, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "created");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.agents.AgentDefinitionCreated");
            json::member(&mut body, "payload");
            wire::encode_event_agency_agents_agent_definition_created(agent_definition_created, &mut body);
            body.push('}');
            body.push(']');
            202
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.auth.CreatePolicy`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_auth_create_policy<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_auth_create_policy(system, &value, taken))
}

/// `agency.auth.CreatePolicy` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_auth_create_policy<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_auth_create_policy(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.create_policy(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_auth_create_policy(&outcome))
}

/// One declared outcome of `agency.auth.CreatePolicy`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_auth_create_policy(outcome: &crate::auth::CreatePolicyOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::auth::CreatePolicyOutcome::Created { policy_created, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "created");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.auth.PolicyCreated");
            json::member(&mut body, "payload");
            wire::encode_event_agency_auth_policy_created(policy_created, &mut body);
            body.push('}');
            body.push(']');
            202
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.auth.CreateTenant`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_auth_create_tenant<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_auth_create_tenant(system, &value, taken))
}

/// `agency.auth.CreateTenant` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_auth_create_tenant<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_auth_create_tenant(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.create_tenant(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_auth_create_tenant(&outcome))
}

/// One declared outcome of `agency.auth.CreateTenant`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_auth_create_tenant(outcome: &crate::auth::CreateTenantOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::auth::CreateTenantOutcome::Created { tenant_created, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "created");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.auth.TenantCreated");
            json::member(&mut body, "payload");
            wire::encode_event_agency_auth_tenant_created(tenant_created, &mut body);
            body.push('}');
            body.push(']');
            202
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.teams.DeclareSwarm`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_teams_declare_swarm<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_teams_declare_swarm(system, &value, taken))
}

/// `agency.teams.DeclareSwarm` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_teams_declare_swarm<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_teams_declare_swarm(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.declare_swarm(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_teams_declare_swarm(&outcome))
}

/// One declared outcome of `agency.teams.DeclareSwarm`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_teams_declare_swarm(outcome: &crate::teams::DeclareSwarmOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::teams::DeclareSwarmOutcome::Declared { swarm_declared, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "declared");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.teams.SwarmDeclared");
            json::member(&mut body, "payload");
            wire::encode_event_agency_teams_swarm_declared(swarm_declared, &mut body);
            body.push('}');
            body.push(']');
            202
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.teams.DefineWorkforce`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_teams_define_workforce<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_teams_define_workforce(system, &value, taken))
}

/// `agency.teams.DefineWorkforce` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_teams_define_workforce<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_teams_define_workforce(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.define_workforce(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_teams_define_workforce(&outcome))
}

/// One declared outcome of `agency.teams.DefineWorkforce`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_teams_define_workforce(outcome: &crate::teams::DefineWorkforceOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::teams::DefineWorkforceOutcome::Defined { workforce_defined, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "defined");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.teams.WorkforceDefined");
            json::member(&mut body, "payload");
            wire::encode_event_agency_teams_workforce_defined(workforce_defined, &mut body);
            body.push('}');
            body.push(']');
            202
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.runs.DenyRunApproval`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_runs_deny_run_approval<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_runs_deny_run_approval(system, &value, taken))
}

/// `agency.runs.DenyRunApproval` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_runs_deny_run_approval<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_runs_deny_run_approval(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.deny_run_approval(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_runs_deny_run_approval(&outcome))
}

/// One declared outcome of `agency.runs.DenyRunApproval`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_runs_deny_run_approval(outcome: &crate::runs::DenyRunApprovalOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::runs::DenyRunApprovalOutcome::NoSuchApproval { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-approval");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.runs.NotFound");
            404
        }
        crate::runs::DenyRunApprovalOutcome::Denied { run_approval_denied, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "denied");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.runs.RunApprovalDenied");
            json::member(&mut body, "payload");
            wire::encode_event_agency_runs_run_approval_denied(run_approval_denied, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::runs::DenyRunApprovalOutcome::WrongState { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.runs.StateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.agents.DeprecateAgentDefinition`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_agents_deprecate_agent_definition<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_agents_deprecate_agent_definition(system, &value, taken))
}

/// `agency.agents.DeprecateAgentDefinition` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_agents_deprecate_agent_definition<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_agents_deprecate_agent_definition(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.deprecate_agent_definition(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_agents_deprecate_agent_definition(&outcome))
}

/// One declared outcome of `agency.agents.DeprecateAgentDefinition`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_agents_deprecate_agent_definition(outcome: &crate::agents::DeprecateAgentDefinitionOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::agents::DeprecateAgentDefinitionOutcome::NoSuchDefinition { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-definition");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.agents.NotFound");
            404
        }
        crate::agents::DeprecateAgentDefinitionOutcome::Deprecated { agent_definition_deprecated, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "deprecated");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.agents.AgentDefinitionDeprecated");
            json::member(&mut body, "payload");
            wire::encode_event_agency_agents_agent_definition_deprecated(agent_definition_deprecated, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::agents::DeprecateAgentDefinitionOutcome::WrongState { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.agents.StateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.auth.DisablePrincipal`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_auth_disable_principal<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_auth_disable_principal(system, &value, taken))
}

/// `agency.auth.DisablePrincipal` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_auth_disable_principal<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_auth_disable_principal(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.disable_principal(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_auth_disable_principal(&outcome))
}

/// One declared outcome of `agency.auth.DisablePrincipal`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_auth_disable_principal(outcome: &crate::auth::DisablePrincipalOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::auth::DisablePrincipalOutcome::NoSuchPrincipal { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-principal");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.auth.NotFound");
            404
        }
        crate::auth::DisablePrincipalOutcome::Disabled { principal_disabled, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "disabled");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.auth.PrincipalDisabled");
            json::member(&mut body, "payload");
            wire::encode_event_agency_auth_principal_disabled(principal_disabled, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::auth::DisablePrincipalOutcome::WrongState { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.auth.StateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.teams.DisbandTeam`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_teams_disband_team<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_teams_disband_team(system, &value, taken))
}

/// `agency.teams.DisbandTeam` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_teams_disband_team<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_teams_disband_team(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.disband_team(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_teams_disband_team(&outcome))
}

/// One declared outcome of `agency.teams.DisbandTeam`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_teams_disband_team(outcome: &crate::teams::DisbandTeamOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::teams::DisbandTeamOutcome::NoSuchTeam { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-team");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.teams.NotFound");
            404
        }
        crate::teams::DisbandTeamOutcome::Disbanded { team_disbanded, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "disbanded");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.teams.TeamDisbanded");
            json::member(&mut body, "payload");
            wire::encode_event_agency_teams_team_disbanded(team_disbanded, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::teams::DisbandTeamOutcome::WrongState { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.teams.StateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.teams.FormTeam`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_teams_form_team<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_teams_form_team(system, &value, taken))
}

/// `agency.teams.FormTeam` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_teams_form_team<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_teams_form_team(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.form_team(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_teams_form_team(&outcome))
}

/// One declared outcome of `agency.teams.FormTeam`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_teams_form_team(outcome: &crate::teams::FormTeamOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::teams::FormTeamOutcome::Formed { team_formed, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "formed");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.teams.TeamFormed");
            json::member(&mut body, "payload");
            wire::encode_event_agency_teams_team_formed(team_formed, &mut body);
            body.push('}');
            body.push(']');
            202
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.auth.GrantCredential`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_auth_grant_credential<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_auth_grant_credential(system, &value, taken))
}

/// `agency.auth.GrantCredential` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_auth_grant_credential<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_auth_grant_credential(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.grant_credential(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_auth_grant_credential(&outcome))
}

/// One declared outcome of `agency.auth.GrantCredential`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_auth_grant_credential(outcome: &crate::auth::GrantCredentialOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::auth::GrantCredentialOutcome::GrantedToAgent { credential_granted, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "granted-to-agent");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.auth.CredentialGranted");
            json::member(&mut body, "payload");
            wire::encode_event_agency_auth_credential_granted(credential_granted, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::auth::GrantCredentialOutcome::GrantedToTeam { credential_granted, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "granted-to-team");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.auth.CredentialGranted");
            json::member(&mut body, "payload");
            wire::encode_event_agency_auth_credential_granted(credential_granted, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::auth::GrantCredentialOutcome::GrantedToWorkforce { credential_granted, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "granted-to-workforce");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.auth.CredentialGranted");
            json::member(&mut body, "payload");
            wire::encode_event_agency_auth_credential_granted(credential_granted, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::auth::GrantCredentialOutcome::GranteeInvalid { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "grantee-invalid");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.auth.GranteeInvalid");
            422
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.runs.GrantRunApproval`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_runs_grant_run_approval<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_runs_grant_run_approval(system, &value, taken))
}

/// `agency.runs.GrantRunApproval` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_runs_grant_run_approval<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_runs_grant_run_approval(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.grant_run_approval(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_runs_grant_run_approval(&outcome))
}

/// One declared outcome of `agency.runs.GrantRunApproval`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_runs_grant_run_approval(outcome: &crate::runs::GrantRunApprovalOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::runs::GrantRunApprovalOutcome::NoSuchApproval { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-approval");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.runs.NotFound");
            404
        }
        crate::runs::GrantRunApprovalOutcome::Granted { run_approval_granted, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "granted");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.runs.RunApprovalGranted");
            json::member(&mut body, "payload");
            wire::encode_event_agency_runs_run_approval_granted(run_approval_granted, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::runs::GrantRunApprovalOutcome::WrongState { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.runs.StateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.auth.InvitePrincipal`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_auth_invite_principal<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_auth_invite_principal(system, &value, taken))
}

/// `agency.auth.InvitePrincipal` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_auth_invite_principal<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_auth_invite_principal(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.invite_principal(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_auth_invite_principal(&outcome))
}

/// One declared outcome of `agency.auth.InvitePrincipal`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_auth_invite_principal(outcome: &crate::auth::InvitePrincipalOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::auth::InvitePrincipalOutcome::Invited { principal_invited, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "invited");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.auth.PrincipalInvited");
            json::member(&mut body, "payload");
            wire::encode_event_agency_auth_principal_invited(principal_invited, &mut body);
            body.push('}');
            body.push(']');
            202
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.service.OpenSession`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_service_open_session<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_service_open_session(system, &value, taken))
}

/// `agency.service.OpenSession` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_service_open_session<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_service_open_session(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.open_session(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_service_open_session(&outcome))
}

/// One declared outcome of `agency.service.OpenSession`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_service_open_session(outcome: &crate::service::OpenSessionOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::service::OpenSessionOutcome::Opened { session_opened, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "opened");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.service.SessionOpened");
            json::member(&mut body, "payload");
            wire::encode_event_agency_service_session_opened(session_opened, &mut body);
            body.push('}');
            body.push(']');
            202
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.work.PauseWorkInput`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_work_pause_work_input<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_work_pause_work_input(system, &value, taken))
}

/// `agency.work.PauseWorkInput` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_work_pause_work_input<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_work_pause_work_input(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.pause_work_input(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_work_pause_work_input(&outcome))
}

/// One declared outcome of `agency.work.PauseWorkInput`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_work_pause_work_input(outcome: &crate::work::PauseWorkInputOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::work::PauseWorkInputOutcome::NoSuchInput { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-input");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.work.NotFound");
            404
        }
        crate::work::PauseWorkInputOutcome::Paused { work_input_paused, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "paused");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.work.WorkInputPaused");
            json::member(&mut body, "payload");
            wire::encode_event_agency_work_work_input_paused(work_input_paused, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::work::PauseWorkInputOutcome::WrongState { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.work.StateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.agents.ProvisionAgent`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_agents_provision_agent<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_agents_provision_agent(system, &value, taken))
}

/// `agency.agents.ProvisionAgent` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_agents_provision_agent<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_agents_provision_agent(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.provision_agent(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_agents_provision_agent(&outcome))
}

/// One declared outcome of `agency.agents.ProvisionAgent`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_agents_provision_agent(outcome: &crate::agents::ProvisionAgentOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::agents::ProvisionAgentOutcome::Provisioned { agent_provisioned, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "provisioned");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.agents.AgentProvisioned");
            json::member(&mut body, "payload");
            wire::encode_event_agency_agents_agent_provisioned(agent_provisioned, &mut body);
            body.push('}');
            body.push(']');
            202
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.agents.PublishAgentDefinition`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_agents_publish_agent_definition<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_agents_publish_agent_definition(system, &value, taken))
}

/// `agency.agents.PublishAgentDefinition` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_agents_publish_agent_definition<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_agents_publish_agent_definition(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.publish_agent_definition(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_agents_publish_agent_definition(&outcome))
}

/// One declared outcome of `agency.agents.PublishAgentDefinition`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_agents_publish_agent_definition(outcome: &crate::agents::PublishAgentDefinitionOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::agents::PublishAgentDefinitionOutcome::NoSuchDefinition { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-definition");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.agents.NotFound");
            404
        }
        crate::agents::PublishAgentDefinitionOutcome::Published { agent_definition_published, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "published");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.agents.AgentDefinitionPublished");
            json::member(&mut body, "payload");
            wire::encode_event_agency_agents_agent_definition_published(agent_definition_published, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::agents::PublishAgentDefinitionOutcome::WrongState { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.agents.StateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.runs.RecordRunFailure`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_runs_record_run_failure<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_runs_record_run_failure(system, &value, taken))
}

/// `agency.runs.RecordRunFailure` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_runs_record_run_failure<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_runs_record_run_failure(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.record_run_failure(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_runs_record_run_failure(&outcome))
}

/// One declared outcome of `agency.runs.RecordRunFailure`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_runs_record_run_failure(outcome: &crate::runs::RecordRunFailureOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::runs::RecordRunFailureOutcome::NoSuchRun { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-run");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.runs.NotFound");
            404
        }
        crate::runs::RecordRunFailureOutcome::Failed { run_failed, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "failed");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.runs.RunFailed");
            json::member(&mut body, "payload");
            wire::encode_event_agency_runs_run_failed(run_failed, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::runs::RecordRunFailureOutcome::WrongState { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.runs.StateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.runs.RecordRunProposal`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_runs_record_run_proposal<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_runs_record_run_proposal(system, &value, taken))
}

/// `agency.runs.RecordRunProposal` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_runs_record_run_proposal<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_runs_record_run_proposal(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.record_run_proposal(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_runs_record_run_proposal(&outcome))
}

/// One declared outcome of `agency.runs.RecordRunProposal`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_runs_record_run_proposal(outcome: &crate::runs::RecordRunProposalOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::runs::RecordRunProposalOutcome::NoSuchRun { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-run");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.runs.NotFound");
            404
        }
        crate::runs::RecordRunProposalOutcome::Proposed { run_proposed, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "proposed");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.runs.RunProposed");
            json::member(&mut body, "payload");
            wire::encode_event_agency_runs_run_proposed(run_proposed, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::runs::RecordRunProposalOutcome::WrongState { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.runs.StateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.runs.RecordRunStarted`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_runs_record_run_started<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_runs_record_run_started(system, &value, taken))
}

/// `agency.runs.RecordRunStarted` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_runs_record_run_started<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_runs_record_run_started(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.record_run_started(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_runs_record_run_started(&outcome))
}

/// One declared outcome of `agency.runs.RecordRunStarted`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_runs_record_run_started(outcome: &crate::runs::RecordRunStartedOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::runs::RecordRunStartedOutcome::NoSuchRun { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-run");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.runs.NotFound");
            404
        }
        crate::runs::RecordRunStartedOutcome::Started { run_started, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "started");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.runs.RunStarted");
            json::member(&mut body, "payload");
            wire::encode_event_agency_runs_run_started(run_started, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::runs::RecordRunStartedOutcome::WrongState { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.runs.StateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.agents.RecordSpecCheck`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_agents_record_spec_check<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_agents_record_spec_check(system, &value, taken))
}

/// `agency.agents.RecordSpecCheck` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_agents_record_spec_check<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_agents_record_spec_check(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.record_spec_check(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_agents_record_spec_check(&outcome))
}

/// One declared outcome of `agency.agents.RecordSpecCheck`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_agents_record_spec_check(outcome: &crate::agents::RecordSpecCheckOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::agents::RecordSpecCheckOutcome::NoSuchDefinition { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-definition");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.agents.NotFound");
            404
        }
        crate::agents::RecordSpecCheckOutcome::Refused { agent_spec_refused, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "refused");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.agents.AgentSpecRefused");
            json::member(&mut body, "payload");
            wire::encode_event_agency_agents_agent_spec_refused(agent_spec_refused, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::agents::RecordSpecCheckOutcome::Checked { agent_spec_checked, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "checked");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.agents.AgentSpecChecked");
            json::member(&mut body, "payload");
            wire::encode_event_agency_agents_agent_spec_checked(agent_spec_checked, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::agents::RecordSpecCheckOutcome::WrongState { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.agents.StateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.auth.RefreshCredential`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_auth_refresh_credential<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_auth_refresh_credential(system, &value, taken))
}

/// `agency.auth.RefreshCredential` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_auth_refresh_credential<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_auth_refresh_credential(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.refresh_credential(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_auth_refresh_credential(&outcome))
}

/// One declared outcome of `agency.auth.RefreshCredential`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_auth_refresh_credential(outcome: &crate::auth::RefreshCredentialOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::auth::RefreshCredentialOutcome::NoSuchCredential { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-credential");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.auth.NotFound");
            404
        }
        crate::auth::RefreshCredentialOutcome::Revoked { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "revoked");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.auth.CredentialUnusable");
            409
        }
        crate::auth::RefreshCredentialOutcome::Refreshed { credential_refreshed, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "refreshed");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.auth.CredentialRefreshed");
            json::member(&mut body, "payload");
            wire::encode_event_agency_auth_credential_refreshed(credential_refreshed, &mut body);
            body.push('}');
            body.push(']');
            202
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.tools.RegisterTool`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_tools_register_tool<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_tools_register_tool(system, &value, taken))
}

/// `agency.tools.RegisterTool` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_tools_register_tool<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_tools_register_tool(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.register_tool(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_tools_register_tool(&outcome))
}

/// One declared outcome of `agency.tools.RegisterTool`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_tools_register_tool(outcome: &crate::tools::RegisterToolOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::tools::RegisterToolOutcome::Registered { tool_registered, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "registered");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.tools.ToolRegistered");
            json::member(&mut body, "payload");
            wire::encode_event_agency_tools_tool_registered(tool_registered, &mut body);
            body.push('}');
            body.push(']');
            202
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.teams.RemoveMember`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_teams_remove_member<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_teams_remove_member(system, &value, taken))
}

/// `agency.teams.RemoveMember` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_teams_remove_member<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_teams_remove_member(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.remove_member(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_teams_remove_member(&outcome))
}

/// One declared outcome of `agency.teams.RemoveMember`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_teams_remove_member(outcome: &crate::teams::RemoveMemberOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::teams::RemoveMemberOutcome::NoSuchTeam { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-team");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.teams.NotFound");
            404
        }
        crate::teams::RemoveMemberOutcome::Removed { member_removed, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "removed");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.teams.MemberRemoved");
            json::member(&mut body, "payload");
            wire::encode_event_agency_teams_member_removed(member_removed, &mut body);
            body.push('}');
            body.push(']');
            202
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.work.RemoveWorkInput`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_work_remove_work_input<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_work_remove_work_input(system, &value, taken))
}

/// `agency.work.RemoveWorkInput` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_work_remove_work_input<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_work_remove_work_input(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.remove_work_input(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_work_remove_work_input(&outcome))
}

/// One declared outcome of `agency.work.RemoveWorkInput`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_work_remove_work_input(outcome: &crate::work::RemoveWorkInputOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::work::RemoveWorkInputOutcome::NoSuchInput { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-input");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.work.NotFound");
            404
        }
        crate::work::RemoveWorkInputOutcome::Removed { work_input_removed, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "removed");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.work.WorkInputRemoved");
            json::member(&mut body, "payload");
            wire::encode_event_agency_work_work_input_removed(work_input_removed, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::work::RemoveWorkInputOutcome::WrongState { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.work.StateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.teams.Report`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_teams_report<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_teams_report(system, &value, taken))
}

/// `agency.teams.Report` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_teams_report<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_teams_report(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.report(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_teams_report(&outcome))
}

/// One declared outcome of `agency.teams.Report`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_teams_report(outcome: &crate::teams::ReportOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::teams::ReportOutcome::NoSuchTeam { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-team");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.teams.NotFound");
            404
        }
        crate::teams::ReportOutcome::NotOneLevelUp { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "not-one-level-up");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.teams.NotOneLevelUp");
            502
        }
        crate::teams::ReportOutcome::Reported { report_sent, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "reported");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.teams.ReportSent");
            json::member(&mut body, "payload");
            wire::encode_event_agency_teams_report_sent(report_sent, &mut body);
            body.push('}');
            body.push(']');
            202
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.runs.RequestRunApproval`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_runs_request_run_approval<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_runs_request_run_approval(system, &value, taken))
}

/// `agency.runs.RequestRunApproval` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_runs_request_run_approval<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_runs_request_run_approval(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.request_run_approval(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_runs_request_run_approval(&outcome))
}

/// One declared outcome of `agency.runs.RequestRunApproval`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_runs_request_run_approval(outcome: &crate::runs::RequestRunApprovalOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::runs::RequestRunApprovalOutcome::Requested { run_approval_requested, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "requested");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.runs.RunApprovalRequested");
            json::member(&mut body, "payload");
            wire::encode_event_agency_runs_run_approval_requested(run_approval_requested, &mut body);
            body.push('}');
            body.push(']');
            202
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.agents.RetireAgent`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_agents_retire_agent<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_agents_retire_agent(system, &value, taken))
}

/// `agency.agents.RetireAgent` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_agents_retire_agent<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_agents_retire_agent(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.retire_agent(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_agents_retire_agent(&outcome))
}

/// One declared outcome of `agency.agents.RetireAgent`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_agents_retire_agent(outcome: &crate::agents::RetireAgentOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::agents::RetireAgentOutcome::NoSuchAgent { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-agent");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.agents.NotFound");
            404
        }
        crate::agents::RetireAgentOutcome::Retired { agent_retired, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "retired");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.agents.AgentRetired");
            json::member(&mut body, "payload");
            wire::encode_event_agency_agents_agent_retired(agent_retired, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::agents::RetireAgentOutcome::WrongState { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.agents.StateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.teams.RetireWorkforce`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_teams_retire_workforce<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_teams_retire_workforce(system, &value, taken))
}

/// `agency.teams.RetireWorkforce` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_teams_retire_workforce<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_teams_retire_workforce(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.retire_workforce(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_teams_retire_workforce(&outcome))
}

/// One declared outcome of `agency.teams.RetireWorkforce`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_teams_retire_workforce(outcome: &crate::teams::RetireWorkforceOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::teams::RetireWorkforceOutcome::NoSuchWorkforce { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-workforce");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.teams.NotFound");
            404
        }
        crate::teams::RetireWorkforceOutcome::Retired { workforce_retired, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "retired");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.teams.WorkforceRetired");
            json::member(&mut body, "payload");
            wire::encode_event_agency_teams_workforce_retired(workforce_retired, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::teams::RetireWorkforceOutcome::WrongState { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.teams.StateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.work.ReturnWorkItem`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_work_return_work_item<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_work_return_work_item(system, &value, taken))
}

/// `agency.work.ReturnWorkItem` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_work_return_work_item<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_work_return_work_item(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.return_work_item(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_work_return_work_item(&outcome))
}

/// One declared outcome of `agency.work.ReturnWorkItem`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_work_return_work_item(outcome: &crate::work::ReturnWorkItemOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::work::ReturnWorkItemOutcome::NoSuchItem { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-item");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.work.NotFound");
            404
        }
        crate::work::ReturnWorkItemOutcome::Returned { work_item_returned, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "returned");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.work.WorkItemReturned");
            json::member(&mut body, "payload");
            wire::encode_event_agency_work_work_item_returned(work_item_returned, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::work::ReturnWorkItemOutcome::WrongState { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.work.StateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.tools.RevokeBinding`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_tools_revoke_binding<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_tools_revoke_binding(system, &value, taken))
}

/// `agency.tools.RevokeBinding` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_tools_revoke_binding<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_tools_revoke_binding(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.revoke_binding(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_tools_revoke_binding(&outcome))
}

/// One declared outcome of `agency.tools.RevokeBinding`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_tools_revoke_binding(outcome: &crate::tools::RevokeBindingOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::tools::RevokeBindingOutcome::NoSuchBinding { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-binding");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.tools.NotFound");
            404
        }
        crate::tools::RevokeBindingOutcome::Revoked { binding_revoked, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "revoked");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.tools.BindingRevoked");
            json::member(&mut body, "payload");
            wire::encode_event_agency_tools_binding_revoked(binding_revoked, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::tools::RevokeBindingOutcome::WrongState { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.tools.StateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.auth.RevokeCredential`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_auth_revoke_credential<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_auth_revoke_credential(system, &value, taken))
}

/// `agency.auth.RevokeCredential` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_auth_revoke_credential<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_auth_revoke_credential(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.revoke_credential(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_auth_revoke_credential(&outcome))
}

/// One declared outcome of `agency.auth.RevokeCredential`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_auth_revoke_credential(outcome: &crate::auth::RevokeCredentialOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::auth::RevokeCredentialOutcome::NoSuchCredential { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-credential");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.auth.NotFound");
            404
        }
        crate::auth::RevokeCredentialOutcome::Revoked { credential_revoked, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "revoked");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.auth.CredentialRevoked");
            json::member(&mut body, "payload");
            wire::encode_event_agency_auth_credential_revoked(credential_revoked, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::auth::RevokeCredentialOutcome::WrongState { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.auth.StateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.auth.RevokeGrant`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_auth_revoke_grant<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_auth_revoke_grant(system, &value, taken))
}

/// `agency.auth.RevokeGrant` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_auth_revoke_grant<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_auth_revoke_grant(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.revoke_grant(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_auth_revoke_grant(&outcome))
}

/// One declared outcome of `agency.auth.RevokeGrant`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_auth_revoke_grant(outcome: &crate::auth::RevokeGrantOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::auth::RevokeGrantOutcome::NoSuchGrant { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-grant");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.auth.NotFound");
            404
        }
        crate::auth::RevokeGrantOutcome::Revoked { grant_revoked, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "revoked");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.auth.GrantRevoked");
            json::member(&mut body, "payload");
            wire::encode_event_agency_auth_grant_revoked(grant_revoked, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::auth::RevokeGrantOutcome::WrongState { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.auth.StateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.teams.Send`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_teams_send<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_teams_send(system, &value, taken))
}

/// `agency.teams.Send` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_teams_send<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_teams_send(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.send(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_teams_send(&outcome))
}

/// One declared outcome of `agency.teams.Send`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_teams_send(outcome: &crate::teams::SendOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::teams::SendOutcome::NoSuchTeam { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-team");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.teams.NotFound");
            404
        }
        crate::teams::SendOutcome::ForeignInstance { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "foreign-instance");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.teams.ForeignInstance");
            502
        }
        crate::teams::SendOutcome::OutsideTeam { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "outside-team");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.teams.OutsideTeam");
            502
        }
        crate::teams::SendOutcome::Sent { message_sent, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "sent");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.teams.MessageSent");
            json::member(&mut body, "payload");
            wire::encode_event_agency_teams_message_sent(message_sent, &mut body);
            body.push('}');
            body.push(']');
            202
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.teams.SetLead`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_teams_set_lead<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_teams_set_lead(system, &value, taken))
}

/// `agency.teams.SetLead` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_teams_set_lead<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_teams_set_lead(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.set_lead(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_teams_set_lead(&outcome))
}

/// One declared outcome of `agency.teams.SetLead`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_teams_set_lead(outcome: &crate::teams::SetLeadOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::teams::SetLeadOutcome::NoSuchTeam { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-team");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.teams.NotFound");
            404
        }
        crate::teams::SetLeadOutcome::Set { lead_set, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "set");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.teams.LeadSet");
            json::member(&mut body, "payload");
            wire::encode_event_agency_teams_lead_set(lead_set, &mut body);
            body.push('}');
            body.push(']');
            202
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.runs.SetRunBudget`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_runs_set_run_budget<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_runs_set_run_budget(system, &value, taken))
}

/// `agency.runs.SetRunBudget` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_runs_set_run_budget<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_runs_set_run_budget(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.set_run_budget(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_runs_set_run_budget(&outcome))
}

/// One declared outcome of `agency.runs.SetRunBudget`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_runs_set_run_budget(outcome: &crate::runs::SetRunBudgetOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::runs::SetRunBudgetOutcome::NoSuchRun { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-run");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.runs.NotFound");
            404
        }
        crate::runs::SetRunBudgetOutcome::Ended { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "ended");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.runs.RunEnded");
            409
        }
        crate::runs::SetRunBudgetOutcome::Set { run_budget_set, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "set");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.runs.RunBudgetSet");
            json::member(&mut body, "payload");
            wire::encode_event_agency_runs_run_budget_set(run_budget_set, &mut body);
            body.push('}');
            body.push(']');
            202
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.work.SettleWorkItem`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_work_settle_work_item<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_work_settle_work_item(system, &value, taken))
}

/// `agency.work.SettleWorkItem` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_work_settle_work_item<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_work_settle_work_item(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.settle_work_item(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_work_settle_work_item(&outcome))
}

/// One declared outcome of `agency.work.SettleWorkItem`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_work_settle_work_item(outcome: &crate::work::SettleWorkItemOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::work::SettleWorkItemOutcome::NoSuchItem { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-item");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.work.NotFound");
            404
        }
        crate::work::SettleWorkItemOutcome::Settled { work_item_settled, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "settled");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.work.WorkItemSettled");
            json::member(&mut body, "payload");
            wire::encode_event_agency_work_work_item_settled(work_item_settled, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::work::SettleWorkItemOutcome::WrongState { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.work.StateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.teams.StaffWorkforce`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_teams_staff_workforce<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_teams_staff_workforce(system, &value, taken))
}

/// `agency.teams.StaffWorkforce` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_teams_staff_workforce<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_teams_staff_workforce(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.staff_workforce(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_teams_staff_workforce(&outcome))
}

/// One declared outcome of `agency.teams.StaffWorkforce`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_teams_staff_workforce(outcome: &crate::teams::StaffWorkforceOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::teams::StaffWorkforceOutcome::NoSuchWorkforce { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-workforce");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.teams.NotFound");
            404
        }
        crate::teams::StaffWorkforceOutcome::Staffed { workforce_staffed, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "staffed");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.teams.WorkforceStaffed");
            json::member(&mut body, "payload");
            wire::encode_event_agency_teams_workforce_staffed(workforce_staffed, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::teams::StaffWorkforceOutcome::WrongState { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.teams.StateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.runs.StartRun`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_runs_start_run<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_runs_start_run(system, &value, taken))
}

/// `agency.runs.StartRun` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_runs_start_run<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_runs_start_run(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.start_run(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_runs_start_run(&outcome))
}

/// One declared outcome of `agency.runs.StartRun`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_runs_start_run(outcome: &crate::runs::StartRunOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::runs::StartRunOutcome::Requested { run_requested, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "requested");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.runs.RunRequested");
            json::member(&mut body, "payload");
            wire::encode_event_agency_runs_run_requested(run_requested, &mut body);
            body.push('}');
            body.push(']');
            202
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.runs.StopRun`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_runs_stop_run<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_runs_stop_run(system, &value, taken))
}

/// `agency.runs.StopRun` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_runs_stop_run<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_runs_stop_run(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.stop_run(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_runs_stop_run(&outcome))
}

/// One declared outcome of `agency.runs.StopRun`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_runs_stop_run(outcome: &crate::runs::StopRunOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::runs::StopRunOutcome::NoSuchRun { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-run");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.runs.NotFound");
            404
        }
        crate::runs::StopRunOutcome::Stopped { run_stopped, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "stopped");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.runs.RunStopped");
            json::member(&mut body, "payload");
            wire::encode_event_agency_runs_run_stopped(run_stopped, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::runs::StopRunOutcome::WrongState { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.runs.StateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.teams.StopSwarm`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_teams_stop_swarm<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_teams_stop_swarm(system, &value, taken))
}

/// `agency.teams.StopSwarm` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_teams_stop_swarm<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_teams_stop_swarm(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.stop_swarm(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_teams_stop_swarm(&outcome))
}

/// One declared outcome of `agency.teams.StopSwarm`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_teams_stop_swarm(outcome: &crate::teams::StopSwarmOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::teams::StopSwarmOutcome::NoSuchSwarm { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-swarm");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.teams.NotFound");
            404
        }
        crate::teams::StopSwarmOutcome::Stopped { swarm_stopped, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "stopped");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.teams.SwarmStopped");
            json::member(&mut body, "payload");
            wire::encode_event_agency_teams_swarm_stopped(swarm_stopped, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::teams::StopSwarmOutcome::WrongState { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.teams.StateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.auth.StoreCredential`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_auth_store_credential<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_auth_store_credential(system, &value, taken))
}

/// `agency.auth.StoreCredential` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_auth_store_credential<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_auth_store_credential(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.store_credential(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_auth_store_credential(&outcome))
}

/// One declared outcome of `agency.auth.StoreCredential`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_auth_store_credential(outcome: &crate::auth::StoreCredentialOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::auth::StoreCredentialOutcome::Stored { credential_stored, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "stored");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.auth.CredentialStored");
            json::member(&mut body, "payload");
            wire::encode_event_agency_auth_credential_stored(credential_stored, &mut body);
            body.push('}');
            body.push(']');
            202
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.work.SubmitWorkItem`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_work_submit_work_item<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_work_submit_work_item(system, &value, taken))
}

/// `agency.work.SubmitWorkItem` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_work_submit_work_item<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_work_submit_work_item(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.submit_work_item(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_work_submit_work_item(&outcome))
}

/// One declared outcome of `agency.work.SubmitWorkItem`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_work_submit_work_item(outcome: &crate::work::SubmitWorkItemOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::work::SubmitWorkItemOutcome::Received { work_item_submitted, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "received");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.work.WorkItemSubmitted");
            json::member(&mut body, "payload");
            wire::encode_event_agency_work_work_item_submitted(work_item_submitted, &mut body);
            body.push('}');
            body.push(']');
            202
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.auth.SupersedePolicy`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_auth_supersede_policy<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_auth_supersede_policy(system, &value, taken))
}

/// `agency.auth.SupersedePolicy` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_auth_supersede_policy<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_auth_supersede_policy(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.supersede_policy(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_auth_supersede_policy(&outcome))
}

/// One declared outcome of `agency.auth.SupersedePolicy`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_auth_supersede_policy(outcome: &crate::auth::SupersedePolicyOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::auth::SupersedePolicyOutcome::NoSuchPolicy { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-policy");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.auth.NotFound");
            404
        }
        crate::auth::SupersedePolicyOutcome::Superseded { policy_superseded, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "superseded");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.auth.PolicySuperseded");
            json::member(&mut body, "payload");
            wire::encode_event_agency_auth_policy_superseded(policy_superseded, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::auth::SupersedePolicyOutcome::WrongState { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.auth.StateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agency.tools.WithdrawTool`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agency_tools_withdraw_tool<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, body: &[u8], taken: &mut Vec<crate::system::SystemEvent>) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agency_tools_withdraw_tool(system, &value, taken))
}

/// `agency.tools.WithdrawTool` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agency_tools_withdraw_tool<AgencyBehaviors>(system: &mut crate::system::System<AgencyBehaviors>, value: &json::Value, taken: &mut Vec<crate::system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agency_tools_withdraw_tool(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.agency.withdraw_tool(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agency_tools_withdraw_tool(&outcome))
}

/// One declared outcome of `agency.tools.WithdrawTool`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agency_tools_withdraw_tool(outcome: &crate::tools::WithdrawToolOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        crate::tools::WithdrawToolOutcome::NoSuchTool { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-tool");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.tools.NotFound");
            404
        }
        crate::tools::WithdrawToolOutcome::Withdrawn { tool_withdrawn, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "withdrawn");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agency.tools.ToolWithdrawn");
            json::member(&mut body, "payload");
            wire::encode_event_agency_tools_tool_withdrawn(tool_withdrawn, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        crate::tools::WithdrawToolOutcome::WrongState { .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agency.tools.StateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `GET` `agency.agents.AgentDefinitionList` at `read_your_writes` consistency: every row the owed projection holds.
///
/// The one path the `GET` route and [`handle`] share.
fn run_agency_agents_agent_definition_list<AgencyBehaviors>(system: &crate::system::System<AgencyBehaviors>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    match system.agency.agent_definition_list() {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agency_agents_agent_definition_list(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agency.agents.AgentList` at `read_your_writes` consistency: every row the owed projection holds.
///
/// The one path the `GET` route and [`handle`] share.
fn run_agency_agents_agent_list<AgencyBehaviors>(system: &crate::system::System<AgencyBehaviors>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    match system.agency.agent_list() {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agency_agents_agent_list(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agency.runs.ApprovalInbox` at `read_your_writes` consistency: every row the owed projection holds.
///
/// The one path the `GET` route and [`handle`] share.
fn run_agency_runs_approval_inbox<AgencyBehaviors>(system: &crate::system::System<AgencyBehaviors>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    match system.agency.approval_inbox() {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agency_runs_approval_inbox(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agency.tools.BindingList` at `read_your_writes` consistency: every row the owed projection holds.
///
/// The one path the `GET` route and [`handle`] share.
fn run_agency_tools_binding_list<AgencyBehaviors>(system: &crate::system::System<AgencyBehaviors>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    match system.agency.binding_list() {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agency_tools_binding_list(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agency.auth.CredentialList` at `read_your_writes` consistency: every row the owed projection holds.
///
/// The one path the `GET` route and [`handle`] share.
fn run_agency_auth_credential_list<AgencyBehaviors>(system: &crate::system::System<AgencyBehaviors>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    match system.agency.credential_list() {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agency_auth_credential_list(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agency.auth.EffectivePolicy`: reads the declared parameters from the query string by their wire names, runs the
/// port, answers its rows. A key the view does not declare is ignored.
fn serve_agency_auth_effective_policy<AgencyBehaviors>(system: &crate::system::System<AgencyBehaviors>, query: &str) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    match http::query_object(query, &[("definition_id", http::Scalar::Text)], "query") {
        Ok(value) => http::answer(run_agency_auth_effective_policy(system, &value)),
        Err(refused) => http::Response::refusal(400, &refused),
    }
}

/// `GET` `agency.auth.EffectivePolicy` at `read_your_writes` consistency: every row the owed projection holds for the declared
/// parameters, read from `value`, an object keyed by their wire names.
///
/// The one path the `GET` route and [`handle`] share. A decoding failure is located under
/// `query`, as the route reports it.
fn run_agency_auth_effective_policy<AgencyBehaviors>(system: &crate::system::System<AgencyBehaviors>, value: &json::Value) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let (param0,) = match wire::decode_params_agency_auth_effective_policy(value, "query") {
        Ok(params) => params,
        Err(error) => return Err(entry::Refused::Input(format!("{error}"))),
    };
    match system.agency.effective_policy(param0) {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agency_auth_effective_policy(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agency.auth.GrantMatrix` at `read_your_writes` consistency: every row the owed projection holds.
///
/// The one path the `GET` route and [`handle`] share.
fn run_agency_auth_grant_matrix<AgencyBehaviors>(system: &crate::system::System<AgencyBehaviors>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    match system.agency.grant_matrix() {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agency_auth_grant_matrix(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agency.auth.PolicyList` at `read_your_writes` consistency: every row the owed projection holds.
///
/// The one path the `GET` route and [`handle`] share.
fn run_agency_auth_policy_list<AgencyBehaviors>(system: &crate::system::System<AgencyBehaviors>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    match system.agency.policy_list() {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agency_auth_policy_list(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agency.auth.PrincipalList` at `read_your_writes` consistency: every row the owed projection holds.
///
/// The one path the `GET` route and [`handle`] share.
fn run_agency_auth_principal_list<AgencyBehaviors>(system: &crate::system::System<AgencyBehaviors>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    match system.agency.principal_list() {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agency_auth_principal_list(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agency.runs.RunDetail`: reads the declared parameters from the query string by their wire names, runs the
/// port, answers its rows. A key the view does not declare is ignored.
fn serve_agency_runs_run_detail<AgencyBehaviors>(system: &crate::system::System<AgencyBehaviors>, query: &str) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    match http::query_object(query, &[("run_id", http::Scalar::Text)], "query") {
        Ok(value) => http::answer(run_agency_runs_run_detail(system, &value)),
        Err(refused) => http::Response::refusal(400, &refused),
    }
}

/// `GET` `agency.runs.RunDetail` at `read_your_writes` consistency: every row the owed projection holds for the declared
/// parameters, read from `value`, an object keyed by their wire names.
///
/// The one path the `GET` route and [`handle`] share. A decoding failure is located under
/// `query`, as the route reports it.
fn run_agency_runs_run_detail<AgencyBehaviors>(system: &crate::system::System<AgencyBehaviors>, value: &json::Value) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let (param0,) = match wire::decode_params_agency_runs_run_detail(value, "query") {
        Ok(params) => params,
        Err(error) => return Err(entry::Refused::Input(format!("{error}"))),
    };
    match system.agency.run_detail(param0) {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agency_runs_run_detail(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agency.runs.RunList` at `read_your_writes` consistency: every row the owed projection holds.
///
/// The one path the `GET` route and [`handle`] share.
fn run_agency_runs_run_list<AgencyBehaviors>(system: &crate::system::System<AgencyBehaviors>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    match system.agency.run_list() {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agency_runs_run_list(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agency.service.SessionList` at `read_your_writes` consistency: every row the owed projection holds.
///
/// The one path the `GET` route and [`handle`] share.
fn run_agency_service_session_list<AgencyBehaviors>(system: &crate::system::System<AgencyBehaviors>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    match system.agency.session_list() {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agency_service_session_list(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agency.teams.SwarmList` at `read_your_writes` consistency: every row the owed projection holds.
///
/// The one path the `GET` route and [`handle`] share.
fn run_agency_teams_swarm_list<AgencyBehaviors>(system: &crate::system::System<AgencyBehaviors>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    match system.agency.swarm_list() {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agency_teams_swarm_list(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agency.teams.TeamDetail`: reads the declared parameters from the query string by their wire names, runs the
/// port, answers its rows. A key the view does not declare is ignored.
fn serve_agency_teams_team_detail<AgencyBehaviors>(system: &crate::system::System<AgencyBehaviors>, query: &str) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    match http::query_object(query, &[("team_id", http::Scalar::Text)], "query") {
        Ok(value) => http::answer(run_agency_teams_team_detail(system, &value)),
        Err(refused) => http::Response::refusal(400, &refused),
    }
}

/// `GET` `agency.teams.TeamDetail` at `read_your_writes` consistency: every row the owed projection holds for the declared
/// parameters, read from `value`, an object keyed by their wire names.
///
/// The one path the `GET` route and [`handle`] share. A decoding failure is located under
/// `query`, as the route reports it.
fn run_agency_teams_team_detail<AgencyBehaviors>(system: &crate::system::System<AgencyBehaviors>, value: &json::Value) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let (param0,) = match wire::decode_params_agency_teams_team_detail(value, "query") {
        Ok(params) => params,
        Err(error) => return Err(entry::Refused::Input(format!("{error}"))),
    };
    match system.agency.team_detail(param0) {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agency_teams_team_detail(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agency.teams.TeamList` at `read_your_writes` consistency: every row the owed projection holds.
///
/// The one path the `GET` route and [`handle`] share.
fn run_agency_teams_team_list<AgencyBehaviors>(system: &crate::system::System<AgencyBehaviors>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    match system.agency.team_list() {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agency_teams_team_list(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agency.tools.ToolCatalogue` at `read_your_writes` consistency: every row the owed projection holds.
///
/// The one path the `GET` route and [`handle`] share.
fn run_agency_tools_tool_catalogue<AgencyBehaviors>(system: &crate::system::System<AgencyBehaviors>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    match system.agency.tool_catalogue() {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agency_tools_tool_catalogue(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agency.runs.UsageByAgent` at `read_your_writes` consistency: every row the owed projection holds.
///
/// The one path the `GET` route and [`handle`] share.
fn run_agency_runs_usage_by_agent<AgencyBehaviors>(system: &crate::system::System<AgencyBehaviors>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    match system.agency.usage_by_agent() {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agency_runs_usage_by_agent(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agency.work.WorkInputList` at `read_your_writes` consistency: every row the owed projection holds.
///
/// The one path the `GET` route and [`handle`] share.
fn run_agency_work_work_input_list<AgencyBehaviors>(system: &crate::system::System<AgencyBehaviors>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    match system.agency.work_input_list() {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agency_work_work_input_list(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agency.work.WorkItemDetail`: reads the declared parameters from the query string by their wire names, runs the
/// port, answers its rows. A key the view does not declare is ignored.
fn serve_agency_work_work_item_detail<AgencyBehaviors>(system: &crate::system::System<AgencyBehaviors>, query: &str) -> http::Response
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    match http::query_object(query, &[("item_ref", http::Scalar::Text)], "query") {
        Ok(value) => http::answer(run_agency_work_work_item_detail(system, &value)),
        Err(refused) => http::Response::refusal(400, &refused),
    }
}

/// `GET` `agency.work.WorkItemDetail` at `read_your_writes` consistency: every row the owed projection holds for the declared
/// parameters, read from `value`, an object keyed by their wire names.
///
/// The one path the `GET` route and [`handle`] share. A decoding failure is located under
/// `query`, as the route reports it.
fn run_agency_work_work_item_detail<AgencyBehaviors>(system: &crate::system::System<AgencyBehaviors>, value: &json::Value) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    let (param0,) = match wire::decode_params_agency_work_work_item_detail(value, "query") {
        Ok(params) => params,
        Err(error) => return Err(entry::Refused::Input(format!("{error}"))),
    };
    match system.agency.work_item_detail(param0) {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agency_work_work_item_detail(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agency.work.WorkQueue` at `read_your_writes` consistency: every row the owed projection holds.
///
/// The one path the `GET` route and [`handle`] share.
fn run_agency_work_work_queue<AgencyBehaviors>(system: &crate::system::System<AgencyBehaviors>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    match system.agency.work_queue() {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agency_work_work_queue(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agency.teams.WorkforceBoard` at `read_your_writes` consistency: every row the owed projection holds.
///
/// The one path the `GET` route and [`handle`] share.
fn run_agency_teams_workforce_board<AgencyBehaviors>(system: &crate::system::System<AgencyBehaviors>) -> Result<(u16, String), entry::Refused>
where
    AgencyBehaviors: crate::agents::obligations::CreateAgentDefinitionBehavior + crate::agents::obligations::DeprecateAgentDefinitionBehavior + crate::agents::obligations::ProvisionAgentBehavior + crate::agents::obligations::PublishAgentDefinitionBehavior + crate::agents::obligations::RecordSpecCheckBehavior + crate::agents::obligations::RetireAgentBehavior + crate::auth::obligations::ActivatePolicyBehavior + crate::auth::obligations::AssignRoleBehavior + crate::auth::obligations::CreatePolicyBehavior + crate::auth::obligations::CreateTenantBehavior + crate::auth::obligations::DisablePrincipalBehavior + crate::auth::obligations::GrantCredentialBehavior + crate::auth::obligations::InvitePrincipalBehavior + crate::auth::obligations::RefreshCredentialBehavior + crate::auth::obligations::RevokeCredentialBehavior + crate::auth::obligations::RevokeGrantBehavior + crate::auth::obligations::StoreCredentialBehavior + crate::auth::obligations::SupersedePolicyBehavior + crate::runs::obligations::AdmitRunBehavior + crate::runs::obligations::DenyRunApprovalBehavior + crate::runs::obligations::GrantRunApprovalBehavior + crate::runs::obligations::RecordRunFailureBehavior + crate::runs::obligations::RecordRunProposalBehavior + crate::runs::obligations::RecordRunStartedBehavior + crate::runs::obligations::RequestRunApprovalBehavior + crate::runs::obligations::SetRunBudgetBehavior + crate::runs::obligations::StartRunBehavior + crate::runs::obligations::StopRunBehavior + crate::service::obligations::CloseSessionBehavior + crate::service::obligations::OpenSessionBehavior + crate::teams::obligations::AddMemberBehavior + crate::teams::obligations::AttachQueueBehavior + crate::teams::obligations::DeclareSwarmBehavior + crate::teams::obligations::DefineWorkforceBehavior + crate::teams::obligations::DisbandTeamBehavior + crate::teams::obligations::FormTeamBehavior + crate::teams::obligations::RemoveMemberBehavior + crate::teams::obligations::ReportBehavior + crate::teams::obligations::RetireWorkforceBehavior + crate::teams::obligations::SendBehavior + crate::teams::obligations::SetLeadBehavior + crate::teams::obligations::StaffWorkforceBehavior + crate::teams::obligations::StopSwarmBehavior + crate::tools::obligations::BindToolBehavior + crate::tools::obligations::RegisterToolBehavior + crate::tools::obligations::RevokeBindingBehavior + crate::tools::obligations::WithdrawToolBehavior + crate::work::obligations::AbandonWorkItemBehavior + crate::work::obligations::AssignWorkItemBehavior + crate::work::obligations::ConfigureWorkInputBehavior + crate::work::obligations::PauseWorkInputBehavior + crate::work::obligations::RemoveWorkInputBehavior + crate::work::obligations::ReturnWorkItemBehavior + crate::work::obligations::SettleWorkItemBehavior + crate::work::obligations::SubmitWorkItemBehavior + crate::agents::obligations::AgentDefinitionListQuery + crate::agents::obligations::AgentListQuery + crate::auth::obligations::CredentialListQuery + crate::auth::obligations::EffectivePolicyQuery + crate::auth::obligations::GrantMatrixQuery + crate::auth::obligations::PolicyListQuery + crate::auth::obligations::PrincipalListQuery + crate::runs::obligations::ApprovalInboxQuery + crate::runs::obligations::RunDetailQuery + crate::runs::obligations::RunListQuery + crate::runs::obligations::UsageByAgentQuery + crate::service::obligations::SessionListQuery + crate::teams::obligations::SwarmListQuery + crate::teams::obligations::TeamDetailQuery + crate::teams::obligations::TeamListQuery + crate::teams::obligations::WorkforceBoardQuery + crate::tools::obligations::BindingListQuery + crate::tools::obligations::ToolCatalogueQuery + crate::work::obligations::WorkInputListQuery + crate::work::obligations::WorkItemDetailQuery + crate::work::obligations::WorkQueueQuery,
{
    match system.agency.workforce_board() {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agency_teams_workforce_board(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}
