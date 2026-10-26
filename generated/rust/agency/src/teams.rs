// generated from agency v1
// model digest 71132e90bde806415603d08bb97a71629b2c2f6cda30e1f22e76f407abd9a3eb
// contract digest a96f55e3e1ca68aeb1579b032c3547fffd27e8ab80e76bcb898f9b6c9258792e
// do not edit: regenerate with `ess synthesize --layout crate`

//! Teams — `agency.teams`.
//!
//! Teams of agents and the messages they exchange, the swarm records Agency holds a stop lever for, and workforces that say who is on duty for which queue.
//!
//! Everything this bounded context declares that the synthesis plan marks generated.

/// Capacity — `agency.teams.Capacity`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capacity {
    /// `max_concurrent_runs` — `Integer`.
    pub max_concurrent_runs: i64,
    /// `max_tokens_per_hour` — `Optional<Integer>`.
    pub max_tokens_per_hour: Option<i64>,
}

/// The states of `agency.teams.Swarm`, as runtime values.
///
/// Synthesised from the lifecycle, so the two cannot disagree. Which *moves* are legal is not
/// carried here — it is carried by `Swarm<S>`, where an undeclared move does not compile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwarmState {
    /// `Declared`.
    Declared,
    /// `Stopped`.
    Stopped,
}

/// SwarmId — `agency.teams.SwarmId`: a distinct wrapper around `Uuid`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwarmId(pub crate::primitives::Uuid);

/// The states of `agency.teams.Team`, as runtime values.
///
/// Synthesised from the lifecycle, so the two cannot disagree. Which *moves* are legal is not
/// carried here — it is carried by `Team<S>`, where an undeclared move does not compile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TeamState {
    /// `Disbanded`.
    Disbanded,
    /// `Formed`.
    Formed,
}

/// TeamId — `agency.teams.TeamId`: a distinct wrapper around `Uuid`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamId(pub crate::primitives::Uuid);

/// The states of `agency.teams.Workforce`, as runtime values.
///
/// Synthesised from the lifecycle, so the two cannot disagree. Which *moves* are legal is not
/// carried here — it is carried by `Workforce<S>`, where an undeclared move does not compile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkforceState {
    /// `Defined`.
    Defined,
    /// `Retired`.
    Retired,
    /// `Staffed`.
    Staffed,
}

/// WorkforceId — `agency.teams.WorkforceId`: a distinct wrapper around `Uuid`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkforceId(pub crate::primitives::Uuid);

/// What Swarm — `agency.teams.Swarm` — holds, apart from where it is in its lifecycle.
///
/// The identity and every declared field. The state is deliberately not one: inside the domain it
/// is carried by the type parameter of [`Swarm<S>`], and at a boundary by [`SwarmSnapshot::state`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwarmData {
    /// The identity: `swarm_id` — `agency.teams.SwarmId`.
    pub swarm_id: SwarmId,
    /// `tenant_id` — `agency.auth.TenantId`.
    ///
    /// Carries `swarms`: `agency.auth.Tenant` owns many `agency.teams.Swarm`.
    pub tenant_id: crate::auth::TenantId,
    /// `goal_ref` — `String`.
    pub goal_ref: String,
    /// `coordinator_agent_id` — `agency.agents.AgentId`.
    ///
    /// Carries `coordinator`: `agency.teams.Swarm` references one `agency.agents.Agent`.
    pub coordinator_agent_id: crate::agents::AgentId,
    /// `frame_digest` — `agency.shared.Digest`.
    pub frame_digest: crate::shared::Digest,
    /// `max_workers` — `Integer`.
    pub max_workers: i64,
    /// `team_id` — `agency.teams.TeamId`.
    ///
    /// Carries `team`: `agency.teams.Swarm` references one `agency.teams.Team`.
    pub team_id: TeamId,
    /// `work_input_id` — `agency.work.InputId`.
    ///
    /// Carries `work_input`: `agency.teams.Swarm` references one `agency.work.WorkInput`.
    pub work_input_id: crate::work::InputId,
}

/// The states of `agency.teams.Swarm`, at the type level.
///
/// One marker type per declared state, sealed: a state the lifecycle does not declare cannot
/// implement [`Marker`](swarm_state::Marker), so [`Swarm<S>`](Swarm) can only ever rest in a real state.
pub mod swarm_state {
    /// Closes [`Marker`] over the declared states.
    mod sealed {
        /// Implemented only by the marker types beside this module.
        pub trait Sealed {}
        impl Sealed for super::Declared {}
        impl Sealed for super::Stopped {}
    }

    /// A declared state of `Swarm`, as a type.
    pub trait Marker: sealed::Sealed {
        /// The same state, as the runtime value.
        const STATE: super::SwarmState;
    }

    /// `Declared`. Where a new instance starts.
    pub struct Declared;

    impl Marker for Declared {
        const STATE: super::SwarmState = super::SwarmState::Declared;
    }

    /// `Stopped`. Terminal: an instance may rest here forever.
    pub struct Stopped;

    impl Marker for Stopped {
        const STATE: super::SwarmState = super::SwarmState::Stopped;
    }
}

/// Swarm — `agency.teams.Swarm` — with its lifecycle state carried by the type.
///
/// The one constructor rests in `Declared`, and the only way to change `S` is a method generated from
/// a declared transition. A move the specification does not declare is therefore not an error
/// case: it does not compile. Where the state is data — wire, storage — use [`SwarmSnapshot`]
/// and [`SwarmSnapshot::refine`].
pub struct Swarm<S: swarm_state::Marker> {
    data: SwarmData,
    state: core::marker::PhantomData<S>,
}

