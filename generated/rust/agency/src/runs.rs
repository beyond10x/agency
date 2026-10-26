// generated from agency v1
// model digest 71132e90bde806415603d08bb97a71629b2c2f6cda30e1f22e76f407abd9a3eb
// contract digest a96f55e3e1ca68aeb1579b032c3547fffd27e8ab80e76bcb898f9b6c9258792e
// do not edit: regenerate with `ess synthesize --layout crate`

//! Runs — `agency.runs`.
//!
//! Agency's record of each run Loom executes for an agent: what it was asked to work on, where, under which budget, what Loom reported back, and the approvals the run asked for.
//!
//! Everything this bounded context declares that the synthesis plan marks generated.

/// ApprovalId — `agency.runs.ApprovalId`: a distinct wrapper around `Uuid`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApprovalId(pub crate::primitives::Uuid);

/// CredentialSource — `agency.runs.CredentialSource`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CredentialSource {
    /// `OperatorLogin`.
    OperatorLogin,
    /// `ApiKey`.
    ApiKey,
    /// `Loopback`.
    Loopback,
    /// `None`.
    None,
}

/// LoomSessionId — `agency.runs.LoomSessionId`: a distinct wrapper around `Uuid`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoomSessionId(pub crate::primitives::Uuid);

/// The states of `agency.runs.Run`, as runtime values.
///
/// Synthesised from the lifecycle, so the two cannot disagree. Which *moves* are legal is not
/// carried here — it is carried by `Run<S>`, where an undeclared move does not compile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunState {
    /// `Admitted`.
    Admitted,
    /// `Failed`.
    Failed,
    /// `Proposed`.
    Proposed,
    /// `Requested`.
    Requested,
    /// `Running`.
    Running,
    /// `Stopped`.
    Stopped,
}

/// The states of `agency.runs.RunApproval`, as runtime values.
///
/// Synthesised from the lifecycle, so the two cannot disagree. Which *moves* are legal is not
/// carried here — it is carried by `RunApproval<S>`, where an undeclared move does not compile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunApprovalState {
    /// `Denied`.
    Denied,
    /// `Granted`.
    Granted,
    /// `Pending`.
    Pending,
}

/// RunEndReason — `agency.runs.RunEndReason`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunEndReason {
    /// `reason` — `String`.
    pub reason: String,
}

/// RunId — `agency.runs.RunId`: a distinct wrapper around `Uuid`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunId(pub crate::primitives::Uuid);

/// RunOutcome — `agency.runs.RunOutcome`: one of a fixed set of shapes, tagged on the wire by `kind`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunOutcome {
    /// Tagged `Failed` — `agency.runs.RunEndReason`.
    Failed(RunEndReason),
    /// Tagged `Proposal`, carrying nothing.
    Proposal,
    /// Tagged `Stopped` — `agency.runs.RunEndReason`.
    Stopped(RunEndReason),
}

/// RunUsage — `agency.runs.RunUsage`.
///
/// The totals of Loom's RunUsageReport for one run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunUsage {
    /// `model_calls` — `Integer`.
    pub model_calls: i64,
    /// `input_tokens` — `Integer`.
    pub input_tokens: i64,
    /// `output_tokens` — `Integer`.
    pub output_tokens: i64,
    /// `cached_input_tokens` — `Integer`.
    pub cached_input_tokens: i64,
    /// `cost_micro_usd` — `Optional<Integer>`.
    pub cost_micro_usd: Option<i64>,
}

/// RunWorkspace — `agency.runs.RunWorkspace`.
///
/// The repository, base branch and volume claim a run works in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunWorkspace {
    /// `repository` — `String`.
    pub repository: String,
    /// `base_branch` — `String`.
    pub base_branch: String,
    /// `volume_claim` — `Optional<String>`.
    pub volume_claim: Option<String>,
}

/// What Run — `agency.runs.Run` — holds, apart from where it is in its lifecycle.
///
/// The identity and every declared field. The state is deliberately not one: inside the domain it
/// is carried by the type parameter of [`Run<S>`], and at a boundary by [`RunSnapshot::state`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunData {
    /// The identity: `run_id` — `agency.runs.RunId`.
    pub run_id: RunId,
    /// `tenant_id` — `agency.auth.TenantId`.
    ///
    /// Carries `runs`: `agency.auth.Tenant` owns many `agency.runs.Run`.
    pub tenant_id: crate::auth::TenantId,
    /// `agent_id` — `agency.agents.AgentId`.
    ///
    /// Carries `agent`: `agency.runs.Run` references one `agency.agents.Agent`.
    pub agent_id: crate::agents::AgentId,
    /// `work_item_ref` — `agency.work.WorkItemRef`.
    ///
    /// Carries `work_item`: `agency.runs.Run` references one `agency.work.WorkItem`.
    pub work_item_ref: crate::work::WorkItemRef,
    /// `budget` — `agency.shared.RunBudget`.
    pub budget: crate::shared::RunBudget,
    /// `workspace` — `Optional<agency.runs.RunWorkspace>`.
    pub workspace: Option<RunWorkspace>,
    /// `session_id` — `Optional<agency.runs.LoomSessionId>`.
    pub session_id: Option<LoomSessionId>,
    /// `credential_source` — `Optional<agency.runs.CredentialSource>`.
    pub credential_source: Option<CredentialSource>,
    /// `started_at` — `Timestamp`.
    pub started_at: crate::primitives::Timestamp,
    /// `ended_at` — `Optional<Timestamp>`.
    pub ended_at: Option<crate::primitives::Timestamp>,
    /// `outcome` — `Optional<agency.runs.RunOutcome>`.
    pub outcome: Option<RunOutcome>,
    /// `usage` — `Optional<agency.runs.RunUsage>`.
    pub usage: Option<RunUsage>,
}

