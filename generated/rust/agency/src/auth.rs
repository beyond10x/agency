// generated from agency v1
// model digest 71132e90bde806415603d08bb97a71629b2c2f6cda30e1f22e76f407abd9a3eb
// contract digest a96f55e3e1ca68aeb1579b032c3547fffd27e8ab80e76bcb898f9b6c9258792e
// do not edit: regenerate with `ess synthesize --layout crate`

//! Auth — `agency.auth`.
//!
//! Who may act in Agency and with what: tenants, principals and their roles, credentials held as custody pointers, grants of those credentials, and the policies runs are sealed under.
//!
//! Everything this bounded context declares that the synthesis plan marks generated.

/// ConnectorsCustody — `agency.auth.ConnectorsCustody`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectorsCustody {
    /// `instance_id` — `String`.
    pub instance_id: String,
}

/// The states of `agency.auth.Credential`, as runtime values.
///
/// Synthesised from the lifecycle, so the two cannot disagree. Which *moves* are legal is not
/// carried here — it is carried by `Credential<S>`, where an undeclared move does not compile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CredentialState {
    /// `Revoked`.
    Revoked,
    /// `Stored`.
    Stored,
}

/// The states of `agency.auth.CredentialGrant`, as runtime values.
///
/// Synthesised from the lifecycle, so the two cannot disagree. Which *moves* are legal is not
/// carried here — it is carried by `CredentialGrant<S>`, where an undeclared move does not compile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CredentialGrantState {
    /// `Granted`.
    Granted,
    /// `Revoked`.
    Revoked,
}

/// CredentialId — `agency.auth.CredentialId`: a distinct wrapper around `Uuid`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CredentialId(pub crate::primitives::Uuid);

/// CredentialKind — `agency.auth.CredentialKind`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CredentialKind {
    /// `OperatorLogin`.
    OperatorLogin,
    /// `ApiKey`.
    ApiKey,
    /// `OAuthToken`.
    OAuthToken,
    /// `ConnectorInstance`.
    ConnectorInstance,
}

/// CredentialScope — `agency.auth.CredentialScope`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CredentialScope {
    /// `models` — `List<String>`.
    pub models: Vec<String>,
    /// `providers` — `List<String>`.
    pub providers: Vec<String>,
    /// `tools` — `List<agency.tools.ToolId>`.
    pub tools: Vec<crate::tools::ToolId>,
}

/// Custody — `agency.auth.Custody`: one of a fixed set of shapes, tagged on the wire by `kind`.
///
/// Where a credential's secret is held. Agency stores this pointer, never the value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Custody {
    /// Tagged `AgencyVault`, carrying nothing.
    AgencyVault,
    /// Tagged `ConnectorsInstance` — `agency.auth.ConnectorsCustody`.
    ConnectorsInstance(ConnectorsCustody),
    /// Tagged `HostFile` — `agency.auth.HostFileCustody`.
    HostFile(HostFileCustody),
}

/// DecisionMode — `agency.auth.DecisionMode`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionMode {
    /// `Frame`.
    Frame,
    /// `Ask`.
    Ask,
    /// `Observe`.
    Observe,
}

/// DefinitionSubject — `agency.auth.DefinitionSubject`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefinitionSubject {
    /// `definition_id` — `agency.agents.DefinitionId`.
    pub definition_id: crate::agents::DefinitionId,
}

/// GrantId — `agency.auth.GrantId`: a distinct wrapper around `Uuid`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrantId(pub crate::primitives::Uuid);

/// GrantPurpose — `agency.auth.GrantPurpose`: one of a fixed set of shapes, tagged on the wire by `kind`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrantPurpose {
    /// Tagged `ModelCalls`, carrying nothing.
    ModelCalls,
    /// Tagged `ToolCalls` — `agency.auth.ToolCallsPurpose`.
    ToolCalls(ToolCallsPurpose),
}

/// HostFileCustody — `agency.auth.HostFileCustody`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostFileCustody {
    /// `path` — `String`.
    pub path: String,
}

/// The states of `agency.auth.Policy`, as runtime values.
///
/// Synthesised from the lifecycle, so the two cannot disagree. Which *moves* are legal is not
/// carried here — it is carried by `Policy<S>`, where an undeclared move does not compile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyState {
    /// `Active`.
    Active,
    /// `Draft`.
    Draft,
    /// `Superseded`.
    Superseded,
}

/// PolicyId — `agency.auth.PolicyId`: a distinct wrapper around `Uuid`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyId(pub crate::primitives::Uuid);

/// The states of `agency.auth.Principal`, as runtime values.
///
/// Synthesised from the lifecycle, so the two cannot disagree. Which *moves* are legal is not
/// carried here — it is carried by `Principal<S>`, where an undeclared move does not compile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrincipalState {
    /// `Disabled`.
    Disabled,
    /// `Invited`.
    Invited,
}

/// PrincipalId — `agency.auth.PrincipalId`: a distinct wrapper around `Uuid`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrincipalId(pub crate::primitives::Uuid);

/// PrincipalKind — `agency.auth.PrincipalKind`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrincipalKind {
    /// `Human`.
    Human,
    /// `Agent`.
    Agent,
    /// `Service`.
    Service,
}

/// RiskCeiling — `agency.auth.RiskCeiling`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskCeiling {
    /// `Low`.
    Low,
    /// `Medium`.
    Medium,
    /// `High`.
    High,
}

/// Role — `agency.auth.Role`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// `Operator`.
    Operator,
    /// `Supervisor`.
    Supervisor,
    /// `Viewer`.
    Viewer,
    /// `Agent`.
    Agent,
}

/// Rule — `agency.auth.Rule`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    /// `subject` — `agency.auth.RuleSubject`.
    pub subject: RuleSubject,
    /// `tool_id` — `Optional<agency.tools.ToolId>`.
    pub tool_id: Option<crate::tools::ToolId>,
    /// `verbs` — `Optional<List<agency.shared.Verb>>`.
    pub verbs: Option<Vec<crate::shared::Verb>>,
    /// `risk_ceiling` — `agency.auth.RiskCeiling`.
    pub risk_ceiling: RiskCeiling,
    /// `decision_mode` — `agency.auth.DecisionMode`.
    pub decision_mode: DecisionMode,
    /// `budget_ceiling` — `agency.shared.RunBudget`.
    pub budget_ceiling: crate::shared::RunBudget,
    /// `hermetic` — `Boolean`.
    pub hermetic: bool,
}

/// RuleSubject — `agency.auth.RuleSubject`: one of a fixed set of shapes, tagged on the wire by `kind`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleSubject {
    /// Tagged `Definition` — `agency.auth.DefinitionSubject`.
    Definition(DefinitionSubject),
    /// Tagged `Team` — `agency.auth.TeamSubject`.
    Team(TeamSubject),
    /// Tagged `Tenant` — `agency.auth.TenantSubject`.
    Tenant(TenantSubject),
    /// Tagged `Workforce` — `agency.auth.WorkforceSubject`.
    Workforce(WorkforceSubject),
}

/// TeamSubject — `agency.auth.TeamSubject`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamSubject {
    /// `team_id` — `agency.teams.TeamId`.
    pub team_id: crate::teams::TeamId,
}

/// The states of `agency.auth.Tenant`, as runtime values.
///
/// Synthesised from the lifecycle, so the two cannot disagree. Which *moves* are legal is not
/// carried here — it is carried by `Tenant<S>`, where an undeclared move does not compile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TenantState {
    /// `Created`.
    Created,
}

/// TenantId — `agency.auth.TenantId`: a distinct wrapper around `Uuid`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TenantId(pub crate::primitives::Uuid);

/// TenantSubject — `agency.auth.TenantSubject`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TenantSubject {
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: TenantId,
}

/// ToolCallsPurpose — `agency.auth.ToolCallsPurpose`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCallsPurpose {
    /// `tool_id` — `agency.tools.ToolId`.
    pub tool_id: crate::tools::ToolId,
}

