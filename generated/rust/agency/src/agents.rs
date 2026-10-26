// generated from agency v1
// model digest 71132e90bde806415603d08bb97a71629b2c2f6cda30e1f22e76f407abd9a3eb
// contract digest a96f55e3e1ca68aeb1579b032c3547fffd27e8ab80e76bcb898f9b6c9258792e
// do not edit: regenerate with `ess synthesize --layout crate`

//! Agents — `agency.agents`.
//!
//! Agent definitions an operator publishes, each a versioned record of one Loom `loom.agent/1` document, and the agents provisioned from them.
//!
//! Everything this bounded context declares that the synthesis plan marks generated.

/// The states of `agency.agents.Agent`, as runtime values.
///
/// Synthesised from the lifecycle, so the two cannot disagree. Which *moves* are legal is not
/// carried here — it is carried by `Agent<S>`, where an undeclared move does not compile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentState {
    /// `Provisioned`.
    Provisioned,
    /// `Retired`.
    Retired,
}

/// The states of `agency.agents.AgentDefinition`, as runtime values.
///
/// Synthesised from the lifecycle, so the two cannot disagree. Which *moves* are legal is not
/// carried here — it is carried by `AgentDefinition<S>`, where an undeclared move does not compile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentDefinitionState {
    /// `Checked`.
    Checked,
    /// `Deprecated`.
    Deprecated,
    /// `Draft`.
    Draft,
    /// `Published`.
    Published,
}

/// AgentId — `agency.agents.AgentId`: a distinct wrapper around `Uuid`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentId(pub crate::primitives::Uuid);

/// AgentSpecDocument — `agency.agents.AgentSpecDocument`: a distinct wrapper around `Json`.
///
/// One loom.agent/1 document, held by value; its schema is Loom's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentSpecDocument(pub crate::json::Value);

/// CheckVerdict — `agency.agents.CheckVerdict`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckVerdict {
    /// `Accepted`.
    Accepted,
    /// `Refused`.
    Refused,
}

/// DefinitionId — `agency.agents.DefinitionId`: a distinct wrapper around `Uuid`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefinitionId(pub crate::primitives::Uuid);

/// What Agent — `agency.agents.Agent` — holds, apart from where it is in its lifecycle.
///
/// The identity and every declared field. The state is deliberately not one: inside the domain it
/// is carried by the type parameter of [`Agent<S>`], and at a boundary by [`AgentSnapshot::state`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentData {
    /// The identity: `agent_id` — `agency.agents.AgentId`.
    pub agent_id: AgentId,
    /// `tenant_id` — `agency.auth.TenantId`.
    ///
    /// Carries `agents`: `agency.auth.Tenant` owns many `agency.agents.Agent`.
    pub tenant_id: crate::auth::TenantId,
    /// `definition_id` — `agency.agents.DefinitionId`.
    ///
    /// Carries `definition`: `agency.agents.Agent` references one `agency.agents.AgentDefinition`.
    pub definition_id: DefinitionId,
    /// `definition_version` — `Integer`.
    pub definition_version: i64,
    /// `display_name` — `String`.
    pub display_name: String,
    /// `principal_id` — `agency.auth.PrincipalId`.
    ///
    /// Carries `principal`: `agency.agents.Agent` references one `agency.auth.Principal`.
    pub principal_id: crate::auth::PrincipalId,
    /// `current_run_id` — `Optional<agency.runs.RunId>`.
    ///
    /// Carries `current_run`: `agency.agents.Agent` references one `agency.runs.Run`.
    pub current_run_id: Option<crate::runs::RunId>,
    /// `last_seen_at` — `Timestamp`.
    pub last_seen_at: crate::primitives::Timestamp,
}

/// The states of `agency.agents.Agent`, at the type level.
///
/// One marker type per declared state, sealed: a state the lifecycle does not declare cannot
/// implement [`Marker`](agent_state::Marker), so [`Agent<S>`](Agent) can only ever rest in a real state.
pub mod agent_state {
    /// Closes [`Marker`] over the declared states.
    mod sealed {
        /// Implemented only by the marker types beside this module.
        pub trait Sealed {}
        impl Sealed for super::Provisioned {}
        impl Sealed for super::Retired {}
    }

