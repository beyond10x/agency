//! The ports the generated behaviours run over: the generated in-memory storage, a context that
//! answers what the specification leaves to the implementation, and the five view queries
//! `generated/rust/agency/PLAN.md` lists as obligations.
//!
//! Storage is `agency::server::memory::MemoryPorts`, the generated ephemeral store, unchanged:
//! every storage call is forwarded to it. The generated context of `MemoryPorts` cannot be used as
//! it is, because it answers no external branch and assigns no `Integer` or `Digest`
//! (`generated/rust/agency/src/server/memory.rs`, its `TryContext` impl), so [`TargetPorts`] answers
//! the context itself:
//!
//! * an assigned identity is a canonical UUID numbered within the scenario, under a prefix no suite
//!   literal uses;
//! * a `Timestamp`, and an `Optional<Timestamp>`, is the wall clock's reading now;
//! * the only `Integer` the model leaves to the implementation is a created definition's or
//!   policy's `version` (`behaviour.rs`, `CreateAgentDefinition` and `CreatePolicy`), answered `1`;
//! * a `Digest` is `sha256:` and 64 hex digits, numbered within the scenario;
//! * an `external:` branch is taken only when the scenario forced it, for the next invocation of
//!   its command ([`Control::force`]).
//!
//! None of these decides an outcome: each is a value the generated behaviour asks for and then
//! decides with.

use std::cell::RefCell;
use std::rc::Rc;

use agency::behaviour::{
    AgentDefinitionStorage, AgentStorage, CredentialGrantStorage, CredentialStorage,
    ExternalCommand, PolicyStorage, PrincipalStorage, RunApprovalStorage, RunStorage,
    SessionStorage, SwarmStorage, TeamStorage, TenantStorage, ToolBindingStorage, ToolStorage,
    TryContext, WorkInputStorage, WorkItemStorage, WorkforceStorage,
};
use agency::obligation::UnmetObligation;
use agency::server::memory::MemoryPorts;
use agency::{agents, auth, primitives, runs, service, shared, teams, tools, work};

/// What a scenario controls in the context, shared between the target and the ports.
#[derive(Debug, Default)]
pub struct Control {
    /// The `(command, outcome)` external branch forced for the next invocation of `command`.
    forced: Option<(String, String)>,
    /// The values assigned so far in this scenario, which number the next one.
    assigned: u64,
}

impl Control {
    /// Takes the external branch `outcome` of `command` on its next invocation.
    pub fn force(&mut self, command: String, outcome: String) {
        self.forced = Some((command, outcome));
    }

    /// Lets a forced branch of `command` lapse, after `command` was invoked once.
    pub fn invoked(&mut self, command: &str) {
        if self
            .forced
            .as_ref()
            .is_some_and(|(forced, _)| forced == command)
        {
            self.forced = None;
        }
    }

    fn next(&mut self) -> u64 {
        self.assigned += 1;
        self.assigned
    }
}

/// The ports of one scenario's system.
pub struct TargetPorts {
    memory: MemoryPorts,
    work_items: Vec<work::WorkItemSnapshot>,
    control: Rc<RefCell<Control>>,
}

impl TargetPorts {
    /// Empty storage, and a context controlled through `control`.
    pub fn new(control: Rc<RefCell<Control>>) -> Self {
        Self {
            memory: MemoryPorts::default(),
            work_items: Vec::new(),
            control,
        }
    }

    fn uuid(&mut self) -> primitives::Uuid {
        let number = self.control.borrow_mut().next();
        primitives::Uuid(format!("7a5e7a90-0000-4000-8000-{number:012x}"))
    }
}

fn now() -> primitives::Timestamp {
    let now = time::OffsetDateTime::now_utc();
    primitives::Timestamp(
        now.format(&time::format_description::well_known::Rfc3339)
            .expect("a UTC instant formats as RFC 3339"),
    )
}

/// Forwards one entity's storage port to `MemoryPorts`; `without list` for a port that declares
/// no `list` (an entity no generated query reads).
macro_rules! storage {
    (@by_identity $trait:ident, $identity:ty, $snapshot:ty) => {
        fn get(&self, identity: &$identity) -> Option<$snapshot> {
            $trait::get(&self.memory, identity)
        }
        fn put(&mut self, snapshot: $snapshot) {
            $trait::put(&mut self.memory, snapshot)
        }
        fn delete(&mut self, identity: &$identity) {
            $trait::delete(&mut self.memory, identity)
        }
    };
    ($trait:ident, $identity:ty, $snapshot:ty, without list) => {
        impl $trait for TargetPorts {
            storage!(@by_identity $trait, $identity, $snapshot);
        }
    };
    ($trait:ident, $identity:ty, $snapshot:ty) => {
        impl $trait for TargetPorts {
            storage!(@by_identity $trait, $identity, $snapshot);
            fn list(&self) -> Vec<$snapshot> {
                $trait::list(&self.memory)
            }
        }
    };
}