/// WorkforceSubject — `agency.auth.WorkforceSubject`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkforceSubject {
    /// `workforce_id` — `agency.teams.WorkforceId`.
    pub workforce_id: crate::teams::WorkforceId,
}

/// What Credential — `agency.auth.Credential` — holds, apart from where it is in its lifecycle.
///
/// The identity and every declared field. The state is deliberately not one: inside the domain it
/// is carried by the type parameter of [`Credential<S>`], and at a boundary by [`CredentialSnapshot::state`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CredentialData {
    /// The identity: `credential_id` — `agency.auth.CredentialId`.
    pub credential_id: CredentialId,
    /// `tenant_id` — `agency.auth.TenantId`.
    ///
    /// Carries `credentials`: `agency.auth.Tenant` owns many `agency.auth.Credential`.
    pub tenant_id: TenantId,
    /// `kind` — `agency.auth.CredentialKind`.
    pub kind: CredentialKind,
    /// `custody` — `agency.auth.Custody`.
    pub custody: Custody,
    /// `scope` — `agency.auth.CredentialScope`.
    pub scope: CredentialScope,
    /// `owner_principal_id` — `agency.auth.PrincipalId`.
    ///
    /// Carries `owner`: `agency.auth.Credential` references one `agency.auth.Principal`.
    pub owner_principal_id: PrincipalId,
    /// `expires_at` — `Optional<Timestamp>`.
    pub expires_at: Option<crate::primitives::Timestamp>,
    /// `last_refreshed_at` — `Timestamp`.
    pub last_refreshed_at: crate::primitives::Timestamp,
}

/// The states of `agency.auth.Credential`, at the type level.
///
/// One marker type per declared state, sealed: a state the lifecycle does not declare cannot
/// implement [`Marker`](credential_state::Marker), so [`Credential<S>`](Credential) can only ever rest in a real state.
pub mod credential_state {
    /// Closes [`Marker`] over the declared states.
    mod sealed {
        /// Implemented only by the marker types beside this module.
        pub trait Sealed {}
        impl Sealed for super::Revoked {}
        impl Sealed for super::Stored {}
    }

    /// A declared state of `Credential`, as a type.
    pub trait Marker: sealed::Sealed {
        /// The same state, as the runtime value.
        const STATE: super::CredentialState;
    }

    /// `Revoked`. Terminal: an instance may rest here forever.
    pub struct Revoked;

    impl Marker for Revoked {
        const STATE: super::CredentialState = super::CredentialState::Revoked;
    }

    /// `Stored`. Where a new instance starts.
    pub struct Stored;

    impl Marker for Stored {
        const STATE: super::CredentialState = super::CredentialState::Stored;
    }
}

/// Credential — `agency.auth.Credential` — with its lifecycle state carried by the type.
///
/// The one constructor rests in `Stored`, and the only way to change `S` is a method generated from
/// a declared transition. A move the specification does not declare is therefore not an error
/// case: it does not compile. Where the state is data — wire, storage — use [`CredentialSnapshot`]
/// and [`CredentialSnapshot::refine`].
pub struct Credential<S: credential_state::Marker> {
    data: CredentialData,
    state: core::marker::PhantomData<S>,
}

impl<S: credential_state::Marker> Credential<S> {
    /// The state this instance rests in, as the runtime value.
    pub fn state(&self) -> CredentialState {
        S::STATE
    }

    /// What it holds.
    pub fn data(&self) -> &CredentialData {
        &self.data
    }

    /// Hands the data back, giving up the typed state.
    pub fn into_data(self) -> CredentialData {
        self.data
    }
}

impl Credential<credential_state::Stored> {
    /// A new instance, resting in `Stored` — the only state the lifecycle starts one in.
    pub fn new(data: CredentialData) -> Self {
        Self {
            data,
            state: core::marker::PhantomData,
        }
    }
}

impl Credential<credential_state::Stored> {
    /// `revoke` — `Stored` → `Revoked`. Taken by the `revoked` outcome of `agency.auth.RevokeCredential`.
    pub fn revoke(self) -> Credential<credential_state::Revoked> {
        Credential {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }
}

/// `agency.auth.Credential` as it crosses a boundary: the state as a value beside the data.
///
/// Wire and storage know states only at runtime; [`CredentialSnapshot::refine`] is the one door back
/// into the typed lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CredentialSnapshot {
    /// Where the instance is in its lifecycle.
    pub state: CredentialState,
    /// What it holds.
    pub data: CredentialData,
}

/// An `Credential` in whichever declared state it was found.
pub enum AnyCredential {
    /// Resting in `Revoked`.
    Revoked(Credential<credential_state::Revoked>),
    /// Resting in `Stored`.
    Stored(Credential<credential_state::Stored>),
}

impl CredentialSnapshot {
    /// Refines the runtime state into the typed one.
    ///
    /// Total: every declared state has an arm, and an undeclared state cannot reach here because
    /// `CredentialState` cannot spell one.
    pub fn refine(self) -> AnyCredential {
        match self.state {
            CredentialState::Revoked => AnyCredential::Revoked(Credential {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            CredentialState::Stored => AnyCredential::Stored(Credential {
                data: self.data,
                state: core::marker::PhantomData,
            }),
        }
    }
}

impl AnyCredential {
    /// The state, as the runtime value.
    pub fn state(&self) -> CredentialState {
        match self {
            Self::Revoked(_) => CredentialState::Revoked,
            Self::Stored(_) => CredentialState::Stored,
        }
    }

    /// Back to the boundary shape.
    pub fn snapshot(self) -> CredentialSnapshot {
        match self {
            Self::Revoked(instance) => CredentialSnapshot {
                state: CredentialState::Revoked,
                data: instance.into_data(),
            },
            Self::Stored(instance) => CredentialSnapshot {
                state: CredentialState::Stored,
                data: instance.into_data(),
            },
        }
    }
}

/// What CredentialGrant — `agency.auth.CredentialGrant` — holds, apart from where it is in its lifecycle.
///
/// The identity and every declared field. The state is deliberately not one: inside the domain it
/// is carried by the type parameter of [`CredentialGrant<S>`], and at a boundary by [`CredentialGrantSnapshot::state`].
///
/// Every value satisfies `((defined(grantee_agent_id) and not (defined(grantee_team_id)) and not (defined(grantee_workforce_id))) or (not (defined(grantee_agent_id)) and defined(grantee_team_id) and not (defined(grantee_workforce_id))) or (not (defined(grantee_agent_id)) and not (defined(grantee_team_id)) and defined(grantee_workforce_id)))` — checked by [`CredentialGrantData::broken_invariant`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CredentialGrantData {
    /// The identity: `grant_id` — `agency.auth.GrantId`.
    pub grant_id: GrantId,
    /// `tenant_id` — `agency.auth.TenantId`.
    ///
    /// Carries `credential_grants`: `agency.auth.Tenant` owns many `agency.auth.CredentialGrant`.
    pub tenant_id: TenantId,
    /// `credential_id` — `agency.auth.CredentialId`.
    ///
    /// Carries `credential`: `agency.auth.CredentialGrant` references one `agency.auth.Credential`.
    pub credential_id: CredentialId,
    /// `grantee_agent_id` — `Optional<agency.agents.AgentId>`.
    ///
    /// Carries `grantee_agent`: `agency.auth.CredentialGrant` references one `agency.agents.Agent`.
    pub grantee_agent_id: Option<crate::agents::AgentId>,
    /// `grantee_team_id` — `Optional<agency.teams.TeamId>`.
    ///
    /// Carries `grantee_team`: `agency.auth.CredentialGrant` references one `agency.teams.Team`.
    pub grantee_team_id: Option<crate::teams::TeamId>,
    /// `grantee_workforce_id` — `Optional<agency.teams.WorkforceId>`.
    ///
    /// Carries `grantee_workforce`: `agency.auth.CredentialGrant` references one `agency.teams.Workforce`.
    pub grantee_workforce_id: Option<crate::teams::WorkforceId>,
    /// `purpose` — `agency.auth.GrantPurpose`.
    pub purpose: GrantPurpose,
    /// `expires_at` — `Optional<Timestamp>`.
    pub expires_at: Option<crate::primitives::Timestamp>,
}

impl CredentialGrantData {
    /// The first declared invariant of `agency.auth.CredentialGrant` this value breaks, as the specification declares it,
    /// or `None` when it breaks none.
    ///
    /// An invariant is broken only when it is false of this value. One that reads something
    /// absent — an empty `Optional`, a list position past the end, or `state`, which this
    /// type does not hold — decides nothing, as the conformance interpreter reads it.
    pub fn broken_invariant(&self) -> Option<&'static str> {
        use crate::primitives::invariant as iv;
        if iv::broken(iv::any([iv::all([Some(self.grantee_agent_id.as_ref().map(|v| &v.0).is_some()), iv::not(Some(self.grantee_team_id.as_ref().map(|v| &v.0).is_some())), iv::not(Some(self.grantee_workforce_id.as_ref().map(|v| &v.0).is_some()))]), iv::all([iv::not(Some(self.grantee_agent_id.as_ref().map(|v| &v.0).is_some())), Some(self.grantee_team_id.as_ref().map(|v| &v.0).is_some()), iv::not(Some(self.grantee_workforce_id.as_ref().map(|v| &v.0).is_some()))]), iv::all([iv::not(Some(self.grantee_agent_id.as_ref().map(|v| &v.0).is_some())), iv::not(Some(self.grantee_team_id.as_ref().map(|v| &v.0).is_some())), Some(self.grantee_workforce_id.as_ref().map(|v| &v.0).is_some())])])) {
            return Some("((defined(grantee_agent_id) and not (defined(grantee_team_id)) and not (defined(grantee_workforce_id))) or (not (defined(grantee_agent_id)) and defined(grantee_team_id) and not (defined(grantee_workforce_id))) or (not (defined(grantee_agent_id)) and not (defined(grantee_team_id)) and defined(grantee_workforce_id)))");
        }
        None
    }
}