    /// A declared state of `Agent`, as a type.
    pub trait Marker: sealed::Sealed {
        /// The same state, as the runtime value.
        const STATE: super::AgentState;
    }

    /// `Provisioned`. Where a new instance starts.
    pub struct Provisioned;

    impl Marker for Provisioned {
        const STATE: super::AgentState = super::AgentState::Provisioned;
    }

    /// `Retired`. Terminal: an instance may rest here forever.
    pub struct Retired;

    impl Marker for Retired {
        const STATE: super::AgentState = super::AgentState::Retired;
    }
}

/// Agent — `agency.agents.Agent` — with its lifecycle state carried by the type.
///
/// The one constructor rests in `Provisioned`, and the only way to change `S` is a method generated from
/// a declared transition. A move the specification does not declare is therefore not an error
/// case: it does not compile. Where the state is data — wire, storage — use [`AgentSnapshot`]
/// and [`AgentSnapshot::refine`].
pub struct Agent<S: agent_state::Marker> {
    data: AgentData,
    state: core::marker::PhantomData<S>,
}

impl<S: agent_state::Marker> Agent<S> {
    /// The state this instance rests in, as the runtime value.
    pub fn state(&self) -> AgentState {
        S::STATE
    }

    /// What it holds.
    pub fn data(&self) -> &AgentData {
        &self.data
    }

    /// Hands the data back, giving up the typed state.
    pub fn into_data(self) -> AgentData {
        self.data
    }
}

impl Agent<agent_state::Provisioned> {
    /// A new instance, resting in `Provisioned` — the only state the lifecycle starts one in.
    pub fn new(data: AgentData) -> Self {
        Self {
            data,
            state: core::marker::PhantomData,
        }
    }
}

impl Agent<agent_state::Provisioned> {
    /// `retire` — `Provisioned` → `Retired`. Taken by the `retired` outcome of `agency.agents.RetireAgent`.
    pub fn retire(self) -> Agent<agent_state::Retired> {
        Agent {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }
}

/// `agency.agents.Agent` as it crosses a boundary: the state as a value beside the data.
///
/// Wire and storage know states only at runtime; [`AgentSnapshot::refine`] is the one door back
/// into the typed lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentSnapshot {
    /// Where the instance is in its lifecycle.
    pub state: AgentState,
    /// What it holds.
    pub data: AgentData,
}

/// An `Agent` in whichever declared state it was found.
pub enum AnyAgent {
    /// Resting in `Provisioned`.
    Provisioned(Agent<agent_state::Provisioned>),
    /// Resting in `Retired`.
    Retired(Agent<agent_state::Retired>),
}