storage!(AgentStorage, agents::AgentId, agents::AgentSnapshot);
storage!(
    AgentDefinitionStorage,
    agents::DefinitionId,
    agents::AgentDefinitionSnapshot
);
storage!(
    CredentialStorage,
    auth::CredentialId,
    auth::CredentialSnapshot
);
storage!(
    CredentialGrantStorage,
    auth::GrantId,
    auth::CredentialGrantSnapshot
);
storage!(PolicyStorage, auth::PolicyId, auth::PolicySnapshot);
storage!(PrincipalStorage, auth::PrincipalId, auth::PrincipalSnapshot);
storage!(TenantStorage, auth::TenantId, auth::TenantSnapshot, without list);
storage!(RunStorage, runs::RunId, runs::RunSnapshot);
storage!(
    RunApprovalStorage,
    runs::ApprovalId,
    runs::RunApprovalSnapshot
);
storage!(SessionStorage, service::SessionId, service::SessionSnapshot);
storage!(SwarmStorage, teams::SwarmId, teams::SwarmSnapshot);
storage!(TeamStorage, teams::TeamId, teams::TeamSnapshot);
storage!(
    WorkforceStorage,
    teams::WorkforceId,
    teams::WorkforceSnapshot
);
storage!(ToolStorage, tools::ToolId, tools::ToolSnapshot);
storage!(
    ToolBindingStorage,
    tools::BindingId,
    tools::ToolBindingSnapshot
);
storage!(WorkInputStorage, work::InputId, work::WorkInputSnapshot);

/// `agency.work.WorkItem`, kept here rather than in `MemoryPorts`: its generated port declares no
/// `list`, because no generated query reads it, and the two obligations `WorkItemDetail` and
/// `WorkQueue` read every item. Ordered by `item_ref`, as `MemoryPorts` orders by identity.
impl WorkItemStorage for TargetPorts {
    fn get(&self, identity: &work::WorkItemRef) -> Option<work::WorkItemSnapshot> {
        self.work_items
            .iter()
            .find(|held| held.data.item_ref == *identity)
            .cloned()
    }
    fn put(&mut self, snapshot: work::WorkItemSnapshot) {
        WorkItemStorage::delete(self, &snapshot.data.item_ref);
        self.work_items.push(snapshot);
        self.work_items
            .sort_by(|held, other| held.data.item_ref.0.cmp(&other.data.item_ref.0));
    }
    fn delete(&mut self, identity: &work::WorkItemRef) {
        self.work_items
            .retain(|held| held.data.item_ref != *identity);
    }
}

type Assigned<T> = Result<T, UnmetObligation>;

impl TryContext for TargetPorts {
    fn try_generate_agency_agents_agent_id(&mut self) -> Assigned<agents::AgentId> {
        Ok(agents::AgentId(self.uuid()))
    }
    fn try_generate_agency_agents_definition_id(&mut self) -> Assigned<agents::DefinitionId> {
        Ok(agents::DefinitionId(self.uuid()))
    }
    fn try_generate_agency_auth_credential_id(&mut self) -> Assigned<auth::CredentialId> {
        Ok(auth::CredentialId(self.uuid()))
    }
    fn try_generate_agency_auth_grant_id(&mut self) -> Assigned<auth::GrantId> {
        Ok(auth::GrantId(self.uuid()))
    }
    fn try_generate_agency_auth_policy_id(&mut self) -> Assigned<auth::PolicyId> {
        Ok(auth::PolicyId(self.uuid()))
    }
    fn try_generate_agency_auth_principal_id(&mut self) -> Assigned<auth::PrincipalId> {
        Ok(auth::PrincipalId(self.uuid()))
    }
    fn try_generate_agency_auth_tenant_id(&mut self) -> Assigned<auth::TenantId> {
        Ok(auth::TenantId(self.uuid()))
    }
    fn try_generate_agency_runs_approval_id(&mut self) -> Assigned<runs::ApprovalId> {
        Ok(runs::ApprovalId(self.uuid()))
    }
    fn try_generate_agency_runs_run_id(&mut self) -> Assigned<runs::RunId> {
        Ok(runs::RunId(self.uuid()))
    }
    fn try_generate_agency_service_session_id(&mut self) -> Assigned<service::SessionId> {
        Ok(service::SessionId(self.uuid()))
    }
    fn try_generate_agency_shared_digest(&mut self) -> Assigned<shared::Digest> {
        let number = self.control.borrow_mut().next();
        Ok(shared::Digest(format!("sha256:{number:064x}")))
    }
    fn try_generate_agency_teams_swarm_id(&mut self) -> Assigned<teams::SwarmId> {
        Ok(teams::SwarmId(self.uuid()))
    }
    fn try_generate_agency_teams_team_id(&mut self) -> Assigned<teams::TeamId> {
        Ok(teams::TeamId(self.uuid()))
    }
    fn try_generate_agency_teams_workforce_id(&mut self) -> Assigned<teams::WorkforceId> {
        Ok(teams::WorkforceId(self.uuid()))
    }
    fn try_generate_agency_tools_binding_id(&mut self) -> Assigned<tools::BindingId> {
        Ok(tools::BindingId(self.uuid()))
    }
    fn try_generate_agency_tools_tool_id(&mut self) -> Assigned<tools::ToolId> {
        Ok(tools::ToolId(self.uuid()))
    }
    fn try_generate_agency_work_input_id(&mut self) -> Assigned<work::InputId> {
        Ok(work::InputId(self.uuid()))
    }
    fn try_generate_integer(&mut self) -> Assigned<i64> {
        Ok(1)
    }
    fn try_generate_optional_timestamp(&mut self) -> Assigned<Option<primitives::Timestamp>> {
        Ok(Some(now()))
    }
    fn try_generate_timestamp(&mut self) -> Assigned<primitives::Timestamp> {
        Ok(now())
    }
    fn try_external(
        &mut self,
        command: ExternalCommand<'_>,
        outcome: &'static str,
    ) -> Assigned<bool> {
        let control = self.control.borrow();
        Ok(control
            .forced
            .as_ref()
            .is_some_and(|(forced, branch)| forced == command.name() && branch == outcome))
    }
}