impl<S: swarm_state::Marker> Swarm<S> {
    /// The state this instance rests in, as the runtime value.
    pub fn state(&self) -> SwarmState {
        S::STATE
    }

    /// What it holds.
    pub fn data(&self) -> &SwarmData {
        &self.data
    }

    /// Hands the data back, giving up the typed state.
    pub fn into_data(self) -> SwarmData {
        self.data
    }
}

impl Swarm<swarm_state::Declared> {
    /// A new instance, resting in `Declared` — the only state the lifecycle starts one in.
    pub fn new(data: SwarmData) -> Self {
        Self {
            data,
            state: core::marker::PhantomData,
        }
    }
}

impl Swarm<swarm_state::Declared> {
    /// `stop` — `Declared` → `Stopped`. Taken by the `stopped` outcome of `agency.teams.StopSwarm`.
    pub fn stop(self) -> Swarm<swarm_state::Stopped> {
        Swarm {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }
}

/// `agency.teams.Swarm` as it crosses a boundary: the state as a value beside the data.
///
/// Wire and storage know states only at runtime; [`SwarmSnapshot::refine`] is the one door back
/// into the typed lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwarmSnapshot {
    /// Where the instance is in its lifecycle.
    pub state: SwarmState,
    /// What it holds.
    pub data: SwarmData,
}

/// An `Swarm` in whichever declared state it was found.
pub enum AnySwarm {
    /// Resting in `Declared`.
    Declared(Swarm<swarm_state::Declared>),
    /// Resting in `Stopped`.
    Stopped(Swarm<swarm_state::Stopped>),
}

impl SwarmSnapshot {
    /// Refines the runtime state into the typed one.
    ///
    /// Total: every declared state has an arm, and an undeclared state cannot reach here because
    /// `SwarmState` cannot spell one.
    pub fn refine(self) -> AnySwarm {
        match self.state {
            SwarmState::Declared => AnySwarm::Declared(Swarm {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            SwarmState::Stopped => AnySwarm::Stopped(Swarm {
                data: self.data,
                state: core::marker::PhantomData,
            }),
        }
    }
}

impl AnySwarm {
    /// The state, as the runtime value.
    pub fn state(&self) -> SwarmState {
        match self {
            Self::Declared(_) => SwarmState::Declared,
            Self::Stopped(_) => SwarmState::Stopped,
        }
    }

    /// Back to the boundary shape.
    pub fn snapshot(self) -> SwarmSnapshot {
        match self {
            Self::Declared(instance) => SwarmSnapshot {
                state: SwarmState::Declared,
                data: instance.into_data(),
            },
            Self::Stopped(instance) => SwarmSnapshot {
                state: SwarmState::Stopped,
                data: instance.into_data(),
            },
        }
    }
}

/// What Team — `agency.teams.Team` — holds, apart from where it is in its lifecycle.
///
/// The identity and every declared field. The state is deliberately not one: inside the domain it
/// is carried by the type parameter of [`Team<S>`], and at a boundary by [`TeamSnapshot::state`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamData {
    /// The identity: `team_id` — `agency.teams.TeamId`.
    pub team_id: TeamId,
    /// `tenant_id` — `agency.auth.TenantId`.
    ///
    /// Carries `teams`: `agency.auth.Tenant` owns many `agency.teams.Team`.
    pub tenant_id: crate::auth::TenantId,
    /// `name` — `String`.
    pub name: String,
    /// `lead_agent_id` — `agency.agents.AgentId`.
    ///
    /// Carries `lead`: `agency.teams.Team` references one `agency.agents.Agent`.
    pub lead_agent_id: crate::agents::AgentId,
    /// `members` — `List<agency.agents.AgentId>`.
    ///
    /// Carries `member_agents`: `agency.teams.Team` references many `agency.agents.Agent`.
    pub members: Vec<crate::agents::AgentId>,
    /// `broker_ref` — `String`.
    pub broker_ref: String,
}

/// The states of `agency.teams.Team`, at the type level.
///
/// One marker type per declared state, sealed: a state the lifecycle does not declare cannot
/// implement [`Marker`](team_state::Marker), so [`Team<S>`](Team) can only ever rest in a real state.
pub mod team_state {
    /// Closes [`Marker`] over the declared states.
    mod sealed {
        /// Implemented only by the marker types beside this module.
        pub trait Sealed {}
        impl Sealed for super::Disbanded {}
        impl Sealed for super::Formed {}
    }

    /// A declared state of `Team`, as a type.
    pub trait Marker: sealed::Sealed {
        /// The same state, as the runtime value.
        const STATE: super::TeamState;
    }

    /// `Disbanded`. Terminal: an instance may rest here forever.
    pub struct Disbanded;

    impl Marker for Disbanded {
        const STATE: super::TeamState = super::TeamState::Disbanded;
    }

    /// `Formed`. Where a new instance starts.
    pub struct Formed;

    impl Marker for Formed {
        const STATE: super::TeamState = super::TeamState::Formed;
    }
}

/// Team — `agency.teams.Team` — with its lifecycle state carried by the type.
///
/// The one constructor rests in `Formed`, and the only way to change `S` is a method generated from
/// a declared transition. A move the specification does not declare is therefore not an error
/// case: it does not compile. Where the state is data — wire, storage — use [`TeamSnapshot`]
/// and [`TeamSnapshot::refine`].
pub struct Team<S: team_state::Marker> {
    data: TeamData,
    state: core::marker::PhantomData<S>,
}

impl<S: team_state::Marker> Team<S> {
    /// The state this instance rests in, as the runtime value.
    pub fn state(&self) -> TeamState {
        S::STATE
    }

    /// What it holds.
    pub fn data(&self) -> &TeamData {
        &self.data
    }