/// The states of `agency.auth.CredentialGrant`, at the type level.
///
/// One marker type per declared state, sealed: a state the lifecycle does not declare cannot
/// implement [`Marker`](credential_grant_state::Marker), so [`CredentialGrant<S>`](CredentialGrant) can only ever rest in a real state.
pub mod credential_grant_state {
    /// Closes [`Marker`] over the declared states.
    mod sealed {
        /// Implemented only by the marker types beside this module.
        pub trait Sealed {}
        impl Sealed for super::Granted {}
        impl Sealed for super::Revoked {}
    }

    /// A declared state of `CredentialGrant`, as a type.
    pub trait Marker: sealed::Sealed {
        /// The same state, as the runtime value.
        const STATE: super::CredentialGrantState;
    }

    /// `Granted`. Where a new instance starts.
    pub struct Granted;

    impl Marker for Granted {
        const STATE: super::CredentialGrantState = super::CredentialGrantState::Granted;
    }

    /// `Revoked`. Terminal: an instance may rest here forever.
    pub struct Revoked;

    impl Marker for Revoked {
        const STATE: super::CredentialGrantState = super::CredentialGrantState::Revoked;
    }
}

/// CredentialGrant — `agency.auth.CredentialGrant` — with its lifecycle state carried by the type.
///
/// The one constructor rests in `Granted`, and the only way to change `S` is a method generated from
/// a declared transition. A move the specification does not declare is therefore not an error
/// case: it does not compile. Where the state is data — wire, storage — use [`CredentialGrantSnapshot`]
/// and [`CredentialGrantSnapshot::refine`].
pub struct CredentialGrant<S: credential_grant_state::Marker> {
    data: CredentialGrantData,
    state: core::marker::PhantomData<S>,
}

impl<S: credential_grant_state::Marker> CredentialGrant<S> {
    /// The state this instance rests in, as the runtime value.
    pub fn state(&self) -> CredentialGrantState {
        S::STATE
    }

    /// What it holds.
    pub fn data(&self) -> &CredentialGrantData {
        &self.data
    }

    /// Hands the data back, giving up the typed state.
    pub fn into_data(self) -> CredentialGrantData {
        self.data
    }
}

impl CredentialGrant<credential_grant_state::Granted> {
    /// A new instance, resting in `Granted` — the only state the lifecycle starts one in.
    pub fn new(data: CredentialGrantData) -> Self {
        Self {
            data,
            state: core::marker::PhantomData,
        }
    }
}

impl CredentialGrant<credential_grant_state::Granted> {
    /// `revoke` — `Granted` → `Revoked`. Taken by the `revoked` outcome of `agency.auth.RevokeGrant`.
    pub fn revoke(self) -> CredentialGrant<credential_grant_state::Revoked> {
        CredentialGrant {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }
}

/// `agency.auth.CredentialGrant` as it crosses a boundary: the state as a value beside the data.
///
/// Wire and storage know states only at runtime; [`CredentialGrantSnapshot::refine`] is the one door back
/// into the typed lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CredentialGrantSnapshot {
    /// Where the instance is in its lifecycle.
    pub state: CredentialGrantState,
    /// What it holds.
    pub data: CredentialGrantData,
}

/// An `CredentialGrant` in whichever declared state it was found.
pub enum AnyCredentialGrant {
    /// Resting in `Granted`.
    Granted(CredentialGrant<credential_grant_state::Granted>),
    /// Resting in `Revoked`.
    Revoked(CredentialGrant<credential_grant_state::Revoked>),
}

impl CredentialGrantSnapshot {
    /// Refines the runtime state into the typed one.
    ///
    /// Total: every declared state has an arm, and an undeclared state cannot reach here because
    /// `CredentialGrantState` cannot spell one.
    pub fn refine(self) -> AnyCredentialGrant {
        match self.state {
            CredentialGrantState::Granted => AnyCredentialGrant::Granted(CredentialGrant {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            CredentialGrantState::Revoked => AnyCredentialGrant::Revoked(CredentialGrant {
                data: self.data,
                state: core::marker::PhantomData,
            }),
        }
    }
}

impl AnyCredentialGrant {
    /// The state, as the runtime value.
    pub fn state(&self) -> CredentialGrantState {
        match self {
            Self::Granted(_) => CredentialGrantState::Granted,
            Self::Revoked(_) => CredentialGrantState::Revoked,
        }
    }

    /// Back to the boundary shape.
    pub fn snapshot(self) -> CredentialGrantSnapshot {
        match self {
            Self::Granted(instance) => CredentialGrantSnapshot {
                state: CredentialGrantState::Granted,
                data: instance.into_data(),
            },
            Self::Revoked(instance) => CredentialGrantSnapshot {
                state: CredentialGrantState::Revoked,
                data: instance.into_data(),
            },
        }
    }
}

/// What Policy — `agency.auth.Policy` — holds, apart from where it is in its lifecycle.
///
/// The identity and every declared field. The state is deliberately not one: inside the domain it
/// is carried by the type parameter of [`Policy<S>`], and at a boundary by [`PolicySnapshot::state`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyData {
    /// The identity: `policy_id` — `agency.auth.PolicyId`.
    pub policy_id: PolicyId,
    /// `tenant_id` — `agency.auth.TenantId`.
    ///
    /// Carries `policies`: `agency.auth.Tenant` owns many `agency.auth.Policy`.
    pub tenant_id: TenantId,
    /// `name` — `String`.
    pub name: String,
    /// `rules` — `List<agency.auth.Rule>`.
    pub rules: Vec<Rule>,
    /// `version` — `Integer`.
    pub version: i64,
    /// `digest` — `agency.shared.Digest`.
    pub digest: crate::shared::Digest,
}

/// The states of `agency.auth.Policy`, at the type level.
///
/// One marker type per declared state, sealed: a state the lifecycle does not declare cannot
/// implement [`Marker`](policy_state::Marker), so [`Policy<S>`](Policy) can only ever rest in a real state.
pub mod policy_state {
    /// Closes [`Marker`] over the declared states.
    mod sealed {
        /// Implemented only by the marker types beside this module.
        pub trait Sealed {}
        impl Sealed for super::Active {}
        impl Sealed for super::Draft {}
        impl Sealed for super::Superseded {}
    }