/// `agency.auth.EffectivePolicy`: the definition's row, its policy fields read through
/// `policy_id`, each absent where no policy is stored under it.
impl auth::obligations::EffectivePolicyQuery for TargetPorts {
    fn effective_policy(
        &self,
        definition_id: agents::DefinitionId,
    ) -> Result<Vec<auth::EffectivePolicy>, UnmetObligation> {
        Ok(AgentDefinitionStorage::list(&self.memory)
            .into_iter()
            .filter(|held| held.data.definition_id == definition_id)
            .map(|held| {
                let policy = PolicyStorage::get(&self.memory, &held.data.policy_id);
                auth::EffectivePolicy {
                    definition_id: held.data.definition_id,
                    policy_id: held.data.policy_id,
                    policy_version: policy.as_ref().map(|policy| policy.data.version),
                    policy_digest: policy.as_ref().map(|policy| policy.data.digest.clone()),
                    rules: policy.as_ref().map(|policy| policy.data.rules.clone()),
                    policy_state: policy.map(|policy| policy.state),
                }
            })
            .collect())
    }
}

/// `agency.runs.RunDetail`: the run whose `run_id` is the parameter.
impl runs::obligations::RunDetailQuery for TargetPorts {
    fn run_detail(&self, run_id: runs::RunId) -> Result<Vec<runs::RunDetail>, UnmetObligation> {
        Ok(RunStorage::list(&self.memory)
            .into_iter()
            .filter(|held| held.data.run_id == run_id)
            .map(|held| runs::RunDetail {
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
                outcome: held.data.outcome,
                usage: held.data.usage,
                state: held.state,
            })
            .collect())
    }
}

/// `agency.teams.TeamDetail`: the team whose `team_id` is the parameter.
impl teams::obligations::TeamDetailQuery for TargetPorts {
    fn team_detail(
        &self,
        team_id: teams::TeamId,
    ) -> Result<Vec<teams::TeamDetail>, UnmetObligation> {
        Ok(TeamStorage::list(&self.memory)
            .into_iter()
            .filter(|held| held.data.team_id == team_id)
            .map(|held| teams::TeamDetail {
                team_id: held.data.team_id,
                tenant_id: held.data.tenant_id,
                name: held.data.name,
                lead_agent_id: held.data.lead_agent_id,
                members: held.data.members,
                broker_ref: held.data.broker_ref,
                state: held.state,
            })
            .collect())
    }
}

/// `agency.work.WorkItemDetail`: the item whose `item_ref` is the parameter.
impl work::obligations::WorkItemDetailQuery for TargetPorts {
    fn work_item_detail(
        &self,
        item_ref: work::WorkItemRef,
    ) -> Result<Vec<work::WorkItemDetail>, UnmetObligation> {
        Ok(self
            .work_items
            .clone()
            .into_iter()
            .filter(|held| held.data.item_ref == item_ref)
            .map(|held| work::WorkItemDetail {
                item_ref: held.data.item_ref,
                tenant_id: held.data.tenant_id,
                input_id: held.data.input_id,
                title: held.data.title,
                payload_digest: held.data.payload_digest,
                assigned_agent_id: held.data.assigned_agent_id,
                received_at: held.data.received_at,
                settled_at: held.data.settled_at,
                state: held.state,
            })
            .collect())
    }
}

/// `agency.work.WorkQueue`: every item, its `workforce_id` read through `input_id`, absent where
/// no input is stored under it.
impl work::obligations::WorkQueueQuery for TargetPorts {
    fn work_queue(&self) -> Result<Vec<work::WorkQueue>, UnmetObligation> {
        Ok(self
            .work_items
            .clone()
            .into_iter()
            .map(|held| {
                let workforce_id = WorkInputStorage::get(&self.memory, &held.data.input_id)
                    .map(|input| input.data.workforce_id);
                work::WorkQueue {
                    item_ref: held.data.item_ref,
                    input_id: held.data.input_id,
                    workforce_id,
                    title: held.data.title,
                    assigned_agent_id: held.data.assigned_agent_id,
                    received_at: held.data.received_at,
                    state: held.state,
                }
            })
            .collect())
    }
}