    /// Hands the data back, giving up the typed state.
    pub fn into_data(self) -> TeamData {
        self.data
    }
}

impl Team<team_state::Formed> {
    /// A new instance, resting in `Formed` — the only state the lifecycle starts one in.
    pub fn new(data: TeamData) -> Self {
        Self {
            data,
            state: core::marker::PhantomData,
        }
    }
}

impl Team<team_state::Formed> {
    /// `disband` — `Formed` → `Disbanded`. Taken by the `disbanded` outcome of `agency.teams.DisbandTeam`.
    pub fn disband(self) -> Team<team_state::Disbanded> {
        Team {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }
}

/// `agency.teams.Team` as it crosses a boundary: the state as a value beside the data.
///
/// Wire and storage know states only at runtime; [`TeamSnapshot::refine`] is the one door back
/// into the typed lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamSnapshot {
    /// Where the instance is in its lifecycle.
    pub state: TeamState,
    /// What it holds.
    pub data: TeamData,
}

/// An `Team` in whichever declared state it was found.
pub enum AnyTeam {
    /// Resting in `Disbanded`.
    Disbanded(Team<team_state::Disbanded>),
    /// Resting in `Formed`.
    Formed(Team<team_state::Formed>),
}

impl TeamSnapshot {
    /// Refines the runtime state into the typed one.
    ///
    /// Total: every declared state has an arm, and an undeclared state cannot reach here because
    /// `TeamState` cannot spell one.
    pub fn refine(self) -> AnyTeam {
        match self.state {
            TeamState::Disbanded => AnyTeam::Disbanded(Team {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            TeamState::Formed => AnyTeam::Formed(Team {
                data: self.data,
                state: core::marker::PhantomData,
            }),
        }
    }
}

impl AnyTeam {
    /// The state, as the runtime value.
    pub fn state(&self) -> TeamState {
        match self {
            Self::Disbanded(_) => TeamState::Disbanded,
            Self::Formed(_) => TeamState::Formed,
        }
    }

    /// Back to the boundary shape.
    pub fn snapshot(self) -> TeamSnapshot {
        match self {
            Self::Disbanded(instance) => TeamSnapshot {
                state: TeamState::Disbanded,
                data: instance.into_data(),
            },
            Self::Formed(instance) => TeamSnapshot {
                state: TeamState::Formed,
                data: instance.into_data(),
            },
        }
    }
}

/// What Workforce — `agency.teams.Workforce` — holds, apart from where it is in its lifecycle.
///
/// The identity and every declared field. The state is deliberately not one: inside the domain it
/// is carried by the type parameter of [`Workforce<S>`], and at a boundary by [`WorkforceSnapshot::state`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkforceData {
    /// The identity: `workforce_id` — `agency.teams.WorkforceId`.
    pub workforce_id: WorkforceId,
    /// `tenant_id` — `agency.auth.TenantId`.
    ///
    /// Carries `workforces`: `agency.auth.Tenant` owns many `agency.teams.Workforce`.
    pub tenant_id: crate::auth::TenantId,
    /// `name` — `String`.
    pub name: String,
    /// `teams` — `List<agency.teams.TeamId>`.
    ///
    /// Carries `staffed_teams`: `agency.teams.Workforce` references many `agency.teams.Team`.
    pub teams: Vec<TeamId>,
    /// `solo_agents` — `List<agency.agents.AgentId>`.
    ///
    /// Carries `agents`: `agency.teams.Workforce` references many `agency.agents.Agent`.
    pub solo_agents: Vec<crate::agents::AgentId>,
    /// `capacity` — `agency.teams.Capacity`.
    pub capacity: Capacity,
}

/// The states of `agency.teams.Workforce`, at the type level.
///
/// One marker type per declared state, sealed: a state the lifecycle does not declare cannot
/// implement [`Marker`](workforce_state::Marker), so [`Workforce<S>`](Workforce) can only ever rest in a real state.
pub mod workforce_state {
    /// Closes [`Marker`] over the declared states.
    mod sealed {
        /// Implemented only by the marker types beside this module.
        pub trait Sealed {}
        impl Sealed for super::Defined {}
        impl Sealed for super::Retired {}
        impl Sealed for super::Staffed {}
    }

    /// A declared state of `Workforce`, as a type.
    pub trait Marker: sealed::Sealed {
        /// The same state, as the runtime value.
        const STATE: super::WorkforceState;
    }

    /// `Defined`. Where a new instance starts.
    pub struct Defined;

    impl Marker for Defined {
        const STATE: super::WorkforceState = super::WorkforceState::Defined;
    }

    /// `Retired`. Terminal: an instance may rest here forever.
    pub struct Retired;

    impl Marker for Retired {
        const STATE: super::WorkforceState = super::WorkforceState::Retired;
    }

    /// `Staffed`.
    pub struct Staffed;

    impl Marker for Staffed {
        const STATE: super::WorkforceState = super::WorkforceState::Staffed;
    }
}

/// Workforce — `agency.teams.Workforce` — with its lifecycle state carried by the type.
///
/// The one constructor rests in `Defined`, and the only way to change `S` is a method generated from
/// a declared transition. A move the specification does not declare is therefore not an error
/// case: it does not compile. Where the state is data — wire, storage — use [`WorkforceSnapshot`]
/// and [`WorkforceSnapshot::refine`].
pub struct Workforce<S: workforce_state::Marker> {
    data: WorkforceData,
    state: core::marker::PhantomData<S>,
}

impl<S: workforce_state::Marker> Workforce<S> {
    /// The state this instance rests in, as the runtime value.
    pub fn state(&self) -> WorkforceState {
        S::STATE
    }

    /// What it holds.
    pub fn data(&self) -> &WorkforceData {
        &self.data
    }