impl AgentSnapshot {
    /// Refines the runtime state into the typed one.
    ///
    /// Total: every declared state has an arm, and an undeclared state cannot reach here because
    /// `AgentState` cannot spell one.
    pub fn refine(self) -> AnyAgent {
        match self.state {
            AgentState::Provisioned => AnyAgent::Provisioned(Agent {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            AgentState::Retired => AnyAgent::Retired(Agent {
                data: self.data,
                state: core::marker::PhantomData,
            }),
        }
    }
}

impl AnyAgent {
    /// The state, as the runtime value.
    pub fn state(&self) -> AgentState {
        match self {
            Self::Provisioned(_) => AgentState::Provisioned,
            Self::Retired(_) => AgentState::Retired,
        }
    }

    /// Back to the boundary shape.
    pub fn snapshot(self) -> AgentSnapshot {
        match self {
            Self::Provisioned(instance) => AgentSnapshot {
                state: AgentState::Provisioned,
                data: instance.into_data(),
            },
            Self::Retired(instance) => AgentSnapshot {
                state: AgentState::Retired,
                data: instance.into_data(),
            },
        }
    }
}

/// What AgentDefinition — `agency.agents.AgentDefinition` — holds, apart from where it is in its lifecycle.
///
/// The identity and every declared field. The state is deliberately not one: inside the domain it
/// is carried by the type parameter of [`AgentDefinition<S>`], and at a boundary by [`AgentDefinitionSnapshot::state`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentDefinitionData {
    /// The identity: `definition_id` — `agency.agents.DefinitionId`.
    pub definition_id: DefinitionId,
    /// `tenant_id` — `agency.auth.TenantId`.
    ///
    /// Carries `agent_definitions`: `agency.auth.Tenant` owns many `agency.agents.AgentDefinition`.
    pub tenant_id: crate::auth::TenantId,
    /// `name` — `String`.
    pub name: String,
    /// `description` — `String`.
    pub description: String,
    /// `agent_spec` — `agency.agents.AgentSpecDocument`.
    pub agent_spec: AgentSpecDocument,
    /// `policy_id` — `agency.auth.PolicyId`.
    ///
    /// Carries `policy`: `agency.agents.AgentDefinition` references one `agency.auth.Policy`.
    pub policy_id: crate::auth::PolicyId,
    /// `version` — `Integer`.
    pub version: i64,
    /// `digest` — `agency.shared.Digest`.
    pub digest: crate::shared::Digest,
    /// `created_by` — `agency.auth.PrincipalId`.
    ///
    /// Carries `creator`: `agency.agents.AgentDefinition` references one `agency.auth.Principal`.
    pub created_by: crate::auth::PrincipalId,
    /// `check_exit_code` — `Optional<Integer>`.
    pub check_exit_code: Option<i64>,
    /// `check_message` — `Optional<String>`.
    pub check_message: Option<String>,
}

/// The states of `agency.agents.AgentDefinition`, at the type level.
///
/// One marker type per declared state, sealed: a state the lifecycle does not declare cannot
/// implement [`Marker`](agent_definition_state::Marker), so [`AgentDefinition<S>`](AgentDefinition) can only ever rest in a real state.
pub mod agent_definition_state {
    /// Closes [`Marker`] over the declared states.
    mod sealed {
        /// Implemented only by the marker types beside this module.
        pub trait Sealed {}
        impl Sealed for super::Checked {}
        impl Sealed for super::Deprecated {}
        impl Sealed for super::Draft {}
        impl Sealed for super::Published {}
    }

    /// A declared state of `AgentDefinition`, as a type.
    pub trait Marker: sealed::Sealed {
        /// The same state, as the runtime value.
        const STATE: super::AgentDefinitionState;
    }

    /// `Checked`.
    pub struct Checked;

    impl Marker for Checked {
        const STATE: super::AgentDefinitionState = super::AgentDefinitionState::Checked;
    }

    /// `Deprecated`. Terminal: an instance may rest here forever.
    pub struct Deprecated;

    impl Marker for Deprecated {
        const STATE: super::AgentDefinitionState = super::AgentDefinitionState::Deprecated;
    }

    /// `Draft`. Where a new instance starts.
    pub struct Draft;

    impl Marker for Draft {
        const STATE: super::AgentDefinitionState = super::AgentDefinitionState::Draft;
    }

    /// `Published`.
    pub struct Published;

    impl Marker for Published {
        const STATE: super::AgentDefinitionState = super::AgentDefinitionState::Published;
    }
}

/// AgentDefinition — `agency.agents.AgentDefinition` — with its lifecycle state carried by the type.
///
/// The one constructor rests in `Draft`, and the only way to change `S` is a method generated from
/// a declared transition. A move the specification does not declare is therefore not an error
/// case: it does not compile. Where the state is data — wire, storage — use [`AgentDefinitionSnapshot`]
/// and [`AgentDefinitionSnapshot::refine`].
pub struct AgentDefinition<S: agent_definition_state::Marker> {
    data: AgentDefinitionData,
    state: core::marker::PhantomData<S>,
}

impl<S: agent_definition_state::Marker> AgentDefinition<S> {
    /// The state this instance rests in, as the runtime value.
    pub fn state(&self) -> AgentDefinitionState {
        S::STATE
    }

    /// What it holds.
    pub fn data(&self) -> &AgentDefinitionData {
        &self.data
    }