    /// A declared state of `Policy`, as a type.
    pub trait Marker: sealed::Sealed {
        /// The same state, as the runtime value.
        const STATE: super::PolicyState;
    }

    /// `Active`.
    pub struct Active;

    impl Marker for Active {
        const STATE: super::PolicyState = super::PolicyState::Active;
    }

    /// `Draft`. Where a new instance starts.
    pub struct Draft;

    impl Marker for Draft {
        const STATE: super::PolicyState = super::PolicyState::Draft;
    }

    /// `Superseded`. Terminal: an instance may rest here forever.
    pub struct Superseded;

    impl Marker for Superseded {
        const STATE: super::PolicyState = super::PolicyState::Superseded;
    }
}

/// Policy — `agency.auth.Policy` — with its lifecycle state carried by the type.
///
/// The one constructor rests in `Draft`, and the only way to change `S` is a method generated from
/// a declared transition. A move the specification does not declare is therefore not an error
/// case: it does not compile. Where the state is data — wire, storage — use [`PolicySnapshot`]
/// and [`PolicySnapshot::refine`].
pub struct Policy<S: policy_state::Marker> {
    data: PolicyData,
    state: core::marker::PhantomData<S>,
}

impl<S: policy_state::Marker> Policy<S> {
    /// The state this instance rests in, as the runtime value.
    pub fn state(&self) -> PolicyState {
        S::STATE
    }

    /// What it holds.
    pub fn data(&self) -> &PolicyData {
        &self.data
    }

