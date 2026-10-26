// generated from agency v1
// model digest 71132e90bde806415603d08bb97a71629b2c2f6cda30e1f22e76f407abd9a3eb
// contract digest a96f55e3e1ca68aeb1579b032c3547fffd27e8ab80e76bcb898f9b6c9258792e
// do not edit: regenerate with `ess synthesize --layout crate`

//! Tools — `agency.tools`.
//!
//! Tools Agency knows from their contracts, and the bindings that grant a definition or a team a subset of a tool's verbs within a scope.
//!
//! Everything this bounded context declares that the synthesis plan marks generated.

/// BindingId — `agency.tools.BindingId`: a distinct wrapper around `Uuid`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingId(pub crate::primitives::Uuid);

/// BindingScope — `agency.tools.BindingScope`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingScope {
    /// `repositories` — `List<String>`.
    pub repositories: Vec<String>,
    /// `paths` — `List<String>`.
    pub paths: Vec<String>,
    /// `subjects` — `List<String>`.
    pub subjects: Vec<String>,
}

/// ContractRef — `agency.tools.ContractRef`.
///
/// The tool-contract/1 contract a tool was registered from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractRef {
    /// `toolset` — `String`.
    pub toolset: String,
    /// `version` — `String`.
    pub version: String,
}

/// RiskEnvelope — `agency.tools.RiskEnvelope`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RiskEnvelope {
    /// `effects` — `List<String>`.
    pub effects: Vec<String>,
    /// `risk` — `agency.tools.ToolRisk`.
    pub risk: ToolRisk,
    /// `idempotency` — `String`.
    pub idempotency: String,
}

/// The states of `agency.tools.Tool`, as runtime values.
///
/// Synthesised from the lifecycle, so the two cannot disagree. Which *moves* are legal is not
/// carried here — it is carried by `Tool<S>`, where an undeclared move does not compile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolState {
    /// `Registered`.
    Registered,
    /// `Withdrawn`.
    Withdrawn,
}

/// The states of `agency.tools.ToolBinding`, as runtime values.
///
/// Synthesised from the lifecycle, so the two cannot disagree. Which *moves* are legal is not
/// carried here — it is carried by `ToolBinding<S>`, where an undeclared move does not compile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolBindingState {
    /// `Granted`.
    Granted,
    /// `Revoked`.
    Revoked,
}

/// ToolId — `agency.tools.ToolId`: a distinct wrapper around `Uuid`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolId(pub crate::primitives::Uuid);

/// ToolRisk — `agency.tools.ToolRisk`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolRisk {
    /// `None`.
    None,
    /// `Low`.
    Low,
    /// `Medium`.
    Medium,
    /// `High`.
    High,
    /// `Critical`.
    Critical,
}

/// What Tool — `agency.tools.Tool` — holds, apart from where it is in its lifecycle.
///
/// The identity and every declared field. The state is deliberately not one: inside the domain it
/// is carried by the type parameter of [`Tool<S>`], and at a boundary by [`ToolSnapshot::state`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolData {
    /// The identity: `tool_id` — `agency.tools.ToolId`.
    pub tool_id: ToolId,
    /// `tenant_id` — `agency.auth.TenantId`.
    ///
    /// Carries `tools`: `agency.auth.Tenant` owns many `agency.tools.Tool`.
    pub tenant_id: crate::auth::TenantId,
    /// `contract_ref` — `agency.tools.ContractRef`.
    pub contract_ref: ContractRef,
    /// `verbs` — `List<agency.shared.Verb>`.
    pub verbs: Vec<crate::shared::Verb>,
    /// `risk` — `agency.tools.RiskEnvelope`.
    pub risk: RiskEnvelope,
    /// `implementation_ref` — `Optional<String>`.
    pub implementation_ref: Option<String>,
    /// `connector_instance_id` — `Optional<String>`.
    pub connector_instance_id: Option<String>,
}

/// The states of `agency.tools.Tool`, at the type level.
///
/// One marker type per declared state, sealed: a state the lifecycle does not declare cannot
/// implement [`Marker`](tool_state::Marker), so [`Tool<S>`](Tool) can only ever rest in a real state.
pub mod tool_state {
    /// Closes [`Marker`] over the declared states.
    mod sealed {
        /// Implemented only by the marker types beside this module.
        pub trait Sealed {}
        impl Sealed for super::Registered {}
        impl Sealed for super::Withdrawn {}
    }