    /// Hands the data back, giving up the typed state.
    pub fn into_data(self) -> AgentDefinitionData {
        self.data
    }
}

impl AgentDefinition<agent_definition_state::Draft> {
    /// A new instance, resting in `Draft` — the only state the lifecycle starts one in.
    pub fn new(data: AgentDefinitionData) -> Self {
        Self {
            data,
            state: core::marker::PhantomData,
        }
    }
}

impl AgentDefinition<agent_definition_state::Checked> {
    /// `publish` — `Checked` → `Published`. Taken by the `published` outcome of `agency.agents.PublishAgentDefinition`.
    pub fn publish(self) -> AgentDefinition<agent_definition_state::Published> {
        AgentDefinition {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }
}

impl AgentDefinition<agent_definition_state::Draft> {
    /// `check` — `Draft` → `Checked`. Taken by the `checked` outcome of `agency.agents.RecordSpecCheck`.
    pub fn check(self) -> AgentDefinition<agent_definition_state::Checked> {
        AgentDefinition {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }

    /// `refuse` — `Draft` → `Draft`. Taken by the `refused` outcome of `agency.agents.RecordSpecCheck`.
    pub fn refuse(self) -> AgentDefinition<agent_definition_state::Draft> {
        AgentDefinition {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }
}

impl AgentDefinition<agent_definition_state::Published> {
    /// `deprecate` — `Published` → `Deprecated`. Taken by the `deprecated` outcome of `agency.agents.DeprecateAgentDefinition`.
    pub fn deprecate(self) -> AgentDefinition<agent_definition_state::Deprecated> {
        AgentDefinition {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }
}

/// `agency.agents.AgentDefinition` as it crosses a boundary: the state as a value beside the data.
///
/// Wire and storage know states only at runtime; [`AgentDefinitionSnapshot::refine`] is the one door back
/// into the typed lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentDefinitionSnapshot {
    /// Where the instance is in its lifecycle.
    pub state: AgentDefinitionState,
    /// What it holds.
    pub data: AgentDefinitionData,
}

/// An `AgentDefinition` in whichever declared state it was found.
pub enum AnyAgentDefinition {
    /// Resting in `Checked`.
    Checked(AgentDefinition<agent_definition_state::Checked>),
    /// Resting in `Deprecated`.
    Deprecated(AgentDefinition<agent_definition_state::Deprecated>),
    /// Resting in `Draft`.
    Draft(AgentDefinition<agent_definition_state::Draft>),
    /// Resting in `Published`.
    Published(AgentDefinition<agent_definition_state::Published>),
}

impl AgentDefinitionSnapshot {
    /// Refines the runtime state into the typed one.
    ///
    /// Total: every declared state has an arm, and an undeclared state cannot reach here because
    /// `AgentDefinitionState` cannot spell one.
    pub fn refine(self) -> AnyAgentDefinition {
        match self.state {
            AgentDefinitionState::Checked => AnyAgentDefinition::Checked(AgentDefinition {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            AgentDefinitionState::Deprecated => AnyAgentDefinition::Deprecated(AgentDefinition {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            AgentDefinitionState::Draft => AnyAgentDefinition::Draft(AgentDefinition {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            AgentDefinitionState::Published => AnyAgentDefinition::Published(AgentDefinition {
                data: self.data,
                state: core::marker::PhantomData,
            }),
        }
    }
}

impl AnyAgentDefinition {
    /// The state, as the runtime value.
    pub fn state(&self) -> AgentDefinitionState {
        match self {
            Self::Checked(_) => AgentDefinitionState::Checked,
            Self::Deprecated(_) => AgentDefinitionState::Deprecated,
            Self::Draft(_) => AgentDefinitionState::Draft,
            Self::Published(_) => AgentDefinitionState::Published,
        }
    }

    /// Back to the boundary shape.
    pub fn snapshot(self) -> AgentDefinitionSnapshot {
        match self {
            Self::Checked(instance) => AgentDefinitionSnapshot {
                state: AgentDefinitionState::Checked,
                data: instance.into_data(),
            },
            Self::Deprecated(instance) => AgentDefinitionSnapshot {
                state: AgentDefinitionState::Deprecated,
                data: instance.into_data(),
            },
            Self::Draft(instance) => AgentDefinitionSnapshot {
                state: AgentDefinitionState::Draft,
                data: instance.into_data(),
            },
            Self::Published(instance) => AgentDefinitionSnapshot {
                state: AgentDefinitionState::Published,
                data: instance.into_data(),
            },
        }
    }
}

/// Create an agent definition — the input of `agency.agents.CreateAgentDefinition`.
///
/// Everything it can result in is [`CreateAgentDefinitionOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateAgentDefinition {
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: crate::auth::TenantId,
    /// `name` — `String`.
    pub name: String,
    /// `description` — `String`.
    pub description: String,
    /// `agent_spec` — `agency.agents.AgentSpecDocument`.
    pub agent_spec: AgentSpecDocument,
    /// `policy_id` — `agency.auth.PolicyId`.
    pub policy_id: crate::auth::PolicyId,
    /// `created_by` — `agency.auth.PrincipalId`.
    pub created_by: crate::auth::PrincipalId,
}

/// Everything `agency.agents.CreateAgentDefinition` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreateAgentDefinitionOutcome {
    /// `created` — otherwise.
    Created {
        /// The `agency.agents.AgentDefinitionCreated` this outcome publishes.
        agent_definition_created: AgentDefinitionCreated,
    },
}

/// Deprecate an agent definition — the input of `agency.agents.DeprecateAgentDefinition`.
///
/// Everything it can result in is [`DeprecateAgentDefinitionOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeprecateAgentDefinition {
    /// `definition_id` — `agency.agents.DefinitionId`.
    pub definition_id: DefinitionId,
}

/// Everything `agency.agents.DeprecateAgentDefinition` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeprecateAgentDefinitionOutcome {
    /// `no-such-definition` — for an identity no record carries.
    NoSuchDefinition {
        /// Why it was refused: `agency.agents.NotFound`.
        error: NotFound,
    },
    /// `deprecated` — otherwise.
    Deprecated {
        /// The `agency.agents.AgentDefinitionDeprecated` this outcome publishes.
        agent_definition_deprecated: AgentDefinitionDeprecated,
    },
    /// `wrong-state` — from a state no declared move starts in.
    WrongState {
        /// Why it was refused: `agency.agents.StateConflict`.
        error: StateConflict,
    },
}

/// Provision an agent — the input of `agency.agents.ProvisionAgent`.
///
/// Everything it can result in is [`ProvisionAgentOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvisionAgent {
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: crate::auth::TenantId,
    /// `definition_id` — `agency.agents.DefinitionId`.
    pub definition_id: DefinitionId,
    /// `definition_version` — `Integer`.
    pub definition_version: i64,
    /// `display_name` — `String`.
    pub display_name: String,
    /// `principal_id` — `agency.auth.PrincipalId`.
    pub principal_id: crate::auth::PrincipalId,
}

/// Everything `agency.agents.ProvisionAgent` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProvisionAgentOutcome {
    /// `provisioned` — otherwise.
    Provisioned {
        /// The `agency.agents.AgentProvisioned` this outcome publishes.
        agent_provisioned: AgentProvisioned,
    },
}

/// Publish an agent definition — the input of `agency.agents.PublishAgentDefinition`.
///
/// Everything it can result in is [`PublishAgentDefinitionOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishAgentDefinition {
    /// `definition_id` — `agency.agents.DefinitionId`.
    pub definition_id: DefinitionId,
}

/// Everything `agency.agents.PublishAgentDefinition` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublishAgentDefinitionOutcome {
    /// `no-such-definition` — for an identity no record carries.
    NoSuchDefinition {
        /// Why it was refused: `agency.agents.NotFound`.
        error: NotFound,
    },
    /// `published` — otherwise.
    Published {
        /// The `agency.agents.AgentDefinitionPublished` this outcome publishes.
        agent_definition_published: AgentDefinitionPublished,
    },
    /// `wrong-state` — from a state no declared move starts in.
    WrongState {
        /// Why it was refused: `agency.agents.StateConflict`.
        error: StateConflict,
    },
}

/// Record Loom's check of a definition's document — the input of `agency.agents.RecordSpecCheck`.
///
/// Everything it can result in is [`RecordSpecCheckOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordSpecCheck {
    /// `definition_id` — `agency.agents.DefinitionId`.
    pub definition_id: DefinitionId,
    /// `exit_code` — `Integer`.
    pub exit_code: i64,
    /// `verdict` — `agency.agents.CheckVerdict`.
    pub verdict: CheckVerdict,
    /// `message` — `String`.
    pub message: String,
}

/// Everything `agency.agents.RecordSpecCheck` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordSpecCheckOutcome {
    /// `no-such-definition` — for an identity no record carries.
    NoSuchDefinition {
        /// Why it was refused: `agency.agents.NotFound`.
        error: NotFound,
    },
    /// `refused` — when `verdict == Refused`.
    Refused {
        /// The `agency.agents.AgentSpecRefused` this outcome publishes.
        agent_spec_refused: AgentSpecRefused,
    },
    /// `checked` — when `verdict == Accepted`.
    Checked {
        /// The `agency.agents.AgentSpecChecked` this outcome publishes.
        agent_spec_checked: AgentSpecChecked,
    },
    /// `wrong-state` — from a state no declared move starts in.
    WrongState {
        /// Why it was refused: `agency.agents.StateConflict`.
        error: StateConflict,
    },
}

