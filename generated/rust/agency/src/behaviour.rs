// generated from agency v1
// model digest 71132e90bde806415603d08bb97a71629b2c2f6cda30e1f22e76f407abd9a3eb
// contract digest a96f55e3e1ca68aeb1579b032c3547fffd27e8ab80e76bcb898f9b6c9258792e
// do not edit: regenerate with `ess synthesize --layout crate`

//! What the specification fully determines, generated: the behaviour of every command the plan
//! lists as generated, written against ports the implementor supplies.
//!
//! Storage is a port: one trait per entity, get, put and delete of a snapshot by identity. ess
//! preserves the trait; generated network entries supply an ephemeral store. `Context` carries the caller's attributes,
//! every identity and value the model says the implementation assigns, and the answer to each
//! `external:` branch. [`Generated`] implements every generated `…Behavior` trait over those ports
//! and forwards every behaviour and query the plan still owes to them, so it is a complete bundle
//! for every component port. To replace one generated behaviour, write a bundle of your own that
//! implements that trait and delegates the rest to a `Generated`.
//!
//! An `Err` from a generated behaviour is the typed refusal naming the command: the model declares
//! no outcome for the request (a guard is undecidable over it, or no declared branch answers it),
//! or — as `entity invariant` — the declared outcome would leave an entity breaking an invariant.

use crate::obligation::UnmetObligation;

/// Where `agency.agents.Agent` is stored — a port the implementor provides.
///
/// Keyed by the identity `agent_id`. Generated network entries supply an ephemeral implementation; durable storage remains a port.
pub trait AgentStorage {
    /// The instance with this identity, or `None` where none is stored.
    fn get(&self, identity: &crate::agents::AgentId) -> Option<crate::agents::AgentSnapshot>;

    /// Stores this instance under its identity, replacing what was held.
    fn put(&mut self, snapshot: crate::agents::AgentSnapshot);

    /// Removes the instance with this identity.
    fn delete(&mut self, identity: &crate::agents::AgentId);

    /// Every stored instance, in the order the store keeps them: the order a generated query
    /// answers an unordered view in.
    fn list(&self) -> Vec<crate::agents::AgentSnapshot>;
}

/// Where `agency.agents.AgentDefinition` is stored — a port the implementor provides.
///
/// Keyed by the identity `definition_id`. Generated network entries supply an ephemeral implementation; durable storage remains a port.
pub trait AgentDefinitionStorage {
    /// The instance with this identity, or `None` where none is stored.
    fn get(&self, identity: &crate::agents::DefinitionId) -> Option<crate::agents::AgentDefinitionSnapshot>;

    /// Stores this instance under its identity, replacing what was held.
    fn put(&mut self, snapshot: crate::agents::AgentDefinitionSnapshot);

    /// Removes the instance with this identity.
    fn delete(&mut self, identity: &crate::agents::DefinitionId);

    /// Every stored instance, in the order the store keeps them: the order a generated query
    /// answers an unordered view in.
    fn list(&self) -> Vec<crate::agents::AgentDefinitionSnapshot>;
}

/// Where `agency.auth.Credential` is stored — a port the implementor provides.
///
/// Keyed by the identity `credential_id`. Generated network entries supply an ephemeral implementation; durable storage remains a port.
pub trait CredentialStorage {
    /// The instance with this identity, or `None` where none is stored.
    fn get(&self, identity: &crate::auth::CredentialId) -> Option<crate::auth::CredentialSnapshot>;

    /// Stores this instance under its identity, replacing what was held.
    fn put(&mut self, snapshot: crate::auth::CredentialSnapshot);

    /// Removes the instance with this identity.
    fn delete(&mut self, identity: &crate::auth::CredentialId);

    /// Every stored instance, in the order the store keeps them: the order a generated query
    /// answers an unordered view in.
    fn list(&self) -> Vec<crate::auth::CredentialSnapshot>;
}

/// Where `agency.auth.CredentialGrant` is stored — a port the implementor provides.
///
/// Keyed by the identity `grant_id`. Generated network entries supply an ephemeral implementation; durable storage remains a port.
pub trait CredentialGrantStorage {
    /// The instance with this identity, or `None` where none is stored.
    fn get(&self, identity: &crate::auth::GrantId) -> Option<crate::auth::CredentialGrantSnapshot>;

    /// Stores this instance under its identity, replacing what was held.
    fn put(&mut self, snapshot: crate::auth::CredentialGrantSnapshot);

    /// Removes the instance with this identity.
    fn delete(&mut self, identity: &crate::auth::GrantId);

    /// Every stored instance, in the order the store keeps them: the order a generated query
    /// answers an unordered view in.
    fn list(&self) -> Vec<crate::auth::CredentialGrantSnapshot>;
}

/// Where `agency.auth.Policy` is stored — a port the implementor provides.
///
/// Keyed by the identity `policy_id`. Generated network entries supply an ephemeral implementation; durable storage remains a port.
pub trait PolicyStorage {
    /// The instance with this identity, or `None` where none is stored.
    fn get(&self, identity: &crate::auth::PolicyId) -> Option<crate::auth::PolicySnapshot>;

    /// Stores this instance under its identity, replacing what was held.
    fn put(&mut self, snapshot: crate::auth::PolicySnapshot);

    /// Removes the instance with this identity.
    fn delete(&mut self, identity: &crate::auth::PolicyId);

    /// Every stored instance, in the order the store keeps them: the order a generated query
    /// answers an unordered view in.
    fn list(&self) -> Vec<crate::auth::PolicySnapshot>;
}

/// Where `agency.auth.Principal` is stored — a port the implementor provides.
///
/// Keyed by the identity `principal_id`. Generated network entries supply an ephemeral implementation; durable storage remains a port.
pub trait PrincipalStorage {
    /// The instance with this identity, or `None` where none is stored.
    fn get(&self, identity: &crate::auth::PrincipalId) -> Option<crate::auth::PrincipalSnapshot>;

    /// Stores this instance under its identity, replacing what was held.
    fn put(&mut self, snapshot: crate::auth::PrincipalSnapshot);

    /// Removes the instance with this identity.
    fn delete(&mut self, identity: &crate::auth::PrincipalId);

    /// Every stored instance, in the order the store keeps them: the order a generated query
    /// answers an unordered view in.
    fn list(&self) -> Vec<crate::auth::PrincipalSnapshot>;
}

/// Where `agency.auth.Tenant` is stored — a port the implementor provides.
///
/// Keyed by the identity `tenant_id`. Generated network entries supply an ephemeral implementation; durable storage remains a port.
pub trait TenantStorage {
    /// The instance with this identity, or `None` where none is stored.
    fn get(&self, identity: &crate::auth::TenantId) -> Option<crate::auth::TenantSnapshot>;

    /// Stores this instance under its identity, replacing what was held.
    fn put(&mut self, snapshot: crate::auth::TenantSnapshot);

    /// Removes the instance with this identity.
    fn delete(&mut self, identity: &crate::auth::TenantId);
}

/// Where `agency.runs.Run` is stored — a port the implementor provides.
///
/// Keyed by the identity `run_id`. Generated network entries supply an ephemeral implementation; durable storage remains a port.
pub trait RunStorage {
    /// The instance with this identity, or `None` where none is stored.
    fn get(&self, identity: &crate::runs::RunId) -> Option<crate::runs::RunSnapshot>;

    /// Stores this instance under its identity, replacing what was held.
    fn put(&mut self, snapshot: crate::runs::RunSnapshot);

    /// Removes the instance with this identity.
    fn delete(&mut self, identity: &crate::runs::RunId);

    /// Every stored instance, in the order the store keeps them: the order a generated query
    /// answers an unordered view in.
    fn list(&self) -> Vec<crate::runs::RunSnapshot>;
}

/// Where `agency.runs.RunApproval` is stored — a port the implementor provides.
///
/// Keyed by the identity `approval_id`. Generated network entries supply an ephemeral implementation; durable storage remains a port.
pub trait RunApprovalStorage {
    /// The instance with this identity, or `None` where none is stored.
    fn get(&self, identity: &crate::runs::ApprovalId) -> Option<crate::runs::RunApprovalSnapshot>;

    /// Stores this instance under its identity, replacing what was held.
    fn put(&mut self, snapshot: crate::runs::RunApprovalSnapshot);

    /// Removes the instance with this identity.
    fn delete(&mut self, identity: &crate::runs::ApprovalId);

    /// Every stored instance, in the order the store keeps them: the order a generated query
    /// answers an unordered view in.
    fn list(&self) -> Vec<crate::runs::RunApprovalSnapshot>;
}

/// Where `agency.service.Session` is stored — a port the implementor provides.
///
/// Keyed by the identity `session_id`. Generated network entries supply an ephemeral implementation; durable storage remains a port.
pub trait SessionStorage {
    /// The instance with this identity, or `None` where none is stored.
    fn get(&self, identity: &crate::service::SessionId) -> Option<crate::service::SessionSnapshot>;

    /// Stores this instance under its identity, replacing what was held.
    fn put(&mut self, snapshot: crate::service::SessionSnapshot);

    /// Removes the instance with this identity.
    fn delete(&mut self, identity: &crate::service::SessionId);

    /// Every stored instance, in the order the store keeps them: the order a generated query
    /// answers an unordered view in.
    fn list(&self) -> Vec<crate::service::SessionSnapshot>;
}

/// Where `agency.teams.Swarm` is stored — a port the implementor provides.
///
/// Keyed by the identity `swarm_id`. Generated network entries supply an ephemeral implementation; durable storage remains a port.
pub trait SwarmStorage {
    /// The instance with this identity, or `None` where none is stored.
    fn get(&self, identity: &crate::teams::SwarmId) -> Option<crate::teams::SwarmSnapshot>;

    /// Stores this instance under its identity, replacing what was held.
    fn put(&mut self, snapshot: crate::teams::SwarmSnapshot);

    /// Removes the instance with this identity.
    fn delete(&mut self, identity: &crate::teams::SwarmId);

    /// Every stored instance, in the order the store keeps them: the order a generated query
    /// answers an unordered view in.
    fn list(&self) -> Vec<crate::teams::SwarmSnapshot>;
}

/// Where `agency.teams.Team` is stored — a port the implementor provides.
///
/// Keyed by the identity `team_id`. Generated network entries supply an ephemeral implementation; durable storage remains a port.
pub trait TeamStorage {
    /// The instance with this identity, or `None` where none is stored.
    fn get(&self, identity: &crate::teams::TeamId) -> Option<crate::teams::TeamSnapshot>;

    /// Stores this instance under its identity, replacing what was held.
    fn put(&mut self, snapshot: crate::teams::TeamSnapshot);

    /// Removes the instance with this identity.
    fn delete(&mut self, identity: &crate::teams::TeamId);

    /// Every stored instance, in the order the store keeps them: the order a generated query
    /// answers an unordered view in.
    fn list(&self) -> Vec<crate::teams::TeamSnapshot>;
}

/// Where `agency.teams.Workforce` is stored — a port the implementor provides.
///
/// Keyed by the identity `workforce_id`. Generated network entries supply an ephemeral implementation; durable storage remains a port.
pub trait WorkforceStorage {
    /// The instance with this identity, or `None` where none is stored.
    fn get(&self, identity: &crate::teams::WorkforceId) -> Option<crate::teams::WorkforceSnapshot>;

    /// Stores this instance under its identity, replacing what was held.
    fn put(&mut self, snapshot: crate::teams::WorkforceSnapshot);

    /// Removes the instance with this identity.
    fn delete(&mut self, identity: &crate::teams::WorkforceId);

    /// Every stored instance, in the order the store keeps them: the order a generated query
    /// answers an unordered view in.
    fn list(&self) -> Vec<crate::teams::WorkforceSnapshot>;
}

/// Where `agency.tools.Tool` is stored — a port the implementor provides.
///
/// Keyed by the identity `tool_id`. Generated network entries supply an ephemeral implementation; durable storage remains a port.
pub trait ToolStorage {
    /// The instance with this identity, or `None` where none is stored.
    fn get(&self, identity: &crate::tools::ToolId) -> Option<crate::tools::ToolSnapshot>;

    /// Stores this instance under its identity, replacing what was held.
    fn put(&mut self, snapshot: crate::tools::ToolSnapshot);

    /// Removes the instance with this identity.
    fn delete(&mut self, identity: &crate::tools::ToolId);

    /// Every stored instance, in the order the store keeps them: the order a generated query
    /// answers an unordered view in.
    fn list(&self) -> Vec<crate::tools::ToolSnapshot>;
}

/// Where `agency.tools.ToolBinding` is stored — a port the implementor provides.
///
/// Keyed by the identity `binding_id`. Generated network entries supply an ephemeral implementation; durable storage remains a port.
pub trait ToolBindingStorage {
    /// The instance with this identity, or `None` where none is stored.
    fn get(&self, identity: &crate::tools::BindingId) -> Option<crate::tools::ToolBindingSnapshot>;

    /// Stores this instance under its identity, replacing what was held.
    fn put(&mut self, snapshot: crate::tools::ToolBindingSnapshot);

    /// Removes the instance with this identity.
    fn delete(&mut self, identity: &crate::tools::BindingId);

    /// Every stored instance, in the order the store keeps them: the order a generated query
    /// answers an unordered view in.
    fn list(&self) -> Vec<crate::tools::ToolBindingSnapshot>;
}

/// Where `agency.work.WorkInput` is stored — a port the implementor provides.
///
/// Keyed by the identity `input_id`. Generated network entries supply an ephemeral implementation; durable storage remains a port.
pub trait WorkInputStorage {
    /// The instance with this identity, or `None` where none is stored.
    fn get(&self, identity: &crate::work::InputId) -> Option<crate::work::WorkInputSnapshot>;

    /// Stores this instance under its identity, replacing what was held.
    fn put(&mut self, snapshot: crate::work::WorkInputSnapshot);

    /// Removes the instance with this identity.
    fn delete(&mut self, identity: &crate::work::InputId);

    /// Every stored instance, in the order the store keeps them: the order a generated query
    /// answers an unordered view in.
    fn list(&self) -> Vec<crate::work::WorkInputSnapshot>;
}

/// Where `agency.work.WorkItem` is stored — a port the implementor provides.
///
/// Keyed by the identity `item_ref`. Generated network entries supply an ephemeral implementation; durable storage remains a port.
pub trait WorkItemStorage {
    /// The instance with this identity, or `None` where none is stored.
    fn get(&self, identity: &crate::work::WorkItemRef) -> Option<crate::work::WorkItemSnapshot>;

    /// Stores this instance under its identity, replacing what was held.
    fn put(&mut self, snapshot: crate::work::WorkItemSnapshot);

    /// Removes the instance with this identity.
    fn delete(&mut self, identity: &crate::work::WorkItemRef);
}

/// The exact executing command input supplied to an external decision.
///
/// This supplies facts, not authority: the context must verify its request-bound proof.
#[derive(Debug, Clone, Copy)]
pub enum ExternalCommand<'a> {
    /// The executing `agency.teams.Report` input.
    AgencyTeamsReport(&'a crate::teams::Report),
    /// The executing `agency.teams.Send` input.
    AgencyTeamsSend(&'a crate::teams::Send),
}

impl ExternalCommand<'_> {
    /// The canonical qualified identity of this command.
    pub fn name(&self) -> &'static str {
        match self {
            Self::AgencyTeamsReport(_) => "agency.teams.Report",
            Self::AgencyTeamsSend(_) => "agency.teams.Send",
        }
    }
}

/// What the specification leaves to the implementor's context — a port the implementor provides.
///
/// The caller's attributes, the values the model says the implementation assigns, and the answer
/// to each `external:` branch.
pub trait Context {
    /// A new `agency.agents.AgentId`, which the model says the implementation assigns — a created identity, a
    /// `{generated: true}` value, or an event field the model leaves undetermined.
    fn generate_agency_agents_agent_id(&mut self) -> crate::agents::AgentId;