    /// Hands the data back, giving up the typed state.
    pub fn into_data(self) -> WorkforceData {
        self.data
    }
}

impl Workforce<workforce_state::Defined> {
    /// A new instance, resting in `Defined` — the only state the lifecycle starts one in.
    pub fn new(data: WorkforceData) -> Self {
        Self {
            data,
            state: core::marker::PhantomData,
        }
    }
}

impl Workforce<workforce_state::Defined> {
    /// `staff` — `Defined` → `Staffed`. Taken by the `staffed` outcome of `agency.teams.StaffWorkforce`.
    pub fn staff(self) -> Workforce<workforce_state::Staffed> {
        Workforce {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }
}

impl Workforce<workforce_state::Staffed> {
    /// `retire` — `Staffed` → `Retired`. Taken by the `retired` outcome of `agency.teams.RetireWorkforce`.
    pub fn retire(self) -> Workforce<workforce_state::Retired> {
        Workforce {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }
}

/// `agency.teams.Workforce` as it crosses a boundary: the state as a value beside the data.
///
/// Wire and storage know states only at runtime; [`WorkforceSnapshot::refine`] is the one door back
/// into the typed lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkforceSnapshot {
    /// Where the instance is in its lifecycle.
    pub state: WorkforceState,
    /// What it holds.
    pub data: WorkforceData,
}

/// An `Workforce` in whichever declared state it was found.
pub enum AnyWorkforce {
    /// Resting in `Defined`.
    Defined(Workforce<workforce_state::Defined>),
    /// Resting in `Retired`.
    Retired(Workforce<workforce_state::Retired>),
    /// Resting in `Staffed`.
    Staffed(Workforce<workforce_state::Staffed>),
}

impl WorkforceSnapshot {
    /// Refines the runtime state into the typed one.
    ///
    /// Total: every declared state has an arm, and an undeclared state cannot reach here because
    /// `WorkforceState` cannot spell one.
    pub fn refine(self) -> AnyWorkforce {
        match self.state {
            WorkforceState::Defined => AnyWorkforce::Defined(Workforce {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            WorkforceState::Retired => AnyWorkforce::Retired(Workforce {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            WorkforceState::Staffed => AnyWorkforce::Staffed(Workforce {
                data: self.data,
                state: core::marker::PhantomData,
            }),
        }
    }
}

impl AnyWorkforce {
    /// The state, as the runtime value.
    pub fn state(&self) -> WorkforceState {
        match self {
            Self::Defined(_) => WorkforceState::Defined,
            Self::Retired(_) => WorkforceState::Retired,
            Self::Staffed(_) => WorkforceState::Staffed,
        }
    }

    /// Back to the boundary shape.
    pub fn snapshot(self) -> WorkforceSnapshot {
        match self {
            Self::Defined(instance) => WorkforceSnapshot {
                state: WorkforceState::Defined,
                data: instance.into_data(),
            },
            Self::Retired(instance) => WorkforceSnapshot {
                state: WorkforceState::Retired,
                data: instance.into_data(),
            },
            Self::Staffed(instance) => WorkforceSnapshot {
                state: WorkforceState::Staffed,
                data: instance.into_data(),
            },
        }
    }
}

/// Add a team member — the input of `agency.teams.AddMember`.
///
/// Everything it can result in is [`AddMemberOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddMember {
    /// `team_id` — `agency.teams.TeamId`.
    pub team_id: TeamId,
    /// `agent_id` — `agency.agents.AgentId`.
    pub agent_id: crate::agents::AgentId,
}

/// Everything `agency.teams.AddMember` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddMemberOutcome {
    /// `no-such-team` — for an identity no record carries.
    NoSuchTeam {
        /// Why it was refused: `agency.teams.NotFound`.
        error: NotFound,
    },
    /// `added` — otherwise.
    Added {
        /// The `agency.teams.MemberAdded` this outcome publishes.
        member_added: MemberAdded,
    },
}

/// Attach a queue to a workforce — the input of `agency.teams.AttachQueue`.
///
/// Everything it can result in is [`AttachQueueOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttachQueue {
    /// `workforce_id` — `agency.teams.WorkforceId`.
    pub workforce_id: WorkforceId,
    /// `input_id` — `agency.work.InputId`.
    pub input_id: crate::work::InputId,
}

/// Everything `agency.teams.AttachQueue` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttachQueueOutcome {
    /// `no-such-input` — for an identity no record carries.
    NoSuchInput {
        /// Why it was refused: `agency.teams.NotFound`.
        error: NotFound,
    },
    /// `attached` — otherwise.
    Attached {
        /// The `agency.teams.QueueAttached` this outcome publishes.
        queue_attached: QueueAttached,
    },
}

/// Declare a swarm — the input of `agency.teams.DeclareSwarm`.
///
/// Everything it can result in is [`DeclareSwarmOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclareSwarm {
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: crate::auth::TenantId,
    /// `goal_ref` — `String`.
    pub goal_ref: String,
    /// `coordinator_agent_id` — `agency.agents.AgentId`.
    pub coordinator_agent_id: crate::agents::AgentId,
    /// `frame_digest` — `agency.shared.Digest`.
    pub frame_digest: crate::shared::Digest,
    /// `max_workers` — `Integer`.
    pub max_workers: i64,
    /// `team_id` — `agency.teams.TeamId`.
    pub team_id: TeamId,
    /// `work_input_id` — `agency.work.InputId`.
    pub work_input_id: crate::work::InputId,
}

/// Everything `agency.teams.DeclareSwarm` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclareSwarmOutcome {
    /// `declared` — otherwise.
    Declared {
        /// The `agency.teams.SwarmDeclared` this outcome publishes.
        swarm_declared: SwarmDeclared,
    },
}

/// Define a workforce — the input of `agency.teams.DefineWorkforce`.
///
/// Everything it can result in is [`DefineWorkforceOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefineWorkforce {
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: crate::auth::TenantId,
    /// `name` — `String`.
    pub name: String,
    /// `capacity` — `agency.teams.Capacity`.
    pub capacity: Capacity,
}