    /// A declared state of `Tool`, as a type.
    pub trait Marker: sealed::Sealed {
        /// The same state, as the runtime value.
        const STATE: super::ToolState;
    }

    /// `Registered`. Where a new instance starts.
    pub struct Registered;

    impl Marker for Registered {
        const STATE: super::ToolState = super::ToolState::Registered;
    }

    /// `Withdrawn`. Terminal: an instance may rest here forever.
    pub struct Withdrawn;

    impl Marker for Withdrawn {
        const STATE: super::ToolState = super::ToolState::Withdrawn;
    }
}

/// Tool — `agency.tools.Tool` — with its lifecycle state carried by the type.
///
/// The one constructor rests in `Registered`, and the only way to change `S` is a method generated from
/// a declared transition. A move the specification does not declare is therefore not an error
/// case: it does not compile. Where the state is data — wire, storage — use [`ToolSnapshot`]
/// and [`ToolSnapshot::refine`].
pub struct Tool<S: tool_state::Marker> {
    data: ToolData,
    state: core::marker::PhantomData<S>,
}

impl<S: tool_state::Marker> Tool<S> {
    /// The state this instance rests in, as the runtime value.
    pub fn state(&self) -> ToolState {
        S::STATE
    }

    /// What it holds.
    pub fn data(&self) -> &ToolData {
        &self.data
    }

    /// Hands the data back, giving up the typed state.
    pub fn into_data(self) -> ToolData {
        self.data
    }
}

impl Tool<tool_state::Registered> {
    /// A new instance, resting in `Registered` — the only state the lifecycle starts one in.
    pub fn new(data: ToolData) -> Self {
        Self {
            data,
            state: core::marker::PhantomData,
        }
    }
}

impl Tool<tool_state::Registered> {
    /// `withdraw` — `Registered` → `Withdrawn`. Taken by the `withdrawn` outcome of `agency.tools.WithdrawTool`.
    pub fn withdraw(self) -> Tool<tool_state::Withdrawn> {
        Tool {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }
}

/// `agency.tools.Tool` as it crosses a boundary: the state as a value beside the data.
///
/// Wire and storage know states only at runtime; [`ToolSnapshot::refine`] is the one door back
/// into the typed lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolSnapshot {
    /// Where the instance is in its lifecycle.
    pub state: ToolState,
    /// What it holds.
    pub data: ToolData,
}

/// An `Tool` in whichever declared state it was found.
pub enum AnyTool {
    /// Resting in `Registered`.
    Registered(Tool<tool_state::Registered>),
    /// Resting in `Withdrawn`.
    Withdrawn(Tool<tool_state::Withdrawn>),
}