    /// A new `agency.agents.DefinitionId`, which the model says the implementation assigns — a created identity, a
    /// `{generated: true}` value, or an event field the model leaves undetermined.
    fn generate_agency_agents_definition_id(&mut self) -> crate::agents::DefinitionId;

    /// A new `agency.auth.CredentialId`, which the model says the implementation assigns — a created identity, a
    /// `{generated: true}` value, or an event field the model leaves undetermined.
    fn generate_agency_auth_credential_id(&mut self) -> crate::auth::CredentialId;

    /// A new `agency.auth.GrantId`, which the model says the implementation assigns — a created identity, a
    /// `{generated: true}` value, or an event field the model leaves undetermined.
    fn generate_agency_auth_grant_id(&mut self) -> crate::auth::GrantId;

    /// A new `agency.auth.PolicyId`, which the model says the implementation assigns — a created identity, a
    /// `{generated: true}` value, or an event field the model leaves undetermined.
    fn generate_agency_auth_policy_id(&mut self) -> crate::auth::PolicyId;

    /// A new `agency.auth.PrincipalId`, which the model says the implementation assigns — a created identity, a
    /// `{generated: true}` value, or an event field the model leaves undetermined.
    fn generate_agency_auth_principal_id(&mut self) -> crate::auth::PrincipalId;

    /// A new `agency.auth.TenantId`, which the model says the implementation assigns — a created identity, a
    /// `{generated: true}` value, or an event field the model leaves undetermined.
    fn generate_agency_auth_tenant_id(&mut self) -> crate::auth::TenantId;

    /// A new `agency.runs.ApprovalId`, which the model says the implementation assigns — a created identity, a
    /// `{generated: true}` value, or an event field the model leaves undetermined.
    fn generate_agency_runs_approval_id(&mut self) -> crate::runs::ApprovalId;

    /// A new `agency.runs.RunId`, which the model says the implementation assigns — a created identity, a
    /// `{generated: true}` value, or an event field the model leaves undetermined.
    fn generate_agency_runs_run_id(&mut self) -> crate::runs::RunId;

    /// A new `agency.service.SessionId`, which the model says the implementation assigns — a created identity, a
    /// `{generated: true}` value, or an event field the model leaves undetermined.
    fn generate_agency_service_session_id(&mut self) -> crate::service::SessionId;

    /// A new `agency.shared.Digest`, which the model says the implementation assigns — a created identity, a
    /// `{generated: true}` value, or an event field the model leaves undetermined.
    fn generate_agency_shared_digest(&mut self) -> crate::shared::Digest;

    /// A new `agency.teams.SwarmId`, which the model says the implementation assigns — a created identity, a
    /// `{generated: true}` value, or an event field the model leaves undetermined.
    fn generate_agency_teams_swarm_id(&mut self) -> crate::teams::SwarmId;

    /// A new `agency.teams.TeamId`, which the model says the implementation assigns — a created identity, a
    /// `{generated: true}` value, or an event field the model leaves undetermined.
    fn generate_agency_teams_team_id(&mut self) -> crate::teams::TeamId;

    /// A new `agency.teams.WorkforceId`, which the model says the implementation assigns — a created identity, a
    /// `{generated: true}` value, or an event field the model leaves undetermined.
    fn generate_agency_teams_workforce_id(&mut self) -> crate::teams::WorkforceId;

    /// A new `agency.tools.BindingId`, which the model says the implementation assigns — a created identity, a
    /// `{generated: true}` value, or an event field the model leaves undetermined.
    fn generate_agency_tools_binding_id(&mut self) -> crate::tools::BindingId;

    /// A new `agency.tools.ToolId`, which the model says the implementation assigns — a created identity, a
    /// `{generated: true}` value, or an event field the model leaves undetermined.
    fn generate_agency_tools_tool_id(&mut self) -> crate::tools::ToolId;

    /// A new `agency.work.InputId`, which the model says the implementation assigns — a created identity, a
    /// `{generated: true}` value, or an event field the model leaves undetermined.
    fn generate_agency_work_input_id(&mut self) -> crate::work::InputId;

    /// A new `Integer`, which the model says the implementation assigns — a created identity, a
    /// `{generated: true}` value, or an event field the model leaves undetermined.
    fn generate_integer(&mut self) -> i64;

    /// A `Optional<Timestamp>` the model says the implementation assigns — a `{generated: true}` value —
    /// or `None` where there is nothing to report.
    fn generate_optional_timestamp(&mut self) -> Option<crate::primitives::Timestamp>;

    /// A new `Timestamp`, which the model says the implementation assigns — a created identity, a
    /// `{generated: true}` value, or an event field the model leaves undetermined.
    fn generate_timestamp(&mut self) -> crate::primitives::Timestamp;

    /// Whether the external branch `outcome` of `command` is taken on this invocation.
    ///
    /// Asked in declaration order, before the branch's input guard is read; the first branch
    /// answered `true` whose guard holds is taken. A test forces a branch by answering `true`
    /// for it alone; a deployment asks whatever decides it.
    fn external(&mut self, command: ExternalCommand<'_>, outcome: &'static str) -> bool;
}

/// Context answers that may be unavailable, without fabricated values.
/// Existing `Context` implementations receive the blanket adapter.
pub trait TryContext {
/// Assigns the value, or names the unavailable answer.
fn try_generate_agency_agents_agent_id(&mut self) -> Result<crate::agents::AgentId, UnmetObligation>;
/// Assigns the value, or names the unavailable answer.
fn try_generate_agency_agents_definition_id(&mut self) -> Result<crate::agents::DefinitionId, UnmetObligation>;
/// Assigns the value, or names the unavailable answer.
fn try_generate_agency_auth_credential_id(&mut self) -> Result<crate::auth::CredentialId, UnmetObligation>;
/// Assigns the value, or names the unavailable answer.
fn try_generate_agency_auth_grant_id(&mut self) -> Result<crate::auth::GrantId, UnmetObligation>;
/// Assigns the value, or names the unavailable answer.
fn try_generate_agency_auth_policy_id(&mut self) -> Result<crate::auth::PolicyId, UnmetObligation>;
/// Assigns the value, or names the unavailable answer.
fn try_generate_agency_auth_principal_id(&mut self) -> Result<crate::auth::PrincipalId, UnmetObligation>;
/// Assigns the value, or names the unavailable answer.
fn try_generate_agency_auth_tenant_id(&mut self) -> Result<crate::auth::TenantId, UnmetObligation>;
/// Assigns the value, or names the unavailable answer.
fn try_generate_agency_runs_approval_id(&mut self) -> Result<crate::runs::ApprovalId, UnmetObligation>;
/// Assigns the value, or names the unavailable answer.
fn try_generate_agency_runs_run_id(&mut self) -> Result<crate::runs::RunId, UnmetObligation>;
/// Assigns the value, or names the unavailable answer.
fn try_generate_agency_service_session_id(&mut self) -> Result<crate::service::SessionId, UnmetObligation>;
/// Assigns the value, or names the unavailable answer.
fn try_generate_agency_shared_digest(&mut self) -> Result<crate::shared::Digest, UnmetObligation>;
/// Assigns the value, or names the unavailable answer.
fn try_generate_agency_teams_swarm_id(&mut self) -> Result<crate::teams::SwarmId, UnmetObligation>;
/// Assigns the value, or names the unavailable answer.
fn try_generate_agency_teams_team_id(&mut self) -> Result<crate::teams::TeamId, UnmetObligation>;
/// Assigns the value, or names the unavailable answer.
fn try_generate_agency_teams_workforce_id(&mut self) -> Result<crate::teams::WorkforceId, UnmetObligation>;
/// Assigns the value, or names the unavailable answer.
fn try_generate_agency_tools_binding_id(&mut self) -> Result<crate::tools::BindingId, UnmetObligation>;
/// Assigns the value, or names the unavailable answer.
fn try_generate_agency_tools_tool_id(&mut self) -> Result<crate::tools::ToolId, UnmetObligation>;
/// Assigns the value, or names the unavailable answer.
fn try_generate_agency_work_input_id(&mut self) -> Result<crate::work::InputId, UnmetObligation>;
/// Assigns the value, or names the unavailable answer.
fn try_generate_integer(&mut self) -> Result<i64, UnmetObligation>;
/// Assigns the value, or names the unavailable answer.
fn try_generate_optional_timestamp(&mut self) -> Result<Option<crate::primitives::Timestamp>, UnmetObligation>;
/// Assigns the value, or names the unavailable answer.
fn try_generate_timestamp(&mut self) -> Result<crate::primitives::Timestamp, UnmetObligation>;
/// Decides the named external branch, or names the unavailable answer.
fn try_external(&mut self, command: ExternalCommand<'_>, outcome: &'static str) -> Result<bool, UnmetObligation>;
}

impl<T: Context + ?Sized> TryContext for T {
fn try_generate_agency_agents_agent_id(&mut self) -> Result<crate::agents::AgentId, UnmetObligation> { Ok(Context::generate_agency_agents_agent_id(self)) }
fn try_generate_agency_agents_definition_id(&mut self) -> Result<crate::agents::DefinitionId, UnmetObligation> { Ok(Context::generate_agency_agents_definition_id(self)) }
fn try_generate_agency_auth_credential_id(&mut self) -> Result<crate::auth::CredentialId, UnmetObligation> { Ok(Context::generate_agency_auth_credential_id(self)) }
fn try_generate_agency_auth_grant_id(&mut self) -> Result<crate::auth::GrantId, UnmetObligation> { Ok(Context::generate_agency_auth_grant_id(self)) }
fn try_generate_agency_auth_policy_id(&mut self) -> Result<crate::auth::PolicyId, UnmetObligation> { Ok(Context::generate_agency_auth_policy_id(self)) }
fn try_generate_agency_auth_principal_id(&mut self) -> Result<crate::auth::PrincipalId, UnmetObligation> { Ok(Context::generate_agency_auth_principal_id(self)) }
fn try_generate_agency_auth_tenant_id(&mut self) -> Result<crate::auth::TenantId, UnmetObligation> { Ok(Context::generate_agency_auth_tenant_id(self)) }
fn try_generate_agency_runs_approval_id(&mut self) -> Result<crate::runs::ApprovalId, UnmetObligation> { Ok(Context::generate_agency_runs_approval_id(self)) }
fn try_generate_agency_runs_run_id(&mut self) -> Result<crate::runs::RunId, UnmetObligation> { Ok(Context::generate_agency_runs_run_id(self)) }
fn try_generate_agency_service_session_id(&mut self) -> Result<crate::service::SessionId, UnmetObligation> { Ok(Context::generate_agency_service_session_id(self)) }
fn try_generate_agency_shared_digest(&mut self) -> Result<crate::shared::Digest, UnmetObligation> { Ok(Context::generate_agency_shared_digest(self)) }
fn try_generate_agency_teams_swarm_id(&mut self) -> Result<crate::teams::SwarmId, UnmetObligation> { Ok(Context::generate_agency_teams_swarm_id(self)) }
fn try_generate_agency_teams_team_id(&mut self) -> Result<crate::teams::TeamId, UnmetObligation> { Ok(Context::generate_agency_teams_team_id(self)) }
fn try_generate_agency_teams_workforce_id(&mut self) -> Result<crate::teams::WorkforceId, UnmetObligation> { Ok(Context::generate_agency_teams_workforce_id(self)) }
fn try_generate_agency_tools_binding_id(&mut self) -> Result<crate::tools::BindingId, UnmetObligation> { Ok(Context::generate_agency_tools_binding_id(self)) }
fn try_generate_agency_tools_tool_id(&mut self) -> Result<crate::tools::ToolId, UnmetObligation> { Ok(Context::generate_agency_tools_tool_id(self)) }
fn try_generate_agency_work_input_id(&mut self) -> Result<crate::work::InputId, UnmetObligation> { Ok(Context::generate_agency_work_input_id(self)) }
fn try_generate_integer(&mut self) -> Result<i64, UnmetObligation> { Ok(Context::generate_integer(self)) }
fn try_generate_optional_timestamp(&mut self) -> Result<Option<crate::primitives::Timestamp>, UnmetObligation> { Ok(Context::generate_optional_timestamp(self)) }
fn try_generate_timestamp(&mut self) -> Result<crate::primitives::Timestamp, UnmetObligation> { Ok(Context::generate_timestamp(self)) }
fn try_external(&mut self, command: ExternalCommand<'_>, outcome: &'static str) -> Result<bool, UnmetObligation> { Ok(Context::external(self, command, outcome)) }
}