/// Everything `agency.teams.DefineWorkforce` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DefineWorkforceOutcome {
    /// `defined` — otherwise.
    Defined {
        /// The `agency.teams.WorkforceDefined` this outcome publishes.
        workforce_defined: WorkforceDefined,
    },
}

/// Disband a team — the input of `agency.teams.DisbandTeam`.
///
/// Everything it can result in is [`DisbandTeamOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisbandTeam {
    /// `team_id` — `agency.teams.TeamId`.
    pub team_id: TeamId,
}

/// Everything `agency.teams.DisbandTeam` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DisbandTeamOutcome {
    /// `no-such-team` — for an identity no record carries.
    NoSuchTeam {
        /// Why it was refused: `agency.teams.NotFound`.
        error: NotFound,
    },
    /// `disbanded` — otherwise.
    Disbanded {
        /// The `agency.teams.TeamDisbanded` this outcome publishes.
        team_disbanded: TeamDisbanded,
    },
    /// `wrong-state` — from a state no declared move starts in.
    WrongState {
        /// Why it was refused: `agency.teams.StateConflict`.
        error: StateConflict,
    },
}

/// Form a team — the input of `agency.teams.FormTeam`.
///
/// Everything it can result in is [`FormTeamOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormTeam {
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: crate::auth::TenantId,
    /// `name` — `String`.
    pub name: String,
    /// `lead_agent_id` — `agency.agents.AgentId`.
    pub lead_agent_id: crate::agents::AgentId,
    /// `members` — `List<agency.agents.AgentId>`.
    pub members: Vec<crate::agents::AgentId>,
    /// `broker_ref` — `String`.
    pub broker_ref: String,
}

/// Everything `agency.teams.FormTeam` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormTeamOutcome {
    /// `formed` — otherwise.
    Formed {
        /// The `agency.teams.TeamFormed` this outcome publishes.
        team_formed: TeamFormed,
    },
}

/// Remove a team member — the input of `agency.teams.RemoveMember`.
///
/// Everything it can result in is [`RemoveMemberOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveMember {
    /// `team_id` — `agency.teams.TeamId`.
    pub team_id: TeamId,
    /// `agent_id` — `agency.agents.AgentId`.
    pub agent_id: crate::agents::AgentId,
}

/// Everything `agency.teams.RemoveMember` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RemoveMemberOutcome {
    /// `no-such-team` — for an identity no record carries.
    NoSuchTeam {
        /// Why it was refused: `agency.teams.NotFound`.
        error: NotFound,
    },
    /// `removed` — otherwise.
    Removed {
        /// The `agency.teams.MemberRemoved` this outcome publishes.
        member_removed: MemberRemoved,
    },
}

/// Report up one level — the input of `agency.teams.Report`.
///
/// Everything it can result in is [`ReportOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    /// `team_id` — `agency.teams.TeamId`.
    pub team_id: TeamId,
    /// `from_agent_id` — `agency.agents.AgentId`.
    pub from_agent_id: crate::agents::AgentId,
    /// `status` — `String`.
    pub status: String,
    /// `line` — `String`.
    pub line: String,
    /// `to` — `Optional<String>`.
    pub to: Option<String>,
}

/// Everything `agency.teams.Report` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReportOutcome {
    /// `no-such-team` — for an identity no record carries.
    NoSuchTeam {
        /// Why it was refused: `agency.teams.NotFound`.
        error: NotFound,
    },
    /// `not-one-level-up` — externally decided (the report names a recipient other than the one above the sender, or nothing is above it).
    NotOneLevelUp {
        /// Why it was refused: `agency.teams.NotOneLevelUp`.
        error: NotOneLevelUp,
    },
    /// `reported` — otherwise.
    Reported {
        /// The `agency.teams.ReportSent` this outcome publishes.
        report_sent: ReportSent,
    },
}

/// Retire a workforce — the input of `agency.teams.RetireWorkforce`.
///
/// Everything it can result in is [`RetireWorkforceOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetireWorkforce {
    /// `workforce_id` — `agency.teams.WorkforceId`.
    pub workforce_id: WorkforceId,
}

/// Everything `agency.teams.RetireWorkforce` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetireWorkforceOutcome {
    /// `no-such-workforce` — for an identity no record carries.
    NoSuchWorkforce {
        /// Why it was refused: `agency.teams.NotFound`.
        error: NotFound,
    },
    /// `retired` — otherwise.
    Retired {
        /// The `agency.teams.WorkforceRetired` this outcome publishes.
        workforce_retired: WorkforceRetired,
    },
    /// `wrong-state` — from a state no declared move starts in.
    WrongState {
        /// Why it was refused: `agency.teams.StateConflict`.
        error: StateConflict,
    },
}

/// Send a team message — the input of `agency.teams.Send`.
///
/// Everything it can result in is [`SendOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Send {
    /// `team_id` — `agency.teams.TeamId`.
    pub team_id: TeamId,
    /// `from_agent_id` — `agency.agents.AgentId`.
    pub from_agent_id: crate::agents::AgentId,
    /// `to` — `String`.
    pub to: String,
    /// `text` — `String`.
    pub text: String,
}