/// The states of `agency.runs.Run`, at the type level.
///
/// One marker type per declared state, sealed: a state the lifecycle does not declare cannot
/// implement [`Marker`](run_state::Marker), so [`Run<S>`](Run) can only ever rest in a real state.
pub mod run_state {
    /// Closes [`Marker`] over the declared states.
    mod sealed {
        /// Implemented only by the marker types beside this module.
        pub trait Sealed {}
        impl Sealed for super::Admitted {}
        impl Sealed for super::Failed {}
        impl Sealed for super::Proposed {}
        impl Sealed for super::Requested {}
        impl Sealed for super::Running {}
        impl Sealed for super::Stopped {}
    }

    /// A declared state of `Run`, as a type.
    pub trait Marker: sealed::Sealed {
        /// The same state, as the runtime value.
        const STATE: super::RunState;
    }

    /// `Admitted`.
    pub struct Admitted;

    impl Marker for Admitted {
        const STATE: super::RunState = super::RunState::Admitted;
    }

    /// `Failed`. Terminal: an instance may rest here forever.
    pub struct Failed;

    impl Marker for Failed {
        const STATE: super::RunState = super::RunState::Failed;
    }

    /// `Proposed`. Terminal: an instance may rest here forever.
    pub struct Proposed;

    impl Marker for Proposed {
        const STATE: super::RunState = super::RunState::Proposed;
    }

    /// `Requested`. Where a new instance starts.
    pub struct Requested;

    impl Marker for Requested {
        const STATE: super::RunState = super::RunState::Requested;
    }

    /// `Running`.
    pub struct Running;

    impl Marker for Running {
        const STATE: super::RunState = super::RunState::Running;
    }

    /// `Stopped`. Terminal: an instance may rest here forever.
    pub struct Stopped;

    impl Marker for Stopped {
        const STATE: super::RunState = super::RunState::Stopped;
    }
}

/// Run — `agency.runs.Run` — with its lifecycle state carried by the type.
///
/// The one constructor rests in `Requested`, and the only way to change `S` is a method generated from
/// a declared transition. A move the specification does not declare is therefore not an error
/// case: it does not compile. Where the state is data — wire, storage — use [`RunSnapshot`]
/// and [`RunSnapshot::refine`].
pub struct Run<S: run_state::Marker> {
    data: RunData,
    state: core::marker::PhantomData<S>,
}

impl<S: run_state::Marker> Run<S> {
    /// The state this instance rests in, as the runtime value.
    pub fn state(&self) -> RunState {
        S::STATE
    }

    /// What it holds.
    pub fn data(&self) -> &RunData {
        &self.data
    }

    /// Hands the data back, giving up the typed state.
    pub fn into_data(self) -> RunData {
        self.data
    }
}

impl Run<run_state::Requested> {
    /// A new instance, resting in `Requested` — the only state the lifecycle starts one in.
    pub fn new(data: RunData) -> Self {
        Self {
            data,
            state: core::marker::PhantomData,
        }
    }
}