/// Retire an agent — the input of `agency.agents.RetireAgent`.
///
/// Everything it can result in is [`RetireAgentOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetireAgent {
    /// `agent_id` — `agency.agents.AgentId`.
    pub agent_id: AgentId,
}

/// Everything `agency.agents.RetireAgent` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetireAgentOutcome {
    /// `no-such-agent` — for an identity no record carries.
    NoSuchAgent {
        /// Why it was refused: `agency.agents.NotFound`.
        error: NotFound,
    },
    /// `retired` — otherwise.
    Retired {
        /// The `agency.agents.AgentRetired` this outcome publishes.
        agent_retired: AgentRetired,
    },
    /// `wrong-state` — from a state no declared move starts in.
    WrongState {
        /// Why it was refused: `agency.agents.StateConflict`.
        error: StateConflict,
    },
}

/// AgentDefinitionCreated — the event `agency.agents.AgentDefinitionCreated`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentDefinitionCreated {
    /// `definition_id` — `agency.agents.DefinitionId`.
    pub definition_id: DefinitionId,
    /// `name` — `String`.
    pub name: String,
}

/// AgentDefinitionDeprecated — the event `agency.agents.AgentDefinitionDeprecated`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentDefinitionDeprecated {
    /// `definition_id` — `agency.agents.DefinitionId`.
    pub definition_id: DefinitionId,
}

