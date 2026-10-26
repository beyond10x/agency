// generated from agency v1
// model digest 71132e90bde806415603d08bb97a71629b2c2f6cda30e1f22e76f407abd9a3eb
// contract digest a96f55e3e1ca68aeb1579b032c3547fffd27e8ab80e76bcb898f9b6c9258792e
// do not edit: regenerate with `ess synthesize --layout crate`

//! Service — `agency.service`.
//!
//! The authenticated sessions of the Agency service: an operator's browser session, an application's token and a run's bearer, each held by the digest of its secret.
//!
//! Everything this bounded context declares that the synthesis plan marks generated.

/// The states of `agency.service.Session`, as runtime values.
///
/// Synthesised from the lifecycle, so the two cannot disagree. Which *moves* are legal is not
/// carried here — it is carried by `Session<S>`, where an undeclared move does not compile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    /// `Closed`.
    Closed,
    /// `Open`.
    Open,
}

/// SessionId — `agency.service.SessionId`: a distinct wrapper around `Uuid`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionId(pub crate::primitives::Uuid);

/// SessionKind — `agency.service.SessionKind`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionKind {
    /// `OperatorCookie`.
    OperatorCookie,
    /// `ApplicationToken`.
    ApplicationToken,
    /// `RunBearer`.
    RunBearer,
}

/// What Session — `agency.service.Session` — holds, apart from where it is in its lifecycle.
///
/// The identity and every declared field. The state is deliberately not one: inside the domain it
/// is carried by the type parameter of [`Session<S>`], and at a boundary by [`SessionSnapshot::state`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionData {
    /// The identity: `session_id` — `agency.service.SessionId`.
    pub session_id: SessionId,
    /// `principal_id` — `agency.auth.PrincipalId`.
    ///
    /// Carries `principal`: `agency.service.Session` references one `agency.auth.Principal`.
    pub principal_id: crate::auth::PrincipalId,
    /// `kind` — `agency.service.SessionKind`.
    pub kind: SessionKind,
    /// `run_id` — `Optional<agency.runs.RunId>`.
    ///
    /// Carries `run`: `agency.service.Session` references one `agency.runs.Run`.
    pub run_id: Option<crate::runs::RunId>,
    /// `token_digest` — `agency.shared.Digest`.
    pub token_digest: crate::shared::Digest,
    /// `csrf_digest` — `Optional<agency.shared.Digest>`.
    pub csrf_digest: Option<crate::shared::Digest>,
    /// `opened_at` — `Timestamp`.
    pub opened_at: crate::primitives::Timestamp,
    /// `expires_at` — `Optional<Timestamp>`.
    pub expires_at: Option<crate::primitives::Timestamp>,
}

/// The states of `agency.service.Session`, at the type level.
///
/// One marker type per declared state, sealed: a state the lifecycle does not declare cannot
/// implement [`Marker`](session_state::Marker), so [`Session<S>`](Session) can only ever rest in a real state.
pub mod session_state {
    /// Closes [`Marker`] over the declared states.
    mod sealed {
        /// Implemented only by the marker types beside this module.
        pub trait Sealed {}
        impl Sealed for super::Closed {}
        impl Sealed for super::Open {}
    }

    /// A declared state of `Session`, as a type.
    pub trait Marker: sealed::Sealed {
        /// The same state, as the runtime value.
        const STATE: super::SessionState;
    }

    /// `Closed`. Terminal: an instance may rest here forever.
    pub struct Closed;

    impl Marker for Closed {
        const STATE: super::SessionState = super::SessionState::Closed;
    }

    /// `Open`. Where a new instance starts.
    pub struct Open;

    impl Marker for Open {
        const STATE: super::SessionState = super::SessionState::Open;
    }
}

/// Session — `agency.service.Session` — with its lifecycle state carried by the type.
///
/// The one constructor rests in `Open`, and the only way to change `S` is a method generated from
/// a declared transition. A move the specification does not declare is therefore not an error
/// case: it does not compile. Where the state is data — wire, storage — use [`SessionSnapshot`]
/// and [`SessionSnapshot::refine`].
pub struct Session<S: session_state::Marker> {
    data: SessionData,
    state: core::marker::PhantomData<S>,
}

impl<S: session_state::Marker> Session<S> {
    /// The state this instance rests in, as the runtime value.
    pub fn state(&self) -> SessionState {
        S::STATE
    }

    /// What it holds.
    pub fn data(&self) -> &SessionData {
        &self.data
    }

    /// Hands the data back, giving up the typed state.
    pub fn into_data(self) -> SessionData {
        self.data
    }
}

impl Session<session_state::Open> {
    /// A new instance, resting in `Open` — the only state the lifecycle starts one in.
    pub fn new(data: SessionData) -> Self {
        Self {
            data,
            state: core::marker::PhantomData,
        }
    }
}

impl Session<session_state::Open> {
    /// `close` — `Open` → `Closed`. Taken by the `closed` outcome of `agency.service.CloseSession`.
    pub fn close(self) -> Session<session_state::Closed> {
        Session {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }
}

/// `agency.service.Session` as it crosses a boundary: the state as a value beside the data.
///
/// Wire and storage know states only at runtime; [`SessionSnapshot::refine`] is the one door back
/// into the typed lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionSnapshot {
    /// Where the instance is in its lifecycle.
    pub state: SessionState,
    /// What it holds.
    pub data: SessionData,
}

/// An `Session` in whichever declared state it was found.
pub enum AnySession {
    /// Resting in `Closed`.
    Closed(Session<session_state::Closed>),
    /// Resting in `Open`.
    Open(Session<session_state::Open>),
}