impl Run<run_state::Admitted> {
    /// `start` — `Admitted` → `Running`. Taken by the `started` outcome of `agency.runs.RecordRunStarted`.
    pub fn start(self) -> Run<run_state::Running> {
        Run {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }

    /// `stop` — `Admitted` → `Stopped`. Taken by the `stopped` outcome of `agency.runs.StopRun`.
    pub fn stop(self) -> Run<run_state::Stopped> {
        Run {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }
}

impl Run<run_state::Requested> {
    /// `admit` — `Requested` → `Admitted`. Taken by the `admitted` outcome of `agency.runs.AdmitRun`.
    pub fn admit(self) -> Run<run_state::Admitted> {
        Run {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }

    /// `stop` — `Requested` → `Stopped`. Taken by the `stopped` outcome of `agency.runs.StopRun`.
    pub fn stop(self) -> Run<run_state::Stopped> {
        Run {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }
}

impl Run<run_state::Running> {
    /// `propose` — `Running` → `Proposed`. Taken by the `proposed` outcome of `agency.runs.RecordRunProposal`.
    pub fn propose(self) -> Run<run_state::Proposed> {
        Run {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }

    /// `stop` — `Running` → `Stopped`. Taken by the `stopped` outcome of `agency.runs.StopRun`.
    pub fn stop(self) -> Run<run_state::Stopped> {
        Run {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }

    /// `fail` — `Running` → `Failed`. Taken by the `failed` outcome of `agency.runs.RecordRunFailure`.
    pub fn fail(self) -> Run<run_state::Failed> {
        Run {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }
}

/// `agency.runs.Run` as it crosses a boundary: the state as a value beside the data.
///
/// Wire and storage know states only at runtime; [`RunSnapshot::refine`] is the one door back
/// into the typed lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunSnapshot {
    /// Where the instance is in its lifecycle.
    pub state: RunState,
    /// What it holds.
    pub data: RunData,
}

/// An `Run` in whichever declared state it was found.
pub enum AnyRun {
    /// Resting in `Admitted`.
    Admitted(Run<run_state::Admitted>),
    /// Resting in `Failed`.
    Failed(Run<run_state::Failed>),
    /// Resting in `Proposed`.
    Proposed(Run<run_state::Proposed>),
    /// Resting in `Requested`.
    Requested(Run<run_state::Requested>),
    /// Resting in `Running`.
    Running(Run<run_state::Running>),
    /// Resting in `Stopped`.
    Stopped(Run<run_state::Stopped>),
}

impl RunSnapshot {
    /// Refines the runtime state into the typed one.
    ///
    /// Total: every declared state has an arm, and an undeclared state cannot reach here because
    /// `RunState` cannot spell one.
    pub fn refine(self) -> AnyRun {
        match self.state {
            RunState::Admitted => AnyRun::Admitted(Run {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            RunState::Failed => AnyRun::Failed(Run {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            RunState::Proposed => AnyRun::Proposed(Run {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            RunState::Requested => AnyRun::Requested(Run {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            RunState::Running => AnyRun::Running(Run {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            RunState::Stopped => AnyRun::Stopped(Run {
                data: self.data,
                state: core::marker::PhantomData,
            }),
        }
    }
}

impl AnyRun {
    /// The state, as the runtime value.
    pub fn state(&self) -> RunState {
        match self {
            Self::Admitted(_) => RunState::Admitted,
            Self::Failed(_) => RunState::Failed,
            Self::Proposed(_) => RunState::Proposed,
            Self::Requested(_) => RunState::Requested,
            Self::Running(_) => RunState::Running,
            Self::Stopped(_) => RunState::Stopped,
        }
    }

    /// Back to the boundary shape.
    pub fn snapshot(self) -> RunSnapshot {
        match self {
            Self::Admitted(instance) => RunSnapshot {
                state: RunState::Admitted,
                data: instance.into_data(),
            },
            Self::Failed(instance) => RunSnapshot {
                state: RunState::Failed,
                data: instance.into_data(),
            },
            Self::Proposed(instance) => RunSnapshot {
                state: RunState::Proposed,
                data: instance.into_data(),
            },
            Self::Requested(instance) => RunSnapshot {
                state: RunState::Requested,
                data: instance.into_data(),
            },
            Self::Running(instance) => RunSnapshot {
                state: RunState::Running,
                data: instance.into_data(),
            },
            Self::Stopped(instance) => RunSnapshot {
                state: RunState::Stopped,
                data: instance.into_data(),
            },
        }
    }
}

/// What RunApproval — `agency.runs.RunApproval` — holds, apart from where it is in its lifecycle.
///
/// The identity and every declared field. The state is deliberately not one: inside the domain it
/// is carried by the type parameter of [`RunApproval<S>`], and at a boundary by [`RunApprovalSnapshot::state`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunApprovalData {
    /// The identity: `approval_id` — `agency.runs.ApprovalId`.
    pub approval_id: ApprovalId,
    /// `run_id` — `agency.runs.RunId`.
    ///
    /// Carries `run`: `agency.runs.RunApproval` references one `agency.runs.Run`.
    pub run_id: RunId,
    /// `subject` — `String`.
    pub subject: String,
    /// `capability` — `String`.
    pub capability: String,
    /// `requested_at` — `Timestamp`.
    pub requested_at: crate::primitives::Timestamp,
    /// `answered_by` — `Optional<agency.auth.PrincipalId>`.
    ///
    /// Carries `answerer`: `agency.runs.RunApproval` references one `agency.auth.Principal`.
    pub answered_by: Option<crate::auth::PrincipalId>,
    /// `answered_at` — `Optional<Timestamp>`.
    pub answered_at: Option<crate::primitives::Timestamp>,
}

/// The states of `agency.runs.RunApproval`, at the type level.
///
/// One marker type per declared state, sealed: a state the lifecycle does not declare cannot
/// implement [`Marker`](run_approval_state::Marker), so [`RunApproval<S>`](RunApproval) can only ever rest in a real state.
pub mod run_approval_state {
    /// Closes [`Marker`] over the declared states.
    mod sealed {
        /// Implemented only by the marker types beside this module.
        pub trait Sealed {}
        impl Sealed for super::Denied {}
        impl Sealed for super::Granted {}
        impl Sealed for super::Pending {}
    }

    /// A declared state of `RunApproval`, as a type.
    pub trait Marker: sealed::Sealed {
        /// The same state, as the runtime value.
        const STATE: super::RunApprovalState;
    }

    /// `Denied`. Terminal: an instance may rest here forever.
    pub struct Denied;

    impl Marker for Denied {
        const STATE: super::RunApprovalState = super::RunApprovalState::Denied;
    }

    /// `Granted`. Terminal: an instance may rest here forever.
    pub struct Granted;

    impl Marker for Granted {
        const STATE: super::RunApprovalState = super::RunApprovalState::Granted;
    }

    /// `Pending`. Where a new instance starts.
    pub struct Pending;

    impl Marker for Pending {
        const STATE: super::RunApprovalState = super::RunApprovalState::Pending;
    }
}

/// RunApproval — `agency.runs.RunApproval` — with its lifecycle state carried by the type.
///
/// The one constructor rests in `Pending`, and the only way to change `S` is a method generated from
/// a declared transition. A move the specification does not declare is therefore not an error
/// case: it does not compile. Where the state is data — wire, storage — use [`RunApprovalSnapshot`]
/// and [`RunApprovalSnapshot::refine`].
pub struct RunApproval<S: run_approval_state::Marker> {
    data: RunApprovalData,
    state: core::marker::PhantomData<S>,
}

impl<S: run_approval_state::Marker> RunApproval<S> {
    /// The state this instance rests in, as the runtime value.
    pub fn state(&self) -> RunApprovalState {
        S::STATE
    }

    /// What it holds.
    pub fn data(&self) -> &RunApprovalData {
        &self.data
    }

    /// Hands the data back, giving up the typed state.
    pub fn into_data(self) -> RunApprovalData {
        self.data
    }
}

impl RunApproval<run_approval_state::Pending> {
    /// A new instance, resting in `Pending` — the only state the lifecycle starts one in.
    pub fn new(data: RunApprovalData) -> Self {
        Self {
            data,
            state: core::marker::PhantomData,
        }
    }
}

impl RunApproval<run_approval_state::Pending> {
    /// `grant` — `Pending` → `Granted`. Taken by the `granted` outcome of `agency.runs.GrantRunApproval`.
    pub fn grant(self) -> RunApproval<run_approval_state::Granted> {
        RunApproval {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }

    /// `deny` — `Pending` → `Denied`. Taken by the `denied` outcome of `agency.runs.DenyRunApproval`.
    pub fn deny(self) -> RunApproval<run_approval_state::Denied> {
        RunApproval {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }
}

/// `agency.runs.RunApproval` as it crosses a boundary: the state as a value beside the data.
///
/// Wire and storage know states only at runtime; [`RunApprovalSnapshot::refine`] is the one door back
/// into the typed lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunApprovalSnapshot {
    /// Where the instance is in its lifecycle.
    pub state: RunApprovalState,
    /// What it holds.
    pub data: RunApprovalData,
}

/// An `RunApproval` in whichever declared state it was found.
pub enum AnyRunApproval {
    /// Resting in `Denied`.
    Denied(RunApproval<run_approval_state::Denied>),
    /// Resting in `Granted`.
    Granted(RunApproval<run_approval_state::Granted>),
    /// Resting in `Pending`.
    Pending(RunApproval<run_approval_state::Pending>),
}

impl RunApprovalSnapshot {
    /// Refines the runtime state into the typed one.
    ///
    /// Total: every declared state has an arm, and an undeclared state cannot reach here because
    /// `RunApprovalState` cannot spell one.
    pub fn refine(self) -> AnyRunApproval {
        match self.state {
            RunApprovalState::Denied => AnyRunApproval::Denied(RunApproval {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            RunApprovalState::Granted => AnyRunApproval::Granted(RunApproval {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            RunApprovalState::Pending => AnyRunApproval::Pending(RunApproval {
                data: self.data,
                state: core::marker::PhantomData,
            }),
        }
    }
}

impl AnyRunApproval {
    /// The state, as the runtime value.
    pub fn state(&self) -> RunApprovalState {
        match self {
            Self::Denied(_) => RunApprovalState::Denied,
            Self::Granted(_) => RunApprovalState::Granted,
            Self::Pending(_) => RunApprovalState::Pending,
        }
    }

    /// Back to the boundary shape.
    pub fn snapshot(self) -> RunApprovalSnapshot {
        match self {
            Self::Denied(instance) => RunApprovalSnapshot {
                state: RunApprovalState::Denied,
                data: instance.into_data(),
            },
            Self::Granted(instance) => RunApprovalSnapshot {
                state: RunApprovalState::Granted,
                data: instance.into_data(),
            },
            Self::Pending(instance) => RunApprovalSnapshot {
                state: RunApprovalState::Pending,
                data: instance.into_data(),
            },
        }
    }
}

/// Record a run admitted by Loom — the input of `agency.runs.AdmitRun`.
///
/// Everything it can result in is [`AdmitRunOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmitRun {
    /// `run_id` — `agency.runs.RunId`.
    pub run_id: RunId,
}

/// Everything `agency.runs.AdmitRun` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdmitRunOutcome {
    /// `no-such-run` — for an identity no record carries.
    NoSuchRun {
        /// Why it was refused: `agency.runs.NotFound`.
        error: NotFound,
    },
    /// `admitted` — otherwise.
    Admitted {
        /// The `agency.runs.RunAdmitted` this outcome publishes.
        run_admitted: RunAdmitted,
    },
    /// `wrong-state` — from a state no declared move starts in.
    WrongState {
        /// Why it was refused: `agency.runs.StateConflict`.
        error: StateConflict,
    },
}

/// Deny a run's approval — the input of `agency.runs.DenyRunApproval`.
///
/// Everything it can result in is [`DenyRunApprovalOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DenyRunApproval {
    /// `approval_id` — `agency.runs.ApprovalId`.
    pub approval_id: ApprovalId,
    /// `answered_by` — `agency.auth.PrincipalId`.
    pub answered_by: crate::auth::PrincipalId,
}

/// Everything `agency.runs.DenyRunApproval` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DenyRunApprovalOutcome {
    /// `no-such-approval` — for an identity no record carries.
    NoSuchApproval {
        /// Why it was refused: `agency.runs.NotFound`.
        error: NotFound,
    },
    /// `denied` — otherwise.
    Denied {
        /// The `agency.runs.RunApprovalDenied` this outcome publishes.
        run_approval_denied: RunApprovalDenied,
    },
    /// `wrong-state` — from a state no declared move starts in.
    WrongState {
        /// Why it was refused: `agency.runs.StateConflict`.
        error: StateConflict,
    },
}

/// Grant a run's approval — the input of `agency.runs.GrantRunApproval`.
///
/// Everything it can result in is [`GrantRunApprovalOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrantRunApproval {
    /// `approval_id` — `agency.runs.ApprovalId`.
    pub approval_id: ApprovalId,
    /// `answered_by` — `agency.auth.PrincipalId`.
    pub answered_by: crate::auth::PrincipalId,
}

/// Everything `agency.runs.GrantRunApproval` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrantRunApprovalOutcome {
    /// `no-such-approval` — for an identity no record carries.
    NoSuchApproval {
        /// Why it was refused: `agency.runs.NotFound`.
        error: NotFound,
    },
    /// `granted` — otherwise.
    Granted {
        /// The `agency.runs.RunApprovalGranted` this outcome publishes.
        run_approval_granted: RunApprovalGranted,
    },
    /// `wrong-state` — from a state no declared move starts in.
    WrongState {
        /// Why it was refused: `agency.runs.StateConflict`.
        error: StateConflict,
    },
}

/// Record a run's failure from Loom — the input of `agency.runs.RecordRunFailure`.
///
/// Everything it can result in is [`RecordRunFailureOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordRunFailure {
    /// `run_id` — `agency.runs.RunId`.
    pub run_id: RunId,
    /// `reason` — `String`.
    pub reason: String,
}

/// Everything `agency.runs.RecordRunFailure` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordRunFailureOutcome {
    /// `no-such-run` — for an identity no record carries.
    NoSuchRun {
        /// Why it was refused: `agency.runs.NotFound`.
        error: NotFound,
    },
    /// `failed` — otherwise.
    Failed {
        /// The `agency.runs.RunFailed` this outcome publishes.
        run_failed: RunFailed,
    },
    /// `wrong-state` — from a state no declared move starts in.
    WrongState {
        /// Why it was refused: `agency.runs.StateConflict`.
        error: StateConflict,
    },
}

/// Record a run's proposal from Loom — the input of `agency.runs.RecordRunProposal`.
///
/// Everything it can result in is [`RecordRunProposalOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordRunProposal {
    /// `run_id` — `agency.runs.RunId`.
    pub run_id: RunId,
    /// `usage` — `agency.runs.RunUsage`.
    pub usage: RunUsage,
}

/// Everything `agency.runs.RecordRunProposal` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordRunProposalOutcome {
    /// `no-such-run` — for an identity no record carries.
    NoSuchRun {
        /// Why it was refused: `agency.runs.NotFound`.
        error: NotFound,
    },
    /// `proposed` — otherwise.
    Proposed {
        /// The `agency.runs.RunProposed` this outcome publishes.
        run_proposed: RunProposed,
    },
    /// `wrong-state` — from a state no declared move starts in.
    WrongState {
        /// Why it was refused: `agency.runs.StateConflict`.
        error: StateConflict,
    },
}

/// Record a run started in Loom — the input of `agency.runs.RecordRunStarted`.
///
/// Everything it can result in is [`RecordRunStartedOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordRunStarted {
    /// `run_id` — `agency.runs.RunId`.
    pub run_id: RunId,
    /// `session_id` — `agency.runs.LoomSessionId`.
    pub session_id: LoomSessionId,
    /// `credential_source` — `agency.runs.CredentialSource`.
    pub credential_source: CredentialSource,
}

/// Everything `agency.runs.RecordRunStarted` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordRunStartedOutcome {
    /// `no-such-run` — for an identity no record carries.
    NoSuchRun {
        /// Why it was refused: `agency.runs.NotFound`.
        error: NotFound,
    },
    /// `started` — otherwise.
    Started {
        /// The `agency.runs.RunStarted` this outcome publishes.
        run_started: RunStarted,
    },
    /// `wrong-state` — from a state no declared move starts in.
    WrongState {
        /// Why it was refused: `agency.runs.StateConflict`.
        error: StateConflict,
    },
}

/// Record a run's request for an approval — the input of `agency.runs.RequestRunApproval`.
///
/// Everything it can result in is [`RequestRunApprovalOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestRunApproval {
    /// `run_id` — `agency.runs.RunId`.
    pub run_id: RunId,
    /// `subject` — `String`.
    pub subject: String,
    /// `capability` — `String`.
    pub capability: String,
}

/// Everything `agency.runs.RequestRunApproval` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequestRunApprovalOutcome {
    /// `requested` — otherwise.
    Requested {
        /// The `agency.runs.RunApprovalRequested` this outcome publishes.
        run_approval_requested: RunApprovalRequested,
    },
}

/// Set a run's budget — the input of `agency.runs.SetRunBudget`.
///
/// Everything it can result in is [`SetRunBudgetOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetRunBudget {
    /// `run_id` — `agency.runs.RunId`.
    pub run_id: RunId,
    /// `budget` — `agency.shared.RunBudget`.
    pub budget: crate::shared::RunBudget,
}

/// Everything `agency.runs.SetRunBudget` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetRunBudgetOutcome {
    /// `no-such-run` — for an identity no record carries.
    NoSuchRun {
        /// Why it was refused: `agency.runs.NotFound`.
        error: NotFound,
    },
    /// `ended` — when the existing subject's stored fields satisfy `(state == Proposed or state == Stopped or state == Failed)`.
    Ended {
        /// Why it was refused: `agency.runs.RunEnded`.
        error: RunEnded,
    },
    /// `set` — otherwise.
    Set {
        /// The `agency.runs.RunBudgetSet` this outcome publishes.
        run_budget_set: RunBudgetSet,
    },
}

/// Start a run — the input of `agency.runs.StartRun`.
///
/// Everything it can result in is [`StartRunOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartRun {
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: crate::auth::TenantId,
    /// `agent_id` — `agency.agents.AgentId`.
    pub agent_id: crate::agents::AgentId,
    /// `work_item_ref` — `agency.work.WorkItemRef`.
    pub work_item_ref: crate::work::WorkItemRef,
    /// `budget` — `agency.shared.RunBudget`.
    pub budget: crate::shared::RunBudget,
    /// `workspace` — `Optional<agency.runs.RunWorkspace>`.
    pub workspace: Option<RunWorkspace>,
}

/// Everything `agency.runs.StartRun` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StartRunOutcome {
    /// `requested` — otherwise.
    Requested {
        /// The `agency.runs.RunRequested` this outcome publishes.
        run_requested: RunRequested,
    },
}

/// Stop a run — the input of `agency.runs.StopRun`.
///
/// Everything it can result in is [`StopRunOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StopRun {
    /// `run_id` — `agency.runs.RunId`.
    pub run_id: RunId,
    /// `reason` — `String`.
    pub reason: String,
}

/// Everything `agency.runs.StopRun` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StopRunOutcome {
    /// `no-such-run` — for an identity no record carries.
    NoSuchRun {
        /// Why it was refused: `agency.runs.NotFound`.
        error: NotFound,
    },
    /// `stopped` — otherwise.
    Stopped {
        /// The `agency.runs.RunStopped` this outcome publishes.
        run_stopped: RunStopped,
    },
    /// `wrong-state` — from a state no declared move starts in.
    WrongState {
        /// Why it was refused: `agency.runs.StateConflict`.
        error: StateConflict,
    },
}

