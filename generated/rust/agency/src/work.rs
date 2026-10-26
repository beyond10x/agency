// generated from agency v1
// model digest 71132e90bde806415603d08bb97a71629b2c2f6cda30e1f22e76f407abd9a3eb
// contract digest a96f55e3e1ca68aeb1579b032c3547fffd27e8ab80e76bcb898f9b6c9258792e
// do not edit: regenerate with `ess synthesize --layout crate`

//! Work — `agency.work`.
//!
//! The inputs work arrives from, and the work items Agency handles by reference: routed, assigned, run and settled without copying their bodies.
//!
//! Everything this bounded context declares that the synthesis plan marks generated.

/// AepStoreSource — `agency.work.AepStoreSource`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AepStoreSource {
    /// `repo` — `String`.
    pub repo: String,
    /// `filter` — `String`.
    pub filter: String,
}

/// GitHubIssuesSource — `agency.work.GitHubIssuesSource`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitHubIssuesSource {
    /// `repo` — `String`.
    pub repo: String,
    /// `label` — `String`.
    pub label: String,
}

/// InputId — `agency.work.InputId`: a distinct wrapper around `Uuid`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputId(pub crate::primitives::Uuid);

/// WebhookSource — `agency.work.WebhookSource`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebhookSource {
    /// `secret_ref` — `String`.
    pub secret_ref: String,
}

/// The states of `agency.work.WorkInput`, as runtime values.
///
/// Synthesised from the lifecycle, so the two cannot disagree. Which *moves* are legal is not
/// carried here — it is carried by `WorkInput<S>`, where an undeclared move does not compile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkInputState {
    /// `Configured`.
    Configured,
    /// `Paused`.
    Paused,
    /// `Removed`.
    Removed,
}

/// The states of `agency.work.WorkItem`, as runtime values.
///
/// Synthesised from the lifecycle, so the two cannot disagree. Which *moves* are legal is not
/// carried here — it is carried by `WorkItem<S>`, where an undeclared move does not compile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkItemState {
    /// `Abandoned`.
    Abandoned,
    /// `Assigned`.
    Assigned,
    /// `Done`.
    Done,
    /// `Received`.
    Received,
    /// `Returned`.
    Returned,
}

/// WorkItemRef — `agency.work.WorkItemRef`: a distinct wrapper around `String`.
///
/// A work item's external id together with its source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkItemRef(pub String);

/// WorkSource — `agency.work.WorkSource`: one of a fixed set of shapes, tagged on the wire by `kind`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkSource {
    /// Tagged `A2aTask`, carrying nothing.
    A2aTask,
    /// Tagged `AepStore` — `agency.work.AepStoreSource`.
    AepStore(AepStoreSource),
    /// Tagged `GitHubIssues` — `agency.work.GitHubIssuesSource`.
    GitHubIssues(GitHubIssuesSource),
    /// Tagged `Manual`, carrying nothing.
    Manual,
    /// Tagged `Webhook` — `agency.work.WebhookSource`.
    Webhook(WebhookSource),
}

/// What WorkInput — `agency.work.WorkInput` — holds, apart from where it is in its lifecycle.
///
/// The identity and every declared field. The state is deliberately not one: inside the domain it
/// is carried by the type parameter of [`WorkInput<S>`], and at a boundary by [`WorkInputSnapshot::state`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkInputData {
    /// The identity: `input_id` — `agency.work.InputId`.
    pub input_id: InputId,
    /// `tenant_id` — `agency.auth.TenantId`.
    ///
    /// Carries `work_inputs`: `agency.auth.Tenant` owns many `agency.work.WorkInput`.
    pub tenant_id: crate::auth::TenantId,
    /// `name` — `String`.
    pub name: String,
    /// `source` — `agency.work.WorkSource`.
    pub source: WorkSource,
    /// `poll_interval` — `Optional<Duration>`.
    pub poll_interval: Option<crate::primitives::Duration>,
    /// `workforce_id` — `agency.teams.WorkforceId`.
    ///
    /// Carries `workforce`: `agency.work.WorkInput` references one `agency.teams.Workforce`.
    pub workforce_id: crate::teams::WorkforceId,
    /// `transform_ref` — `Optional<String>`.
    pub transform_ref: Option<String>,
}