/// Everything `agency.teams.Send` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SendOutcome {
    /// `no-such-team` — for an identity no record carries.
    NoSuchTeam {
        /// Why it was refused: `agency.teams.NotFound`.
        error: NotFound,
    },
    /// `foreign-instance` — externally decided (the recipient is registered under another instance).
    ForeignInstance {
        /// Why it was refused: `agency.teams.ForeignInstance`.
        error: ForeignInstance,
    },
    /// `outside-team` — externally decided (the recipient is outside the sender's team, or the sender has no team).
    OutsideTeam {
        /// Why it was refused: `agency.teams.OutsideTeam`.
        error: OutsideTeam,
    },
    /// `sent` — otherwise.
    Sent {
        /// The `agency.teams.MessageSent` this outcome publishes.
        message_sent: MessageSent,
    },
}

/// Set a team's lead — the input of `agency.teams.SetLead`.
///
/// Everything it can result in is [`SetLeadOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetLead {
    /// `team_id` — `agency.teams.TeamId`.
    pub team_id: TeamId,
    /// `lead_agent_id` — `agency.agents.AgentId`.
    pub lead_agent_id: crate::agents::AgentId,
}

/// Everything `agency.teams.SetLead` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetLeadOutcome {
    /// `no-such-team` — for an identity no record carries.
    NoSuchTeam {
        /// Why it was refused: `agency.teams.NotFound`.
        error: NotFound,
    },
    /// `set` — otherwise.
    Set {
        /// The `agency.teams.LeadSet` this outcome publishes.
        lead_set: LeadSet,
    },
}

/// Staff a workforce — the input of `agency.teams.StaffWorkforce`.
///
/// Everything it can result in is [`StaffWorkforceOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaffWorkforce {
    /// `workforce_id` — `agency.teams.WorkforceId`.
    pub workforce_id: WorkforceId,
    /// `teams` — `List<agency.teams.TeamId>`.
    pub teams: Vec<TeamId>,
    /// `solo_agents` — `List<agency.agents.AgentId>`.
    pub solo_agents: Vec<crate::agents::AgentId>,
}

/// Everything `agency.teams.StaffWorkforce` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StaffWorkforceOutcome {
    /// `no-such-workforce` — for an identity no record carries.
    NoSuchWorkforce {
        /// Why it was refused: `agency.teams.NotFound`.
        error: NotFound,
    },
    /// `staffed` — otherwise.
    Staffed {
        /// The `agency.teams.WorkforceStaffed` this outcome publishes.
        workforce_staffed: WorkforceStaffed,
    },
    /// `wrong-state` — from a state no declared move starts in.
    WrongState {
        /// Why it was refused: `agency.teams.StateConflict`.
        error: StateConflict,
    },
}

/// Stop a swarm — the input of `agency.teams.StopSwarm`.
///
/// Everything it can result in is [`StopSwarmOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StopSwarm {
    /// `swarm_id` — `agency.teams.SwarmId`.
    pub swarm_id: SwarmId,
}

/// Everything `agency.teams.StopSwarm` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StopSwarmOutcome {
    /// `no-such-swarm` — for an identity no record carries.
    NoSuchSwarm {
        /// Why it was refused: `agency.teams.NotFound`.
        error: NotFound,
    },
    /// `stopped` — otherwise.
    Stopped {
        /// The `agency.teams.SwarmStopped` this outcome publishes.
        swarm_stopped: SwarmStopped,
    },
    /// `wrong-state` — from a state no declared move starts in.
    WrongState {
        /// Why it was refused: `agency.teams.StateConflict`.
        error: StateConflict,
    },
}

/// LeadSet — the event `agency.teams.LeadSet`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeadSet {
    /// `team_id` — `agency.teams.TeamId`.
    pub team_id: TeamId,
    /// `lead_agent_id` — `agency.agents.AgentId`.
    pub lead_agent_id: crate::agents::AgentId,
}

/// MemberAdded — the event `agency.teams.MemberAdded`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberAdded {
    /// `team_id` — `agency.teams.TeamId`.
    pub team_id: TeamId,
    /// `agent_id` — `agency.agents.AgentId`.
    pub agent_id: crate::agents::AgentId,
}

/// MemberRemoved — the event `agency.teams.MemberRemoved`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberRemoved {
    /// `team_id` — `agency.teams.TeamId`.
    pub team_id: TeamId,
    /// `agent_id` — `agency.agents.AgentId`.
    pub agent_id: crate::agents::AgentId,
}

/// MessageSent — the event `agency.teams.MessageSent`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageSent {
    /// `team_id` — `agency.teams.TeamId`.
    pub team_id: TeamId,
    /// `from_agent_id` — `agency.agents.AgentId`.
    pub from_agent_id: crate::agents::AgentId,
    /// `to` — `String`.
    pub to: String,
    /// `text` — `String`.
    pub text: String,
}

/// QueueAttached — the event `agency.teams.QueueAttached`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueueAttached {
    /// `workforce_id` — `agency.teams.WorkforceId`.
    pub workforce_id: WorkforceId,
    /// `input_id` — `agency.work.InputId`.
    pub input_id: crate::work::InputId,
}

/// ReportSent — the event `agency.teams.ReportSent`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportSent {
    /// `team_id` — `agency.teams.TeamId`.
    pub team_id: TeamId,
    /// `from_agent_id` — `agency.agents.AgentId`.
    pub from_agent_id: crate::agents::AgentId,
    /// `status` — `String`.
    pub status: String,
    /// `line` — `String`.
    pub line: String,
}

/// SwarmDeclared — the event `agency.teams.SwarmDeclared`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwarmDeclared {
    /// `swarm_id` — `agency.teams.SwarmId`.
    pub swarm_id: SwarmId,
    /// `goal_ref` — `String`.
    pub goal_ref: String,
    /// `team_id` — `agency.teams.TeamId`.
    pub team_id: TeamId,
}