/// RunAdmitted — the event `agency.runs.RunAdmitted`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunAdmitted {
    /// `run_id` — `agency.runs.RunId`.
    pub run_id: RunId,
}

/// RunApprovalDenied — the event `agency.runs.RunApprovalDenied`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunApprovalDenied {
    /// `approval_id` — `agency.runs.ApprovalId`.
    pub approval_id: ApprovalId,
    /// `answered_by` — `agency.auth.PrincipalId`.
    pub answered_by: crate::auth::PrincipalId,
}

/// RunApprovalGranted — the event `agency.runs.RunApprovalGranted`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunApprovalGranted {
    /// `approval_id` — `agency.runs.ApprovalId`.
    pub approval_id: ApprovalId,
    /// `answered_by` — `agency.auth.PrincipalId`.
    pub answered_by: crate::auth::PrincipalId,
}

/// RunApprovalRequested — the event `agency.runs.RunApprovalRequested`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunApprovalRequested {
    /// `approval_id` — `agency.runs.ApprovalId`.
    pub approval_id: ApprovalId,
    /// `run_id` — `agency.runs.RunId`.
    pub run_id: RunId,
    /// `subject` — `String`.
    pub subject: String,
    /// `capability` — `String`.
    pub capability: String,
}

/// RunBudgetSet — the event `agency.runs.RunBudgetSet`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunBudgetSet {
    /// `run_id` — `agency.runs.RunId`.
    pub run_id: RunId,
    /// `budget` — `agency.shared.RunBudget`.
    pub budget: crate::shared::RunBudget,
}