/// AgentDefinitionPublished — the event `agency.agents.AgentDefinitionPublished`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentDefinitionPublished {
    /// `definition_id` — `agency.agents.DefinitionId`.
    pub definition_id: DefinitionId,
}

/// AgentProvisioned — the event `agency.agents.AgentProvisioned`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentProvisioned {
    /// `agent_id` — `agency.agents.AgentId`.
    pub agent_id: AgentId,
    /// `definition_id` — `agency.agents.DefinitionId`.
    pub definition_id: DefinitionId,
    /// `definition_version` — `Integer`.
    pub definition_version: i64,
}

/// AgentRetired — the event `agency.agents.AgentRetired`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentRetired {
    /// `agent_id` — `agency.agents.AgentId`.
    pub agent_id: AgentId,
}

/// AgentSpecChecked — the event `agency.agents.AgentSpecChecked`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentSpecChecked {
    /// `definition_id` — `agency.agents.DefinitionId`.
    pub definition_id: DefinitionId,
    /// `message` — `String`.
    pub message: String,
}

/// AgentSpecRefused — the event `agency.agents.AgentSpecRefused`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentSpecRefused {
    /// `definition_id` — `agency.agents.DefinitionId`.
    pub definition_id: DefinitionId,
    /// `exit_code` — `Integer`.
    pub exit_code: i64,
    /// `message` — `String`.
    pub message: String,
}

/// The declared error `agency.agents.NotFound`.
///
/// The addressed record does not exist, and nothing changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotFound;

/// The declared error `agency.agents.StateConflict`.
///
/// The addressed record is not in a state this command acts from, and nothing changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateConflict;

/// AgentDefinitionList — one row of the view `agency.agents.AgentDefinitionList`.
///
/// Projects `agency.agents.AgentDefinition` at `read_your_writes` consistency.
/// The specification fully determines every row, so its query is generated over the storage port —
/// see the plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentDefinitionList {
    /// `definition_id` — `agency.agents.DefinitionId`.
    pub definition_id: DefinitionId,
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: crate::auth::TenantId,
    /// `name` — `String`.
    pub name: String,
    /// `description` — `String`.
    pub description: String,
    /// `agent_spec` — `agency.agents.AgentSpecDocument`.
    pub agent_spec: AgentSpecDocument,
    /// `policy_id` — `agency.auth.PolicyId`.
    pub policy_id: crate::auth::PolicyId,
    /// `version` — `Integer`.
    pub version: i64,
    /// `digest` — `agency.shared.Digest`.
    pub digest: crate::shared::Digest,
    /// `created_by` — `agency.auth.PrincipalId`.
    pub created_by: crate::auth::PrincipalId,
    /// `check_exit_code` — `Optional<Integer>`.
    pub check_exit_code: Option<i64>,
    /// `check_message` — `Optional<String>`.
    pub check_message: Option<String>,
    /// `state` — `agency.agents.AgentDefinition.State`.
    pub state: AgentDefinitionState,
}