/// The states of `agency.work.WorkInput`, at the type level.
///
/// One marker type per declared state, sealed: a state the lifecycle does not declare cannot
/// implement [`Marker`](work_input_state::Marker), so [`WorkInput<S>`](WorkInput) can only ever rest in a real state.
pub mod work_input_state {
    /// Closes [`Marker`] over the declared states.
    mod sealed {
        /// Implemented only by the marker types beside this module.
        pub trait Sealed {}
        impl Sealed for super::Configured {}
        impl Sealed for super::Paused {}
        impl Sealed for super::Removed {}
    }

    /// A declared state of `WorkInput`, as a type.
    pub trait Marker: sealed::Sealed {
        /// The same state, as the runtime value.
        const STATE: super::WorkInputState;
    }

    /// `Configured`. Where a new instance starts.
    pub struct Configured;

    impl Marker for Configured {
        const STATE: super::WorkInputState = super::WorkInputState::Configured;
    }

    /// `Paused`.
    pub struct Paused;

    impl Marker for Paused {
        const STATE: super::WorkInputState = super::WorkInputState::Paused;
    }

    /// `Removed`. Terminal: an instance may rest here forever.
    pub struct Removed;

    impl Marker for Removed {
        const STATE: super::WorkInputState = super::WorkInputState::Removed;
    }
}

/// WorkInput — `agency.work.WorkInput` — with its lifecycle state carried by the type.
///
/// The one constructor rests in `Configured`, and the only way to change `S` is a method generated from
/// a declared transition. A move the specification does not declare is therefore not an error
/// case: it does not compile. Where the state is data — wire, storage — use [`WorkInputSnapshot`]
/// and [`WorkInputSnapshot::refine`].
pub struct WorkInput<S: work_input_state::Marker> {
    data: WorkInputData,
    state: core::marker::PhantomData<S>,
}

impl<S: work_input_state::Marker> WorkInput<S> {
    /// The state this instance rests in, as the runtime value.
    pub fn state(&self) -> WorkInputState {
        S::STATE
    }

    /// What it holds.
    pub fn data(&self) -> &WorkInputData {
        &self.data
    }

    /// Hands the data back, giving up the typed state.
    pub fn into_data(self) -> WorkInputData {
        self.data
    }
}

impl WorkInput<work_input_state::Configured> {
    /// A new instance, resting in `Configured` — the only state the lifecycle starts one in.
    pub fn new(data: WorkInputData) -> Self {
        Self {
            data,
            state: core::marker::PhantomData,
        }
    }
}