/// RunFailed — the event `agency.runs.RunFailed`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunFailed {
    /// `run_id` — `agency.runs.RunId`.
    pub run_id: RunId,
    /// `reason` — `String`.
    pub reason: String,
}

/// RunProposed — the event `agency.runs.RunProposed`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunProposed {
    /// `run_id` — `agency.runs.RunId`.
    pub run_id: RunId,
    /// `usage` — `agency.runs.RunUsage`.
    pub usage: RunUsage,
}

/// RunRequested — the event `agency.runs.RunRequested`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunRequested {
    /// `run_id` — `agency.runs.RunId`.
    pub run_id: RunId,
    /// `agent_id` — `agency.agents.AgentId`.
    pub agent_id: crate::agents::AgentId,
    /// `work_item_ref` — `agency.work.WorkItemRef`.
    pub work_item_ref: crate::work::WorkItemRef,
}

/// RunStarted — the event `agency.runs.RunStarted`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunStarted {
    /// `run_id` — `agency.runs.RunId`.
    pub run_id: RunId,
    /// `session_id` — `agency.runs.LoomSessionId`.
    pub session_id: LoomSessionId,
    /// `credential_source` — `agency.runs.CredentialSource`.
    pub credential_source: CredentialSource,
}

/// RunStopped — the event `agency.runs.RunStopped`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunStopped {
    /// `run_id` — `agency.runs.RunId`.
    pub run_id: RunId,
    /// `reason` — `String`.
    pub reason: String,
}