impl ToolSnapshot {
    /// Refines the runtime state into the typed one.
    ///
    /// Total: every declared state has an arm, and an undeclared state cannot reach here because
    /// `ToolState` cannot spell one.
    pub fn refine(self) -> AnyTool {
        match self.state {
            ToolState::Registered => AnyTool::Registered(Tool {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            ToolState::Withdrawn => AnyTool::Withdrawn(Tool {
                data: self.data,
                state: core::marker::PhantomData,
            }),
        }
    }
}

impl AnyTool {
    /// The state, as the runtime value.
    pub fn state(&self) -> ToolState {
        match self {
            Self::Registered(_) => ToolState::Registered,
            Self::Withdrawn(_) => ToolState::Withdrawn,
        }
    }

    /// Back to the boundary shape.
    pub fn snapshot(self) -> ToolSnapshot {
        match self {
            Self::Registered(instance) => ToolSnapshot {
                state: ToolState::Registered,
                data: instance.into_data(),
            },
            Self::Withdrawn(instance) => ToolSnapshot {
                state: ToolState::Withdrawn,
                data: instance.into_data(),
            },
        }
    }
}

/// What ToolBinding — `agency.tools.ToolBinding` — holds, apart from where it is in its lifecycle.
///
/// The identity and every declared field. The state is deliberately not one: inside the domain it
/// is carried by the type parameter of [`ToolBinding<S>`], and at a boundary by [`ToolBindingSnapshot::state`].
///
/// Every value satisfies `((defined(definition_id) and not (defined(team_id))) or (not (defined(definition_id)) and defined(team_id)))` — checked by [`ToolBindingData::broken_invariant`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolBindingData {
    /// The identity: `binding_id` — `agency.tools.BindingId`.
    pub binding_id: BindingId,
    /// `tenant_id` — `agency.auth.TenantId`.
    ///
    /// Carries `tool_bindings`: `agency.auth.Tenant` owns many `agency.tools.ToolBinding`.
    pub tenant_id: crate::auth::TenantId,
    /// `definition_id` — `Optional<agency.agents.DefinitionId>`.
    ///
    /// Carries `definition`: `agency.tools.ToolBinding` references one `agency.agents.AgentDefinition`.
    pub definition_id: Option<crate::agents::DefinitionId>,
    /// `team_id` — `Optional<agency.teams.TeamId>`.
    ///
    /// Carries `team`: `agency.tools.ToolBinding` references one `agency.teams.Team`.
    pub team_id: Option<crate::teams::TeamId>,
    /// `tool_id` — `agency.tools.ToolId`.
    ///
    /// Carries `tool`: `agency.tools.ToolBinding` references one `agency.tools.Tool`.
    pub tool_id: ToolId,
    /// `verbs` — `List<agency.shared.Verb>`.
    pub verbs: Vec<crate::shared::Verb>,
    /// `scope` — `agency.tools.BindingScope`.
    pub scope: BindingScope,
    /// `max_output_bytes` — `Integer`.
    pub max_output_bytes: i64,
    /// `expires_at` — `Optional<Timestamp>`.
    pub expires_at: Option<crate::primitives::Timestamp>,
    /// `credential_id` — `Optional<agency.auth.CredentialId>`.
    ///
    /// Carries `credential`: `agency.tools.ToolBinding` references one `agency.auth.Credential`.
    pub credential_id: Option<crate::auth::CredentialId>,
}

impl ToolBindingData {
    /// The first declared invariant of `agency.tools.ToolBinding` this value breaks, as the specification declares it,
    /// or `None` when it breaks none.
    ///
    /// An invariant is broken only when it is false of this value. One that reads something
    /// absent — an empty `Optional`, a list position past the end, or `state`, which this
    /// type does not hold — decides nothing, as the conformance interpreter reads it.
    pub fn broken_invariant(&self) -> Option<&'static str> {
        use crate::primitives::invariant as iv;
        if iv::broken(iv::any([iv::all([Some(self.definition_id.as_ref().map(|v| &v.0).is_some()), iv::not(Some(self.team_id.as_ref().map(|v| &v.0).is_some()))]), iv::all([iv::not(Some(self.definition_id.as_ref().map(|v| &v.0).is_some())), Some(self.team_id.as_ref().map(|v| &v.0).is_some())])])) {
            return Some("((defined(definition_id) and not (defined(team_id))) or (not (defined(definition_id)) and defined(team_id)))");
        }
        None
    }
}

/// The states of `agency.tools.ToolBinding`, at the type level.
///
/// One marker type per declared state, sealed: a state the lifecycle does not declare cannot
/// implement [`Marker`](tool_binding_state::Marker), so [`ToolBinding<S>`](ToolBinding) can only ever rest in a real state.
pub mod tool_binding_state {
    /// Closes [`Marker`] over the declared states.
    mod sealed {
        /// Implemented only by the marker types beside this module.
        pub trait Sealed {}
        impl Sealed for super::Granted {}
        impl Sealed for super::Revoked {}
    }

    /// A declared state of `ToolBinding`, as a type.
    pub trait Marker: sealed::Sealed {
        /// The same state, as the runtime value.
        const STATE: super::ToolBindingState;
    }

    /// `Granted`. Where a new instance starts.
    pub struct Granted;

    impl Marker for Granted {
        const STATE: super::ToolBindingState = super::ToolBindingState::Granted;
    }

    /// `Revoked`. Terminal: an instance may rest here forever.
    pub struct Revoked;

    impl Marker for Revoked {
        const STATE: super::ToolBindingState = super::ToolBindingState::Revoked;
    }
}

/// ToolBinding — `agency.tools.ToolBinding` — with its lifecycle state carried by the type.
///
/// The one constructor rests in `Granted`, and the only way to change `S` is a method generated from
/// a declared transition. A move the specification does not declare is therefore not an error
/// case: it does not compile. Where the state is data — wire, storage — use [`ToolBindingSnapshot`]
/// and [`ToolBindingSnapshot::refine`].
pub struct ToolBinding<S: tool_binding_state::Marker> {
    data: ToolBindingData,
    state: core::marker::PhantomData<S>,
}