    /// Hands the data back, giving up the typed state.
    pub fn into_data(self) -> PolicyData {
        self.data
    }
}

impl Policy<policy_state::Draft> {
    /// A new instance, resting in `Draft` — the only state the lifecycle starts one in.
    pub fn new(data: PolicyData) -> Self {
        Self {
            data,
            state: core::marker::PhantomData,
        }
    }
}

impl Policy<policy_state::Active> {
    /// `supersede` — `Active` → `Superseded`. Taken by the `superseded` outcome of `agency.auth.SupersedePolicy`.
    pub fn supersede(self) -> Policy<policy_state::Superseded> {
        Policy {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }
}

impl Policy<policy_state::Draft> {
    /// `activate` — `Draft` → `Active`. Taken by the `activated` outcome of `agency.auth.ActivatePolicy`.
    pub fn activate(self) -> Policy<policy_state::Active> {
        Policy {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }
}

/// `agency.auth.Policy` as it crosses a boundary: the state as a value beside the data.
///
/// Wire and storage know states only at runtime; [`PolicySnapshot::refine`] is the one door back
/// into the typed lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicySnapshot {
    /// Where the instance is in its lifecycle.
    pub state: PolicyState,
    /// What it holds.
    pub data: PolicyData,
}

/// An `Policy` in whichever declared state it was found.
pub enum AnyPolicy {
    /// Resting in `Active`.
    Active(Policy<policy_state::Active>),
    /// Resting in `Draft`.
    Draft(Policy<policy_state::Draft>),
    /// Resting in `Superseded`.
    Superseded(Policy<policy_state::Superseded>),
}

impl PolicySnapshot {
    /// Refines the runtime state into the typed one.
    ///
    /// Total: every declared state has an arm, and an undeclared state cannot reach here because
    /// `PolicyState` cannot spell one.
    pub fn refine(self) -> AnyPolicy {
        match self.state {
            PolicyState::Active => AnyPolicy::Active(Policy {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            PolicyState::Draft => AnyPolicy::Draft(Policy {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            PolicyState::Superseded => AnyPolicy::Superseded(Policy {
                data: self.data,
                state: core::marker::PhantomData,
            }),
        }
    }
}

impl AnyPolicy {
    /// The state, as the runtime value.
    pub fn state(&self) -> PolicyState {
        match self {
            Self::Active(_) => PolicyState::Active,
            Self::Draft(_) => PolicyState::Draft,
            Self::Superseded(_) => PolicyState::Superseded,
        }
    }

    /// Back to the boundary shape.
    pub fn snapshot(self) -> PolicySnapshot {
        match self {
            Self::Active(instance) => PolicySnapshot {
                state: PolicyState::Active,
                data: instance.into_data(),
            },
            Self::Draft(instance) => PolicySnapshot {
                state: PolicyState::Draft,
                data: instance.into_data(),
            },
            Self::Superseded(instance) => PolicySnapshot {
                state: PolicyState::Superseded,
                data: instance.into_data(),
            },
        }
    }
}

/// What Principal — `agency.auth.Principal` — holds, apart from where it is in its lifecycle.
///
/// The identity and every declared field. The state is deliberately not one: inside the domain it
/// is carried by the type parameter of [`Principal<S>`], and at a boundary by [`PrincipalSnapshot::state`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrincipalData {
    /// The identity: `principal_id` — `agency.auth.PrincipalId`.
    pub principal_id: PrincipalId,
    /// `tenant_id` — `agency.auth.TenantId`.
    ///
    /// Carries `principals`: `agency.auth.Tenant` owns many `agency.auth.Principal`.
    pub tenant_id: TenantId,
    /// `kind` — `agency.auth.PrincipalKind`.
    pub kind: PrincipalKind,
    /// `subject` — `String`.
    pub subject: String,
    /// `roles` — `List<agency.auth.Role>`.
    pub roles: Vec<Role>,
    /// `mandate_ref` — `Optional<String>`.
    pub mandate_ref: Option<String>,
}

/// The states of `agency.auth.Principal`, at the type level.
///
/// One marker type per declared state, sealed: a state the lifecycle does not declare cannot
/// implement [`Marker`](principal_state::Marker), so [`Principal<S>`](Principal) can only ever rest in a real state.
pub mod principal_state {
    /// Closes [`Marker`] over the declared states.
    mod sealed {
        /// Implemented only by the marker types beside this module.
        pub trait Sealed {}
        impl Sealed for super::Disabled {}
        impl Sealed for super::Invited {}
    }

    /// A declared state of `Principal`, as a type.
    pub trait Marker: sealed::Sealed {
        /// The same state, as the runtime value.
        const STATE: super::PrincipalState;
    }

    /// `Disabled`. Terminal: an instance may rest here forever.
    pub struct Disabled;

    impl Marker for Disabled {
        const STATE: super::PrincipalState = super::PrincipalState::Disabled;
    }

    /// `Invited`. Where a new instance starts.
    pub struct Invited;

    impl Marker for Invited {
        const STATE: super::PrincipalState = super::PrincipalState::Invited;
    }
}

/// Principal — `agency.auth.Principal` — with its lifecycle state carried by the type.
///
/// The one constructor rests in `Invited`, and the only way to change `S` is a method generated from
/// a declared transition. A move the specification does not declare is therefore not an error
/// case: it does not compile. Where the state is data — wire, storage — use [`PrincipalSnapshot`]
/// and [`PrincipalSnapshot::refine`].
pub struct Principal<S: principal_state::Marker> {
    data: PrincipalData,
    state: core::marker::PhantomData<S>,
}

impl<S: principal_state::Marker> Principal<S> {
    /// The state this instance rests in, as the runtime value.
    pub fn state(&self) -> PrincipalState {
        S::STATE
    }

    /// What it holds.
    pub fn data(&self) -> &PrincipalData {
        &self.data
    }

    /// Hands the data back, giving up the typed state.
    pub fn into_data(self) -> PrincipalData {
        self.data
    }
}

impl Principal<principal_state::Invited> {
    /// A new instance, resting in `Invited` — the only state the lifecycle starts one in.
    pub fn new(data: PrincipalData) -> Self {
        Self {
            data,
            state: core::marker::PhantomData,
        }
    }
}

impl Principal<principal_state::Invited> {
    /// `disable` — `Invited` → `Disabled`. Taken by the `disabled` outcome of `agency.auth.DisablePrincipal`.
    pub fn disable(self) -> Principal<principal_state::Disabled> {
        Principal {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }
}

/// `agency.auth.Principal` as it crosses a boundary: the state as a value beside the data.
///
/// Wire and storage know states only at runtime; [`PrincipalSnapshot::refine`] is the one door back
/// into the typed lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrincipalSnapshot {
    /// Where the instance is in its lifecycle.
    pub state: PrincipalState,
    /// What it holds.
    pub data: PrincipalData,
}

/// An `Principal` in whichever declared state it was found.
pub enum AnyPrincipal {
    /// Resting in `Disabled`.
    Disabled(Principal<principal_state::Disabled>),
    /// Resting in `Invited`.
    Invited(Principal<principal_state::Invited>),
}

impl PrincipalSnapshot {
    /// Refines the runtime state into the typed one.
    ///
    /// Total: every declared state has an arm, and an undeclared state cannot reach here because
    /// `PrincipalState` cannot spell one.
    pub fn refine(self) -> AnyPrincipal {
        match self.state {
            PrincipalState::Disabled => AnyPrincipal::Disabled(Principal {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            PrincipalState::Invited => AnyPrincipal::Invited(Principal {
                data: self.data,
                state: core::marker::PhantomData,
            }),
        }
    }
}

impl AnyPrincipal {
    /// The state, as the runtime value.
    pub fn state(&self) -> PrincipalState {
        match self {
            Self::Disabled(_) => PrincipalState::Disabled,
            Self::Invited(_) => PrincipalState::Invited,
        }
    }

    /// Back to the boundary shape.
    pub fn snapshot(self) -> PrincipalSnapshot {
        match self {
            Self::Disabled(instance) => PrincipalSnapshot {
                state: PrincipalState::Disabled,
                data: instance.into_data(),
            },
            Self::Invited(instance) => PrincipalSnapshot {
                state: PrincipalState::Invited,
                data: instance.into_data(),
            },
        }
    }
}

/// What Tenant — `agency.auth.Tenant` — holds, apart from where it is in its lifecycle.
///
/// The identity and every declared field. The state is deliberately not one: inside the domain it
/// is carried by the type parameter of [`Tenant<S>`], and at a boundary by [`TenantSnapshot::state`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TenantData {
    /// The identity: `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: TenantId,
    /// `name` — `String`.
    pub name: String,
    /// `mandate_tenant_ref` — `Optional<String>`.
    pub mandate_tenant_ref: Option<String>,
}

/// The states of `agency.auth.Tenant`, at the type level.
///
/// One marker type per declared state, sealed: a state the lifecycle does not declare cannot
/// implement [`Marker`](tenant_state::Marker), so [`Tenant<S>`](Tenant) can only ever rest in a real state.
pub mod tenant_state {
    /// Closes [`Marker`] over the declared states.
    mod sealed {
        /// Implemented only by the marker types beside this module.
        pub trait Sealed {}
        impl Sealed for super::Created {}
    }

    /// A declared state of `Tenant`, as a type.
    pub trait Marker: sealed::Sealed {
        /// The same state, as the runtime value.
        const STATE: super::TenantState;
    }

    /// `Created`. Where a new instance starts.
    pub struct Created;

    impl Marker for Created {
        const STATE: super::TenantState = super::TenantState::Created;
    }
}

/// Tenant — `agency.auth.Tenant` — with its lifecycle state carried by the type.
///
/// The one constructor rests in `Created`, and the only way to change `S` is a method generated from
/// a declared transition. A move the specification does not declare is therefore not an error
/// case: it does not compile. Where the state is data — wire, storage — use [`TenantSnapshot`]
/// and [`TenantSnapshot::refine`].
pub struct Tenant<S: tenant_state::Marker> {
    data: TenantData,
    state: core::marker::PhantomData<S>,
}

impl<S: tenant_state::Marker> Tenant<S> {
    /// The state this instance rests in, as the runtime value.
    pub fn state(&self) -> TenantState {
        S::STATE
    }

    /// What it holds.
    pub fn data(&self) -> &TenantData {
        &self.data
    }

    /// Hands the data back, giving up the typed state.
    pub fn into_data(self) -> TenantData {
        self.data
    }
}

impl Tenant<tenant_state::Created> {
    /// A new instance, resting in `Created` — the only state the lifecycle starts one in.
    pub fn new(data: TenantData) -> Self {
        Self {
            data,
            state: core::marker::PhantomData,
        }
    }
}

/// `agency.auth.Tenant` as it crosses a boundary: the state as a value beside the data.
///
/// Wire and storage know states only at runtime; [`TenantSnapshot::refine`] is the one door back
/// into the typed lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TenantSnapshot {
    /// Where the instance is in its lifecycle.
    pub state: TenantState,
    /// What it holds.
    pub data: TenantData,
}

/// An `Tenant` in whichever declared state it was found.
pub enum AnyTenant {
    /// Resting in `Created`.
    Created(Tenant<tenant_state::Created>),
}

impl TenantSnapshot {
    /// Refines the runtime state into the typed one.
    ///
    /// Total: every declared state has an arm, and an undeclared state cannot reach here because
    /// `TenantState` cannot spell one.
    pub fn refine(self) -> AnyTenant {
        match self.state {
            TenantState::Created => AnyTenant::Created(Tenant {
                data: self.data,
                state: core::marker::PhantomData,
            }),
        }
    }
}

impl AnyTenant {
    /// The state, as the runtime value.
    pub fn state(&self) -> TenantState {
        match self {
            Self::Created(_) => TenantState::Created,
        }
    }

    /// Back to the boundary shape.
    pub fn snapshot(self) -> TenantSnapshot {
        match self {
            Self::Created(instance) => TenantSnapshot {
                state: TenantState::Created,
                data: instance.into_data(),
            },
        }
    }
}

/// Activate a policy — the input of `agency.auth.ActivatePolicy`.
///
/// Everything it can result in is [`ActivatePolicyOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivatePolicy {
    /// `policy_id` — `agency.auth.PolicyId`.
    pub policy_id: PolicyId,
}

/// Everything `agency.auth.ActivatePolicy` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActivatePolicyOutcome {
    /// `no-such-policy` — for an identity no record carries.
    NoSuchPolicy {
        /// Why it was refused: `agency.auth.NotFound`.
        error: NotFound,
    },
    /// `activated` — otherwise.
    Activated {
        /// The `agency.auth.PolicyActivated` this outcome publishes.
        policy_activated: PolicyActivated,
    },
    /// `wrong-state` — from a state no declared move starts in.
    WrongState {
        /// Why it was refused: `agency.auth.StateConflict`.
        error: StateConflict,
    },
}

/// Assign a role — the input of `agency.auth.AssignRole`.
///
/// Everything it can result in is [`AssignRoleOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignRole {
    /// `principal_id` — `agency.auth.PrincipalId`.
    pub principal_id: PrincipalId,
    /// `role` — `agency.auth.Role`.
    pub role: Role,
}

/// Everything `agency.auth.AssignRole` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignRoleOutcome {
    /// `no-such-principal` — for an identity no record carries.
    NoSuchPrincipal {
        /// Why it was refused: `agency.auth.NotFound`.
        error: NotFound,
    },
    /// `assigned` — otherwise.
    Assigned {
        /// The `agency.auth.RoleAssigned` this outcome publishes.
        role_assigned: RoleAssigned,
    },
}

/// Create a policy — the input of `agency.auth.CreatePolicy`.
///
/// Everything it can result in is [`CreatePolicyOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatePolicy {
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: TenantId,
    /// `name` — `String`.
    pub name: String,
    /// `rules` — `List<agency.auth.Rule>`.
    pub rules: Vec<Rule>,
}