/// The declared error `agency.runs.NotFound`.
///
/// The addressed record does not exist, and nothing changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotFound;

/// The declared error `agency.runs.RunEnded`.
///
/// The run has ended, so its budget is not changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunEnded;

/// The declared error `agency.runs.StateConflict`.
///
/// The addressed record is not in a state this command acts from, and nothing changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateConflict;

/// ApprovalInbox — one row of the view `agency.runs.ApprovalInbox`.
///
/// Projects `agency.runs.RunApproval` at `read_your_writes` consistency.
/// The specification fully determines every row, so its query is generated over the storage port —
/// see the plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApprovalInbox {
    /// `approval_id` — `agency.runs.ApprovalId`.
    pub approval_id: ApprovalId,
    /// `run_id` — `agency.runs.RunId`.
    pub run_id: RunId,
    /// `subject` — `String`.
    pub subject: String,
    /// `capability` — `String`.
    pub capability: String,
    /// `requested_at` — `Timestamp`.
    pub requested_at: crate::primitives::Timestamp,
    /// `answered_by` — `Optional<agency.auth.PrincipalId>`.
    pub answered_by: Option<crate::auth::PrincipalId>,
    /// `answered_at` — `Optional<Timestamp>`.
    pub answered_at: Option<crate::primitives::Timestamp>,
    /// `state` — `agency.runs.RunApproval.State`.
    pub state: RunApprovalState,
}