impl<S: tool_binding_state::Marker> ToolBinding<S> {
    /// The state this instance rests in, as the runtime value.
    pub fn state(&self) -> ToolBindingState {
        S::STATE
    }

    /// What it holds.
    pub fn data(&self) -> &ToolBindingData {
        &self.data
    }

    /// Hands the data back, giving up the typed state.
    pub fn into_data(self) -> ToolBindingData {
        self.data
    }
}

impl ToolBinding<tool_binding_state::Granted> {
    /// A new instance, resting in `Granted` — the only state the lifecycle starts one in.
    pub fn new(data: ToolBindingData) -> Self {
        Self {
            data,
            state: core::marker::PhantomData,
        }
    }
}

impl ToolBinding<tool_binding_state::Granted> {
    /// `revoke` — `Granted` → `Revoked`. Taken by the `revoked` outcome of `agency.tools.RevokeBinding`.
    pub fn revoke(self) -> ToolBinding<tool_binding_state::Revoked> {
        ToolBinding {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }
}

/// `agency.tools.ToolBinding` as it crosses a boundary: the state as a value beside the data.
///
/// Wire and storage know states only at runtime; [`ToolBindingSnapshot::refine`] is the one door back
/// into the typed lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolBindingSnapshot {
    /// Where the instance is in its lifecycle.
    pub state: ToolBindingState,
    /// What it holds.
    pub data: ToolBindingData,
}

/// An `ToolBinding` in whichever declared state it was found.
pub enum AnyToolBinding {
    /// Resting in `Granted`.
    Granted(ToolBinding<tool_binding_state::Granted>),
    /// Resting in `Revoked`.
    Revoked(ToolBinding<tool_binding_state::Revoked>),
}

impl ToolBindingSnapshot {
    /// Refines the runtime state into the typed one.
    ///
    /// Total: every declared state has an arm, and an undeclared state cannot reach here because
    /// `ToolBindingState` cannot spell one.
    pub fn refine(self) -> AnyToolBinding {
        match self.state {
            ToolBindingState::Granted => AnyToolBinding::Granted(ToolBinding {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            ToolBindingState::Revoked => AnyToolBinding::Revoked(ToolBinding {
                data: self.data,
                state: core::marker::PhantomData,
            }),
        }
    }
}

impl AnyToolBinding {
    /// The state, as the runtime value.
    pub fn state(&self) -> ToolBindingState {
        match self {
            Self::Granted(_) => ToolBindingState::Granted,
            Self::Revoked(_) => ToolBindingState::Revoked,
        }
    }

    /// Back to the boundary shape.
    pub fn snapshot(self) -> ToolBindingSnapshot {
        match self {
            Self::Granted(instance) => ToolBindingSnapshot {
                state: ToolBindingState::Granted,
                data: instance.into_data(),
            },
            Self::Revoked(instance) => ToolBindingSnapshot {
                state: ToolBindingState::Revoked,
                data: instance.into_data(),
            },
        }
    }
}

/// Bind a tool — the input of `agency.tools.BindTool`.
///
/// Everything it can result in is [`BindToolOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindTool {
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: crate::auth::TenantId,
    /// `definition_id` — `Optional<agency.agents.DefinitionId>`.
    pub definition_id: Option<crate::agents::DefinitionId>,
    /// `team_id` — `Optional<agency.teams.TeamId>`.
    pub team_id: Option<crate::teams::TeamId>,
    /// `tool_id` — `agency.tools.ToolId`.
    pub tool_id: ToolId,
    /// `verbs` — `List<agency.shared.Verb>`.
    pub verbs: Vec<crate::shared::Verb>,
    /// `scope` — `agency.tools.BindingScope`.
    pub scope: BindingScope,
    /// `max_output_bytes` — `Integer`.
    pub max_output_bytes: i64,
    /// `expires_at` — `Optional<Timestamp>`.
    pub expires_at: Option<crate::primitives::Timestamp>,
    /// `credential_id` — `Optional<agency.auth.CredentialId>`.
    pub credential_id: Option<crate::auth::CredentialId>,
}

/// Everything `agency.tools.BindTool` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindToolOutcome {
    /// `bound-to-definition` — when `(defined(definition_id) and not (defined(team_id)))`.
    BoundToDefinition {
        /// The `agency.tools.ToolBound` this outcome publishes.
        tool_bound: ToolBound,
    },
    /// `bound-to-team` — when `(not (defined(definition_id)) and defined(team_id))`.
    BoundToTeam {
        /// The `agency.tools.ToolBound` this outcome publishes.
        tool_bound: ToolBound,
    },
    /// `target-invalid` — otherwise.
    TargetInvalid {
        /// Why it was refused: `agency.tools.BindingTargetInvalid`.
        error: BindingTargetInvalid,
    },
}