/// SwarmStopped — the event `agency.teams.SwarmStopped`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwarmStopped {
    /// `swarm_id` — `agency.teams.SwarmId`.
    pub swarm_id: SwarmId,
}

/// TeamDisbanded — the event `agency.teams.TeamDisbanded`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamDisbanded {
    /// `team_id` — `agency.teams.TeamId`.
    pub team_id: TeamId,
}

/// TeamFormed — the event `agency.teams.TeamFormed`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamFormed {
    /// `team_id` — `agency.teams.TeamId`.
    pub team_id: TeamId,
    /// `name` — `String`.
    pub name: String,
    /// `lead_agent_id` — `agency.agents.AgentId`.
    pub lead_agent_id: crate::agents::AgentId,
}

/// WorkforceDefined — the event `agency.teams.WorkforceDefined`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkforceDefined {
    /// `workforce_id` — `agency.teams.WorkforceId`.
    pub workforce_id: WorkforceId,
    /// `name` — `String`.
    pub name: String,
}

/// WorkforceRetired — the event `agency.teams.WorkforceRetired`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkforceRetired {
    /// `workforce_id` — `agency.teams.WorkforceId`.
    pub workforce_id: WorkforceId,
}

/// WorkforceStaffed — the event `agency.teams.WorkforceStaffed`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkforceStaffed {
    /// `workforce_id` — `agency.teams.WorkforceId`.
    pub workforce_id: WorkforceId,
    /// `teams` — `List<agency.teams.TeamId>`.
    pub teams: Vec<TeamId>,
    /// `solo_agents` — `List<agency.agents.AgentId>`.
    pub solo_agents: Vec<crate::agents::AgentId>,
}

/// The declared error `agency.teams.ForeignInstance`.
///
/// The recipient is registered under another instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignInstance;

/// The declared error `agency.teams.NotFound`.
///
/// The addressed record does not exist, and nothing changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotFound;

/// The declared error `agency.teams.NotOneLevelUp`.
///
/// The report names a recipient other than the one one level above the sender.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotOneLevelUp;

/// The declared error `agency.teams.OutsideTeam`.
///
/// The recipient is outside the sender's team, or the sender has no team.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutsideTeam;

/// The declared error `agency.teams.StateConflict`.
///
/// The addressed record is not in a state this command acts from, and nothing changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateConflict;

/// SwarmList — one row of the view `agency.teams.SwarmList`.
///
/// Projects `agency.teams.Swarm` at `read_your_writes` consistency.
/// The specification fully determines every row, so its query is generated over the storage port —
/// see the plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwarmList {
    /// `swarm_id` — `agency.teams.SwarmId`.
    pub swarm_id: SwarmId,
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: crate::auth::TenantId,
    /// `goal_ref` — `String`.
    pub goal_ref: String,
    /// `coordinator_agent_id` — `agency.agents.AgentId`.
    pub coordinator_agent_id: crate::agents::AgentId,
    /// `frame_digest` — `agency.shared.Digest`.
    pub frame_digest: crate::shared::Digest,
    /// `max_workers` — `Integer`.
    pub max_workers: i64,
    /// `team_id` — `agency.teams.TeamId`.
    pub team_id: TeamId,
    /// `work_input_id` — `agency.work.InputId`.
    pub work_input_id: crate::work::InputId,
    /// `state` — `agency.teams.Swarm.State`.
    pub state: SwarmState,
}

/// TeamDetail — one row of the view `agency.teams.TeamDetail`.
///
/// Projects `agency.teams.Team` at `read_your_writes` consistency, containing instances where `team_id == param.team_id`.
/// Serving it is an implementation obligation — see the plan — because how a projection is kept
/// current is a storage decision the specification does not take.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamDetail {
    /// `team_id` — `agency.teams.TeamId`.
    pub team_id: TeamId,
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: crate::auth::TenantId,
    /// `name` — `String`.
    pub name: String,
    /// `lead_agent_id` — `agency.agents.AgentId`.
    pub lead_agent_id: crate::agents::AgentId,
    /// `members` — `List<agency.agents.AgentId>`.
    pub members: Vec<crate::agents::AgentId>,
    /// `broker_ref` — `String`.
    pub broker_ref: String,
    /// `state` — `agency.teams.Team.State`.
    pub state: TeamState,
}

/// TeamList — one row of the view `agency.teams.TeamList`.
///
/// Projects `agency.teams.Team` at `read_your_writes` consistency.
/// The specification fully determines every row, so its query is generated over the storage port —
/// see the plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamList {
    /// `team_id` — `agency.teams.TeamId`.
    pub team_id: TeamId,
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: crate::auth::TenantId,
    /// `name` — `String`.
    pub name: String,
    /// `lead_agent_id` — `agency.agents.AgentId`.
    pub lead_agent_id: crate::agents::AgentId,
    /// `members` — `List<agency.agents.AgentId>`.
    pub members: Vec<crate::agents::AgentId>,
    /// `state` — `agency.teams.Team.State`.
    pub state: TeamState,
}

/// WorkforceBoard — one row of the view `agency.teams.WorkforceBoard`.
///
/// Projects `agency.teams.Workforce` at `read_your_writes` consistency.
/// The specification fully determines every row, so its query is generated over the storage port —
/// see the plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkforceBoard {
    /// `workforce_id` — `agency.teams.WorkforceId`.
    pub workforce_id: WorkforceId,
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: crate::auth::TenantId,
    /// `name` — `String`.
    pub name: String,
    /// `teams` — `List<agency.teams.TeamId>`.
    pub teams: Vec<TeamId>,
    /// `solo_agents` — `List<agency.agents.AgentId>`.
    pub solo_agents: Vec<crate::agents::AgentId>,
    /// `capacity` — `agency.teams.Capacity`.
    pub capacity: Capacity,
    /// `state` — `agency.teams.Workforce.State`.
    pub state: WorkforceState,
}