/// RunDetail — one row of the view `agency.runs.RunDetail`.
///
/// Projects `agency.runs.Run` at `read_your_writes` consistency, containing instances where `run_id == param.run_id`.
/// Serving it is an implementation obligation — see the plan — because how a projection is kept
/// current is a storage decision the specification does not take.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunDetail {
    /// `run_id` — `agency.runs.RunId`.
    pub run_id: RunId,
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: crate::auth::TenantId,
    /// `agent_id` — `agency.agents.AgentId`.
    pub agent_id: crate::agents::AgentId,
    /// `work_item_ref` — `agency.work.WorkItemRef`.
    pub work_item_ref: crate::work::WorkItemRef,
    /// `budget` — `agency.shared.RunBudget`.
    pub budget: crate::shared::RunBudget,
    /// `workspace` — `Optional<agency.runs.RunWorkspace>`.
    pub workspace: Option<RunWorkspace>,
    /// `session_id` — `Optional<agency.runs.LoomSessionId>`.
    pub session_id: Option<LoomSessionId>,
    /// `credential_source` — `Optional<agency.runs.CredentialSource>`.
    pub credential_source: Option<CredentialSource>,
    /// `started_at` — `Timestamp`.
    pub started_at: crate::primitives::Timestamp,
    /// `ended_at` — `Optional<Timestamp>`.
    pub ended_at: Option<crate::primitives::Timestamp>,
    /// `outcome` — `Optional<agency.runs.RunOutcome>`.
    pub outcome: Option<RunOutcome>,
    /// `usage` — `Optional<agency.runs.RunUsage>`.
    pub usage: Option<RunUsage>,
    /// `state` — `agency.runs.Run.State`.
    pub state: RunState,
}

/// RunList — one row of the view `agency.runs.RunList`.
///
/// Projects `agency.runs.Run` at `read_your_writes` consistency.
/// The specification fully determines every row, so its query is generated over the storage port —
/// see the plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunList {
    /// `run_id` — `agency.runs.RunId`.
    pub run_id: RunId,
    /// `tenant_id` — `agency.auth.TenantId`.
    pub tenant_id: crate::auth::TenantId,
    /// `agent_id` — `agency.agents.AgentId`.
    pub agent_id: crate::agents::AgentId,
    /// `work_item_ref` — `agency.work.WorkItemRef`.
    pub work_item_ref: crate::work::WorkItemRef,
    /// `budget` — `agency.shared.RunBudget`.
    pub budget: crate::shared::RunBudget,
    /// `workspace` — `Optional<agency.runs.RunWorkspace>`.
    pub workspace: Option<RunWorkspace>,
    /// `session_id` — `Optional<agency.runs.LoomSessionId>`.
    pub session_id: Option<LoomSessionId>,
    /// `credential_source` — `Optional<agency.runs.CredentialSource>`.
    pub credential_source: Option<CredentialSource>,
    /// `started_at` — `Timestamp`.
    pub started_at: crate::primitives::Timestamp,
    /// `ended_at` — `Optional<Timestamp>`.
    pub ended_at: Option<crate::primitives::Timestamp>,
    /// `usage` — `Optional<agency.runs.RunUsage>`.
    pub usage: Option<RunUsage>,
    /// `state` — `agency.runs.Run.State`.
    pub state: RunState,
}

/// UsageByAgent — one row of the view `agency.runs.UsageByAgent`.
///
/// Projects `agency.runs.Run` at `read_your_writes` consistency.
///
/// Grouped by `agent_id`; one row per group holding at least one instance.
/// The specification fully determines every row, so its query is generated over the storage port —
/// see the plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsageByAgent {
    /// `agent_id` — `agency.agents.AgentId`.
    pub agent_id: crate::agents::AgentId,
    /// `runs` — `Integer`.
    pub runs: i64,
}