/// Register a tool — the input of `agency.tools.RegisterTool`.
///
/// Everything it can result in is [`RegisterToolOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisterTool {
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: crate::auth::TenantId,
    /// `contract_ref` — `agency.tools.ContractRef`.
    pub contract_ref: ContractRef,
    /// `verbs` — `List<agency.shared.Verb>`.
    pub verbs: Vec<crate::shared::Verb>,
    /// `risk` — `agency.tools.RiskEnvelope`.
    pub risk: RiskEnvelope,
    /// `implementation_ref` — `Optional<String>`.
    pub implementation_ref: Option<String>,
    /// `connector_instance_id` — `Optional<String>`.
    pub connector_instance_id: Option<String>,
}

/// Everything `agency.tools.RegisterTool` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterToolOutcome {
    /// `registered` — otherwise.
    Registered {
        /// The `agency.tools.ToolRegistered` this outcome publishes.
        tool_registered: ToolRegistered,
    },
}

/// Revoke a binding — the input of `agency.tools.RevokeBinding`.
///
/// Everything it can result in is [`RevokeBindingOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevokeBinding {
    /// `binding_id` — `agency.tools.BindingId`.
    pub binding_id: BindingId,
}

/// Everything `agency.tools.RevokeBinding` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RevokeBindingOutcome {
    /// `no-such-binding` — for an identity no record carries.
    NoSuchBinding {
        /// Why it was refused: `agency.tools.NotFound`.
        error: NotFound,
    },
    /// `revoked` — otherwise.
    Revoked {
        /// The `agency.tools.BindingRevoked` this outcome publishes.
        binding_revoked: BindingRevoked,
    },
    /// `wrong-state` — from a state no declared move starts in.
    WrongState {
        /// Why it was refused: `agency.tools.StateConflict`.
        error: StateConflict,
    },
}

/// Withdraw a tool — the input of `agency.tools.WithdrawTool`.
///
/// Everything it can result in is [`WithdrawToolOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WithdrawTool {
    /// `tool_id` — `agency.tools.ToolId`.
    pub tool_id: ToolId,
}

/// Everything `agency.tools.WithdrawTool` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WithdrawToolOutcome {
    /// `no-such-tool` — for an identity no record carries.
    NoSuchTool {
        /// Why it was refused: `agency.tools.NotFound`.
        error: NotFound,
    },
    /// `withdrawn` — otherwise.
    Withdrawn {
        /// The `agency.tools.ToolWithdrawn` this outcome publishes.
        tool_withdrawn: ToolWithdrawn,
    },
    /// `wrong-state` — from a state no declared move starts in.
    WrongState {
        /// Why it was refused: `agency.tools.StateConflict`.
        error: StateConflict,
    },
}

/// BindingRevoked — the event `agency.tools.BindingRevoked`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingRevoked {
    /// `binding_id` — `agency.tools.BindingId`.
    pub binding_id: BindingId,
}

/// ToolBound — the event `agency.tools.ToolBound`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolBound {
    /// `binding_id` — `agency.tools.BindingId`.
    pub binding_id: BindingId,
    /// `tool_id` — `agency.tools.ToolId`.
    pub tool_id: ToolId,
    /// `verbs` — `List<agency.shared.Verb>`.
    pub verbs: Vec<crate::shared::Verb>,
}

/// ToolRegistered — the event `agency.tools.ToolRegistered`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolRegistered {
    /// `tool_id` — `agency.tools.ToolId`.
    pub tool_id: ToolId,
    /// `contract_ref` — `agency.tools.ContractRef`.
    pub contract_ref: ContractRef,
}

/// ToolWithdrawn — the event `agency.tools.ToolWithdrawn`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolWithdrawn {
    /// `tool_id` — `agency.tools.ToolId`.
    pub tool_id: ToolId,
}

/// The declared error `agency.tools.BindingTargetInvalid`.
///
/// A binding names neither or both of a definition and a team, and nothing changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingTargetInvalid;

/// The declared error `agency.tools.NotFound`.
///
/// The addressed record does not exist, and nothing changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotFound;

/// The declared error `agency.tools.StateConflict`.
///
/// The addressed record is not in a state this command acts from, and nothing changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateConflict;

