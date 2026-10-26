// generated from agency v1
// model digest 71132e90bde806415603d08bb97a71629b2c2f6cda30e1f22e76f407abd9a3eb
// contract digest a96f55e3e1ca68aeb1579b032c3547fffd27e8ab80e76bcb898f9b6c9258792e
// do not edit: regenerate with `ess synthesize --layout crate`

//! Every actor the specification declares, and the commands each may invoke — as data.
//!
//! A grant is checked against a caller identity, which these types do not read from anywhere:
//! whatever authenticates a request builds a [`Caller`], and a served surface checks it against
//! [`may`] before the command runs. The `PLAN.md` beside this workspace says, per actor,
//! whether a generated surface enforces the grant or the caller does.

/// An actor the specification declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Actor {
    /// `agency.agents.SpecChecker`.
    SpecChecker,
    /// `agency.auth.Application`.
    Application,
    /// `agency.auth.Operator`.
    Operator,
    /// `agency.auth.RunningAgent`.
    RunningAgent,
    /// `agency.auth.Supervisor`.
    Supervisor,
    /// `agency.auth.Viewer`.
    Viewer,
    /// `agency.runs.LoomReadback`.
    LoomReadback,
    /// `agency.service.Gatekeeper`.
    Gatekeeper,
}

impl Actor {
    /// Every declared actor, ordered by qualified name.
    pub const ALL: &'static [Actor] = &[
        Actor::SpecChecker,
        Actor::Application,
        Actor::Operator,
        Actor::RunningAgent,
        Actor::Supervisor,
        Actor::Viewer,
        Actor::LoomReadback,
        Actor::Gatekeeper,
    ];

    /// The actor's qualified name, as the specification spells it.
    pub const fn name(self) -> &'static str {
        match self {
            Actor::SpecChecker => "agency.agents.SpecChecker",
            Actor::Application => "agency.auth.Application",
            Actor::Operator => "agency.auth.Operator",
            Actor::RunningAgent => "agency.auth.RunningAgent",
            Actor::Supervisor => "agency.auth.Supervisor",
            Actor::Viewer => "agency.auth.Viewer",
            Actor::LoomReadback => "agency.runs.LoomReadback",
            Actor::Gatekeeper => "agency.service.Gatekeeper",
        }
    }
}

/// The qualified names of the commands `actor` may invoke, ordered by name; empty for an
/// actor that only observes.
pub fn may(actor: Actor) -> &'static [&'static str] {
    match actor {
        Actor::SpecChecker => &[
            "agency.agents.RecordSpecCheck",
        ],
        Actor::Application => &[
            "agency.runs.DenyRunApproval",
            "agency.runs.GrantRunApproval",
            "agency.runs.StartRun",
            "agency.runs.StopRun",
            "agency.teams.AddMember",
            "agency.teams.AttachQueue",
            "agency.teams.DeclareSwarm",
            "agency.teams.DefineWorkforce",
            "agency.teams.DisbandTeam",
            "agency.teams.FormTeam",
            "agency.teams.RemoveMember",
            "agency.teams.Report",
            "agency.teams.RetireWorkforce",
            "agency.teams.Send",
            "agency.teams.SetLead",
            "agency.teams.StaffWorkforce",
            "agency.teams.StopSwarm",
            "agency.work.AbandonWorkItem",
            "agency.work.AssignWorkItem",
            "agency.work.ReturnWorkItem",
            "agency.work.SettleWorkItem",
            "agency.work.SubmitWorkItem",
        ],
        Actor::Operator => &[
            "agency.agents.CreateAgentDefinition",
            "agency.agents.ProvisionAgent",
            "agency.agents.PublishAgentDefinition",
            "agency.agents.RetireAgent",
            "agency.auth.GrantCredential",
            "agency.auth.RevokeGrant",
            "agency.runs.DenyRunApproval",
            "agency.runs.GrantRunApproval",
            "agency.runs.SetRunBudget",
            "agency.runs.StartRun",
            "agency.runs.StopRun",
            "agency.teams.AddMember",
            "agency.teams.AttachQueue",
            "agency.teams.DeclareSwarm",
            "agency.teams.DefineWorkforce",
            "agency.teams.DisbandTeam",
            "agency.teams.FormTeam",
            "agency.teams.RemoveMember",
            "agency.teams.Report",
            "agency.teams.RetireWorkforce",
            "agency.teams.Send",
            "agency.teams.SetLead",
            "agency.teams.StaffWorkforce",
            "agency.teams.StopSwarm",
            "agency.tools.BindTool",
            "agency.tools.RevokeBinding",
            "agency.work.AbandonWorkItem",
            "agency.work.AssignWorkItem",
            "agency.work.ConfigureWorkInput",
            "agency.work.PauseWorkInput",
            "agency.work.RemoveWorkInput",
            "agency.work.ReturnWorkItem",
            "agency.work.SettleWorkItem",
            "agency.work.SubmitWorkItem",
        ],
        Actor::RunningAgent => &[
            "agency.runs.RequestRunApproval",
            "agency.runs.StopRun",
            "agency.teams.Report",
            "agency.teams.Send",
            "agency.work.ReturnWorkItem",
            "agency.work.SettleWorkItem",
        ],
        Actor::Supervisor => &[
            "agency.agents.CreateAgentDefinition",
            "agency.agents.DeprecateAgentDefinition",
            "agency.agents.ProvisionAgent",
            "agency.agents.PublishAgentDefinition",
            "agency.agents.RetireAgent",
            "agency.auth.ActivatePolicy",
            "agency.auth.AssignRole",
            "agency.auth.CreatePolicy",
            "agency.auth.CreateTenant",
            "agency.auth.DisablePrincipal",
            "agency.auth.GrantCredential",
            "agency.auth.InvitePrincipal",
            "agency.auth.RefreshCredential",
            "agency.auth.RevokeCredential",
            "agency.auth.RevokeGrant",
            "agency.auth.StoreCredential",
            "agency.auth.SupersedePolicy",
            "agency.runs.DenyRunApproval",
            "agency.runs.GrantRunApproval",
            "agency.runs.SetRunBudget",
            "agency.runs.StartRun",
            "agency.runs.StopRun",
            "agency.service.CloseSession",
            "agency.service.OpenSession",
            "agency.tools.RegisterTool",
            "agency.tools.WithdrawTool",
        ],
        Actor::Viewer => &[],
        Actor::LoomReadback => &[
            "agency.runs.AdmitRun",
            "agency.runs.RecordRunFailure",
            "agency.runs.RecordRunProposal",
            "agency.runs.RecordRunStarted",
            "agency.runs.RequestRunApproval",
        ],
        Actor::Gatekeeper => &[
            "agency.service.CloseSession",
            "agency.service.OpenSession",
        ],
    }
}