/// What this bounded context owes its implementor, and the seams of what is generated.
///
/// One trait per obligation in the synthesis plan, each carrying the plan's own contract, and one
/// per generated behaviour, which [`Generated`](crate::behaviour::Generated) implements.
/// [`Unimplemented`](obligations::Unimplemented) satisfies every owed trait by refusing in the type system.
pub mod obligations {
    /// The behaviour `agency.runs.AdmitRun` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait AdmitRunBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.runs.AdmitRun`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn admit_run(&mut self, input: super::AdmitRun) -> Result<super::AdmitRunOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.runs.DenyRunApproval` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait DenyRunApprovalBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.runs.DenyRunApproval`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn deny_run_approval(&mut self, input: super::DenyRunApproval) -> Result<super::DenyRunApprovalOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.runs.GrantRunApproval` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait GrantRunApprovalBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.runs.GrantRunApproval`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn grant_run_approval(&mut self, input: super::GrantRunApproval) -> Result<super::GrantRunApprovalOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.runs.RecordRunFailure` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait RecordRunFailureBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.runs.RecordRunFailure`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn record_run_failure(&mut self, input: super::RecordRunFailure) -> Result<super::RecordRunFailureOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.runs.RecordRunProposal` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait RecordRunProposalBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.runs.RecordRunProposal`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn record_run_proposal(&mut self, input: super::RecordRunProposal) -> Result<super::RecordRunProposalOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.runs.RecordRunStarted` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait RecordRunStartedBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.runs.RecordRunStarted`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn record_run_started(&mut self, input: super::RecordRunStarted) -> Result<super::RecordRunStartedOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.runs.RequestRunApproval` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait RequestRunApprovalBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.runs.RequestRunApproval`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn request_run_approval(&mut self, input: super::RequestRunApproval) -> Result<super::RequestRunApprovalOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.runs.SetRunBudget` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait SetRunBudgetBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.runs.SetRunBudget`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn set_run_budget(&mut self, input: super::SetRunBudget) -> Result<super::SetRunBudgetOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.runs.StartRun` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait StartRunBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.runs.StartRun`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn start_run(&mut self, input: super::StartRun) -> Result<super::StartRunOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.runs.StopRun` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait StopRunBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.runs.StopRun`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn stop_run(&mut self, input: super::StopRun) -> Result<super::StopRunOutcome, crate::obligation::UnmetObligation>;
    }

    /// The query `agency.runs.ApprovalInbox` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage port. Implement it yourself to replace that query.
    pub trait ApprovalInboxQuery {
        /// Serves `agency.runs.ApprovalInbox` rows at the view's declared consistency.
        ///
        /// `Err` is the typed refusal of a row whose declared type cannot hold its value.
        fn approval_inbox(&self) -> Result<Vec<super::ApprovalInbox>, crate::obligation::UnmetObligation>;
    }

    /// The query `agency.runs.RunDetail` — an implementation obligation.
    ///
    /// Why it is not generated: kept an obligation by a view parameter (`params:`), which a generated query does not apply.
    ///
    /// Contract: a query answering `agency.runs.RunDetail` with rows projected from `agency.runs.Run` at `read_your_writes` consistency, containing instances where `run_id == param.run_id`.
    pub trait RunDetailQuery {
        /// Serves `agency.runs.RunDetail` rows at the view's declared consistency.
        ///
        /// `Err` is the typed refusal of an obligation nothing has satisfied; a satisfying
        /// implementation never returns it.
        fn run_detail(&self, run_id: crate::runs::RunId) -> Result<Vec<super::RunDetail>, crate::obligation::UnmetObligation>;
    }

    /// The query `agency.runs.RunList` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage port. Implement it yourself to replace that query.
    pub trait RunListQuery {
        /// Serves `agency.runs.RunList` rows at the view's declared consistency.
        ///
        /// `Err` is the typed refusal of a row whose declared type cannot hold its value.
        fn run_list(&self) -> Result<Vec<super::RunList>, crate::obligation::UnmetObligation>;
    }

    /// The query `agency.runs.UsageByAgent` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage port. Implement it yourself to replace that query.
    pub trait UsageByAgentQuery {
        /// Serves `agency.runs.UsageByAgent` rows at the view's declared consistency.
        ///
        /// `Err` is the typed refusal of a row whose declared type cannot hold its value.
        fn usage_by_agent(&self) -> Result<Vec<super::UsageByAgent>, crate::obligation::UnmetObligation>;
    }

    /// Every obligation of this bounded context, refused in the type system.
    ///
    /// Each method returns the typed refusal naming what is owed — never a panic, never a guessed
    /// value — so a workspace built on this stub compiles and reports its own gaps.
    pub struct Unimplemented;

    impl RunDetailQuery for Unimplemented {
        fn run_detail(&self, _run_id: crate::runs::RunId) -> Result<Vec<super::RunDetail>, crate::obligation::UnmetObligation> {
            Err(crate::obligation::UnmetObligation { capability: "view query", source: "agency.runs.RunDetail" })
        }
    }
}