/// AgentList — one row of the view `agency.agents.AgentList`.
///
/// Projects `agency.agents.Agent` at `read_your_writes` consistency.
/// The specification fully determines every row, so its query is generated over the storage port —
/// see the plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentList {
    /// `agent_id` — `agency.agents.AgentId`.
    pub agent_id: AgentId,
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: crate::auth::TenantId,
    /// `definition_id` — `agency.agents.DefinitionId`.
    pub definition_id: DefinitionId,
    /// `definition_version` — `Integer`.
    pub definition_version: i64,
    /// `display_name` — `String`.
    pub display_name: String,
    /// `principal_id` — `agency.auth.PrincipalId`.
    pub principal_id: crate::auth::PrincipalId,
    /// `current_run_id` — `Optional<agency.runs.RunId>`.
    pub current_run_id: Option<crate::runs::RunId>,
    /// `last_seen_at` — `Timestamp`.
    pub last_seen_at: crate::primitives::Timestamp,
    /// `state` — `agency.agents.Agent.State`.
    pub state: AgentState,
}

/// What this bounded context owes its implementor, and the seams of what is generated.
///
/// One trait per obligation in the synthesis plan, each carrying the plan's own contract, and one
/// per generated behaviour, which [`Generated`](crate::behaviour::Generated) implements.
pub mod obligations {
    /// The behaviour `agency.agents.CreateAgentDefinition` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait CreateAgentDefinitionBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.agents.CreateAgentDefinition`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn create_agent_definition(&mut self, input: super::CreateAgentDefinition) -> Result<super::CreateAgentDefinitionOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.agents.DeprecateAgentDefinition` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait DeprecateAgentDefinitionBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.agents.DeprecateAgentDefinition`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn deprecate_agent_definition(&mut self, input: super::DeprecateAgentDefinition) -> Result<super::DeprecateAgentDefinitionOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.agents.ProvisionAgent` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait ProvisionAgentBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.agents.ProvisionAgent`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn provision_agent(&mut self, input: super::ProvisionAgent) -> Result<super::ProvisionAgentOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.agents.PublishAgentDefinition` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait PublishAgentDefinitionBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.agents.PublishAgentDefinition`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn publish_agent_definition(&mut self, input: super::PublishAgentDefinition) -> Result<super::PublishAgentDefinitionOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.agents.RecordSpecCheck` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait RecordSpecCheckBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.agents.RecordSpecCheck`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn record_spec_check(&mut self, input: super::RecordSpecCheck) -> Result<super::RecordSpecCheckOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.agents.RetireAgent` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait RetireAgentBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.agents.RetireAgent`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn retire_agent(&mut self, input: super::RetireAgent) -> Result<super::RetireAgentOutcome, crate::obligation::UnmetObligation>;
    }

    /// The query `agency.agents.AgentDefinitionList` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage port. Implement it yourself to replace that query.
    pub trait AgentDefinitionListQuery {
        /// Serves `agency.agents.AgentDefinitionList` rows at the view's declared consistency.
        ///
        /// `Err` is the typed refusal of a row whose declared type cannot hold its value.
        fn agent_definition_list(&self) -> Result<Vec<super::AgentDefinitionList>, crate::obligation::UnmetObligation>;
    }

    /// The query `agency.agents.AgentList` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage port. Implement it yourself to replace that query.
    pub trait AgentListQuery {
        /// Serves `agency.agents.AgentList` rows at the view's declared consistency.
        ///
        /// `Err` is the typed refusal of a row whose declared type cannot hold its value.
        fn agent_list(&self) -> Result<Vec<super::AgentList>, crate::obligation::UnmetObligation>;
    }

}