/// What this bounded context owes its implementor, and the seams of what is generated.
///
/// One trait per obligation in the synthesis plan, each carrying the plan's own contract, and one
/// per generated behaviour, which [`Generated`](crate::behaviour::Generated) implements.
/// [`Unimplemented`](obligations::Unimplemented) satisfies every owed trait by refusing in the type system.
pub mod obligations {
    /// The behaviour `agency.teams.AddMember` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait AddMemberBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.teams.AddMember`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn add_member(&mut self, input: super::AddMember) -> Result<super::AddMemberOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.teams.AttachQueue` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait AttachQueueBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.teams.AttachQueue`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn attach_queue(&mut self, input: super::AttachQueue) -> Result<super::AttachQueueOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.teams.DeclareSwarm` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait DeclareSwarmBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.teams.DeclareSwarm`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn declare_swarm(&mut self, input: super::DeclareSwarm) -> Result<super::DeclareSwarmOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.teams.DefineWorkforce` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait DefineWorkforceBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.teams.DefineWorkforce`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn define_workforce(&mut self, input: super::DefineWorkforce) -> Result<super::DefineWorkforceOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.teams.DisbandTeam` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait DisbandTeamBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.teams.DisbandTeam`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn disband_team(&mut self, input: super::DisbandTeam) -> Result<super::DisbandTeamOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.teams.FormTeam` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait FormTeamBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.teams.FormTeam`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn form_team(&mut self, input: super::FormTeam) -> Result<super::FormTeamOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.teams.RemoveMember` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait RemoveMemberBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.teams.RemoveMember`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn remove_member(&mut self, input: super::RemoveMember) -> Result<super::RemoveMemberOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.teams.Report` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait ReportBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.teams.Report`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn report(&mut self, input: super::Report) -> Result<super::ReportOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.teams.RetireWorkforce` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait RetireWorkforceBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.teams.RetireWorkforce`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn retire_workforce(&mut self, input: super::RetireWorkforce) -> Result<super::RetireWorkforceOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.teams.Send` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait SendBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.teams.Send`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn send(&mut self, input: super::Send) -> Result<super::SendOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.teams.SetLead` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait SetLeadBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.teams.SetLead`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn set_lead(&mut self, input: super::SetLead) -> Result<super::SetLeadOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.teams.StaffWorkforce` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait StaffWorkforceBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.teams.StaffWorkforce`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn staff_workforce(&mut self, input: super::StaffWorkforce) -> Result<super::StaffWorkforceOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.teams.StopSwarm` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait StopSwarmBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.teams.StopSwarm`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn stop_swarm(&mut self, input: super::StopSwarm) -> Result<super::StopSwarmOutcome, crate::obligation::UnmetObligation>;
    }

    /// The query `agency.teams.SwarmList` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage port. Implement it yourself to replace that query.
    pub trait SwarmListQuery {
        /// Serves `agency.teams.SwarmList` rows at the view's declared consistency.
        ///
        /// `Err` is the typed refusal of a row whose declared type cannot hold its value.
        fn swarm_list(&self) -> Result<Vec<super::SwarmList>, crate::obligation::UnmetObligation>;
    }

    /// The query `agency.teams.TeamDetail` — an implementation obligation.
    ///
    /// Why it is not generated: kept an obligation by a view parameter (`params:`), which a generated query does not apply.
    ///
    /// Contract: a query answering `agency.teams.TeamDetail` with rows projected from `agency.teams.Team` at `read_your_writes` consistency, containing instances where `team_id == param.team_id`.
    pub trait TeamDetailQuery {
        /// Serves `agency.teams.TeamDetail` rows at the view's declared consistency.
        ///
        /// `Err` is the typed refusal of an obligation nothing has satisfied; a satisfying
        /// implementation never returns it.
        fn team_detail(&self, team_id: crate::teams::TeamId) -> Result<Vec<super::TeamDetail>, crate::obligation::UnmetObligation>;
    }

    /// The query `agency.teams.TeamList` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage port. Implement it yourself to replace that query.
    pub trait TeamListQuery {
        /// Serves `agency.teams.TeamList` rows at the view's declared consistency.
        ///
        /// `Err` is the typed refusal of a row whose declared type cannot hold its value.
        fn team_list(&self) -> Result<Vec<super::TeamList>, crate::obligation::UnmetObligation>;
    }

    /// The query `agency.teams.WorkforceBoard` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage port. Implement it yourself to replace that query.
    pub trait WorkforceBoardQuery {
        /// Serves `agency.teams.WorkforceBoard` rows at the view's declared consistency.
        ///
        /// `Err` is the typed refusal of a row whose declared type cannot hold its value.
        fn workforce_board(&self) -> Result<Vec<super::WorkforceBoard>, crate::obligation::UnmetObligation>;
    }

    /// Every obligation of this bounded context, refused in the type system.
    ///
    /// Each method returns the typed refusal naming what is owed — never a panic, never a guessed
    /// value — so a workspace built on this stub compiles and reports its own gaps.
    pub struct Unimplemented;

    impl TeamDetailQuery for Unimplemented {
        fn team_detail(&self, _team_id: crate::teams::TeamId) -> Result<Vec<super::TeamDetail>, crate::obligation::UnmetObligation> {
            Err(crate::obligation::UnmetObligation { capability: "view query", source: "agency.teams.TeamDetail" })
        }
    }
}