impl WorkInput<work_input_state::Configured> {
    /// `pause` — `Configured` → `Paused`. Taken by the `paused` outcome of `agency.work.PauseWorkInput`.
    pub fn pause(self) -> WorkInput<work_input_state::Paused> {
        WorkInput {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }
}

impl WorkInput<work_input_state::Paused> {
    /// `remove` — `Paused` → `Removed`. Taken by the `removed` outcome of `agency.work.RemoveWorkInput`.
    pub fn remove(self) -> WorkInput<work_input_state::Removed> {
        WorkInput {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }
}

/// `agency.work.WorkInput` as it crosses a boundary: the state as a value beside the data.
///
/// Wire and storage know states only at runtime; [`WorkInputSnapshot::refine`] is the one door back
/// into the typed lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkInputSnapshot {
    /// Where the instance is in its lifecycle.
    pub state: WorkInputState,
    /// What it holds.
    pub data: WorkInputData,
}

/// An `WorkInput` in whichever declared state it was found.
pub enum AnyWorkInput {
    /// Resting in `Configured`.
    Configured(WorkInput<work_input_state::Configured>),
    /// Resting in `Paused`.
    Paused(WorkInput<work_input_state::Paused>),
    /// Resting in `Removed`.
    Removed(WorkInput<work_input_state::Removed>),
}

impl WorkInputSnapshot {
    /// Refines the runtime state into the typed one.
    ///
    /// Total: every declared state has an arm, and an undeclared state cannot reach here because
    /// `WorkInputState` cannot spell one.
    pub fn refine(self) -> AnyWorkInput {
        match self.state {
            WorkInputState::Configured => AnyWorkInput::Configured(WorkInput {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            WorkInputState::Paused => AnyWorkInput::Paused(WorkInput {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            WorkInputState::Removed => AnyWorkInput::Removed(WorkInput {
                data: self.data,
                state: core::marker::PhantomData,
            }),
        }
    }
}

impl AnyWorkInput {
    /// The state, as the runtime value.
    pub fn state(&self) -> WorkInputState {
        match self {
            Self::Configured(_) => WorkInputState::Configured,
            Self::Paused(_) => WorkInputState::Paused,
            Self::Removed(_) => WorkInputState::Removed,
        }
    }

    /// Back to the boundary shape.
    pub fn snapshot(self) -> WorkInputSnapshot {
        match self {
            Self::Configured(instance) => WorkInputSnapshot {
                state: WorkInputState::Configured,
                data: instance.into_data(),
            },
            Self::Paused(instance) => WorkInputSnapshot {
                state: WorkInputState::Paused,
                data: instance.into_data(),
            },
            Self::Removed(instance) => WorkInputSnapshot {
                state: WorkInputState::Removed,
                data: instance.into_data(),
            },
        }
    }
}

/// What WorkItem — `agency.work.WorkItem` — holds, apart from where it is in its lifecycle.
///
/// The identity and every declared field. The state is deliberately not one: inside the domain it
/// is carried by the type parameter of [`WorkItem<S>`], and at a boundary by [`WorkItemSnapshot::state`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkItemData {
    /// The identity: `item_ref` — `agency.work.WorkItemRef`.
    pub item_ref: WorkItemRef,
    /// `tenant_id` — `agency.auth.TenantId`.
    ///
    /// Carries `work_items`: `agency.auth.Tenant` owns many `agency.work.WorkItem`.
    pub tenant_id: crate::auth::TenantId,
    /// `input_id` — `agency.work.InputId`.
    ///
    /// Carries `work_input`: `agency.work.WorkItem` references one `agency.work.WorkInput`.
    pub input_id: InputId,
    /// `title` — `String`.
    pub title: String,
    /// `payload_digest` — `agency.shared.Digest`.
    pub payload_digest: crate::shared::Digest,
    /// `assigned_agent_id` — `Optional<agency.agents.AgentId>`.
    ///
    /// Carries `agent`: `agency.work.WorkItem` references one `agency.agents.Agent`.
    pub assigned_agent_id: Option<crate::agents::AgentId>,
    /// `received_at` — `Timestamp`.
    pub received_at: crate::primitives::Timestamp,
    /// `settled_at` — `Optional<Timestamp>`.
    pub settled_at: Option<crate::primitives::Timestamp>,
}

/// The states of `agency.work.WorkItem`, at the type level.
///
/// One marker type per declared state, sealed: a state the lifecycle does not declare cannot
/// implement [`Marker`](work_item_state::Marker), so [`WorkItem<S>`](WorkItem) can only ever rest in a real state.
pub mod work_item_state {
    /// Closes [`Marker`] over the declared states.
    mod sealed {
        /// Implemented only by the marker types beside this module.
        pub trait Sealed {}
        impl Sealed for super::Abandoned {}
        impl Sealed for super::Assigned {}
        impl Sealed for super::Done {}
        impl Sealed for super::Received {}
        impl Sealed for super::Returned {}
    }

    /// A declared state of `WorkItem`, as a type.
    pub trait Marker: sealed::Sealed {
        /// The same state, as the runtime value.
        const STATE: super::WorkItemState;
    }

    /// `Abandoned`. Terminal: an instance may rest here forever.
    pub struct Abandoned;

    impl Marker for Abandoned {
        const STATE: super::WorkItemState = super::WorkItemState::Abandoned;
    }

    /// `Assigned`.
    pub struct Assigned;

    impl Marker for Assigned {
        const STATE: super::WorkItemState = super::WorkItemState::Assigned;
    }

    /// `Done`. Terminal: an instance may rest here forever.
    pub struct Done;

    impl Marker for Done {
        const STATE: super::WorkItemState = super::WorkItemState::Done;
    }

    /// `Received`. Where a new instance starts.
    pub struct Received;

    impl Marker for Received {
        const STATE: super::WorkItemState = super::WorkItemState::Received;
    }

    /// `Returned`. Terminal: an instance may rest here forever.
    pub struct Returned;

    impl Marker for Returned {
        const STATE: super::WorkItemState = super::WorkItemState::Returned;
    }
}

/// WorkItem — `agency.work.WorkItem` — with its lifecycle state carried by the type.
///
/// The one constructor rests in `Received`, and the only way to change `S` is a method generated from
/// a declared transition. A move the specification does not declare is therefore not an error
/// case: it does not compile. Where the state is data — wire, storage — use [`WorkItemSnapshot`]
/// and [`WorkItemSnapshot::refine`].
pub struct WorkItem<S: work_item_state::Marker> {
    data: WorkItemData,
    state: core::marker::PhantomData<S>,
}

impl<S: work_item_state::Marker> WorkItem<S> {
    /// The state this instance rests in, as the runtime value.
    pub fn state(&self) -> WorkItemState {
        S::STATE
    }

    /// What it holds.
    pub fn data(&self) -> &WorkItemData {
        &self.data
    }

    /// Hands the data back, giving up the typed state.
    pub fn into_data(self) -> WorkItemData {
        self.data
    }
}

impl WorkItem<work_item_state::Received> {
    /// A new instance, resting in `Received` — the only state the lifecycle starts one in.
    pub fn new(data: WorkItemData) -> Self {
        Self {
            data,
            state: core::marker::PhantomData,
        }
    }
}

impl WorkItem<work_item_state::Assigned> {
    /// `return` — `Assigned` → `Returned`. Taken by the `returned` outcome of `agency.work.ReturnWorkItem`.
    pub fn r#return(self) -> WorkItem<work_item_state::Returned> {
        WorkItem {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }

    /// `settle` — `Assigned` → `Done`. Taken by the `settled` outcome of `agency.work.SettleWorkItem`.
    pub fn settle(self) -> WorkItem<work_item_state::Done> {
        WorkItem {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }

    /// `abandon` — `Assigned` → `Abandoned`. Taken by the `abandoned` outcome of `agency.work.AbandonWorkItem`.
    pub fn abandon(self) -> WorkItem<work_item_state::Abandoned> {
        WorkItem {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }
}

impl WorkItem<work_item_state::Received> {
    /// `assign` — `Received` → `Assigned`. Taken by the `assigned` outcome of `agency.work.AssignWorkItem`.
    pub fn assign(self) -> WorkItem<work_item_state::Assigned> {
        WorkItem {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }
}

/// `agency.work.WorkItem` as it crosses a boundary: the state as a value beside the data.
///
/// Wire and storage know states only at runtime; [`WorkItemSnapshot::refine`] is the one door back
/// into the typed lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkItemSnapshot {
    /// Where the instance is in its lifecycle.
    pub state: WorkItemState,
    /// What it holds.
    pub data: WorkItemData,
}

/// An `WorkItem` in whichever declared state it was found.
pub enum AnyWorkItem {
    /// Resting in `Abandoned`.
    Abandoned(WorkItem<work_item_state::Abandoned>),
    /// Resting in `Assigned`.
    Assigned(WorkItem<work_item_state::Assigned>),
    /// Resting in `Done`.
    Done(WorkItem<work_item_state::Done>),
    /// Resting in `Received`.
    Received(WorkItem<work_item_state::Received>),
    /// Resting in `Returned`.
    Returned(WorkItem<work_item_state::Returned>),
}

impl WorkItemSnapshot {
    /// Refines the runtime state into the typed one.
    ///
    /// Total: every declared state has an arm, and an undeclared state cannot reach here because
    /// `WorkItemState` cannot spell one.
    pub fn refine(self) -> AnyWorkItem {
        match self.state {
            WorkItemState::Abandoned => AnyWorkItem::Abandoned(WorkItem {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            WorkItemState::Assigned => AnyWorkItem::Assigned(WorkItem {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            WorkItemState::Done => AnyWorkItem::Done(WorkItem {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            WorkItemState::Received => AnyWorkItem::Received(WorkItem {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            WorkItemState::Returned => AnyWorkItem::Returned(WorkItem {
                data: self.data,
                state: core::marker::PhantomData,
            }),
        }
    }
}

impl AnyWorkItem {
    /// The state, as the runtime value.
    pub fn state(&self) -> WorkItemState {
        match self {
            Self::Abandoned(_) => WorkItemState::Abandoned,
            Self::Assigned(_) => WorkItemState::Assigned,
            Self::Done(_) => WorkItemState::Done,
            Self::Received(_) => WorkItemState::Received,
            Self::Returned(_) => WorkItemState::Returned,
        }
    }

    /// Back to the boundary shape.
    pub fn snapshot(self) -> WorkItemSnapshot {
        match self {
            Self::Abandoned(instance) => WorkItemSnapshot {
                state: WorkItemState::Abandoned,
                data: instance.into_data(),
            },
            Self::Assigned(instance) => WorkItemSnapshot {
                state: WorkItemState::Assigned,
                data: instance.into_data(),
            },
            Self::Done(instance) => WorkItemSnapshot {
                state: WorkItemState::Done,
                data: instance.into_data(),
            },
            Self::Received(instance) => WorkItemSnapshot {
                state: WorkItemState::Received,
                data: instance.into_data(),
            },
            Self::Returned(instance) => WorkItemSnapshot {
                state: WorkItemState::Returned,
                data: instance.into_data(),
            },
        }
    }
}

/// Abandon a work item — the input of `agency.work.AbandonWorkItem`.
///
/// Everything it can result in is [`AbandonWorkItemOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbandonWorkItem {
    /// `item_ref` — `agency.work.WorkItemRef`.
    pub item_ref: WorkItemRef,
}

/// Everything `agency.work.AbandonWorkItem` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbandonWorkItemOutcome {
    /// `no-such-item` — for an identity no record carries.
    NoSuchItem {
        /// Why it was refused: `agency.work.NotFound`.
        error: NotFound,
    },
    /// `abandoned` — otherwise.
    Abandoned {
        /// The `agency.work.WorkItemAbandoned` this outcome publishes.
        work_item_abandoned: WorkItemAbandoned,
    },
    /// `wrong-state` — from a state no declared move starts in.
    WrongState {
        /// Why it was refused: `agency.work.StateConflict`.
        error: StateConflict,
    },
}

/// Assign a work item — the input of `agency.work.AssignWorkItem`.
///
/// Everything it can result in is [`AssignWorkItemOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignWorkItem {
    /// `item_ref` — `agency.work.WorkItemRef`.
    pub item_ref: WorkItemRef,
    /// `agent_id` — `agency.agents.AgentId`.
    pub agent_id: crate::agents::AgentId,
}

/// Everything `agency.work.AssignWorkItem` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignWorkItemOutcome {
    /// `no-such-item` — for an identity no record carries.
    NoSuchItem {
        /// Why it was refused: `agency.work.NotFound`.
        error: NotFound,
    },
    /// `assigned` — otherwise.
    Assigned {
        /// The `agency.work.WorkItemAssigned` this outcome publishes.
        work_item_assigned: WorkItemAssigned,
    },
    /// `wrong-state` — from a state no declared move starts in.
    WrongState {
        /// Why it was refused: `agency.work.StateConflict`.
        error: StateConflict,
    },
}

/// Configure a work input — the input of `agency.work.ConfigureWorkInput`.
///
/// Everything it can result in is [`ConfigureWorkInputOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigureWorkInput {
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: crate::auth::TenantId,
    /// `name` — `String`.
    pub name: String,
    /// `source` — `agency.work.WorkSource`.
    pub source: WorkSource,
    /// `poll_interval` — `Optional<Duration>`.
    pub poll_interval: Option<crate::primitives::Duration>,
    /// `workforce_id` — `agency.teams.WorkforceId`.
    pub workforce_id: crate::teams::WorkforceId,
    /// `transform_ref` — `Optional<String>`.
    pub transform_ref: Option<String>,
}

/// Everything `agency.work.ConfigureWorkInput` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigureWorkInputOutcome {
    /// `configured` — otherwise.
    Configured {
        /// The `agency.work.WorkInputConfigured` this outcome publishes.
        work_input_configured: WorkInputConfigured,
    },
}

/// Pause a work input — the input of `agency.work.PauseWorkInput`.
///
/// Everything it can result in is [`PauseWorkInputOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PauseWorkInput {
    /// `input_id` — `agency.work.InputId`.
    pub input_id: InputId,
}

/// Everything `agency.work.PauseWorkInput` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PauseWorkInputOutcome {
    /// `no-such-input` — for an identity no record carries.
    NoSuchInput {
        /// Why it was refused: `agency.work.NotFound`.
        error: NotFound,
    },
    /// `paused` — otherwise.
    Paused {
        /// The `agency.work.WorkInputPaused` this outcome publishes.
        work_input_paused: WorkInputPaused,
    },
    /// `wrong-state` — from a state no declared move starts in.
    WrongState {
        /// Why it was refused: `agency.work.StateConflict`.
        error: StateConflict,
    },
}

/// Remove a work input — the input of `agency.work.RemoveWorkInput`.
///
/// Everything it can result in is [`RemoveWorkInputOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveWorkInput {
    /// `input_id` — `agency.work.InputId`.
    pub input_id: InputId,
}

/// Everything `agency.work.RemoveWorkInput` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RemoveWorkInputOutcome {
    /// `no-such-input` — for an identity no record carries.
    NoSuchInput {
        /// Why it was refused: `agency.work.NotFound`.
        error: NotFound,
    },
    /// `removed` — otherwise.
    Removed {
        /// The `agency.work.WorkInputRemoved` this outcome publishes.
        work_input_removed: WorkInputRemoved,
    },
    /// `wrong-state` — from a state no declared move starts in.
    WrongState {
        /// Why it was refused: `agency.work.StateConflict`.
        error: StateConflict,
    },
}

/// Return a work item — the input of `agency.work.ReturnWorkItem`.
///
/// Everything it can result in is [`ReturnWorkItemOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnWorkItem {
    /// `item_ref` — `agency.work.WorkItemRef`.
    pub item_ref: WorkItemRef,
}

/// Everything `agency.work.ReturnWorkItem` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReturnWorkItemOutcome {
    /// `no-such-item` — for an identity no record carries.
    NoSuchItem {
        /// Why it was refused: `agency.work.NotFound`.
        error: NotFound,
    },
    /// `returned` — otherwise.
    Returned {
        /// The `agency.work.WorkItemReturned` this outcome publishes.
        work_item_returned: WorkItemReturned,
    },
    /// `wrong-state` — from a state no declared move starts in.
    WrongState {
        /// Why it was refused: `agency.work.StateConflict`.
        error: StateConflict,
    },
}

/// Settle a work item — the input of `agency.work.SettleWorkItem`.
///
/// Everything it can result in is [`SettleWorkItemOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleWorkItem {
    /// `item_ref` — `agency.work.WorkItemRef`.
    pub item_ref: WorkItemRef,
}

/// Everything `agency.work.SettleWorkItem` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettleWorkItemOutcome {
    /// `no-such-item` — for an identity no record carries.
    NoSuchItem {
        /// Why it was refused: `agency.work.NotFound`.
        error: NotFound,
    },
    /// `settled` — otherwise.
    Settled {
        /// The `agency.work.WorkItemSettled` this outcome publishes.
        work_item_settled: WorkItemSettled,
    },
    /// `wrong-state` — from a state no declared move starts in.
    WrongState {
        /// Why it was refused: `agency.work.StateConflict`.
        error: StateConflict,
    },
}