/// Everything `agency.auth.CreatePolicy` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreatePolicyOutcome {
    /// `created` — otherwise.
    Created {
        /// The `agency.auth.PolicyCreated` this outcome publishes.
        policy_created: PolicyCreated,
    },
}

/// Create a tenant — the input of `agency.auth.CreateTenant`.
///
/// Everything it can result in is [`CreateTenantOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateTenant {
    /// `name` — `String`.
    pub name: String,
    /// `mandate_tenant_ref` — `Optional<String>`.
    pub mandate_tenant_ref: Option<String>,
}

/// Everything `agency.auth.CreateTenant` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreateTenantOutcome {
    /// `created` — otherwise.
    Created {
        /// The `agency.auth.TenantCreated` this outcome publishes.
        tenant_created: TenantCreated,
    },
}

/// Disable a principal — the input of `agency.auth.DisablePrincipal`.
///
/// Everything it can result in is [`DisablePrincipalOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisablePrincipal {
    /// `principal_id` — `agency.auth.PrincipalId`.
    pub principal_id: PrincipalId,
}

/// Everything `agency.auth.DisablePrincipal` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DisablePrincipalOutcome {
    /// `no-such-principal` — for an identity no record carries.
    NoSuchPrincipal {
        /// Why it was refused: `agency.auth.NotFound`.
        error: NotFound,
    },
    /// `disabled` — otherwise.
    Disabled {
        /// The `agency.auth.PrincipalDisabled` this outcome publishes.
        principal_disabled: PrincipalDisabled,
    },
    /// `wrong-state` — from a state no declared move starts in.
    WrongState {
        /// Why it was refused: `agency.auth.StateConflict`.
        error: StateConflict,
    },
}

/// Grant a credential — the input of `agency.auth.GrantCredential`.
///
/// Everything it can result in is [`GrantCredentialOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrantCredential {
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: TenantId,
    /// `credential_id` — `agency.auth.CredentialId`.
    pub credential_id: CredentialId,
    /// `grantee_agent_id` — `Optional<agency.agents.AgentId>`.
    pub grantee_agent_id: Option<crate::agents::AgentId>,
    /// `grantee_team_id` — `Optional<agency.teams.TeamId>`.
    pub grantee_team_id: Option<crate::teams::TeamId>,
    /// `grantee_workforce_id` — `Optional<agency.teams.WorkforceId>`.
    pub grantee_workforce_id: Option<crate::teams::WorkforceId>,
    /// `purpose` — `agency.auth.GrantPurpose`.
    pub purpose: GrantPurpose,
    /// `expires_at` — `Optional<Timestamp>`.
    pub expires_at: Option<crate::primitives::Timestamp>,
}

/// Everything `agency.auth.GrantCredential` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrantCredentialOutcome {
    /// `granted-to-agent` — when `(defined(grantee_agent_id) and not (defined(grantee_team_id)) and not (defined(grantee_workforce_id)))`.
    GrantedToAgent {
        /// The `agency.auth.CredentialGranted` this outcome publishes.
        credential_granted: CredentialGranted,
    },
    /// `granted-to-team` — when `(not (defined(grantee_agent_id)) and defined(grantee_team_id) and not (defined(grantee_workforce_id)))`.
    GrantedToTeam {
        /// The `agency.auth.CredentialGranted` this outcome publishes.
        credential_granted: CredentialGranted,
    },
    /// `granted-to-workforce` — when `(not (defined(grantee_agent_id)) and not (defined(grantee_team_id)) and defined(grantee_workforce_id))`.
    GrantedToWorkforce {
        /// The `agency.auth.CredentialGranted` this outcome publishes.
        credential_granted: CredentialGranted,
    },
    /// `grantee-invalid` — otherwise.
    GranteeInvalid {
        /// Why it was refused: `agency.auth.GranteeInvalid`.
        error: GranteeInvalid,
    },
}

/// Invite a principal — the input of `agency.auth.InvitePrincipal`.
///
/// Everything it can result in is [`InvitePrincipalOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvitePrincipal {
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: TenantId,
    /// `kind` — `agency.auth.PrincipalKind`.
    pub kind: PrincipalKind,
    /// `subject` — `String`.
    pub subject: String,
    /// `roles` — `List<agency.auth.Role>`.
    pub roles: Vec<Role>,
    /// `mandate_ref` — `Optional<String>`.
    pub mandate_ref: Option<String>,
}

/// Everything `agency.auth.InvitePrincipal` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvitePrincipalOutcome {
    /// `invited` — otherwise.
    Invited {
        /// The `agency.auth.PrincipalInvited` this outcome publishes.
        principal_invited: PrincipalInvited,
    },
}

/// Refresh a credential — the input of `agency.auth.RefreshCredential`.
///
/// Everything it can result in is [`RefreshCredentialOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefreshCredential {
    /// `credential_id` — `agency.auth.CredentialId`.
    pub credential_id: CredentialId,
}

/// Everything `agency.auth.RefreshCredential` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefreshCredentialOutcome {
    /// `no-such-credential` — for an identity no record carries.
    NoSuchCredential {
        /// Why it was refused: `agency.auth.NotFound`.
        error: NotFound,
    },
    /// `revoked` — when the existing subject's stored fields satisfy `state == Revoked`.
    Revoked {
        /// Why it was refused: `agency.auth.CredentialUnusable`.
        error: CredentialUnusable,
    },
    /// `refreshed` — otherwise.
    Refreshed {
        /// The `agency.auth.CredentialRefreshed` this outcome publishes.
        credential_refreshed: CredentialRefreshed,
    },
}

/// Revoke a credential — the input of `agency.auth.RevokeCredential`.
///
/// Everything it can result in is [`RevokeCredentialOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevokeCredential {
    /// `credential_id` — `agency.auth.CredentialId`.
    pub credential_id: CredentialId,
}

/// Everything `agency.auth.RevokeCredential` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RevokeCredentialOutcome {
    /// `no-such-credential` — for an identity no record carries.
    NoSuchCredential {
        /// Why it was refused: `agency.auth.NotFound`.
        error: NotFound,
    },
    /// `revoked` — otherwise.
    Revoked {
        /// The `agency.auth.CredentialRevoked` this outcome publishes.
        credential_revoked: CredentialRevoked,
    },
    /// `wrong-state` — from a state no declared move starts in.
    WrongState {
        /// Why it was refused: `agency.auth.StateConflict`.
        error: StateConflict,
    },
}

/// Revoke a grant — the input of `agency.auth.RevokeGrant`.
///
/// Everything it can result in is [`RevokeGrantOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevokeGrant {
    /// `grant_id` — `agency.auth.GrantId`.
    pub grant_id: GrantId,
}

/// Everything `agency.auth.RevokeGrant` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RevokeGrantOutcome {
    /// `no-such-grant` — for an identity no record carries.
    NoSuchGrant {
        /// Why it was refused: `agency.auth.NotFound`.
        error: NotFound,
    },
    /// `revoked` — otherwise.
    Revoked {
        /// The `agency.auth.GrantRevoked` this outcome publishes.
        grant_revoked: GrantRevoked,
    },
    /// `wrong-state` — from a state no declared move starts in.
    WrongState {
        /// Why it was refused: `agency.auth.StateConflict`.
        error: StateConflict,
    },
}

/// Store a credential — the input of `agency.auth.StoreCredential`.
///
/// Everything it can result in is [`StoreCredentialOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreCredential {
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: TenantId,
    /// `kind` — `agency.auth.CredentialKind`.
    pub kind: CredentialKind,
    /// `custody` — `agency.auth.Custody`.
    pub custody: Custody,
    /// `scope` — `agency.auth.CredentialScope`.
    pub scope: CredentialScope,
    /// `owner_principal_id` — `agency.auth.PrincipalId`.
    pub owner_principal_id: PrincipalId,
    /// `expires_at` — `Optional<Timestamp>`.
    pub expires_at: Option<crate::primitives::Timestamp>,
}