/// Who a request was authenticated as.
///
/// Built by whatever authenticates the request — a session, a token, a certificate — and
/// handed to the served surface's `dispatch` and `handle`, which check its grant
/// before the command runs. Never derived from the request itself: a client can write
/// anything into a request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Caller {
    /// The declared actor.
    pub actor: Actor,
}

impl Caller {
    /// `true` when this caller may invoke `command`, named by its qualified name.
    pub fn may(&self, command: &str) -> bool {
        may(self.actor).contains(&command)
    }
}

/// The qualified names of the views `actor` may read, ordered by name: the views its grant
/// names. A view no actor's grant names is open to every caller and listed for none.
pub fn may_read(actor: Actor) -> &'static [&'static str] {
    match actor {
        Actor::SpecChecker => &[],
        Actor::Application => &[
            "agency.agents.AgentDefinitionList",
            "agency.agents.AgentList",
            "agency.auth.CredentialList",
            "agency.auth.EffectivePolicy",
            "agency.auth.GrantMatrix",
            "agency.auth.PolicyList",
            "agency.auth.PrincipalList",
            "agency.runs.ApprovalInbox",
            "agency.runs.RunDetail",
            "agency.runs.RunList",
            "agency.runs.UsageByAgent",
            "agency.teams.SwarmList",
            "agency.teams.TeamDetail",
            "agency.teams.TeamList",
            "agency.teams.WorkforceBoard",
        ],
        Actor::Operator => &[
            "agency.agents.AgentDefinitionList",
            "agency.agents.AgentList",
            "agency.auth.CredentialList",
            "agency.auth.EffectivePolicy",
            "agency.auth.GrantMatrix",
            "agency.auth.PolicyList",
            "agency.auth.PrincipalList",
            "agency.runs.ApprovalInbox",
            "agency.runs.RunDetail",
            "agency.runs.RunList",
            "agency.runs.UsageByAgent",
            "agency.teams.SwarmList",
            "agency.teams.TeamDetail",
            "agency.teams.TeamList",
            "agency.teams.WorkforceBoard",
            "agency.tools.BindingList",
            "agency.tools.ToolCatalogue",
            "agency.work.WorkInputList",
            "agency.work.WorkItemDetail",
            "agency.work.WorkQueue",
        ],
        Actor::RunningAgent => &[
            "agency.tools.ToolCatalogue",
        ],
        Actor::Supervisor => &[
            "agency.agents.AgentDefinitionList",
            "agency.agents.AgentList",
            "agency.auth.CredentialList",
            "agency.auth.EffectivePolicy",
            "agency.auth.GrantMatrix",
            "agency.auth.PolicyList",
            "agency.auth.PrincipalList",
            "agency.runs.ApprovalInbox",
            "agency.runs.RunDetail",
            "agency.runs.RunList",
            "agency.runs.UsageByAgent",
            "agency.service.SessionList",
            "agency.teams.SwarmList",
            "agency.teams.TeamDetail",
            "agency.teams.TeamList",
            "agency.teams.WorkforceBoard",
            "agency.tools.BindingList",
            "agency.tools.ToolCatalogue",
            "agency.work.WorkInputList",
            "agency.work.WorkItemDetail",
            "agency.work.WorkQueue",
        ],
        Actor::Viewer => &[
            "agency.agents.AgentDefinitionList",
            "agency.agents.AgentList",
            "agency.auth.CredentialList",
            "agency.auth.EffectivePolicy",
            "agency.auth.GrantMatrix",
            "agency.auth.PolicyList",
            "agency.auth.PrincipalList",
            "agency.runs.ApprovalInbox",
            "agency.runs.RunDetail",
            "agency.runs.RunList",
            "agency.runs.UsageByAgent",
            "agency.teams.SwarmList",
            "agency.teams.TeamDetail",
            "agency.teams.TeamList",
            "agency.teams.WorkforceBoard",
            "agency.tools.BindingList",
            "agency.tools.ToolCatalogue",
            "agency.work.WorkInputList",
            "agency.work.WorkItemDetail",
            "agency.work.WorkQueue",
        ],
        Actor::LoomReadback => &[],
        Actor::Gatekeeper => &[],
    }
}

impl Caller {
    /// `true` when this caller may read `view`, a view some actor's grant names, named by
    /// its qualified name.
    pub fn may_read(&self, view: &str) -> bool {
        may_read(self.actor).contains(&view)
    }
}