/// Submit a work item — the input of `agency.work.SubmitWorkItem`.
///
/// Everything it can result in is [`SubmitWorkItemOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmitWorkItem {
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: crate::auth::TenantId,
    /// `item_ref` — `agency.work.WorkItemRef`.
    pub item_ref: WorkItemRef,
    /// `input_id` — `agency.work.InputId`.
    pub input_id: InputId,
    /// `title` — `String`.
    pub title: String,
    /// `payload_digest` — `agency.shared.Digest`.
    pub payload_digest: crate::shared::Digest,
}

/// Everything `agency.work.SubmitWorkItem` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubmitWorkItemOutcome {
    /// `received` — otherwise.
    Received {
        /// The `agency.work.WorkItemSubmitted` this outcome publishes.
        work_item_submitted: WorkItemSubmitted,
    },
}

/// WorkInputConfigured — the event `agency.work.WorkInputConfigured`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkInputConfigured {
    /// `input_id` — `agency.work.InputId`.
    pub input_id: InputId,
    /// `name` — `String`.
    pub name: String,
    /// `workforce_id` — `agency.teams.WorkforceId`.
    pub workforce_id: crate::teams::WorkforceId,
}

/// WorkInputPaused — the event `agency.work.WorkInputPaused`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkInputPaused {
    /// `input_id` — `agency.work.InputId`.
    pub input_id: InputId,
}