/// Everything `agency.auth.StoreCredential` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreCredentialOutcome {
    /// `stored` — otherwise.
    Stored {
        /// The `agency.auth.CredentialStored` this outcome publishes.
        credential_stored: CredentialStored,
    },
}

/// Supersede a policy — the input of `agency.auth.SupersedePolicy`.
///
/// Everything it can result in is [`SupersedePolicyOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupersedePolicy {
    /// `policy_id` — `agency.auth.PolicyId`.
    pub policy_id: PolicyId,
}

/// Everything `agency.auth.SupersedePolicy` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SupersedePolicyOutcome {
    /// `no-such-policy` — for an identity no record carries.
    NoSuchPolicy {
        /// Why it was refused: `agency.auth.NotFound`.
        error: NotFound,
    },
    /// `superseded` — otherwise.
    Superseded {
        /// The `agency.auth.PolicySuperseded` this outcome publishes.
        policy_superseded: PolicySuperseded,
    },
    /// `wrong-state` — from a state no declared move starts in.
    WrongState {
        /// Why it was refused: `agency.auth.StateConflict`.
        error: StateConflict,
    },
}

/// CredentialGranted — the event `agency.auth.CredentialGranted`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CredentialGranted {
    /// `grant_id` — `agency.auth.GrantId`.
    pub grant_id: GrantId,
    /// `credential_id` — `agency.auth.CredentialId`.
    pub credential_id: CredentialId,
    /// `purpose` — `agency.auth.GrantPurpose`.
    pub purpose: GrantPurpose,
}

/// CredentialRefreshed — the event `agency.auth.CredentialRefreshed`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CredentialRefreshed {
    /// `credential_id` — `agency.auth.CredentialId`.
    pub credential_id: CredentialId,
}

/// CredentialRevoked — the event `agency.auth.CredentialRevoked`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CredentialRevoked {
    /// `credential_id` — `agency.auth.CredentialId`.
    pub credential_id: CredentialId,
}

/// CredentialStored — the event `agency.auth.CredentialStored`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CredentialStored {
    /// `credential_id` — `agency.auth.CredentialId`.
    pub credential_id: CredentialId,
    /// `kind` — `agency.auth.CredentialKind`.
    pub kind: CredentialKind,
    /// `custody` — `agency.auth.Custody`.
    pub custody: Custody,
}

/// GrantRevoked — the event `agency.auth.GrantRevoked`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrantRevoked {
    /// `grant_id` — `agency.auth.GrantId`.
    pub grant_id: GrantId,
}

/// PolicyActivated — the event `agency.auth.PolicyActivated`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyActivated {
    /// `policy_id` — `agency.auth.PolicyId`.
    pub policy_id: PolicyId,
}

/// PolicyCreated — the event `agency.auth.PolicyCreated`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyCreated {
    /// `policy_id` — `agency.auth.PolicyId`.
    pub policy_id: PolicyId,
    /// `name` — `String`.
    pub name: String,
}

/// PolicySuperseded — the event `agency.auth.PolicySuperseded`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicySuperseded {
    /// `policy_id` — `agency.auth.PolicyId`.
    pub policy_id: PolicyId,
}

/// PrincipalDisabled — the event `agency.auth.PrincipalDisabled`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrincipalDisabled {
    /// `principal_id` — `agency.auth.PrincipalId`.
    pub principal_id: PrincipalId,
}

/// PrincipalInvited — the event `agency.auth.PrincipalInvited`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrincipalInvited {
    /// `principal_id` — `agency.auth.PrincipalId`.
    pub principal_id: PrincipalId,
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: TenantId,
    /// `kind` — `agency.auth.PrincipalKind`.
    pub kind: PrincipalKind,
    /// `subject` — `String`.
    pub subject: String,
}

/// RoleAssigned — the event `agency.auth.RoleAssigned`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleAssigned {
    /// `principal_id` — `agency.auth.PrincipalId`.
    pub principal_id: PrincipalId,
    /// `role` — `agency.auth.Role`.
    pub role: Role,
}

/// TenantCreated — the event `agency.auth.TenantCreated`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TenantCreated {
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: TenantId,
    /// `name` — `String`.
    pub name: String,
}

/// The declared error `agency.auth.CredentialUnusable`.
///
/// The credential is revoked, so it is not refreshed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CredentialUnusable;

/// The declared error `agency.auth.GranteeInvalid`.
///
/// A grant names no grantee or more than one, and nothing changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GranteeInvalid;

/// The declared error `agency.auth.NotFound`.
///
/// The addressed record does not exist, and nothing changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotFound;

/// The declared error `agency.auth.StateConflict`.
///
/// The addressed record is not in a state this command acts from, and nothing changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateConflict;

/// CredentialList — one row of the view `agency.auth.CredentialList`.
///
/// Projects `agency.auth.Credential` at `read_your_writes` consistency.
/// The specification fully determines every row, so its query is generated over the storage port —
/// see the plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CredentialList {
    /// `credential_id` — `agency.auth.CredentialId`.
    pub credential_id: CredentialId,
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: TenantId,
    /// `kind` — `agency.auth.CredentialKind`.
    pub kind: CredentialKind,
    /// `custody` — `agency.auth.Custody`.
    pub custody: Custody,
    /// `scope` — `agency.auth.CredentialScope`.
    pub scope: CredentialScope,
    /// `owner_principal_id` — `agency.auth.PrincipalId`.
    pub owner_principal_id: PrincipalId,
    /// `expires_at` — `Optional<Timestamp>`.
    pub expires_at: Option<crate::primitives::Timestamp>,
    /// `last_refreshed_at` — `Timestamp`.
    pub last_refreshed_at: crate::primitives::Timestamp,
    /// `state` — `agency.auth.Credential.State`.
    pub state: CredentialState,
}

/// EffectivePolicy — one row of the view `agency.auth.EffectivePolicy`.
///
/// Projects `agency.agents.AgentDefinition` at `read_your_writes` consistency, containing instances where `definition_id == param.definition_id`.
/// Serving it is an implementation obligation — see the plan — because how a projection is kept
/// current is a storage decision the specification does not take.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectivePolicy {
    /// `definition_id` — `agency.agents.DefinitionId`.
    pub definition_id: crate::agents::DefinitionId,
    /// `policy_id` — `agency.auth.PolicyId`.
    pub policy_id: PolicyId,
    /// `policy_version` — `Optional<Integer>`.
    pub policy_version: Option<i64>,
    /// `policy_digest` — `Optional<agency.shared.Digest>`.
    pub policy_digest: Option<crate::shared::Digest>,
    /// `rules` — `Optional<List<agency.auth.Rule>>`.
    pub rules: Option<Vec<Rule>>,
    /// `policy_state` — `Optional<agency.auth.Policy.State>`.
    pub policy_state: Option<PolicyState>,
}

/// GrantMatrix — one row of the view `agency.auth.GrantMatrix`.
///
/// Projects `agency.auth.CredentialGrant` at `read_your_writes` consistency.
/// The specification fully determines every row, so its query is generated over the storage port —
/// see the plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrantMatrix {
    /// `grant_id` — `agency.auth.GrantId`.
    pub grant_id: GrantId,
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: TenantId,
    /// `credential_id` — `agency.auth.CredentialId`.
    pub credential_id: CredentialId,
    /// `grantee_agent_id` — `Optional<agency.agents.AgentId>`.
    pub grantee_agent_id: Option<crate::agents::AgentId>,
    /// `grantee_team_id` — `Optional<agency.teams.TeamId>`.
    pub grantee_team_id: Option<crate::teams::TeamId>,
    /// `grantee_workforce_id` — `Optional<agency.teams.WorkforceId>`.
    pub grantee_workforce_id: Option<crate::teams::WorkforceId>,
    /// `purpose` — `agency.auth.GrantPurpose`.
    pub purpose: GrantPurpose,
    /// `expires_at` — `Optional<Timestamp>`.
    pub expires_at: Option<crate::primitives::Timestamp>,
    /// `state` — `agency.auth.CredentialGrant.State`.
    pub state: CredentialGrantState,
}