/// BindingList — one row of the view `agency.tools.BindingList`.
///
/// Projects `agency.tools.ToolBinding` at `read_your_writes` consistency.
/// The specification fully determines every row, so its query is generated over the storage port —
/// see the plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingList {
    /// `binding_id` — `agency.tools.BindingId`.
    pub binding_id: BindingId,
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: crate::auth::TenantId,
    /// `definition_id` — `Optional<agency.agents.DefinitionId>`.
    pub definition_id: Option<crate::agents::DefinitionId>,
    /// `team_id` — `Optional<agency.teams.TeamId>`.
    pub team_id: Option<crate::teams::TeamId>,
    /// `tool_id` — `agency.tools.ToolId`.
    pub tool_id: ToolId,
    /// `verbs` — `List<agency.shared.Verb>`.
    pub verbs: Vec<crate::shared::Verb>,
    /// `scope` — `agency.tools.BindingScope`.
    pub scope: BindingScope,
    /// `max_output_bytes` — `Integer`.
    pub max_output_bytes: i64,
    /// `expires_at` — `Optional<Timestamp>`.
    pub expires_at: Option<crate::primitives::Timestamp>,
    /// `credential_id` — `Optional<agency.auth.CredentialId>`.
    pub credential_id: Option<crate::auth::CredentialId>,
    /// `state` — `agency.tools.ToolBinding.State`.
    pub state: ToolBindingState,
}

/// ToolCatalogue — one row of the view `agency.tools.ToolCatalogue`.
///
/// Projects `agency.tools.Tool` at `read_your_writes` consistency.
/// The specification fully determines every row, so its query is generated over the storage port —
/// see the plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCatalogue {
    /// `tool_id` — `agency.tools.ToolId`.
    pub tool_id: ToolId,
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: crate::auth::TenantId,
    /// `contract_ref` — `agency.tools.ContractRef`.
    pub contract_ref: ContractRef,
    /// `verbs` — `List<agency.shared.Verb>`.
    pub verbs: Vec<crate::shared::Verb>,
    /// `risk` — `agency.tools.RiskEnvelope`.
    pub risk: RiskEnvelope,
    /// `implementation_ref` — `Optional<String>`.
    pub implementation_ref: Option<String>,
    /// `connector_instance_id` — `Optional<String>`.
    pub connector_instance_id: Option<String>,
    /// `state` — `agency.tools.Tool.State`.
    pub state: ToolState,
}

/// What this bounded context owes its implementor, and the seams of what is generated.
///
/// One trait per obligation in the synthesis plan, each carrying the plan's own contract, and one
/// per generated behaviour, which [`Generated`](crate::behaviour::Generated) implements.
pub mod obligations {
    /// The behaviour `agency.tools.BindTool` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait BindToolBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.tools.BindTool`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn bind_tool(&mut self, input: super::BindTool) -> Result<super::BindToolOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.tools.RegisterTool` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait RegisterToolBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.tools.RegisterTool`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn register_tool(&mut self, input: super::RegisterTool) -> Result<super::RegisterToolOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.tools.RevokeBinding` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait RevokeBindingBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.tools.RevokeBinding`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn revoke_binding(&mut self, input: super::RevokeBinding) -> Result<super::RevokeBindingOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.tools.WithdrawTool` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait WithdrawToolBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.tools.WithdrawTool`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn withdraw_tool(&mut self, input: super::WithdrawTool) -> Result<super::WithdrawToolOutcome, crate::obligation::UnmetObligation>;
    }

    /// The query `agency.tools.BindingList` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage port. Implement it yourself to replace that query.
    pub trait BindingListQuery {
        /// Serves `agency.tools.BindingList` rows at the view's declared consistency.
        ///
        /// `Err` is the typed refusal of a row whose declared type cannot hold its value.
        fn binding_list(&self) -> Result<Vec<super::BindingList>, crate::obligation::UnmetObligation>;
    }

    /// The query `agency.tools.ToolCatalogue` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage port. Implement it yourself to replace that query.
    pub trait ToolCatalogueQuery {
        /// Serves `agency.tools.ToolCatalogue` rows at the view's declared consistency.
        ///
        /// `Err` is the typed refusal of a row whose declared type cannot hold its value.
        fn tool_catalogue(&self) -> Result<Vec<super::ToolCatalogue>, crate::obligation::UnmetObligation>;
    }

}