/// WorkInputRemoved — the event `agency.work.WorkInputRemoved`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkInputRemoved {
    /// `input_id` — `agency.work.InputId`.
    pub input_id: InputId,
}

/// WorkItemAbandoned — the event `agency.work.WorkItemAbandoned`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkItemAbandoned {
    /// `item_ref` — `agency.work.WorkItemRef`.
    pub item_ref: WorkItemRef,
}

/// WorkItemAssigned — the event `agency.work.WorkItemAssigned`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkItemAssigned {
    /// `item_ref` — `agency.work.WorkItemRef`.
    pub item_ref: WorkItemRef,
    /// `agent_id` — `agency.agents.AgentId`.
    pub agent_id: crate::agents::AgentId,
}

/// WorkItemReturned — the event `agency.work.WorkItemReturned`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkItemReturned {
    /// `item_ref` — `agency.work.WorkItemRef`.
    pub item_ref: WorkItemRef,
}

/// WorkItemSettled — the event `agency.work.WorkItemSettled`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkItemSettled {
    /// `item_ref` — `agency.work.WorkItemRef`.
    pub item_ref: WorkItemRef,
}

/// WorkItemSubmitted — the event `agency.work.WorkItemSubmitted`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkItemSubmitted {
    /// `item_ref` — `agency.work.WorkItemRef`.
    pub item_ref: WorkItemRef,
    /// `input_id` — `agency.work.InputId`.
    pub input_id: InputId,
    /// `payload_digest` — `agency.shared.Digest`.
    pub payload_digest: crate::shared::Digest,
}