/// PolicyList — one row of the view `agency.auth.PolicyList`.
///
/// Projects `agency.auth.Policy` at `read_your_writes` consistency.
/// The specification fully determines every row, so its query is generated over the storage port —
/// see the plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyList {
    /// `policy_id` — `agency.auth.PolicyId`.
    pub policy_id: PolicyId,
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: TenantId,
    /// `name` — `String`.
    pub name: String,
    /// `rules` — `List<agency.auth.Rule>`.
    pub rules: Vec<Rule>,
    /// `version` — `Integer`.
    pub version: i64,
    /// `digest` — `agency.shared.Digest`.
    pub digest: crate::shared::Digest,
    /// `state` — `agency.auth.Policy.State`.
    pub state: PolicyState,
}

/// PrincipalList — one row of the view `agency.auth.PrincipalList`.
///
/// Projects `agency.auth.Principal` at `read_your_writes` consistency.
/// The specification fully determines every row, so its query is generated over the storage port —
/// see the plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrincipalList {
    /// `principal_id` — `agency.auth.PrincipalId`.
    pub principal_id: PrincipalId,
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: TenantId,
    /// `kind` — `agency.auth.PrincipalKind`.
    pub kind: PrincipalKind,
    /// `subject` — `String`.
    pub subject: String,
    /// `roles` — `List<agency.auth.Role>`.
    pub roles: Vec<Role>,
    /// `mandate_ref` — `Optional<String>`.
    pub mandate_ref: Option<String>,
    /// `state` — `agency.auth.Principal.State`.
    pub state: PrincipalState,
}

/// What this bounded context owes its implementor, and the seams of what is generated.
///
/// One trait per obligation in the synthesis plan, each carrying the plan's own contract, and one
/// per generated behaviour, which [`Generated`](crate::behaviour::Generated) implements.
/// [`Unimplemented`](obligations::Unimplemented) satisfies every owed trait by refusing in the type system.
pub mod obligations {
    /// The behaviour `agency.auth.ActivatePolicy` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait ActivatePolicyBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.auth.ActivatePolicy`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn activate_policy(&mut self, input: super::ActivatePolicy) -> Result<super::ActivatePolicyOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.auth.AssignRole` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait AssignRoleBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.auth.AssignRole`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn assign_role(&mut self, input: super::AssignRole) -> Result<super::AssignRoleOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.auth.CreatePolicy` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait CreatePolicyBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.auth.CreatePolicy`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn create_policy(&mut self, input: super::CreatePolicy) -> Result<super::CreatePolicyOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.auth.CreateTenant` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait CreateTenantBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.auth.CreateTenant`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn create_tenant(&mut self, input: super::CreateTenant) -> Result<super::CreateTenantOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.auth.DisablePrincipal` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait DisablePrincipalBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.auth.DisablePrincipal`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn disable_principal(&mut self, input: super::DisablePrincipal) -> Result<super::DisablePrincipalOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.auth.GrantCredential` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait GrantCredentialBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.auth.GrantCredential`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn grant_credential(&mut self, input: super::GrantCredential) -> Result<super::GrantCredentialOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.auth.InvitePrincipal` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait InvitePrincipalBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.auth.InvitePrincipal`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn invite_principal(&mut self, input: super::InvitePrincipal) -> Result<super::InvitePrincipalOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.auth.RefreshCredential` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait RefreshCredentialBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.auth.RefreshCredential`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn refresh_credential(&mut self, input: super::RefreshCredential) -> Result<super::RefreshCredentialOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.auth.RevokeCredential` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait RevokeCredentialBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.auth.RevokeCredential`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn revoke_credential(&mut self, input: super::RevokeCredential) -> Result<super::RevokeCredentialOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.auth.RevokeGrant` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait RevokeGrantBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.auth.RevokeGrant`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn revoke_grant(&mut self, input: super::RevokeGrant) -> Result<super::RevokeGrantOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.auth.StoreCredential` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait StoreCredentialBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.auth.StoreCredential`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn store_credential(&mut self, input: super::StoreCredential) -> Result<super::StoreCredentialOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.auth.SupersedePolicy` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait SupersedePolicyBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.auth.SupersedePolicy`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn supersede_policy(&mut self, input: super::SupersedePolicy) -> Result<super::SupersedePolicyOutcome, crate::obligation::UnmetObligation>;
    }

    /// The query `agency.auth.CredentialList` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage port. Implement it yourself to replace that query.
    pub trait CredentialListQuery {
        /// Serves `agency.auth.CredentialList` rows at the view's declared consistency.
        ///
        /// `Err` is the typed refusal of a row whose declared type cannot hold its value.
        fn credential_list(&self) -> Result<Vec<super::CredentialList>, crate::obligation::UnmetObligation>;
    }

    /// The query `agency.auth.EffectivePolicy` — an implementation obligation.
    ///
    /// Why it is not generated: kept an obligation by a read through the reference `policy_id` to `agency.auth.Policy` (`field: {related: …}` or `policy_id.<field>` in `filter:`), which reads a second entity's rows.
    ///
    /// Contract: a query answering `agency.auth.EffectivePolicy` with rows projected from `agency.agents.AgentDefinition` at `read_your_writes` consistency, containing instances where `definition_id == param.definition_id`.
    pub trait EffectivePolicyQuery {
        /// Serves `agency.auth.EffectivePolicy` rows at the view's declared consistency.
        ///
        /// `Err` is the typed refusal of an obligation nothing has satisfied; a satisfying
        /// implementation never returns it.
        fn effective_policy(&self, definition_id: crate::agents::DefinitionId) -> Result<Vec<super::EffectivePolicy>, crate::obligation::UnmetObligation>;
    }

    /// The query `agency.auth.GrantMatrix` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage port. Implement it yourself to replace that query.
    pub trait GrantMatrixQuery {
        /// Serves `agency.auth.GrantMatrix` rows at the view's declared consistency.
        ///
        /// `Err` is the typed refusal of a row whose declared type cannot hold its value.
        fn grant_matrix(&self) -> Result<Vec<super::GrantMatrix>, crate::obligation::UnmetObligation>;
    }

    /// The query `agency.auth.PolicyList` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage port. Implement it yourself to replace that query.
    pub trait PolicyListQuery {
        /// Serves `agency.auth.PolicyList` rows at the view's declared consistency.
        ///
        /// `Err` is the typed refusal of a row whose declared type cannot hold its value.
        fn policy_list(&self) -> Result<Vec<super::PolicyList>, crate::obligation::UnmetObligation>;
    }

    /// The query `agency.auth.PrincipalList` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage port. Implement it yourself to replace that query.
    pub trait PrincipalListQuery {
        /// Serves `agency.auth.PrincipalList` rows at the view's declared consistency.
        ///
        /// `Err` is the typed refusal of a row whose declared type cannot hold its value.
        fn principal_list(&self) -> Result<Vec<super::PrincipalList>, crate::obligation::UnmetObligation>;
    }

    /// Every obligation of this bounded context, refused in the type system.
    ///
    /// Each method returns the typed refusal naming what is owed — never a panic, never a guessed
    /// value — so a workspace built on this stub compiles and reports its own gaps.
    pub struct Unimplemented;

    impl EffectivePolicyQuery for Unimplemented {
        fn effective_policy(&self, _definition_id: crate::agents::DefinitionId) -> Result<Vec<super::EffectivePolicy>, crate::obligation::UnmetObligation> {
            Err(crate::obligation::UnmetObligation { capability: "view query", source: "agency.auth.EffectivePolicy" })
        }
    }
}