impl SessionSnapshot {
    /// Refines the runtime state into the typed one.
    ///
    /// Total: every declared state has an arm, and an undeclared state cannot reach here because
    /// `SessionState` cannot spell one.
    pub fn refine(self) -> AnySession {
        match self.state {
            SessionState::Closed => AnySession::Closed(Session {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            SessionState::Open => AnySession::Open(Session {
                data: self.data,
                state: core::marker::PhantomData,
            }),
        }
    }
}

impl AnySession {
    /// The state, as the runtime value.
    pub fn state(&self) -> SessionState {
        match self {
            Self::Closed(_) => SessionState::Closed,
            Self::Open(_) => SessionState::Open,
        }
    }

    /// Back to the boundary shape.
    pub fn snapshot(self) -> SessionSnapshot {
        match self {
            Self::Closed(instance) => SessionSnapshot {
                state: SessionState::Closed,
                data: instance.into_data(),
            },
            Self::Open(instance) => SessionSnapshot {
                state: SessionState::Open,
                data: instance.into_data(),
            },
        }
    }
}

/// Close a session — the input of `agency.service.CloseSession`.
///
/// Everything it can result in is [`CloseSessionOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloseSession {
    /// `session_id` — `agency.service.SessionId`.
    pub session_id: SessionId,
}

/// Everything `agency.service.CloseSession` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CloseSessionOutcome {
    /// `no-such-session` — for an identity no record carries.
    NoSuchSession {
        /// Why it was refused: `agency.service.NotFound`.
        error: NotFound,
    },
    /// `closed` — otherwise.
    Closed {
        /// The `agency.service.SessionClosed` this outcome publishes.
        session_closed: SessionClosed,
    },
    /// `wrong-state` — from a state no declared move starts in.
    WrongState {
        /// Why it was refused: `agency.service.StateConflict`.
        error: StateConflict,
    },
}

/// Open a session — the input of `agency.service.OpenSession`.
///
/// Everything it can result in is [`OpenSessionOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenSession {
    /// `principal_id` — `agency.auth.PrincipalId`.
    pub principal_id: crate::auth::PrincipalId,
    /// `kind` — `agency.service.SessionKind`.
    pub kind: SessionKind,
    /// `run_id` — `Optional<agency.runs.RunId>`.
    pub run_id: Option<crate::runs::RunId>,
    /// `token_digest` — `agency.shared.Digest`.
    pub token_digest: crate::shared::Digest,
    /// `csrf_digest` — `Optional<agency.shared.Digest>`.
    pub csrf_digest: Option<crate::shared::Digest>,
    /// `expires_at` — `Optional<Timestamp>`.
    pub expires_at: Option<crate::primitives::Timestamp>,
}

/// Everything `agency.service.OpenSession` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenSessionOutcome {
    /// `opened` — otherwise.
    Opened {
        /// The `agency.service.SessionOpened` this outcome publishes.
        session_opened: SessionOpened,
    },
}

/// SessionClosed — the event `agency.service.SessionClosed`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionClosed {
    /// `session_id` — `agency.service.SessionId`.
    pub session_id: SessionId,
}

/// SessionOpened — the event `agency.service.SessionOpened`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionOpened {
    /// `session_id` — `agency.service.SessionId`.
    pub session_id: SessionId,
    /// `principal_id` — `agency.auth.PrincipalId`.
    pub principal_id: crate::auth::PrincipalId,
    /// `kind` — `agency.service.SessionKind`.
    pub kind: SessionKind,
}

/// The declared error `agency.service.NotFound`.
///
/// The addressed record does not exist, and nothing changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotFound;

/// The declared error `agency.service.StateConflict`.
///
/// The addressed record is not in a state this command acts from, and nothing changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateConflict;

/// SessionList — one row of the view `agency.service.SessionList`.
///
/// Projects `agency.service.Session` at `read_your_writes` consistency.
/// The specification fully determines every row, so its query is generated over the storage port —
/// see the plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionList {
    /// `session_id` — `agency.service.SessionId`.
    pub session_id: SessionId,
    /// `principal_id` — `agency.auth.PrincipalId`.
    pub principal_id: crate::auth::PrincipalId,
    /// `kind` — `agency.service.SessionKind`.
    pub kind: SessionKind,
    /// `run_id` — `Optional<agency.runs.RunId>`.
    pub run_id: Option<crate::runs::RunId>,
    /// `opened_at` — `Timestamp`.
    pub opened_at: crate::primitives::Timestamp,
    /// `expires_at` — `Optional<Timestamp>`.
    pub expires_at: Option<crate::primitives::Timestamp>,
    /// `state` — `agency.service.Session.State`.
    pub state: SessionState,
}

/// What this bounded context owes its implementor, and the seams of what is generated.
///
/// One trait per obligation in the synthesis plan, each carrying the plan's own contract, and one
/// per generated behaviour, which [`Generated`](crate::behaviour::Generated) implements.
pub mod obligations {
    /// The behaviour `agency.service.CloseSession` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait CloseSessionBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.service.CloseSession`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn close_session(&mut self, input: super::CloseSession) -> Result<super::CloseSessionOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agency.service.OpenSession` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait OpenSessionBehavior {
        /// Decides and enacts exactly one declared outcome of `agency.service.OpenSession`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn open_session(&mut self, input: super::OpenSession) -> Result<super::OpenSessionOutcome, crate::obligation::UnmetObligation>;
    }

    /// The query `agency.service.SessionList` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage port. Implement it yourself to replace that query.
    pub trait SessionListQuery {
        /// Serves `agency.service.SessionList` rows at the view's declared consistency.
        ///
        /// `Err` is the typed refusal of a row whose declared type cannot hold its value.
        fn session_list(&self) -> Result<Vec<super::SessionList>, crate::obligation::UnmetObligation>;
    }

}