/// The declared error `agency.work.NotFound`.
///
/// The addressed record does not exist, and nothing changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotFound;

/// The declared error `agency.work.StateConflict`.
///
/// The addressed record is not in a state this command acts from, and nothing changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateConflict;

/// WorkInputList — one row of the view `agency.work.WorkInputList`.
///
/// Projects `agency.work.WorkInput` at `read_your_writes` consistency.
/// The specification fully determines every row, so its query is generated over the storage port —
/// see the plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkInputList {
    /// `input_id` — `agency.work.InputId`.
    pub input_id: InputId,
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: crate::auth::TenantId,
    /// `name` — `String`.
    pub name: String,
    /// `source` — `agency.work.WorkSource`.
    pub source: WorkSource,
    /// `poll_interval` — `Optional<Duration>`.
    pub poll_interval: Option<crate::primitives::Duration>,
    /// `workforce_id` — `agency.teams.WorkforceId`.
    pub workforce_id: crate::teams::WorkforceId,
    /// `transform_ref` — `Optional<String>`.
    pub transform_ref: Option<String>,
    /// `state` — `agency.work.WorkInput.State`.
    pub state: WorkInputState,
}

/// WorkItemDetail — one row of the view `agency.work.WorkItemDetail`.
///
/// Projects `agency.work.WorkItem` at `read_your_writes` consistency, containing instances where `item_ref == param.item_ref`.
/// Serving it is an implementation obligation — see the plan — because how a projection is kept
/// current is a storage decision the specification does not take.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkItemDetail {
    /// `item_ref` — `agency.work.WorkItemRef`.
    pub item_ref: WorkItemRef,
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: crate::auth::TenantId,
    /// `input_id` — `agency.work.InputId`.
    pub input_id: InputId,
    /// `title` — `String`.
    pub title: String,
    /// `payload_digest` — `agency.shared.Digest`.
    pub payload_digest: crate::shared::Digest,
    /// `assigned_agent_id` — `Optional<agency.agents.AgentId>`.
    pub assigned_agent_id: Option<crate::agents::AgentId>,
    /// `received_at` — `Timestamp`.
    pub received_at: crate::primitives::Timestamp,
    /// `settled_at` — `Optional<Timestamp>`.
    pub settled_at: Option<crate::primitives::Timestamp>,
    /// `state` — `agency.work.WorkItem.State`.
    pub state: WorkItemState,
}