/// An unavailable runtime context answer, rather than a new planned capability.
pub fn unmet_context(source: &'static str) -> UnmetObligation { UnmetObligation { capability: "context answer", source } }

/// Every generated behaviour of this workspace, over the ports `P` supplies.
///
/// `P` implements the storage trait of each entity a generated behaviour reads or writes,
/// `TryContext` (or its legacy `Context` blanket adapter) where one asks it anything, and every `…Behavior` and `…Query` trait the plan still
/// owes; `Generated<P>` forwards those to it.
pub struct Generated<P> {
    /// The storage and context ports, and every behaviour or query still owed.
    pub ports: P,
}

impl<P> Generated<P> {
    /// The generated behaviours, over `ports`.
    pub fn new(ports: P) -> Self {
        Self { ports }
    }
}

/// `agency.agents.CreateAgentDefinition`, generated: every outcome is one the specification fully determines.
impl<P> crate::agents::obligations::CreateAgentDefinitionBehavior for Generated<P>
where
    P: TryContext + AgentDefinitionStorage,
{
    fn create_agent_definition(&mut self, input: crate::agents::CreateAgentDefinition) -> Result<crate::agents::CreateAgentDefinitionOutcome, UnmetObligation> {
        let _ = &input;
        // `created`: the default.
        let identity: crate::agents::DefinitionId = self.ports.try_generate_agency_agents_definition_id()?;
        let data = crate::agents::AgentDefinitionData {
            definition_id: identity.clone(),
            tenant_id: input.tenant_id.clone(),
            name: input.name.clone(),
            description: input.description.clone(),
            agent_spec: input.agent_spec.clone(),
            policy_id: input.policy_id.clone(),
            version: self.ports.try_generate_integer()?,
            digest: self.ports.try_generate_agency_shared_digest()?,
            created_by: input.created_by.clone(),
            check_exit_code: None,
            check_message: None,
        };
        let answer = crate::agents::CreateAgentDefinitionOutcome::Created { agent_definition_created: crate::agents::AgentDefinitionCreated { definition_id: identity.clone(), name: input.name.clone() } };
        AgentDefinitionStorage::put(&mut self.ports, crate::agents::AnyAgentDefinition::Draft(crate::agents::AgentDefinition::new(data)).snapshot());
        return Ok(answer);
    }
}

/// `agency.agents.DeprecateAgentDefinition`, generated: every outcome is one the specification fully determines.
impl<P> crate::agents::obligations::DeprecateAgentDefinitionBehavior for Generated<P>
where
    P: AgentDefinitionStorage,
{
    fn deprecate_agent_definition(&mut self, input: crate::agents::DeprecateAgentDefinition) -> Result<crate::agents::DeprecateAgentDefinitionOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if AgentDefinitionStorage::get(&self.ports, &input.definition_id).is_none() {
            return Ok(crate::agents::DeprecateAgentDefinitionOutcome::NoSuchDefinition { error: crate::agents::NotFound });
        }
        // `deprecated`: the default.
        let Some(held) = AgentDefinitionStorage::get(&self.ports, &input.definition_id) else {
            return Ok(crate::agents::DeprecateAgentDefinitionOutcome::NoSuchDefinition { error: crate::agents::NotFound });
        };
        let _ = &held;
        let moved = match held.refine() {
            crate::agents::AnyAgentDefinition::Published(instance) => crate::agents::AnyAgentDefinition::Deprecated(instance.deprecate()),
            _ => return Ok(crate::agents::DeprecateAgentDefinitionOutcome::WrongState { error: crate::agents::StateConflict }),
        };
        let next = moved.snapshot();
        let answer = crate::agents::DeprecateAgentDefinitionOutcome::Deprecated { agent_definition_deprecated: crate::agents::AgentDefinitionDeprecated { definition_id: input.definition_id.clone() } };
        AgentDefinitionStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.agents.ProvisionAgent`, generated: every outcome is one the specification fully determines.
impl<P> crate::agents::obligations::ProvisionAgentBehavior for Generated<P>
where
    P: TryContext + AgentStorage,
{
    fn provision_agent(&mut self, input: crate::agents::ProvisionAgent) -> Result<crate::agents::ProvisionAgentOutcome, UnmetObligation> {
        let _ = &input;
        // `provisioned`: the default.
        let identity: crate::agents::AgentId = self.ports.try_generate_agency_agents_agent_id()?;
        let data = crate::agents::AgentData {
            agent_id: identity.clone(),
            tenant_id: input.tenant_id.clone(),
            definition_id: input.definition_id.clone(),
            definition_version: input.definition_version.clone(),
            display_name: input.display_name.clone(),
            principal_id: input.principal_id.clone(),
            current_run_id: None,
            last_seen_at: self.ports.try_generate_timestamp()?,
        };
        let answer = crate::agents::ProvisionAgentOutcome::Provisioned { agent_provisioned: crate::agents::AgentProvisioned { agent_id: identity.clone(), definition_id: input.definition_id.clone(), definition_version: input.definition_version.clone() } };
        AgentStorage::put(&mut self.ports, crate::agents::AnyAgent::Provisioned(crate::agents::Agent::new(data)).snapshot());
        return Ok(answer);
    }
}

/// `agency.agents.PublishAgentDefinition`, generated: every outcome is one the specification fully determines.
impl<P> crate::agents::obligations::PublishAgentDefinitionBehavior for Generated<P>
where
    P: AgentDefinitionStorage,
{
    fn publish_agent_definition(&mut self, input: crate::agents::PublishAgentDefinition) -> Result<crate::agents::PublishAgentDefinitionOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if AgentDefinitionStorage::get(&self.ports, &input.definition_id).is_none() {
            return Ok(crate::agents::PublishAgentDefinitionOutcome::NoSuchDefinition { error: crate::agents::NotFound });
        }
        // `published`: the default.
        let Some(held) = AgentDefinitionStorage::get(&self.ports, &input.definition_id) else {
            return Ok(crate::agents::PublishAgentDefinitionOutcome::NoSuchDefinition { error: crate::agents::NotFound });
        };
        let _ = &held;
        let moved = match held.refine() {
            crate::agents::AnyAgentDefinition::Checked(instance) => crate::agents::AnyAgentDefinition::Published(instance.publish()),
            _ => return Ok(crate::agents::PublishAgentDefinitionOutcome::WrongState { error: crate::agents::StateConflict }),
        };
        let next = moved.snapshot();
        let answer = crate::agents::PublishAgentDefinitionOutcome::Published { agent_definition_published: crate::agents::AgentDefinitionPublished { definition_id: input.definition_id.clone() } };
        AgentDefinitionStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.agents.RecordSpecCheck`, generated: every outcome is one the specification fully determines.
impl<P> crate::agents::obligations::RecordSpecCheckBehavior for Generated<P>
where
    P: AgentDefinitionStorage,
{
    fn record_spec_check(&mut self, input: crate::agents::RecordSpecCheck) -> Result<crate::agents::RecordSpecCheckOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if AgentDefinitionStorage::get(&self.ports, &input.definition_id).is_none() {
            return Ok(crate::agents::RecordSpecCheckOutcome::NoSuchDefinition { error: crate::agents::NotFound });
        }
        // `refused`: an accepting branch, in declaration order.
        if decided(equal(Some(&input.verdict).map(|value| match value { crate::agents::CheckVerdict::Accepted => "Accepted", crate::agents::CheckVerdict::Refused => "Refused" }.to_owned()), Some("Refused".to_owned())), "agency.agents.RecordSpecCheck")? {
            let Some(held) = AgentDefinitionStorage::get(&self.ports, &input.definition_id) else {
                return Ok(crate::agents::RecordSpecCheckOutcome::NoSuchDefinition { error: crate::agents::NotFound });
            };
            let _ = &held;
            let moved = match held.refine() {
                crate::agents::AnyAgentDefinition::Draft(instance) => crate::agents::AnyAgentDefinition::Draft(instance.refuse()),
                _ => return Ok(crate::agents::RecordSpecCheckOutcome::WrongState { error: crate::agents::StateConflict }),
            };
            let mut next = moved.snapshot();
            next.data.check_exit_code = Some(input.exit_code.clone());
            next.data.check_message = Some(input.message.clone());
            let answer = crate::agents::RecordSpecCheckOutcome::Refused { agent_spec_refused: crate::agents::AgentSpecRefused { definition_id: input.definition_id.clone(), exit_code: input.exit_code.clone(), message: input.message.clone() } };
            AgentDefinitionStorage::put(&mut self.ports, next);
            return Ok(answer);
        }
        // `checked`: an accepting branch, in declaration order.
        if decided(equal(Some(&input.verdict).map(|value| match value { crate::agents::CheckVerdict::Accepted => "Accepted", crate::agents::CheckVerdict::Refused => "Refused" }.to_owned()), Some("Accepted".to_owned())), "agency.agents.RecordSpecCheck")? {
            let Some(held) = AgentDefinitionStorage::get(&self.ports, &input.definition_id) else {
                return Ok(crate::agents::RecordSpecCheckOutcome::NoSuchDefinition { error: crate::agents::NotFound });
            };
            let _ = &held;
            let moved = match held.refine() {
                crate::agents::AnyAgentDefinition::Draft(instance) => crate::agents::AnyAgentDefinition::Checked(instance.check()),
                _ => return Ok(crate::agents::RecordSpecCheckOutcome::WrongState { error: crate::agents::StateConflict }),
            };
            let mut next = moved.snapshot();
            next.data.check_exit_code = Some(input.exit_code.clone());
            next.data.check_message = Some(input.message.clone());
            let answer = crate::agents::RecordSpecCheckOutcome::Checked { agent_spec_checked: crate::agents::AgentSpecChecked { definition_id: input.definition_id.clone(), message: input.message.clone() } };
            AgentDefinitionStorage::put(&mut self.ports, next);
            return Ok(answer);
        }
        // No declared branch answers this request.
        Err(undeclared("agency.agents.RecordSpecCheck"))
    }
}

/// `agency.agents.RetireAgent`, generated: every outcome is one the specification fully determines.
impl<P> crate::agents::obligations::RetireAgentBehavior for Generated<P>
where
    P: AgentStorage,
{
    fn retire_agent(&mut self, input: crate::agents::RetireAgent) -> Result<crate::agents::RetireAgentOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if AgentStorage::get(&self.ports, &input.agent_id).is_none() {
            return Ok(crate::agents::RetireAgentOutcome::NoSuchAgent { error: crate::agents::NotFound });
        }
        // `retired`: the default.
        let Some(held) = AgentStorage::get(&self.ports, &input.agent_id) else {
            return Ok(crate::agents::RetireAgentOutcome::NoSuchAgent { error: crate::agents::NotFound });
        };
        let _ = &held;
        let moved = match held.refine() {
            crate::agents::AnyAgent::Provisioned(instance) => crate::agents::AnyAgent::Retired(instance.retire()),
            _ => return Ok(crate::agents::RetireAgentOutcome::WrongState { error: crate::agents::StateConflict }),
        };
        let next = moved.snapshot();
        let answer = crate::agents::RetireAgentOutcome::Retired { agent_retired: crate::agents::AgentRetired { agent_id: input.agent_id.clone() } };
        AgentStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.auth.ActivatePolicy`, generated: every outcome is one the specification fully determines.
impl<P> crate::auth::obligations::ActivatePolicyBehavior for Generated<P>
where
    P: PolicyStorage,
{
    fn activate_policy(&mut self, input: crate::auth::ActivatePolicy) -> Result<crate::auth::ActivatePolicyOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if PolicyStorage::get(&self.ports, &input.policy_id).is_none() {
            return Ok(crate::auth::ActivatePolicyOutcome::NoSuchPolicy { error: crate::auth::NotFound });
        }
        // `activated`: the default.
        let Some(held) = PolicyStorage::get(&self.ports, &input.policy_id) else {
            return Ok(crate::auth::ActivatePolicyOutcome::NoSuchPolicy { error: crate::auth::NotFound });
        };
        let _ = &held;
        let moved = match held.refine() {
            crate::auth::AnyPolicy::Draft(instance) => crate::auth::AnyPolicy::Active(instance.activate()),
            _ => return Ok(crate::auth::ActivatePolicyOutcome::WrongState { error: crate::auth::StateConflict }),
        };
        let next = moved.snapshot();
        let answer = crate::auth::ActivatePolicyOutcome::Activated { policy_activated: crate::auth::PolicyActivated { policy_id: input.policy_id.clone() } };
        PolicyStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.auth.AssignRole`, generated: every outcome is one the specification fully determines.
impl<P> crate::auth::obligations::AssignRoleBehavior for Generated<P>
where
    P: PrincipalStorage,
{
    fn assign_role(&mut self, input: crate::auth::AssignRole) -> Result<crate::auth::AssignRoleOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if PrincipalStorage::get(&self.ports, &input.principal_id).is_none() {
            return Ok(crate::auth::AssignRoleOutcome::NoSuchPrincipal { error: crate::auth::NotFound });
        }
        // `assigned`: the default.
        let Some(held) = PrincipalStorage::get(&self.ports, &input.principal_id) else {
            return Ok(crate::auth::AssignRoleOutcome::NoSuchPrincipal { error: crate::auth::NotFound });
        };
        let _ = &held;
        let before = held.data.clone();
        let mut next = held;
        next.data.roles = { let mut items = before.roles.clone(); items.push(input.role.clone()); items };
        let answer = crate::auth::AssignRoleOutcome::Assigned { role_assigned: crate::auth::RoleAssigned { principal_id: input.principal_id.clone(), role: input.role.clone() } };
        PrincipalStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.auth.CreatePolicy`, generated: every outcome is one the specification fully determines.
impl<P> crate::auth::obligations::CreatePolicyBehavior for Generated<P>
where
    P: TryContext + PolicyStorage,
{
    fn create_policy(&mut self, input: crate::auth::CreatePolicy) -> Result<crate::auth::CreatePolicyOutcome, UnmetObligation> {
        let _ = &input;
        // `created`: the default.
        let identity: crate::auth::PolicyId = self.ports.try_generate_agency_auth_policy_id()?;
        let data = crate::auth::PolicyData {
            policy_id: identity.clone(),
            tenant_id: input.tenant_id.clone(),
            name: input.name.clone(),
            rules: input.rules.clone(),
            version: self.ports.try_generate_integer()?,
            digest: self.ports.try_generate_agency_shared_digest()?,
        };
        let answer = crate::auth::CreatePolicyOutcome::Created { policy_created: crate::auth::PolicyCreated { policy_id: identity.clone(), name: input.name.clone() } };
        PolicyStorage::put(&mut self.ports, crate::auth::AnyPolicy::Draft(crate::auth::Policy::new(data)).snapshot());
        return Ok(answer);
    }
}

/// `agency.auth.CreateTenant`, generated: every outcome is one the specification fully determines.
impl<P> crate::auth::obligations::CreateTenantBehavior for Generated<P>
where
    P: TryContext + TenantStorage,
{
    fn create_tenant(&mut self, input: crate::auth::CreateTenant) -> Result<crate::auth::CreateTenantOutcome, UnmetObligation> {
        let _ = &input;
        // `created`: the default.
        let identity: crate::auth::TenantId = self.ports.try_generate_agency_auth_tenant_id()?;
        let data = crate::auth::TenantData {
            tenant_id: identity.clone(),
            name: input.name.clone(),
            mandate_tenant_ref: input.mandate_tenant_ref.clone(),
        };
        let answer = crate::auth::CreateTenantOutcome::Created { tenant_created: crate::auth::TenantCreated { tenant_id: identity.clone(), name: input.name.clone() } };
        TenantStorage::put(&mut self.ports, crate::auth::AnyTenant::Created(crate::auth::Tenant::new(data)).snapshot());
        return Ok(answer);
    }
}

/// `agency.auth.DisablePrincipal`, generated: every outcome is one the specification fully determines.
impl<P> crate::auth::obligations::DisablePrincipalBehavior for Generated<P>
where
    P: PrincipalStorage,
{
    fn disable_principal(&mut self, input: crate::auth::DisablePrincipal) -> Result<crate::auth::DisablePrincipalOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if PrincipalStorage::get(&self.ports, &input.principal_id).is_none() {
            return Ok(crate::auth::DisablePrincipalOutcome::NoSuchPrincipal { error: crate::auth::NotFound });
        }
        // `disabled`: the default.
        let Some(held) = PrincipalStorage::get(&self.ports, &input.principal_id) else {
            return Ok(crate::auth::DisablePrincipalOutcome::NoSuchPrincipal { error: crate::auth::NotFound });
        };
        let _ = &held;
        let moved = match held.refine() {
            crate::auth::AnyPrincipal::Invited(instance) => crate::auth::AnyPrincipal::Disabled(instance.disable()),
            _ => return Ok(crate::auth::DisablePrincipalOutcome::WrongState { error: crate::auth::StateConflict }),
        };
        let next = moved.snapshot();
        let answer = crate::auth::DisablePrincipalOutcome::Disabled { principal_disabled: crate::auth::PrincipalDisabled { principal_id: input.principal_id.clone() } };
        PrincipalStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.auth.GrantCredential`, generated: every outcome is one the specification fully determines.
impl<P> crate::auth::obligations::GrantCredentialBehavior for Generated<P>
where
    P: TryContext + CredentialGrantStorage,
{
    fn grant_credential(&mut self, input: crate::auth::GrantCredential) -> Result<crate::auth::GrantCredentialOutcome, UnmetObligation> {
        let _ = &input;
        // `granted-to-agent`: an accepting branch, in declaration order.
        if decided(all(&[Some(Some(&input.grantee_agent_id).and_then(|value| value.as_ref()).map(|value| &value.0).is_some()), (Some(Some(&input.grantee_team_id).and_then(|value| value.as_ref()).map(|value| &value.0).is_some())).map(|value| !value), (Some(Some(&input.grantee_workforce_id).and_then(|value| value.as_ref()).map(|value| &value.0).is_some())).map(|value| !value)]), "agency.auth.GrantCredential")? {
            let identity: crate::auth::GrantId = self.ports.try_generate_agency_auth_grant_id()?;
            let data = crate::auth::CredentialGrantData {
                grant_id: identity.clone(),
                tenant_id: input.tenant_id.clone(),
                credential_id: input.credential_id.clone(),
                grantee_agent_id: input.grantee_agent_id.clone(),
                grantee_team_id: None,
                grantee_workforce_id: None,
                purpose: input.purpose.clone(),
                expires_at: input.expires_at.clone(),
            };
            if let Some(broken) = data.broken_invariant() {
                let capability = "entity invariant";
                return Err(UnmetObligation { capability, source: broken });
            }
            let answer = crate::auth::GrantCredentialOutcome::GrantedToAgent { credential_granted: crate::auth::CredentialGranted { grant_id: identity.clone(), credential_id: input.credential_id.clone(), purpose: input.purpose.clone() } };
            CredentialGrantStorage::put(&mut self.ports, crate::auth::AnyCredentialGrant::Granted(crate::auth::CredentialGrant::new(data)).snapshot());
            return Ok(answer);
        }
        // `granted-to-team`: an accepting branch, in declaration order.
        if decided(all(&[(Some(Some(&input.grantee_agent_id).and_then(|value| value.as_ref()).map(|value| &value.0).is_some())).map(|value| !value), Some(Some(&input.grantee_team_id).and_then(|value| value.as_ref()).map(|value| &value.0).is_some()), (Some(Some(&input.grantee_workforce_id).and_then(|value| value.as_ref()).map(|value| &value.0).is_some())).map(|value| !value)]), "agency.auth.GrantCredential")? {
            let identity: crate::auth::GrantId = self.ports.try_generate_agency_auth_grant_id()?;
            let data = crate::auth::CredentialGrantData {
                grant_id: identity.clone(),
                tenant_id: input.tenant_id.clone(),
                credential_id: input.credential_id.clone(),
                grantee_agent_id: None,
                grantee_team_id: input.grantee_team_id.clone(),
                grantee_workforce_id: None,
                purpose: input.purpose.clone(),
                expires_at: input.expires_at.clone(),
            };
            if let Some(broken) = data.broken_invariant() {
                let capability = "entity invariant";
                return Err(UnmetObligation { capability, source: broken });
            }
            let answer = crate::auth::GrantCredentialOutcome::GrantedToTeam { credential_granted: crate::auth::CredentialGranted { grant_id: identity.clone(), credential_id: input.credential_id.clone(), purpose: input.purpose.clone() } };
            CredentialGrantStorage::put(&mut self.ports, crate::auth::AnyCredentialGrant::Granted(crate::auth::CredentialGrant::new(data)).snapshot());
            return Ok(answer);
        }
        // `granted-to-workforce`: an accepting branch, in declaration order.
        if decided(all(&[(Some(Some(&input.grantee_agent_id).and_then(|value| value.as_ref()).map(|value| &value.0).is_some())).map(|value| !value), (Some(Some(&input.grantee_team_id).and_then(|value| value.as_ref()).map(|value| &value.0).is_some())).map(|value| !value), Some(Some(&input.grantee_workforce_id).and_then(|value| value.as_ref()).map(|value| &value.0).is_some())]), "agency.auth.GrantCredential")? {
            let identity: crate::auth::GrantId = self.ports.try_generate_agency_auth_grant_id()?;
            let data = crate::auth::CredentialGrantData {
                grant_id: identity.clone(),
                tenant_id: input.tenant_id.clone(),
                credential_id: input.credential_id.clone(),
                grantee_agent_id: None,
                grantee_team_id: None,
                grantee_workforce_id: input.grantee_workforce_id.clone(),
                purpose: input.purpose.clone(),
                expires_at: input.expires_at.clone(),
            };
            if let Some(broken) = data.broken_invariant() {
                let capability = "entity invariant";
                return Err(UnmetObligation { capability, source: broken });
            }
            let answer = crate::auth::GrantCredentialOutcome::GrantedToWorkforce { credential_granted: crate::auth::CredentialGranted { grant_id: identity.clone(), credential_id: input.credential_id.clone(), purpose: input.purpose.clone() } };
            CredentialGrantStorage::put(&mut self.ports, crate::auth::AnyCredentialGrant::Granted(crate::auth::CredentialGrant::new(data)).snapshot());
            return Ok(answer);
        }
        // `grantee-invalid`: the default.
        return Ok(crate::auth::GrantCredentialOutcome::GranteeInvalid { error: crate::auth::GranteeInvalid });
    }
}

/// `agency.auth.InvitePrincipal`, generated: every outcome is one the specification fully determines.
impl<P> crate::auth::obligations::InvitePrincipalBehavior for Generated<P>
where
    P: TryContext + PrincipalStorage,
{
    fn invite_principal(&mut self, input: crate::auth::InvitePrincipal) -> Result<crate::auth::InvitePrincipalOutcome, UnmetObligation> {
        let _ = &input;
        // `invited`: the default.
        let identity: crate::auth::PrincipalId = self.ports.try_generate_agency_auth_principal_id()?;
        let data = crate::auth::PrincipalData {
            principal_id: identity.clone(),
            tenant_id: input.tenant_id.clone(),
            kind: input.kind.clone(),
            subject: input.subject.clone(),
            roles: input.roles.clone(),
            mandate_ref: input.mandate_ref.clone(),
        };
        let answer = crate::auth::InvitePrincipalOutcome::Invited { principal_invited: crate::auth::PrincipalInvited { principal_id: identity.clone(), tenant_id: input.tenant_id.clone(), kind: input.kind.clone(), subject: input.subject.clone() } };
        PrincipalStorage::put(&mut self.ports, crate::auth::AnyPrincipal::Invited(crate::auth::Principal::new(data)).snapshot());
        return Ok(answer);
    }
}

/// `agency.auth.RefreshCredential`, generated: every outcome is one the specification fully determines.
impl<P> crate::auth::obligations::RefreshCredentialBehavior for Generated<P>
where
    P: TryContext + CredentialStorage,
{
    fn refresh_credential(&mut self, input: crate::auth::RefreshCredential) -> Result<crate::auth::RefreshCredentialOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if CredentialStorage::get(&self.ports, &input.credential_id).is_none() {
            return Ok(crate::auth::RefreshCredentialOutcome::NoSuchCredential { error: crate::auth::NotFound });
        }
        // The addressed row, read before the branches that select by it.
        let Some(held) = CredentialStorage::get(&self.ports, &input.credential_id) else {
            return Ok(crate::auth::RefreshCredentialOutcome::NoSuchCredential { error: crate::auth::NotFound });
        };
        let _ = &held;
        // `revoked`: selected by the addressed row.
        if decided(equal(Some(&held.state).map(|value| match value { crate::auth::CredentialState::Revoked => "Revoked", crate::auth::CredentialState::Stored => "Stored" }.to_owned()), Some("Revoked".to_owned())), "agency.auth.RefreshCredential")? {
            return Ok(crate::auth::RefreshCredentialOutcome::Revoked { error: crate::auth::CredentialUnusable });
        }
        // `refreshed`: the default.
        let Some(held) = CredentialStorage::get(&self.ports, &input.credential_id) else {
            return Ok(crate::auth::RefreshCredentialOutcome::NoSuchCredential { error: crate::auth::NotFound });
        };
        let _ = &held;
        let mut next = held;
        next.data.last_refreshed_at = self.ports.try_generate_timestamp()?;
        let answer = crate::auth::RefreshCredentialOutcome::Refreshed { credential_refreshed: crate::auth::CredentialRefreshed { credential_id: input.credential_id.clone() } };
        CredentialStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.auth.RevokeCredential`, generated: every outcome is one the specification fully determines.
impl<P> crate::auth::obligations::RevokeCredentialBehavior for Generated<P>
where
    P: CredentialStorage,
{
    fn revoke_credential(&mut self, input: crate::auth::RevokeCredential) -> Result<crate::auth::RevokeCredentialOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if CredentialStorage::get(&self.ports, &input.credential_id).is_none() {
            return Ok(crate::auth::RevokeCredentialOutcome::NoSuchCredential { error: crate::auth::NotFound });
        }
        // `revoked`: the default.
        let Some(held) = CredentialStorage::get(&self.ports, &input.credential_id) else {
            return Ok(crate::auth::RevokeCredentialOutcome::NoSuchCredential { error: crate::auth::NotFound });
        };
        let _ = &held;
        let moved = match held.refine() {
            crate::auth::AnyCredential::Stored(instance) => crate::auth::AnyCredential::Revoked(instance.revoke()),
            _ => return Ok(crate::auth::RevokeCredentialOutcome::WrongState { error: crate::auth::StateConflict }),
        };
        let next = moved.snapshot();
        let answer = crate::auth::RevokeCredentialOutcome::Revoked { credential_revoked: crate::auth::CredentialRevoked { credential_id: input.credential_id.clone() } };
        CredentialStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.auth.RevokeGrant`, generated: every outcome is one the specification fully determines.
impl<P> crate::auth::obligations::RevokeGrantBehavior for Generated<P>
where
    P: CredentialGrantStorage,
{
    fn revoke_grant(&mut self, input: crate::auth::RevokeGrant) -> Result<crate::auth::RevokeGrantOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if CredentialGrantStorage::get(&self.ports, &input.grant_id).is_none() {
            return Ok(crate::auth::RevokeGrantOutcome::NoSuchGrant { error: crate::auth::NotFound });
        }
        // `revoked`: the default.
        let Some(held) = CredentialGrantStorage::get(&self.ports, &input.grant_id) else {
            return Ok(crate::auth::RevokeGrantOutcome::NoSuchGrant { error: crate::auth::NotFound });
        };
        let _ = &held;
        let moved = match held.refine() {
            crate::auth::AnyCredentialGrant::Granted(instance) => crate::auth::AnyCredentialGrant::Revoked(instance.revoke()),
            _ => return Ok(crate::auth::RevokeGrantOutcome::WrongState { error: crate::auth::StateConflict }),
        };
        let next = moved.snapshot();
        if let Some(broken) = next.data.broken_invariant() {
            let capability = "entity invariant";
            return Err(UnmetObligation { capability, source: broken });
        }
        let answer = crate::auth::RevokeGrantOutcome::Revoked { grant_revoked: crate::auth::GrantRevoked { grant_id: input.grant_id.clone() } };
        CredentialGrantStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.auth.StoreCredential`, generated: every outcome is one the specification fully determines.
impl<P> crate::auth::obligations::StoreCredentialBehavior for Generated<P>
where
    P: TryContext + CredentialStorage,
{
    fn store_credential(&mut self, input: crate::auth::StoreCredential) -> Result<crate::auth::StoreCredentialOutcome, UnmetObligation> {
        let _ = &input;
        // `stored`: the default.
        let identity: crate::auth::CredentialId = self.ports.try_generate_agency_auth_credential_id()?;
        let data = crate::auth::CredentialData {
            credential_id: identity.clone(),
            tenant_id: input.tenant_id.clone(),
            kind: input.kind.clone(),
            custody: input.custody.clone(),
            scope: input.scope.clone(),
            owner_principal_id: input.owner_principal_id.clone(),
            expires_at: input.expires_at.clone(),
            last_refreshed_at: self.ports.try_generate_timestamp()?,
        };
        let answer = crate::auth::StoreCredentialOutcome::Stored { credential_stored: crate::auth::CredentialStored { credential_id: identity.clone(), kind: input.kind.clone(), custody: input.custody.clone() } };
        CredentialStorage::put(&mut self.ports, crate::auth::AnyCredential::Stored(crate::auth::Credential::new(data)).snapshot());
        return Ok(answer);
    }
}

/// `agency.auth.SupersedePolicy`, generated: every outcome is one the specification fully determines.
impl<P> crate::auth::obligations::SupersedePolicyBehavior for Generated<P>
where
    P: PolicyStorage,
{
    fn supersede_policy(&mut self, input: crate::auth::SupersedePolicy) -> Result<crate::auth::SupersedePolicyOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if PolicyStorage::get(&self.ports, &input.policy_id).is_none() {
            return Ok(crate::auth::SupersedePolicyOutcome::NoSuchPolicy { error: crate::auth::NotFound });
        }
        // `superseded`: the default.
        let Some(held) = PolicyStorage::get(&self.ports, &input.policy_id) else {
            return Ok(crate::auth::SupersedePolicyOutcome::NoSuchPolicy { error: crate::auth::NotFound });
        };
        let _ = &held;
        let moved = match held.refine() {
            crate::auth::AnyPolicy::Active(instance) => crate::auth::AnyPolicy::Superseded(instance.supersede()),
            _ => return Ok(crate::auth::SupersedePolicyOutcome::WrongState { error: crate::auth::StateConflict }),
        };
        let next = moved.snapshot();
        let answer = crate::auth::SupersedePolicyOutcome::Superseded { policy_superseded: crate::auth::PolicySuperseded { policy_id: input.policy_id.clone() } };
        PolicyStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.runs.AdmitRun`, generated: every outcome is one the specification fully determines.
impl<P> crate::runs::obligations::AdmitRunBehavior for Generated<P>
where
    P: RunStorage,
{
    fn admit_run(&mut self, input: crate::runs::AdmitRun) -> Result<crate::runs::AdmitRunOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if RunStorage::get(&self.ports, &input.run_id).is_none() {
            return Ok(crate::runs::AdmitRunOutcome::NoSuchRun { error: crate::runs::NotFound });
        }
        // `admitted`: the default.
        let Some(held) = RunStorage::get(&self.ports, &input.run_id) else {
            return Ok(crate::runs::AdmitRunOutcome::NoSuchRun { error: crate::runs::NotFound });
        };
        let _ = &held;
        let moved = match held.refine() {
            crate::runs::AnyRun::Requested(instance) => crate::runs::AnyRun::Admitted(instance.admit()),
            _ => return Ok(crate::runs::AdmitRunOutcome::WrongState { error: crate::runs::StateConflict }),
        };
        let next = moved.snapshot();
        let answer = crate::runs::AdmitRunOutcome::Admitted { run_admitted: crate::runs::RunAdmitted { run_id: input.run_id.clone() } };
        RunStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.runs.DenyRunApproval`, generated: every outcome is one the specification fully determines.
impl<P> crate::runs::obligations::DenyRunApprovalBehavior for Generated<P>
where
    P: TryContext + RunApprovalStorage,
{
    fn deny_run_approval(&mut self, input: crate::runs::DenyRunApproval) -> Result<crate::runs::DenyRunApprovalOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if RunApprovalStorage::get(&self.ports, &input.approval_id).is_none() {
            return Ok(crate::runs::DenyRunApprovalOutcome::NoSuchApproval { error: crate::runs::NotFound });
        }
        // `denied`: the default.
        let Some(held) = RunApprovalStorage::get(&self.ports, &input.approval_id) else {
            return Ok(crate::runs::DenyRunApprovalOutcome::NoSuchApproval { error: crate::runs::NotFound });
        };
        let _ = &held;
        let moved = match held.refine() {
            crate::runs::AnyRunApproval::Pending(instance) => crate::runs::AnyRunApproval::Denied(instance.deny()),
            _ => return Ok(crate::runs::DenyRunApprovalOutcome::WrongState { error: crate::runs::StateConflict }),
        };
        let mut next = moved.snapshot();
        next.data.answered_by = Some(input.answered_by.clone());
        next.data.answered_at = self.ports.try_generate_optional_timestamp()?;
        let answer = crate::runs::DenyRunApprovalOutcome::Denied { run_approval_denied: crate::runs::RunApprovalDenied { approval_id: input.approval_id.clone(), answered_by: input.answered_by.clone() } };
        RunApprovalStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.runs.GrantRunApproval`, generated: every outcome is one the specification fully determines.
impl<P> crate::runs::obligations::GrantRunApprovalBehavior for Generated<P>
where
    P: TryContext + RunApprovalStorage,
{
    fn grant_run_approval(&mut self, input: crate::runs::GrantRunApproval) -> Result<crate::runs::GrantRunApprovalOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if RunApprovalStorage::get(&self.ports, &input.approval_id).is_none() {
            return Ok(crate::runs::GrantRunApprovalOutcome::NoSuchApproval { error: crate::runs::NotFound });
        }
        // `granted`: the default.
        let Some(held) = RunApprovalStorage::get(&self.ports, &input.approval_id) else {
            return Ok(crate::runs::GrantRunApprovalOutcome::NoSuchApproval { error: crate::runs::NotFound });
        };
        let _ = &held;
        let moved = match held.refine() {
            crate::runs::AnyRunApproval::Pending(instance) => crate::runs::AnyRunApproval::Granted(instance.grant()),
            _ => return Ok(crate::runs::GrantRunApprovalOutcome::WrongState { error: crate::runs::StateConflict }),
        };
        let mut next = moved.snapshot();
        next.data.answered_by = Some(input.answered_by.clone());
        next.data.answered_at = self.ports.try_generate_optional_timestamp()?;
        let answer = crate::runs::GrantRunApprovalOutcome::Granted { run_approval_granted: crate::runs::RunApprovalGranted { approval_id: input.approval_id.clone(), answered_by: input.answered_by.clone() } };
        RunApprovalStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.runs.RecordRunFailure`, generated: every outcome is one the specification fully determines.
impl<P> crate::runs::obligations::RecordRunFailureBehavior for Generated<P>
where
    P: TryContext + RunStorage,
{
    fn record_run_failure(&mut self, input: crate::runs::RecordRunFailure) -> Result<crate::runs::RecordRunFailureOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if RunStorage::get(&self.ports, &input.run_id).is_none() {
            return Ok(crate::runs::RecordRunFailureOutcome::NoSuchRun { error: crate::runs::NotFound });
        }
        // `failed`: the default.
        let Some(held) = RunStorage::get(&self.ports, &input.run_id) else {
            return Ok(crate::runs::RecordRunFailureOutcome::NoSuchRun { error: crate::runs::NotFound });
        };
        let _ = &held;
        let moved = match held.refine() {
            crate::runs::AnyRun::Running(instance) => crate::runs::AnyRun::Failed(instance.fail()),
            _ => return Ok(crate::runs::RecordRunFailureOutcome::WrongState { error: crate::runs::StateConflict }),
        };
        let mut next = moved.snapshot();
        next.data.ended_at = self.ports.try_generate_optional_timestamp()?;
        let answer = crate::runs::RecordRunFailureOutcome::Failed { run_failed: crate::runs::RunFailed { run_id: input.run_id.clone(), reason: input.reason.clone() } };
        RunStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.runs.RecordRunProposal`, generated: every outcome is one the specification fully determines.
impl<P> crate::runs::obligations::RecordRunProposalBehavior for Generated<P>
where
    P: TryContext + RunStorage,
{
    fn record_run_proposal(&mut self, input: crate::runs::RecordRunProposal) -> Result<crate::runs::RecordRunProposalOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if RunStorage::get(&self.ports, &input.run_id).is_none() {
            return Ok(crate::runs::RecordRunProposalOutcome::NoSuchRun { error: crate::runs::NotFound });
        }
        // `proposed`: the default.
        let Some(held) = RunStorage::get(&self.ports, &input.run_id) else {
            return Ok(crate::runs::RecordRunProposalOutcome::NoSuchRun { error: crate::runs::NotFound });
        };
        let _ = &held;
        let moved = match held.refine() {
            crate::runs::AnyRun::Running(instance) => crate::runs::AnyRun::Proposed(instance.propose()),
            _ => return Ok(crate::runs::RecordRunProposalOutcome::WrongState { error: crate::runs::StateConflict }),
        };
        let mut next = moved.snapshot();
        next.data.ended_at = self.ports.try_generate_optional_timestamp()?;
        next.data.usage = Some(input.usage.clone());
        let answer = crate::runs::RecordRunProposalOutcome::Proposed { run_proposed: crate::runs::RunProposed { run_id: input.run_id.clone(), usage: input.usage.clone() } };
        RunStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.runs.RecordRunStarted`, generated: every outcome is one the specification fully determines.
impl<P> crate::runs::obligations::RecordRunStartedBehavior for Generated<P>
where
    P: RunStorage,
{
    fn record_run_started(&mut self, input: crate::runs::RecordRunStarted) -> Result<crate::runs::RecordRunStartedOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if RunStorage::get(&self.ports, &input.run_id).is_none() {
            return Ok(crate::runs::RecordRunStartedOutcome::NoSuchRun { error: crate::runs::NotFound });
        }
        // `started`: the default.
        let Some(held) = RunStorage::get(&self.ports, &input.run_id) else {
            return Ok(crate::runs::RecordRunStartedOutcome::NoSuchRun { error: crate::runs::NotFound });
        };
        let _ = &held;
        let moved = match held.refine() {
            crate::runs::AnyRun::Admitted(instance) => crate::runs::AnyRun::Running(instance.start()),
            _ => return Ok(crate::runs::RecordRunStartedOutcome::WrongState { error: crate::runs::StateConflict }),
        };
        let mut next = moved.snapshot();
        next.data.session_id = Some(input.session_id.clone());
        next.data.credential_source = Some(input.credential_source.clone());
        let answer = crate::runs::RecordRunStartedOutcome::Started { run_started: crate::runs::RunStarted { run_id: input.run_id.clone(), session_id: input.session_id.clone(), credential_source: input.credential_source.clone() } };
        RunStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.runs.RequestRunApproval`, generated: every outcome is one the specification fully determines.
impl<P> crate::runs::obligations::RequestRunApprovalBehavior for Generated<P>
where
    P: TryContext + RunApprovalStorage,
{
    fn request_run_approval(&mut self, input: crate::runs::RequestRunApproval) -> Result<crate::runs::RequestRunApprovalOutcome, UnmetObligation> {
        let _ = &input;
        // `requested`: the default.
        let identity: crate::runs::ApprovalId = self.ports.try_generate_agency_runs_approval_id()?;
        let data = crate::runs::RunApprovalData {
            approval_id: identity.clone(),
            run_id: input.run_id.clone(),
            subject: input.subject.clone(),
            capability: input.capability.clone(),
            requested_at: self.ports.try_generate_timestamp()?,
            answered_by: None,
            answered_at: None,
        };
        let answer = crate::runs::RequestRunApprovalOutcome::Requested { run_approval_requested: crate::runs::RunApprovalRequested { approval_id: identity.clone(), run_id: input.run_id.clone(), subject: input.subject.clone(), capability: input.capability.clone() } };
        RunApprovalStorage::put(&mut self.ports, crate::runs::AnyRunApproval::Pending(crate::runs::RunApproval::new(data)).snapshot());
        return Ok(answer);
    }
}

/// `agency.runs.SetRunBudget`, generated: every outcome is one the specification fully determines.
impl<P> crate::runs::obligations::SetRunBudgetBehavior for Generated<P>
where
    P: RunStorage,
{
    fn set_run_budget(&mut self, input: crate::runs::SetRunBudget) -> Result<crate::runs::SetRunBudgetOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if RunStorage::get(&self.ports, &input.run_id).is_none() {
            return Ok(crate::runs::SetRunBudgetOutcome::NoSuchRun { error: crate::runs::NotFound });
        }
        // The addressed row, read before the branches that select by it.
        let Some(held) = RunStorage::get(&self.ports, &input.run_id) else {
            return Ok(crate::runs::SetRunBudgetOutcome::NoSuchRun { error: crate::runs::NotFound });
        };
        let _ = &held;
        // `ended`: selected by the addressed row.
        if decided(any(&[equal(Some(&held.state).map(|value| match value { crate::runs::RunState::Admitted => "Admitted", crate::runs::RunState::Failed => "Failed", crate::runs::RunState::Proposed => "Proposed", crate::runs::RunState::Requested => "Requested", crate::runs::RunState::Running => "Running", crate::runs::RunState::Stopped => "Stopped" }.to_owned()), Some("Proposed".to_owned())), equal(Some(&held.state).map(|value| match value { crate::runs::RunState::Admitted => "Admitted", crate::runs::RunState::Failed => "Failed", crate::runs::RunState::Proposed => "Proposed", crate::runs::RunState::Requested => "Requested", crate::runs::RunState::Running => "Running", crate::runs::RunState::Stopped => "Stopped" }.to_owned()), Some("Stopped".to_owned())), equal(Some(&held.state).map(|value| match value { crate::runs::RunState::Admitted => "Admitted", crate::runs::RunState::Failed => "Failed", crate::runs::RunState::Proposed => "Proposed", crate::runs::RunState::Requested => "Requested", crate::runs::RunState::Running => "Running", crate::runs::RunState::Stopped => "Stopped" }.to_owned()), Some("Failed".to_owned()))]), "agency.runs.SetRunBudget")? {
            return Ok(crate::runs::SetRunBudgetOutcome::Ended { error: crate::runs::RunEnded });
        }
        // `set`: the default.
        let Some(held) = RunStorage::get(&self.ports, &input.run_id) else {
            return Ok(crate::runs::SetRunBudgetOutcome::NoSuchRun { error: crate::runs::NotFound });
        };
        let _ = &held;
        let mut next = held;
        next.data.budget = input.budget.clone();
        let answer = crate::runs::SetRunBudgetOutcome::Set { run_budget_set: crate::runs::RunBudgetSet { run_id: input.run_id.clone(), budget: input.budget.clone() } };
        RunStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.runs.StartRun`, generated: every outcome is one the specification fully determines.
impl<P> crate::runs::obligations::StartRunBehavior for Generated<P>
where
    P: TryContext + RunStorage,
{
    fn start_run(&mut self, input: crate::runs::StartRun) -> Result<crate::runs::StartRunOutcome, UnmetObligation> {
        let _ = &input;
        // `requested`: the default.
        let identity: crate::runs::RunId = self.ports.try_generate_agency_runs_run_id()?;
        let data = crate::runs::RunData {
            run_id: identity.clone(),
            tenant_id: input.tenant_id.clone(),
            agent_id: input.agent_id.clone(),
            work_item_ref: input.work_item_ref.clone(),
            budget: input.budget.clone(),
            workspace: input.workspace.clone(),
            session_id: None,
            credential_source: None,
            started_at: self.ports.try_generate_timestamp()?,
            ended_at: None,
            outcome: None,
            usage: None,
        };
        let answer = crate::runs::StartRunOutcome::Requested { run_requested: crate::runs::RunRequested { run_id: identity.clone(), agent_id: input.agent_id.clone(), work_item_ref: input.work_item_ref.clone() } };
        RunStorage::put(&mut self.ports, crate::runs::AnyRun::Requested(crate::runs::Run::new(data)).snapshot());
        return Ok(answer);
    }
}

/// `agency.runs.StopRun`, generated: every outcome is one the specification fully determines.
impl<P> crate::runs::obligations::StopRunBehavior for Generated<P>
where
    P: TryContext + RunStorage,
{
    fn stop_run(&mut self, input: crate::runs::StopRun) -> Result<crate::runs::StopRunOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if RunStorage::get(&self.ports, &input.run_id).is_none() {
            return Ok(crate::runs::StopRunOutcome::NoSuchRun { error: crate::runs::NotFound });
        }
        // `stopped`: the default.
        let Some(held) = RunStorage::get(&self.ports, &input.run_id) else {
            return Ok(crate::runs::StopRunOutcome::NoSuchRun { error: crate::runs::NotFound });
        };
        let _ = &held;
        let moved = match held.refine() {
            crate::runs::AnyRun::Admitted(instance) => crate::runs::AnyRun::Stopped(instance.stop()),
            crate::runs::AnyRun::Requested(instance) => crate::runs::AnyRun::Stopped(instance.stop()),
            crate::runs::AnyRun::Running(instance) => crate::runs::AnyRun::Stopped(instance.stop()),
            _ => return Ok(crate::runs::StopRunOutcome::WrongState { error: crate::runs::StateConflict }),
        };
        let mut next = moved.snapshot();
        next.data.ended_at = self.ports.try_generate_optional_timestamp()?;
        let answer = crate::runs::StopRunOutcome::Stopped { run_stopped: crate::runs::RunStopped { run_id: input.run_id.clone(), reason: input.reason.clone() } };
        RunStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.service.CloseSession`, generated: every outcome is one the specification fully determines.
impl<P> crate::service::obligations::CloseSessionBehavior for Generated<P>
where
    P: SessionStorage,
{
    fn close_session(&mut self, input: crate::service::CloseSession) -> Result<crate::service::CloseSessionOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if SessionStorage::get(&self.ports, &input.session_id).is_none() {
            return Ok(crate::service::CloseSessionOutcome::NoSuchSession { error: crate::service::NotFound });
        }
        // `closed`: the default.
        let Some(held) = SessionStorage::get(&self.ports, &input.session_id) else {
            return Ok(crate::service::CloseSessionOutcome::NoSuchSession { error: crate::service::NotFound });
        };
        let _ = &held;
        let moved = match held.refine() {
            crate::service::AnySession::Open(instance) => crate::service::AnySession::Closed(instance.close()),
            _ => return Ok(crate::service::CloseSessionOutcome::WrongState { error: crate::service::StateConflict }),
        };
        let next = moved.snapshot();
        let answer = crate::service::CloseSessionOutcome::Closed { session_closed: crate::service::SessionClosed { session_id: input.session_id.clone() } };
        SessionStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.service.OpenSession`, generated: every outcome is one the specification fully determines.
impl<P> crate::service::obligations::OpenSessionBehavior for Generated<P>
where
    P: TryContext + SessionStorage,
{
    fn open_session(&mut self, input: crate::service::OpenSession) -> Result<crate::service::OpenSessionOutcome, UnmetObligation> {
        let _ = &input;
        // `opened`: the default.
        let identity: crate::service::SessionId = self.ports.try_generate_agency_service_session_id()?;
        let data = crate::service::SessionData {
            session_id: identity.clone(),
            principal_id: input.principal_id.clone(),
            kind: input.kind.clone(),
            run_id: input.run_id.clone(),
            token_digest: input.token_digest.clone(),
            csrf_digest: input.csrf_digest.clone(),
            opened_at: self.ports.try_generate_timestamp()?,
            expires_at: input.expires_at.clone(),
        };
        let answer = crate::service::OpenSessionOutcome::Opened { session_opened: crate::service::SessionOpened { session_id: identity.clone(), principal_id: input.principal_id.clone(), kind: input.kind.clone() } };
        SessionStorage::put(&mut self.ports, crate::service::AnySession::Open(crate::service::Session::new(data)).snapshot());
        return Ok(answer);
    }
}

/// `agency.teams.AddMember`, generated: every outcome is one the specification fully determines.
impl<P> crate::teams::obligations::AddMemberBehavior for Generated<P>
where
    P: TeamStorage,
{
    fn add_member(&mut self, input: crate::teams::AddMember) -> Result<crate::teams::AddMemberOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if TeamStorage::get(&self.ports, &input.team_id).is_none() {
            return Ok(crate::teams::AddMemberOutcome::NoSuchTeam { error: crate::teams::NotFound });
        }
        // `added`: the default.
        let Some(held) = TeamStorage::get(&self.ports, &input.team_id) else {
            return Ok(crate::teams::AddMemberOutcome::NoSuchTeam { error: crate::teams::NotFound });
        };
        let _ = &held;
        let next = held;
        let answer = crate::teams::AddMemberOutcome::Added { member_added: crate::teams::MemberAdded { team_id: input.team_id.clone(), agent_id: input.agent_id.clone() } };
        TeamStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.teams.AttachQueue`, generated: every outcome is one the specification fully determines.
impl<P> crate::teams::obligations::AttachQueueBehavior for Generated<P>
where
    P: WorkInputStorage,
{
    fn attach_queue(&mut self, input: crate::teams::AttachQueue) -> Result<crate::teams::AttachQueueOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if WorkInputStorage::get(&self.ports, &input.input_id).is_none() {
            return Ok(crate::teams::AttachQueueOutcome::NoSuchInput { error: crate::teams::NotFound });
        }
        // `attached`: the default.
        let Some(held) = WorkInputStorage::get(&self.ports, &input.input_id) else {
            return Ok(crate::teams::AttachQueueOutcome::NoSuchInput { error: crate::teams::NotFound });
        };
        let _ = &held;
        let mut next = held;
        next.data.workforce_id = input.workforce_id.clone();
        let answer = crate::teams::AttachQueueOutcome::Attached { queue_attached: crate::teams::QueueAttached { workforce_id: input.workforce_id.clone(), input_id: input.input_id.clone() } };
        WorkInputStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.teams.DeclareSwarm`, generated: every outcome is one the specification fully determines.
impl<P> crate::teams::obligations::DeclareSwarmBehavior for Generated<P>
where
    P: TryContext + SwarmStorage,
{
    fn declare_swarm(&mut self, input: crate::teams::DeclareSwarm) -> Result<crate::teams::DeclareSwarmOutcome, UnmetObligation> {
        let _ = &input;
        // `declared`: the default.
        let identity: crate::teams::SwarmId = self.ports.try_generate_agency_teams_swarm_id()?;
        let data = crate::teams::SwarmData {
            swarm_id: identity.clone(),
            tenant_id: input.tenant_id.clone(),
            goal_ref: input.goal_ref.clone(),
            coordinator_agent_id: input.coordinator_agent_id.clone(),
            frame_digest: input.frame_digest.clone(),
            max_workers: input.max_workers.clone(),
            team_id: input.team_id.clone(),
            work_input_id: input.work_input_id.clone(),
        };
        let answer = crate::teams::DeclareSwarmOutcome::Declared { swarm_declared: crate::teams::SwarmDeclared { swarm_id: identity.clone(), goal_ref: input.goal_ref.clone(), team_id: input.team_id.clone() } };
        SwarmStorage::put(&mut self.ports, crate::teams::AnySwarm::Declared(crate::teams::Swarm::new(data)).snapshot());
        return Ok(answer);
    }
}

/// `agency.teams.DefineWorkforce`, generated: every outcome is one the specification fully determines.
impl<P> crate::teams::obligations::DefineWorkforceBehavior for Generated<P>
where
    P: TryContext + WorkforceStorage,
{
    fn define_workforce(&mut self, input: crate::teams::DefineWorkforce) -> Result<crate::teams::DefineWorkforceOutcome, UnmetObligation> {
        let _ = &input;
        // `defined`: the default.
        let identity: crate::teams::WorkforceId = self.ports.try_generate_agency_teams_workforce_id()?;
        let data = crate::teams::WorkforceData {
            workforce_id: identity.clone(),
            tenant_id: input.tenant_id.clone(),
            name: input.name.clone(),
            teams: Vec::new(),
            solo_agents: Vec::new(),
            capacity: input.capacity.clone(),
        };
        let answer = crate::teams::DefineWorkforceOutcome::Defined { workforce_defined: crate::teams::WorkforceDefined { workforce_id: identity.clone(), name: input.name.clone() } };
        WorkforceStorage::put(&mut self.ports, crate::teams::AnyWorkforce::Defined(crate::teams::Workforce::new(data)).snapshot());
        return Ok(answer);
    }
}

/// `agency.teams.DisbandTeam`, generated: every outcome is one the specification fully determines.
impl<P> crate::teams::obligations::DisbandTeamBehavior for Generated<P>
where
    P: TeamStorage,
{
    fn disband_team(&mut self, input: crate::teams::DisbandTeam) -> Result<crate::teams::DisbandTeamOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if TeamStorage::get(&self.ports, &input.team_id).is_none() {
            return Ok(crate::teams::DisbandTeamOutcome::NoSuchTeam { error: crate::teams::NotFound });
        }
        // `disbanded`: the default.
        let Some(held) = TeamStorage::get(&self.ports, &input.team_id) else {
            return Ok(crate::teams::DisbandTeamOutcome::NoSuchTeam { error: crate::teams::NotFound });
        };
        let _ = &held;
        let moved = match held.refine() {
            crate::teams::AnyTeam::Formed(instance) => crate::teams::AnyTeam::Disbanded(instance.disband()),
            _ => return Ok(crate::teams::DisbandTeamOutcome::WrongState { error: crate::teams::StateConflict }),
        };
        let next = moved.snapshot();
        let answer = crate::teams::DisbandTeamOutcome::Disbanded { team_disbanded: crate::teams::TeamDisbanded { team_id: input.team_id.clone() } };
        TeamStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.teams.FormTeam`, generated: every outcome is one the specification fully determines.
impl<P> crate::teams::obligations::FormTeamBehavior for Generated<P>
where
    P: TryContext + TeamStorage,
{
    fn form_team(&mut self, input: crate::teams::FormTeam) -> Result<crate::teams::FormTeamOutcome, UnmetObligation> {
        let _ = &input;
        // `formed`: the default.
        let identity: crate::teams::TeamId = self.ports.try_generate_agency_teams_team_id()?;
        let data = crate::teams::TeamData {
            team_id: identity.clone(),
            tenant_id: input.tenant_id.clone(),
            name: input.name.clone(),
            lead_agent_id: input.lead_agent_id.clone(),
            members: input.members.clone(),
            broker_ref: input.broker_ref.clone(),
        };
        let answer = crate::teams::FormTeamOutcome::Formed { team_formed: crate::teams::TeamFormed { team_id: identity.clone(), name: input.name.clone(), lead_agent_id: input.lead_agent_id.clone() } };
        TeamStorage::put(&mut self.ports, crate::teams::AnyTeam::Formed(crate::teams::Team::new(data)).snapshot());
        return Ok(answer);
    }
}

/// `agency.teams.RemoveMember`, generated: every outcome is one the specification fully determines.
impl<P> crate::teams::obligations::RemoveMemberBehavior for Generated<P>
where
    P: TeamStorage,
{
    fn remove_member(&mut self, input: crate::teams::RemoveMember) -> Result<crate::teams::RemoveMemberOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if TeamStorage::get(&self.ports, &input.team_id).is_none() {
            return Ok(crate::teams::RemoveMemberOutcome::NoSuchTeam { error: crate::teams::NotFound });
        }
        // `removed`: the default.
        let Some(held) = TeamStorage::get(&self.ports, &input.team_id) else {
            return Ok(crate::teams::RemoveMemberOutcome::NoSuchTeam { error: crate::teams::NotFound });
        };
        let _ = &held;
        let next = held;
        let answer = crate::teams::RemoveMemberOutcome::Removed { member_removed: crate::teams::MemberRemoved { team_id: input.team_id.clone(), agent_id: input.agent_id.clone() } };
        TeamStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.teams.Report`, generated: every outcome is one the specification fully determines.
impl<P> crate::teams::obligations::ReportBehavior for Generated<P>
where
    P: TryContext + TeamStorage,
{
    fn report(&mut self, input: crate::teams::Report) -> Result<crate::teams::ReportOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if TeamStorage::get(&self.ports, &input.team_id).is_none() {
            return Ok(crate::teams::ReportOutcome::NoSuchTeam { error: crate::teams::NotFound });
        }
        // `not-one-level-up`: an external branch, where the context takes it.
        if self.ports.try_external(ExternalCommand::AgencyTeamsReport(&input), "not-one-level-up")? {
            return Ok(crate::teams::ReportOutcome::NotOneLevelUp { error: crate::teams::NotOneLevelUp });
        }
        // `reported`: the default.
        let Some(held) = TeamStorage::get(&self.ports, &input.team_id) else {
            return Ok(crate::teams::ReportOutcome::NoSuchTeam { error: crate::teams::NotFound });
        };
        let _ = &held;
        let next = held;
        let answer = crate::teams::ReportOutcome::Reported { report_sent: crate::teams::ReportSent { team_id: input.team_id.clone(), from_agent_id: input.from_agent_id.clone(), status: input.status.clone(), line: input.line.clone() } };
        TeamStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.teams.RetireWorkforce`, generated: every outcome is one the specification fully determines.
impl<P> crate::teams::obligations::RetireWorkforceBehavior for Generated<P>
where
    P: WorkforceStorage,
{
    fn retire_workforce(&mut self, input: crate::teams::RetireWorkforce) -> Result<crate::teams::RetireWorkforceOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if WorkforceStorage::get(&self.ports, &input.workforce_id).is_none() {
            return Ok(crate::teams::RetireWorkforceOutcome::NoSuchWorkforce { error: crate::teams::NotFound });
        }
        // `retired`: the default.
        let Some(held) = WorkforceStorage::get(&self.ports, &input.workforce_id) else {
            return Ok(crate::teams::RetireWorkforceOutcome::NoSuchWorkforce { error: crate::teams::NotFound });
        };
        let _ = &held;
        let moved = match held.refine() {
            crate::teams::AnyWorkforce::Staffed(instance) => crate::teams::AnyWorkforce::Retired(instance.retire()),
            _ => return Ok(crate::teams::RetireWorkforceOutcome::WrongState { error: crate::teams::StateConflict }),
        };
        let next = moved.snapshot();
        let answer = crate::teams::RetireWorkforceOutcome::Retired { workforce_retired: crate::teams::WorkforceRetired { workforce_id: input.workforce_id.clone() } };
        WorkforceStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.teams.Send`, generated: every outcome is one the specification fully determines.
impl<P> crate::teams::obligations::SendBehavior for Generated<P>
where
    P: TryContext + TeamStorage,
{
    fn send(&mut self, input: crate::teams::Send) -> Result<crate::teams::SendOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if TeamStorage::get(&self.ports, &input.team_id).is_none() {
            return Ok(crate::teams::SendOutcome::NoSuchTeam { error: crate::teams::NotFound });
        }
        // `foreign-instance`: an external branch, where the context takes it.
        if self.ports.try_external(ExternalCommand::AgencyTeamsSend(&input), "foreign-instance")? {
            return Ok(crate::teams::SendOutcome::ForeignInstance { error: crate::teams::ForeignInstance });
        }
        // `outside-team`: an external branch, where the context takes it.
        if self.ports.try_external(ExternalCommand::AgencyTeamsSend(&input), "outside-team")? {
            return Ok(crate::teams::SendOutcome::OutsideTeam { error: crate::teams::OutsideTeam });
        }
        // `sent`: the default.
        let Some(held) = TeamStorage::get(&self.ports, &input.team_id) else {
            return Ok(crate::teams::SendOutcome::NoSuchTeam { error: crate::teams::NotFound });
        };
        let _ = &held;
        let next = held;
        let answer = crate::teams::SendOutcome::Sent { message_sent: crate::teams::MessageSent { team_id: input.team_id.clone(), from_agent_id: input.from_agent_id.clone(), to: input.to.clone(), text: input.text.clone() } };
        TeamStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.teams.SetLead`, generated: every outcome is one the specification fully determines.
impl<P> crate::teams::obligations::SetLeadBehavior for Generated<P>
where
    P: TeamStorage,
{
    fn set_lead(&mut self, input: crate::teams::SetLead) -> Result<crate::teams::SetLeadOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if TeamStorage::get(&self.ports, &input.team_id).is_none() {
            return Ok(crate::teams::SetLeadOutcome::NoSuchTeam { error: crate::teams::NotFound });
        }
        // `set`: the default.
        let Some(held) = TeamStorage::get(&self.ports, &input.team_id) else {
            return Ok(crate::teams::SetLeadOutcome::NoSuchTeam { error: crate::teams::NotFound });
        };
        let _ = &held;
        let mut next = held;
        next.data.lead_agent_id = input.lead_agent_id.clone();
        let answer = crate::teams::SetLeadOutcome::Set { lead_set: crate::teams::LeadSet { team_id: input.team_id.clone(), lead_agent_id: input.lead_agent_id.clone() } };
        TeamStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.teams.StaffWorkforce`, generated: every outcome is one the specification fully determines.
impl<P> crate::teams::obligations::StaffWorkforceBehavior for Generated<P>
where
    P: WorkforceStorage,
{
    fn staff_workforce(&mut self, input: crate::teams::StaffWorkforce) -> Result<crate::teams::StaffWorkforceOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if WorkforceStorage::get(&self.ports, &input.workforce_id).is_none() {
            return Ok(crate::teams::StaffWorkforceOutcome::NoSuchWorkforce { error: crate::teams::NotFound });
        }
        // `staffed`: the default.
        let Some(held) = WorkforceStorage::get(&self.ports, &input.workforce_id) else {
            return Ok(crate::teams::StaffWorkforceOutcome::NoSuchWorkforce { error: crate::teams::NotFound });
        };
        let _ = &held;
        let moved = match held.refine() {
            crate::teams::AnyWorkforce::Defined(instance) => crate::teams::AnyWorkforce::Staffed(instance.staff()),
            _ => return Ok(crate::teams::StaffWorkforceOutcome::WrongState { error: crate::teams::StateConflict }),
        };
        let mut next = moved.snapshot();
        next.data.teams = input.teams.clone();
        next.data.solo_agents = input.solo_agents.clone();
        let answer = crate::teams::StaffWorkforceOutcome::Staffed { workforce_staffed: crate::teams::WorkforceStaffed { workforce_id: input.workforce_id.clone(), teams: input.teams.clone(), solo_agents: input.solo_agents.clone() } };
        WorkforceStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.teams.StopSwarm`, generated: every outcome is one the specification fully determines.
impl<P> crate::teams::obligations::StopSwarmBehavior for Generated<P>
where
    P: SwarmStorage,
{
    fn stop_swarm(&mut self, input: crate::teams::StopSwarm) -> Result<crate::teams::StopSwarmOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if SwarmStorage::get(&self.ports, &input.swarm_id).is_none() {
            return Ok(crate::teams::StopSwarmOutcome::NoSuchSwarm { error: crate::teams::NotFound });
        }
        // `stopped`: the default.
        let Some(held) = SwarmStorage::get(&self.ports, &input.swarm_id) else {
            return Ok(crate::teams::StopSwarmOutcome::NoSuchSwarm { error: crate::teams::NotFound });
        };
        let _ = &held;
        let moved = match held.refine() {
            crate::teams::AnySwarm::Declared(instance) => crate::teams::AnySwarm::Stopped(instance.stop()),
            _ => return Ok(crate::teams::StopSwarmOutcome::WrongState { error: crate::teams::StateConflict }),
        };
        let next = moved.snapshot();
        let answer = crate::teams::StopSwarmOutcome::Stopped { swarm_stopped: crate::teams::SwarmStopped { swarm_id: input.swarm_id.clone() } };
        SwarmStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.tools.BindTool`, generated: every outcome is one the specification fully determines.
impl<P> crate::tools::obligations::BindToolBehavior for Generated<P>
where
    P: TryContext + ToolBindingStorage,
{
    fn bind_tool(&mut self, input: crate::tools::BindTool) -> Result<crate::tools::BindToolOutcome, UnmetObligation> {
        let _ = &input;
        // `bound-to-definition`: an accepting branch, in declaration order.
        if decided(all(&[Some(Some(&input.definition_id).and_then(|value| value.as_ref()).map(|value| &value.0).is_some()), (Some(Some(&input.team_id).and_then(|value| value.as_ref()).map(|value| &value.0).is_some())).map(|value| !value)]), "agency.tools.BindTool")? {
            let identity: crate::tools::BindingId = self.ports.try_generate_agency_tools_binding_id()?;
            let data = crate::tools::ToolBindingData {
                binding_id: identity.clone(),
                tenant_id: input.tenant_id.clone(),
                definition_id: input.definition_id.clone(),
                team_id: None,
                tool_id: input.tool_id.clone(),
                verbs: input.verbs.clone(),
                scope: input.scope.clone(),
                max_output_bytes: input.max_output_bytes.clone(),
                expires_at: input.expires_at.clone(),
                credential_id: input.credential_id.clone(),
            };
            if let Some(broken) = data.broken_invariant() {
                let capability = "entity invariant";
                return Err(UnmetObligation { capability, source: broken });
            }
            let answer = crate::tools::BindToolOutcome::BoundToDefinition { tool_bound: crate::tools::ToolBound { binding_id: identity.clone(), tool_id: input.tool_id.clone(), verbs: input.verbs.clone() } };
            ToolBindingStorage::put(&mut self.ports, crate::tools::AnyToolBinding::Granted(crate::tools::ToolBinding::new(data)).snapshot());
            return Ok(answer);
        }
        // `bound-to-team`: an accepting branch, in declaration order.
        if decided(all(&[(Some(Some(&input.definition_id).and_then(|value| value.as_ref()).map(|value| &value.0).is_some())).map(|value| !value), Some(Some(&input.team_id).and_then(|value| value.as_ref()).map(|value| &value.0).is_some())]), "agency.tools.BindTool")? {
            let identity: crate::tools::BindingId = self.ports.try_generate_agency_tools_binding_id()?;
            let data = crate::tools::ToolBindingData {
                binding_id: identity.clone(),
                tenant_id: input.tenant_id.clone(),
                definition_id: None,
                team_id: input.team_id.clone(),
                tool_id: input.tool_id.clone(),
                verbs: input.verbs.clone(),
                scope: input.scope.clone(),
                max_output_bytes: input.max_output_bytes.clone(),
                expires_at: input.expires_at.clone(),
                credential_id: input.credential_id.clone(),
            };
            if let Some(broken) = data.broken_invariant() {
                let capability = "entity invariant";
                return Err(UnmetObligation { capability, source: broken });
            }
            let answer = crate::tools::BindToolOutcome::BoundToTeam { tool_bound: crate::tools::ToolBound { binding_id: identity.clone(), tool_id: input.tool_id.clone(), verbs: input.verbs.clone() } };
            ToolBindingStorage::put(&mut self.ports, crate::tools::AnyToolBinding::Granted(crate::tools::ToolBinding::new(data)).snapshot());
            return Ok(answer);
        }
        // `target-invalid`: the default.
        return Ok(crate::tools::BindToolOutcome::TargetInvalid { error: crate::tools::BindingTargetInvalid });
    }
}

/// `agency.tools.RegisterTool`, generated: every outcome is one the specification fully determines.
impl<P> crate::tools::obligations::RegisterToolBehavior for Generated<P>
where
    P: TryContext + ToolStorage,
{
    fn register_tool(&mut self, input: crate::tools::RegisterTool) -> Result<crate::tools::RegisterToolOutcome, UnmetObligation> {
        let _ = &input;
        // `registered`: the default.
        let identity: crate::tools::ToolId = self.ports.try_generate_agency_tools_tool_id()?;
        let data = crate::tools::ToolData {
            tool_id: identity.clone(),
            tenant_id: input.tenant_id.clone(),
            contract_ref: input.contract_ref.clone(),
            verbs: input.verbs.clone(),
            risk: input.risk.clone(),
            implementation_ref: input.implementation_ref.clone(),
            connector_instance_id: input.connector_instance_id.clone(),
        };
        let answer = crate::tools::RegisterToolOutcome::Registered { tool_registered: crate::tools::ToolRegistered { tool_id: identity.clone(), contract_ref: input.contract_ref.clone() } };
        ToolStorage::put(&mut self.ports, crate::tools::AnyTool::Registered(crate::tools::Tool::new(data)).snapshot());
        return Ok(answer);
    }
}

/// `agency.tools.RevokeBinding`, generated: every outcome is one the specification fully determines.
impl<P> crate::tools::obligations::RevokeBindingBehavior for Generated<P>
where
    P: ToolBindingStorage,
{
    fn revoke_binding(&mut self, input: crate::tools::RevokeBinding) -> Result<crate::tools::RevokeBindingOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if ToolBindingStorage::get(&self.ports, &input.binding_id).is_none() {
            return Ok(crate::tools::RevokeBindingOutcome::NoSuchBinding { error: crate::tools::NotFound });
        }
        // `revoked`: the default.
        let Some(held) = ToolBindingStorage::get(&self.ports, &input.binding_id) else {
            return Ok(crate::tools::RevokeBindingOutcome::NoSuchBinding { error: crate::tools::NotFound });
        };
        let _ = &held;
        let moved = match held.refine() {
            crate::tools::AnyToolBinding::Granted(instance) => crate::tools::AnyToolBinding::Revoked(instance.revoke()),
            _ => return Ok(crate::tools::RevokeBindingOutcome::WrongState { error: crate::tools::StateConflict }),
        };
        let next = moved.snapshot();
        if let Some(broken) = next.data.broken_invariant() {
            let capability = "entity invariant";
            return Err(UnmetObligation { capability, source: broken });
        }
        let answer = crate::tools::RevokeBindingOutcome::Revoked { binding_revoked: crate::tools::BindingRevoked { binding_id: input.binding_id.clone() } };
        ToolBindingStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.tools.WithdrawTool`, generated: every outcome is one the specification fully determines.
impl<P> crate::tools::obligations::WithdrawToolBehavior for Generated<P>
where
    P: ToolStorage,
{
    fn withdraw_tool(&mut self, input: crate::tools::WithdrawTool) -> Result<crate::tools::WithdrawToolOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if ToolStorage::get(&self.ports, &input.tool_id).is_none() {
            return Ok(crate::tools::WithdrawToolOutcome::NoSuchTool { error: crate::tools::NotFound });
        }
        // `withdrawn`: the default.
        let Some(held) = ToolStorage::get(&self.ports, &input.tool_id) else {
            return Ok(crate::tools::WithdrawToolOutcome::NoSuchTool { error: crate::tools::NotFound });
        };
        let _ = &held;
        let moved = match held.refine() {
            crate::tools::AnyTool::Registered(instance) => crate::tools::AnyTool::Withdrawn(instance.withdraw()),
            _ => return Ok(crate::tools::WithdrawToolOutcome::WrongState { error: crate::tools::StateConflict }),
        };
        let next = moved.snapshot();
        let answer = crate::tools::WithdrawToolOutcome::Withdrawn { tool_withdrawn: crate::tools::ToolWithdrawn { tool_id: input.tool_id.clone() } };
        ToolStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.work.AbandonWorkItem`, generated: every outcome is one the specification fully determines.
impl<P> crate::work::obligations::AbandonWorkItemBehavior for Generated<P>
where
    P: WorkItemStorage,
{
    fn abandon_work_item(&mut self, input: crate::work::AbandonWorkItem) -> Result<crate::work::AbandonWorkItemOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if WorkItemStorage::get(&self.ports, &input.item_ref).is_none() {
            return Ok(crate::work::AbandonWorkItemOutcome::NoSuchItem { error: crate::work::NotFound });
        }
        // `abandoned`: the default.
        let Some(held) = WorkItemStorage::get(&self.ports, &input.item_ref) else {
            return Ok(crate::work::AbandonWorkItemOutcome::NoSuchItem { error: crate::work::NotFound });
        };
        let _ = &held;
        let moved = match held.refine() {
            crate::work::AnyWorkItem::Assigned(instance) => crate::work::AnyWorkItem::Abandoned(instance.abandon()),
            _ => return Ok(crate::work::AbandonWorkItemOutcome::WrongState { error: crate::work::StateConflict }),
        };
        let next = moved.snapshot();
        let answer = crate::work::AbandonWorkItemOutcome::Abandoned { work_item_abandoned: crate::work::WorkItemAbandoned { item_ref: input.item_ref.clone() } };
        WorkItemStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.work.AssignWorkItem`, generated: every outcome is one the specification fully determines.
impl<P> crate::work::obligations::AssignWorkItemBehavior for Generated<P>
where
    P: WorkItemStorage,
{
    fn assign_work_item(&mut self, input: crate::work::AssignWorkItem) -> Result<crate::work::AssignWorkItemOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if WorkItemStorage::get(&self.ports, &input.item_ref).is_none() {
            return Ok(crate::work::AssignWorkItemOutcome::NoSuchItem { error: crate::work::NotFound });
        }
        // `assigned`: the default.
        let Some(held) = WorkItemStorage::get(&self.ports, &input.item_ref) else {
            return Ok(crate::work::AssignWorkItemOutcome::NoSuchItem { error: crate::work::NotFound });
        };
        let _ = &held;
        let moved = match held.refine() {
            crate::work::AnyWorkItem::Received(instance) => crate::work::AnyWorkItem::Assigned(instance.assign()),
            _ => return Ok(crate::work::AssignWorkItemOutcome::WrongState { error: crate::work::StateConflict }),
        };
        let mut next = moved.snapshot();
        next.data.assigned_agent_id = Some(input.agent_id.clone());
        let answer = crate::work::AssignWorkItemOutcome::Assigned { work_item_assigned: crate::work::WorkItemAssigned { item_ref: input.item_ref.clone(), agent_id: input.agent_id.clone() } };
        WorkItemStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.work.ConfigureWorkInput`, generated: every outcome is one the specification fully determines.
impl<P> crate::work::obligations::ConfigureWorkInputBehavior for Generated<P>
where
    P: TryContext + WorkInputStorage,
{
    fn configure_work_input(&mut self, input: crate::work::ConfigureWorkInput) -> Result<crate::work::ConfigureWorkInputOutcome, UnmetObligation> {
        let _ = &input;
        // `configured`: the default.
        let identity: crate::work::InputId = self.ports.try_generate_agency_work_input_id()?;
        let data = crate::work::WorkInputData {
            input_id: identity.clone(),
            tenant_id: input.tenant_id.clone(),
            name: input.name.clone(),
            source: input.source.clone(),
            poll_interval: input.poll_interval.clone(),
            workforce_id: input.workforce_id.clone(),
            transform_ref: input.transform_ref.clone(),
        };
        let answer = crate::work::ConfigureWorkInputOutcome::Configured { work_input_configured: crate::work::WorkInputConfigured { input_id: identity.clone(), name: input.name.clone(), workforce_id: input.workforce_id.clone() } };
        WorkInputStorage::put(&mut self.ports, crate::work::AnyWorkInput::Configured(crate::work::WorkInput::new(data)).snapshot());
        return Ok(answer);
    }
}

/// `agency.work.PauseWorkInput`, generated: every outcome is one the specification fully determines.
impl<P> crate::work::obligations::PauseWorkInputBehavior for Generated<P>
where
    P: WorkInputStorage,
{
    fn pause_work_input(&mut self, input: crate::work::PauseWorkInput) -> Result<crate::work::PauseWorkInputOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if WorkInputStorage::get(&self.ports, &input.input_id).is_none() {
            return Ok(crate::work::PauseWorkInputOutcome::NoSuchInput { error: crate::work::NotFound });
        }
        // `paused`: the default.
        let Some(held) = WorkInputStorage::get(&self.ports, &input.input_id) else {
            return Ok(crate::work::PauseWorkInputOutcome::NoSuchInput { error: crate::work::NotFound });
        };
        let _ = &held;
        let moved = match held.refine() {
            crate::work::AnyWorkInput::Configured(instance) => crate::work::AnyWorkInput::Paused(instance.pause()),
            _ => return Ok(crate::work::PauseWorkInputOutcome::WrongState { error: crate::work::StateConflict }),
        };
        let next = moved.snapshot();
        let answer = crate::work::PauseWorkInputOutcome::Paused { work_input_paused: crate::work::WorkInputPaused { input_id: input.input_id.clone() } };
        WorkInputStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.work.RemoveWorkInput`, generated: every outcome is one the specification fully determines.
impl<P> crate::work::obligations::RemoveWorkInputBehavior for Generated<P>
where
    P: WorkInputStorage,
{
    fn remove_work_input(&mut self, input: crate::work::RemoveWorkInput) -> Result<crate::work::RemoveWorkInputOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if WorkInputStorage::get(&self.ports, &input.input_id).is_none() {
            return Ok(crate::work::RemoveWorkInputOutcome::NoSuchInput { error: crate::work::NotFound });
        }
        // `removed`: the default.
        let Some(held) = WorkInputStorage::get(&self.ports, &input.input_id) else {
            return Ok(crate::work::RemoveWorkInputOutcome::NoSuchInput { error: crate::work::NotFound });
        };
        let _ = &held;
        let moved = match held.refine() {
            crate::work::AnyWorkInput::Paused(instance) => crate::work::AnyWorkInput::Removed(instance.remove()),
            _ => return Ok(crate::work::RemoveWorkInputOutcome::WrongState { error: crate::work::StateConflict }),
        };
        let next = moved.snapshot();
        let answer = crate::work::RemoveWorkInputOutcome::Removed { work_input_removed: crate::work::WorkInputRemoved { input_id: input.input_id.clone() } };
        WorkInputStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.work.ReturnWorkItem`, generated: every outcome is one the specification fully determines.
impl<P> crate::work::obligations::ReturnWorkItemBehavior for Generated<P>
where
    P: WorkItemStorage,
{
    fn return_work_item(&mut self, input: crate::work::ReturnWorkItem) -> Result<crate::work::ReturnWorkItemOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if WorkItemStorage::get(&self.ports, &input.item_ref).is_none() {
            return Ok(crate::work::ReturnWorkItemOutcome::NoSuchItem { error: crate::work::NotFound });
        }
        // `returned`: the default.
        let Some(held) = WorkItemStorage::get(&self.ports, &input.item_ref) else {
            return Ok(crate::work::ReturnWorkItemOutcome::NoSuchItem { error: crate::work::NotFound });
        };
        let _ = &held;
        let moved = match held.refine() {
            crate::work::AnyWorkItem::Assigned(instance) => crate::work::AnyWorkItem::Returned(instance.r#return()),
            _ => return Ok(crate::work::ReturnWorkItemOutcome::WrongState { error: crate::work::StateConflict }),
        };
        let next = moved.snapshot();
        let answer = crate::work::ReturnWorkItemOutcome::Returned { work_item_returned: crate::work::WorkItemReturned { item_ref: input.item_ref.clone() } };
        WorkItemStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.work.SettleWorkItem`, generated: every outcome is one the specification fully determines.
impl<P> crate::work::obligations::SettleWorkItemBehavior for Generated<P>
where
    P: TryContext + WorkItemStorage,
{
    fn settle_work_item(&mut self, input: crate::work::SettleWorkItem) -> Result<crate::work::SettleWorkItemOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if WorkItemStorage::get(&self.ports, &input.item_ref).is_none() {
            return Ok(crate::work::SettleWorkItemOutcome::NoSuchItem { error: crate::work::NotFound });
        }
        // `settled`: the default.
        let Some(held) = WorkItemStorage::get(&self.ports, &input.item_ref) else {
            return Ok(crate::work::SettleWorkItemOutcome::NoSuchItem { error: crate::work::NotFound });
        };
        let _ = &held;
        let moved = match held.refine() {
            crate::work::AnyWorkItem::Assigned(instance) => crate::work::AnyWorkItem::Done(instance.settle()),
            _ => return Ok(crate::work::SettleWorkItemOutcome::WrongState { error: crate::work::StateConflict }),
        };
        let mut next = moved.snapshot();
        next.data.settled_at = self.ports.try_generate_optional_timestamp()?;
        let answer = crate::work::SettleWorkItemOutcome::Settled { work_item_settled: crate::work::WorkItemSettled { item_ref: input.item_ref.clone() } };
        WorkItemStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agency.work.SubmitWorkItem`, generated: every outcome is one the specification fully determines.
impl<P> crate::work::obligations::SubmitWorkItemBehavior for Generated<P>
where
    P: TryContext + WorkItemStorage,
{
    fn submit_work_item(&mut self, input: crate::work::SubmitWorkItem) -> Result<crate::work::SubmitWorkItemOutcome, UnmetObligation> {
        let _ = &input;
        // `received`: the default.
        let identity: crate::work::WorkItemRef = input.item_ref.clone();
        let data = crate::work::WorkItemData {
            item_ref: identity.clone(),
            tenant_id: input.tenant_id.clone(),
            input_id: input.input_id.clone(),
            title: input.title.clone(),
            payload_digest: input.payload_digest.clone(),
            assigned_agent_id: None,
            received_at: self.ports.try_generate_timestamp()?,
            settled_at: None,
        };
        let answer = crate::work::SubmitWorkItemOutcome::Received { work_item_submitted: crate::work::WorkItemSubmitted { item_ref: identity.clone(), input_id: input.input_id.clone(), payload_digest: input.payload_digest.clone() } };
        WorkItemStorage::put(&mut self.ports, crate::work::AnyWorkItem::Received(crate::work::WorkItem::new(data)).snapshot());
        return Ok(answer);
    }
}

/// `agency.agents.AgentDefinitionList`, generated: every row is one the specification fully determines from the stored `agency.agents.AgentDefinition`s.
impl<P> crate::agents::obligations::AgentDefinitionListQuery for Generated<P>
where
    P: AgentDefinitionStorage,
{
    fn agent_definition_list(&self) -> Result<Vec<crate::agents::AgentDefinitionList>, UnmetObligation> {
        let admitted = AgentDefinitionStorage::list(&self.ports);
        Ok(admitted
            .into_iter()
            .map(|held| crate::agents::AgentDefinitionList {
                definition_id: held.data.definition_id,
                tenant_id: held.data.tenant_id,
                name: held.data.name,
                description: held.data.description,
                agent_spec: held.data.agent_spec,
                policy_id: held.data.policy_id,
                version: held.data.version,
                digest: held.data.digest,
                created_by: held.data.created_by,
                check_exit_code: held.data.check_exit_code,
                check_message: held.data.check_message,
                state: held.state,
            })
            .collect())
    }
}

/// `agency.agents.AgentList`, generated: every row is one the specification fully determines from the stored `agency.agents.Agent`s.
impl<P> crate::agents::obligations::AgentListQuery for Generated<P>
where
    P: AgentStorage,
{
    fn agent_list(&self) -> Result<Vec<crate::agents::AgentList>, UnmetObligation> {
        let admitted = AgentStorage::list(&self.ports);
        Ok(admitted
            .into_iter()
            .map(|held| crate::agents::AgentList {
                agent_id: held.data.agent_id,
                tenant_id: held.data.tenant_id,
                definition_id: held.data.definition_id,
                definition_version: held.data.definition_version,
                display_name: held.data.display_name,
                principal_id: held.data.principal_id,
                current_run_id: held.data.current_run_id,
                last_seen_at: held.data.last_seen_at,
                state: held.state,
            })
            .collect())
    }
}

/// `agency.auth.CredentialList`, generated: every row is one the specification fully determines from the stored `agency.auth.Credential`s.
impl<P> crate::auth::obligations::CredentialListQuery for Generated<P>
where
    P: CredentialStorage,
{
    fn credential_list(&self) -> Result<Vec<crate::auth::CredentialList>, UnmetObligation> {
        let admitted = CredentialStorage::list(&self.ports);
        Ok(admitted
            .into_iter()
            .map(|held| crate::auth::CredentialList {
                credential_id: held.data.credential_id,
                tenant_id: held.data.tenant_id,
                kind: held.data.kind,
                custody: held.data.custody,
                scope: held.data.scope,
                owner_principal_id: held.data.owner_principal_id,
                expires_at: held.data.expires_at,
                last_refreshed_at: held.data.last_refreshed_at,
                state: held.state,
            })
            .collect())
    }
}

impl<P: crate::auth::obligations::EffectivePolicyQuery> crate::auth::obligations::EffectivePolicyQuery for Generated<P> {
    fn effective_policy(&self, definition_id: crate::agents::DefinitionId) -> Result<Vec<crate::auth::EffectivePolicy>, UnmetObligation> {
        crate::auth::obligations::EffectivePolicyQuery::effective_policy(&self.ports, definition_id)
    }
}

/// `agency.auth.GrantMatrix`, generated: every row is one the specification fully determines from the stored `agency.auth.CredentialGrant`s.
impl<P> crate::auth::obligations::GrantMatrixQuery for Generated<P>
where
    P: CredentialGrantStorage,
{
    fn grant_matrix(&self) -> Result<Vec<crate::auth::GrantMatrix>, UnmetObligation> {
        let admitted = CredentialGrantStorage::list(&self.ports);
        Ok(admitted
            .into_iter()
            .map(|held| crate::auth::GrantMatrix {
                grant_id: held.data.grant_id,
                tenant_id: held.data.tenant_id,
                credential_id: held.data.credential_id,
                grantee_agent_id: held.data.grantee_agent_id,
                grantee_team_id: held.data.grantee_team_id,
                grantee_workforce_id: held.data.grantee_workforce_id,
                purpose: held.data.purpose,
                expires_at: held.data.expires_at,
                state: held.state,
            })
            .collect())
    }
}

/// `agency.auth.PolicyList`, generated: every row is one the specification fully determines from the stored `agency.auth.Policy`s.
impl<P> crate::auth::obligations::PolicyListQuery for Generated<P>
where
    P: PolicyStorage,
{
    fn policy_list(&self) -> Result<Vec<crate::auth::PolicyList>, UnmetObligation> {
        let admitted = PolicyStorage::list(&self.ports);
        Ok(admitted
            .into_iter()
            .map(|held| crate::auth::PolicyList {
                policy_id: held.data.policy_id,
                tenant_id: held.data.tenant_id,
                name: held.data.name,
                rules: held.data.rules,
                version: held.data.version,
                digest: held.data.digest,
                state: held.state,
            })
            .collect())
    }
}

/// `agency.auth.PrincipalList`, generated: every row is one the specification fully determines from the stored `agency.auth.Principal`s.
impl<P> crate::auth::obligations::PrincipalListQuery for Generated<P>
where
    P: PrincipalStorage,
{
    fn principal_list(&self) -> Result<Vec<crate::auth::PrincipalList>, UnmetObligation> {
        let admitted = PrincipalStorage::list(&self.ports);
        Ok(admitted
            .into_iter()
            .map(|held| crate::auth::PrincipalList {
                principal_id: held.data.principal_id,
                tenant_id: held.data.tenant_id,
                kind: held.data.kind,
                subject: held.data.subject,
                roles: held.data.roles,
                mandate_ref: held.data.mandate_ref,
                state: held.state,
            })
            .collect())
    }
}

/// `agency.runs.ApprovalInbox`, generated: every row is one the specification fully determines from the stored `agency.runs.RunApproval`s.
impl<P> crate::runs::obligations::ApprovalInboxQuery for Generated<P>
where
    P: RunApprovalStorage,
{
    fn approval_inbox(&self) -> Result<Vec<crate::runs::ApprovalInbox>, UnmetObligation> {
        let admitted = RunApprovalStorage::list(&self.ports);
        Ok(admitted
            .into_iter()
            .map(|held| crate::runs::ApprovalInbox {
                approval_id: held.data.approval_id,
                run_id: held.data.run_id,
                subject: held.data.subject,
                capability: held.data.capability,
                requested_at: held.data.requested_at,
                answered_by: held.data.answered_by,
                answered_at: held.data.answered_at,
                state: held.state,
            })
            .collect())
    }
}

impl<P: crate::runs::obligations::RunDetailQuery> crate::runs::obligations::RunDetailQuery for Generated<P> {
    fn run_detail(&self, run_id: crate::runs::RunId) -> Result<Vec<crate::runs::RunDetail>, UnmetObligation> {
        crate::runs::obligations::RunDetailQuery::run_detail(&self.ports, run_id)
    }
}

/// `agency.runs.RunList`, generated: every row is one the specification fully determines from the stored `agency.runs.Run`s.
impl<P> crate::runs::obligations::RunListQuery for Generated<P>
where
    P: RunStorage,
{
    fn run_list(&self) -> Result<Vec<crate::runs::RunList>, UnmetObligation> {
        let admitted = RunStorage::list(&self.ports);
        Ok(admitted
            .into_iter()
            .map(|held| crate::runs::RunList {
                run_id: held.data.run_id,
                tenant_id: held.data.tenant_id,
                agent_id: held.data.agent_id,
                work_item_ref: held.data.work_item_ref,
                budget: held.data.budget,
                workspace: held.data.workspace,
                session_id: held.data.session_id,
                credential_source: held.data.credential_source,
                started_at: held.data.started_at,
                ended_at: held.data.ended_at,
                usage: held.data.usage,
                state: held.state,
            })
            .collect())
    }
}

/// `agency.runs.UsageByAgent`, generated: every row is one the specification fully determines from the stored `agency.runs.Run`s.
impl<P> crate::runs::obligations::UsageByAgentQuery for Generated<P>
where
    P: RunStorage,
{
    fn usage_by_agent(&self) -> Result<Vec<crate::runs::UsageByAgent>, UnmetObligation> {
        let admitted = RunStorage::list(&self.ports);
        // One partition per group-key tuple, in the order its first row arrives; an absent key is
        // one value, and a number is keyed by its value, not its spelling.
        let mut groups: Vec<(_, Vec<crate::runs::RunSnapshot>)> = Vec::new();
        for held in admitted {
            let key = (Some(&held.data.agent_id).map(|value| &value.0).map(|value| value.0.clone()),);
            match groups.iter_mut().find(|(seen, _)| *seen == key) {
                Some((_, members)) => members.push(held),
                None => groups.push((key, vec![held])),
            }
        }
        let mut rows = Vec::new();
        for (_, members) in groups {
            let first = &members[0];
            rows.push(crate::runs::UsageByAgent {
                agent_id: first.data.agent_id.clone(),
                runs: count(members.len()),
            });
        }
        Ok(rows)
    }
}

/// `agency.service.SessionList`, generated: every row is one the specification fully determines from the stored `agency.service.Session`s.
impl<P> crate::service::obligations::SessionListQuery for Generated<P>
where
    P: SessionStorage,
{
    fn session_list(&self) -> Result<Vec<crate::service::SessionList>, UnmetObligation> {
        let admitted = SessionStorage::list(&self.ports);
        Ok(admitted
            .into_iter()
            .map(|held| crate::service::SessionList {
                session_id: held.data.session_id,
                principal_id: held.data.principal_id,
                kind: held.data.kind,
                run_id: held.data.run_id,
                opened_at: held.data.opened_at,
                expires_at: held.data.expires_at,
                state: held.state,
            })
            .collect())
    }
}

/// `agency.teams.SwarmList`, generated: every row is one the specification fully determines from the stored `agency.teams.Swarm`s.
impl<P> crate::teams::obligations::SwarmListQuery for Generated<P>
where
    P: SwarmStorage,
{
    fn swarm_list(&self) -> Result<Vec<crate::teams::SwarmList>, UnmetObligation> {
        let admitted = SwarmStorage::list(&self.ports);
        Ok(admitted
            .into_iter()
            .map(|held| crate::teams::SwarmList {
                swarm_id: held.data.swarm_id,
                tenant_id: held.data.tenant_id,
                goal_ref: held.data.goal_ref,
                coordinator_agent_id: held.data.coordinator_agent_id,
                frame_digest: held.data.frame_digest,
                max_workers: held.data.max_workers,
                team_id: held.data.team_id,
                work_input_id: held.data.work_input_id,
                state: held.state,
            })
            .collect())
    }
}

impl<P: crate::teams::obligations::TeamDetailQuery> crate::teams::obligations::TeamDetailQuery for Generated<P> {
    fn team_detail(&self, team_id: crate::teams::TeamId) -> Result<Vec<crate::teams::TeamDetail>, UnmetObligation> {
        crate::teams::obligations::TeamDetailQuery::team_detail(&self.ports, team_id)
    }
}

/// `agency.teams.TeamList`, generated: every row is one the specification fully determines from the stored `agency.teams.Team`s.
impl<P> crate::teams::obligations::TeamListQuery for Generated<P>
where
    P: TeamStorage,
{
    fn team_list(&self) -> Result<Vec<crate::teams::TeamList>, UnmetObligation> {
        let admitted = TeamStorage::list(&self.ports);
        Ok(admitted
            .into_iter()
            .map(|held| crate::teams::TeamList {
                team_id: held.data.team_id,
                tenant_id: held.data.tenant_id,
                name: held.data.name,
                lead_agent_id: held.data.lead_agent_id,
                members: held.data.members,
                state: held.state,
            })
            .collect())
    }
}

/// `agency.teams.WorkforceBoard`, generated: every row is one the specification fully determines from the stored `agency.teams.Workforce`s.
impl<P> crate::teams::obligations::WorkforceBoardQuery for Generated<P>
where
    P: WorkforceStorage,
{
    fn workforce_board(&self) -> Result<Vec<crate::teams::WorkforceBoard>, UnmetObligation> {
        let admitted = WorkforceStorage::list(&self.ports);
        Ok(admitted
            .into_iter()
            .map(|held| crate::teams::WorkforceBoard {
                workforce_id: held.data.workforce_id,
                tenant_id: held.data.tenant_id,
                name: held.data.name,
                teams: held.data.teams,
                solo_agents: held.data.solo_agents,
                capacity: held.data.capacity,
                state: held.state,
            })
            .collect())
    }
}

/// `agency.tools.BindingList`, generated: every row is one the specification fully determines from the stored `agency.tools.ToolBinding`s.
impl<P> crate::tools::obligations::BindingListQuery for Generated<P>
where
    P: ToolBindingStorage,
{
    fn binding_list(&self) -> Result<Vec<crate::tools::BindingList>, UnmetObligation> {
        let admitted = ToolBindingStorage::list(&self.ports);
        Ok(admitted
            .into_iter()
            .map(|held| crate::tools::BindingList {
                binding_id: held.data.binding_id,
                tenant_id: held.data.tenant_id,
                definition_id: held.data.definition_id,
                team_id: held.data.team_id,
                tool_id: held.data.tool_id,
                verbs: held.data.verbs,
                scope: held.data.scope,
                max_output_bytes: held.data.max_output_bytes,
                expires_at: held.data.expires_at,
                credential_id: held.data.credential_id,
                state: held.state,
            })
            .collect())
    }
}

/// `agency.tools.ToolCatalogue`, generated: every row is one the specification fully determines from the stored `agency.tools.Tool`s.
impl<P> crate::tools::obligations::ToolCatalogueQuery for Generated<P>
where
    P: ToolStorage,
{
    fn tool_catalogue(&self) -> Result<Vec<crate::tools::ToolCatalogue>, UnmetObligation> {
        let admitted = ToolStorage::list(&self.ports);
        Ok(admitted
            .into_iter()
            .map(|held| crate::tools::ToolCatalogue {
                tool_id: held.data.tool_id,
                tenant_id: held.data.tenant_id,
                contract_ref: held.data.contract_ref,
                verbs: held.data.verbs,
                risk: held.data.risk,
                implementation_ref: held.data.implementation_ref,
                connector_instance_id: held.data.connector_instance_id,
                state: held.state,
            })
            .collect())
    }
}

/// `agency.work.WorkInputList`, generated: every row is one the specification fully determines from the stored `agency.work.WorkInput`s.
impl<P> crate::work::obligations::WorkInputListQuery for Generated<P>
where
    P: WorkInputStorage,
{
    fn work_input_list(&self) -> Result<Vec<crate::work::WorkInputList>, UnmetObligation> {
        let admitted = WorkInputStorage::list(&self.ports);
        Ok(admitted
            .into_iter()
            .map(|held| crate::work::WorkInputList {
                input_id: held.data.input_id,
                tenant_id: held.data.tenant_id,
                name: held.data.name,
                source: held.data.source,
                poll_interval: held.data.poll_interval,
                workforce_id: held.data.workforce_id,
                transform_ref: held.data.transform_ref,
                state: held.state,
            })
            .collect())
    }
}

impl<P: crate::work::obligations::WorkItemDetailQuery> crate::work::obligations::WorkItemDetailQuery for Generated<P> {
    fn work_item_detail(&self, item_ref: crate::work::WorkItemRef) -> Result<Vec<crate::work::WorkItemDetail>, UnmetObligation> {
        crate::work::obligations::WorkItemDetailQuery::work_item_detail(&self.ports, item_ref)
    }
}

impl<P: crate::work::obligations::WorkQueueQuery> crate::work::obligations::WorkQueueQuery for Generated<P> {
    fn work_queue(&self) -> Result<Vec<crate::work::WorkQueue>, UnmetObligation> {
        crate::work::obligations::WorkQueueQuery::work_queue(&self.ports)
    }
}

/// The typed refusal of a request the model declares no outcome for.
fn undeclared(source: &'static str) -> UnmetObligation {
    let capability = "command behaviour";
    UnmetObligation { capability, source }
}

/// A guard's truth, where it has one: Unknown selects no branch, so the model declares no outcome.
fn decided(truth: Option<bool>, command: &'static str) -> Result<bool, UnmetObligation> {
    truth.ok_or_else(|| undeclared(command))
}

/// Three-valued conjunction: false wins, then Unknown.
fn all(truths: &[Option<bool>]) -> Option<bool> {
    if truths.contains(&Some(false)) {
        Some(false)
    } else if truths.contains(&None) {
        None
    } else {
        Some(true)
    }
}

/// Three-valued disjunction: true wins, then Unknown.
fn any(truths: &[Option<bool>]) -> Option<bool> {
    if truths.contains(&Some(true)) {
        Some(true)
    } else if truths.contains(&None) {
        None
    } else {
        Some(false)
    }
}

/// Equality of two read values; an unread one is Unknown.
fn equal<T: PartialEq>(left: Option<T>, right: Option<T>) -> Option<bool> {
    Some(left? == right?)
}

/// A number of rows, as the `Integer` it is reported as.
fn count(rows: usize) -> i64 {
    i64::try_from(rows).unwrap_or(i64::MAX)
}