/// WorkQueue — one row of the view `agency.work.WorkQueue`.
///
/// Projects `agency.work.WorkItem` at `read_your_writes` consistency.
/// Serving it is an implementation obligation — see the plan — because how a projection is kept
/// current is a storage decision the specification does not take.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkQueue {
    /// `item_ref` — `agency.work.WorkItemRef`.
    pub item_ref: WorkItemRef,
    /// `input_id` — `agency.work.InputId`.
    pub input_id: InputId,
    /// `workforce_id` — `Optional<agency.teams.WorkforceId>`.
    pub workforce_id: Option<crate::teams::WorkforceId>,
    /// `title` — `String`.
    pub title: String,
    /// `assigned_agent_id` — `Optional<agency.agents.AgentId>`.
    pub assigned_agent_id: Option<crate::agents::AgentId>,
    /// `received_at` — `Timestamp`.
    pub received_at: crate::primitives::Timestamp,
    /// `state` — `agency.work.WorkItem.State`.
    pub state: WorkItemState,
}

/// What this bounded context owes its implementor, and the seams of what is generated.
///
/// One trait per obligation in the synthesis plan, each carrying the plan's own contract, and one
/// per generated behaviour, which [`Generated`](crate::behaviour::Generated) implements.
/// [`Unimplemented`](obligations::Unimplemented) satisfies every owed trait by refusing in the type system.
pub mod obligations {
    /// The behaviour `agency.work.AbandonWorkItem` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait AbandonWorkItemBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.work.AbandonWorkItem`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn abandon_work_item(&mut self, input: super::AbandonWorkItem) -> Result<super::AbandonWorkItemOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.work.AssignWorkItem` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait AssignWorkItemBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.work.AssignWorkItem`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn assign_work_item(&mut self, input: super::AssignWorkItem) -> Result<super::AssignWorkItemOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.work.ConfigureWorkInput` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait ConfigureWorkInputBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.work.ConfigureWorkInput`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn configure_work_input(&mut self, input: super::ConfigureWorkInput) -> Result<super::ConfigureWorkInputOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.work.PauseWorkInput` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait PauseWorkInputBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.work.PauseWorkInput`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn pause_work_input(&mut self, input: super::PauseWorkInput) -> Result<super::PauseWorkInputOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.work.RemoveWorkInput` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait RemoveWorkInputBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.work.RemoveWorkInput`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn remove_work_input(&mut self, input: super::RemoveWorkInput) -> Result<super::RemoveWorkInputOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.work.ReturnWorkItem` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait ReturnWorkItemBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.work.ReturnWorkItem`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn return_work_item(&mut self, input: super::ReturnWorkItem) -> Result<super::ReturnWorkItemOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.work.SettleWorkItem` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait SettleWorkItemBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.work.SettleWorkItem`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn settle_work_item(&mut self, input: super::SettleWorkItem) -> Result<super::SettleWorkItemOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.work.SubmitWorkItem` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait SubmitWorkItemBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.work.SubmitWorkItem`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn submit_work_item(&mut self, input: super::SubmitWorkItem) -> Result<super::SubmitWorkItemOutcome, crate::obligation::UnmetObligation>;
    }

    /// The query `agency.work.WorkInputList` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage port. Implement it yourself to replace that query.
    pub trait WorkInputListQuery {
        /// Serves `agency.work.WorkInputList` rows at the view's declared consistency.
        ///
        /// `Err` is the typed refusal of a row whose declared type cannot hold its value.
        fn work_input_list(&self) -> Result<Vec<super::WorkInputList>, crate::obligation::UnmetObligation>;
    }

    /// The query `agency.work.WorkItemDetail` — an implementation obligation.
    ///
    /// Why it is not generated: kept an obligation by a view parameter (`params:`), which a generated query does not apply.
    ///
    /// Contract: a query answering `agency.work.WorkItemDetail` with rows projected from `agency.work.WorkItem` at `read_your_writes` consistency, containing instances where `item_ref == param.item_ref`.
    pub trait WorkItemDetailQuery {
        /// Serves `agency.work.WorkItemDetail` rows at the view's declared consistency.
        ///
        /// `Err` is the typed refusal of an obligation nothing has satisfied; a satisfying
        /// implementation never returns it.
        fn work_item_detail(&self, item_ref: crate::work::WorkItemRef) -> Result<Vec<super::WorkItemDetail>, crate::obligation::UnmetObligation>;
    }

    /// The query `agency.work.WorkQueue` — an implementation obligation.
    ///
    /// Why it is not generated: kept an obligation by a read through the reference `input_id` to `agency.work.WorkInput` (`field: {related: …}` or `input_id.<field>` in `filter:`), which reads a second entity's rows.
    ///
    /// Contract: a query answering `agency.work.WorkQueue` with rows projected from `agency.work.WorkItem` at `read_your_writes` consistency.
    pub trait WorkQueueQuery {
        /// Serves `agency.work.WorkQueue` rows at the view's declared consistency.
        ///
        /// `Err` is the typed refusal of an obligation nothing has satisfied; a satisfying
        /// implementation never returns it.
        fn work_queue(&self) -> Result<Vec<super::WorkQueue>, crate::obligation::UnmetObligation>;
    }

    /// Every obligation of this bounded context, refused in the type system.
    ///
    /// Each method returns the typed refusal naming what is owed — never a panic, never a guessed
    /// value — so a workspace built on this stub compiles and reports its own gaps.
    pub struct Unimplemented;

    impl WorkItemDetailQuery for Unimplemented {
        fn work_item_detail(&self, _item_ref: crate::work::WorkItemRef) -> Result<Vec<super::WorkItemDetail>, crate::obligation::UnmetObligation> {
            Err(crate::obligation::UnmetObligation { capability: "view query", source: "agency.work.WorkItemDetail" })
        }
    }

    impl WorkQueueQuery for Unimplemented {
        fn work_queue(&self) -> Result<Vec<super::WorkQueue>, crate::obligation::UnmetObligation> {
            Err(crate::obligation::UnmetObligation { capability: "view query", source: "agency.work.WorkQueue" })
        }
    }
}
