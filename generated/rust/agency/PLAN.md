<!--
  generated from agency v1
  model digest 71132e90bde806415603d08bb97a71629b2c2f6cda30e1f22e76f407abd9a3eb
  contract digest a96f55e3e1ca68aeb1579b032c3547fffd27e8ab80e76bcb898f9b6c9258792e
  do not edit: regenerate with `ess synthesize --layout crate`
-->
# Synthesis plan — agency v1

Scope: `component-skeletons`, laid out as `crate`, planned by `ess-synth`. Regenerate with `ess synthesize --layout crate`.

328 capabilities: **323 generated**, **5 obligations**, **0 refused**. An obligation is yours to implement against its contract; a refusal is a fact about this synthesis scope, not about the specification.

## Generated

| capability | source |
| --- | --- |
| domain type | `agency.agents.Agent.State` |
| domain type | `agency.agents.AgentDefinition.State` |
| domain type | `agency.agents.AgentId` |
| domain type | `agency.agents.AgentSpecDocument` |
| domain type | `agency.agents.CheckVerdict` |
| domain type | `agency.agents.DefinitionId` |
| domain type | `agency.auth.ConnectorsCustody` |
| domain type | `agency.auth.Credential.State` |
| domain type | `agency.auth.CredentialGrant.State` |
| domain type | `agency.auth.CredentialId` |
| domain type | `agency.auth.CredentialKind` |
| domain type | `agency.auth.CredentialScope` |
| domain type | `agency.auth.Custody` |
| domain type | `agency.auth.DecisionMode` |
| domain type | `agency.auth.DefinitionSubject` |
| domain type | `agency.auth.GrantId` |
| domain type | `agency.auth.GrantPurpose` |
| domain type | `agency.auth.HostFileCustody` |
| domain type | `agency.auth.Policy.State` |
| domain type | `agency.auth.PolicyId` |
| domain type | `agency.auth.Principal.State` |
| domain type | `agency.auth.PrincipalId` |
| domain type | `agency.auth.PrincipalKind` |
| domain type | `agency.auth.RiskCeiling` |
| domain type | `agency.auth.Role` |
| domain type | `agency.auth.Rule` |
| domain type | `agency.auth.RuleSubject` |
| domain type | `agency.auth.TeamSubject` |
| domain type | `agency.auth.Tenant.State` |
| domain type | `agency.auth.TenantId` |
| domain type | `agency.auth.TenantSubject` |
| domain type | `agency.auth.ToolCallsPurpose` |
| domain type | `agency.auth.WorkforceSubject` |
| domain type | `agency.runs.ApprovalId` |
| domain type | `agency.runs.CredentialSource` |
| domain type | `agency.runs.LoomSessionId` |
| domain type | `agency.runs.Run.State` |
| domain type | `agency.runs.RunApproval.State` |
| domain type | `agency.runs.RunEndReason` |
| domain type | `agency.runs.RunId` |
| domain type | `agency.runs.RunOutcome` |
| domain type | `agency.runs.RunUsage` |
| domain type | `agency.runs.RunWorkspace` |
| domain type | `agency.service.Session.State` |
| domain type | `agency.service.SessionId` |
| domain type | `agency.service.SessionKind` |
| domain type | `agency.shared.Digest` |
| domain type | `agency.shared.RunBudget` |
| domain type | `agency.shared.Verb` |
| domain type | `agency.teams.Capacity` |
| domain type | `agency.teams.Swarm.State` |
| domain type | `agency.teams.SwarmId` |
| domain type | `agency.teams.Team.State` |
| domain type | `agency.teams.TeamId` |
| domain type | `agency.teams.Workforce.State` |
| domain type | `agency.teams.WorkforceId` |
| domain type | `agency.tools.BindingId` |
| domain type | `agency.tools.BindingScope` |
| domain type | `agency.tools.ContractRef` |
| domain type | `agency.tools.RiskEnvelope` |
| domain type | `agency.tools.Tool.State` |
| domain type | `agency.tools.ToolBinding.State` |
| domain type | `agency.tools.ToolId` |
| domain type | `agency.tools.ToolRisk` |
| domain type | `agency.work.AepStoreSource` |
| domain type | `agency.work.GitHubIssuesSource` |
| domain type | `agency.work.InputId` |
| domain type | `agency.work.WebhookSource` |
| domain type | `agency.work.WorkInput.State` |
| domain type | `agency.work.WorkItem.State` |
| domain type | `agency.work.WorkItemRef` |
| domain type | `agency.work.WorkSource` |
| entity lifecycle | `agency.agents.Agent` |
| entity lifecycle | `agency.agents.AgentDefinition` |
| entity lifecycle | `agency.auth.Credential` |
| entity lifecycle | `agency.auth.CredentialGrant` |
| entity lifecycle | `agency.auth.Policy` |
| entity lifecycle | `agency.auth.Principal` |
| entity lifecycle | `agency.auth.Tenant` |
| entity lifecycle | `agency.runs.Run` |
| entity lifecycle | `agency.runs.RunApproval` |
| entity lifecycle | `agency.service.Session` |
| entity lifecycle | `agency.teams.Swarm` |
| entity lifecycle | `agency.teams.Team` |
| entity lifecycle | `agency.teams.Workforce` |
| entity lifecycle | `agency.tools.Tool` |
| entity lifecycle | `agency.tools.ToolBinding` |
| entity lifecycle | `agency.work.WorkInput` |
| entity lifecycle | `agency.work.WorkItem` |
| command contract | `agency.agents.CreateAgentDefinition` |
| command behaviour | `agency.agents.CreateAgentDefinition` |
| command contract | `agency.agents.DeprecateAgentDefinition` |
| command behaviour | `agency.agents.DeprecateAgentDefinition` |
| command contract | `agency.agents.ProvisionAgent` |
| command behaviour | `agency.agents.ProvisionAgent` |
| command contract | `agency.agents.PublishAgentDefinition` |
| command behaviour | `agency.agents.PublishAgentDefinition` |
| command contract | `agency.agents.RecordSpecCheck` |
| command behaviour | `agency.agents.RecordSpecCheck` |
| command contract | `agency.agents.RetireAgent` |
| command behaviour | `agency.agents.RetireAgent` |
| command contract | `agency.auth.ActivatePolicy` |
| command behaviour | `agency.auth.ActivatePolicy` |
| command contract | `agency.auth.AssignRole` |
| command behaviour | `agency.auth.AssignRole` |
| command contract | `agency.auth.CreatePolicy` |
| command behaviour | `agency.auth.CreatePolicy` |
| command contract | `agency.auth.CreateTenant` |
| command behaviour | `agency.auth.CreateTenant` |
| command contract | `agency.auth.DisablePrincipal` |
| command behaviour | `agency.auth.DisablePrincipal` |
| command contract | `agency.auth.GrantCredential` |
| command behaviour | `agency.auth.GrantCredential` |
| command contract | `agency.auth.InvitePrincipal` |
| command behaviour | `agency.auth.InvitePrincipal` |
| command contract | `agency.auth.RefreshCredential` |
| command behaviour | `agency.auth.RefreshCredential` |
| command contract | `agency.auth.RevokeCredential` |
| command behaviour | `agency.auth.RevokeCredential` |
| command contract | `agency.auth.RevokeGrant` |
| command behaviour | `agency.auth.RevokeGrant` |
| command contract | `agency.auth.StoreCredential` |
| command behaviour | `agency.auth.StoreCredential` |
| command contract | `agency.auth.SupersedePolicy` |
| command behaviour | `agency.auth.SupersedePolicy` |
| command contract | `agency.runs.AdmitRun` |
| command behaviour | `agency.runs.AdmitRun` |
| command contract | `agency.runs.DenyRunApproval` |
| command behaviour | `agency.runs.DenyRunApproval` |
| command contract | `agency.runs.GrantRunApproval` |
| command behaviour | `agency.runs.GrantRunApproval` |
| command contract | `agency.runs.RecordRunFailure` |
| command behaviour | `agency.runs.RecordRunFailure` |
| command contract | `agency.runs.RecordRunProposal` |
| command behaviour | `agency.runs.RecordRunProposal` |
| command contract | `agency.runs.RecordRunStarted` |
| command behaviour | `agency.runs.RecordRunStarted` |
| command contract | `agency.runs.RequestRunApproval` |
| command behaviour | `agency.runs.RequestRunApproval` |
| command contract | `agency.runs.SetRunBudget` |
| command behaviour | `agency.runs.SetRunBudget` |
| command contract | `agency.runs.StartRun` |
| command behaviour | `agency.runs.StartRun` |
| command contract | `agency.runs.StopRun` |
| command behaviour | `agency.runs.StopRun` |
| command contract | `agency.service.CloseSession` |
| command behaviour | `agency.service.CloseSession` |
| command contract | `agency.service.OpenSession` |
| command behaviour | `agency.service.OpenSession` |
| command contract | `agency.teams.AddMember` |
| command behaviour | `agency.teams.AddMember` |
| command contract | `agency.teams.AttachQueue` |
| command behaviour | `agency.teams.AttachQueue` |
| command contract | `agency.teams.DeclareSwarm` |
| command behaviour | `agency.teams.DeclareSwarm` |
| command contract | `agency.teams.DefineWorkforce` |
| command behaviour | `agency.teams.DefineWorkforce` |
| command contract | `agency.teams.DisbandTeam` |
| command behaviour | `agency.teams.DisbandTeam` |
| command contract | `agency.teams.FormTeam` |
| command behaviour | `agency.teams.FormTeam` |
| command contract | `agency.teams.RemoveMember` |
| command behaviour | `agency.teams.RemoveMember` |
| command contract | `agency.teams.Report` |
| command behaviour | `agency.teams.Report` |
| command contract | `agency.teams.RetireWorkforce` |
| command behaviour | `agency.teams.RetireWorkforce` |
| command contract | `agency.teams.Send` |
| command behaviour | `agency.teams.Send` |
| command contract | `agency.teams.SetLead` |
| command behaviour | `agency.teams.SetLead` |
| command contract | `agency.teams.StaffWorkforce` |
| command behaviour | `agency.teams.StaffWorkforce` |
| command contract | `agency.teams.StopSwarm` |
| command behaviour | `agency.teams.StopSwarm` |
| command contract | `agency.tools.BindTool` |
| command behaviour | `agency.tools.BindTool` |
| command contract | `agency.tools.RegisterTool` |
| command behaviour | `agency.tools.RegisterTool` |
| command contract | `agency.tools.RevokeBinding` |
| command behaviour | `agency.tools.RevokeBinding` |
| command contract | `agency.tools.WithdrawTool` |
| command behaviour | `agency.tools.WithdrawTool` |
| command contract | `agency.work.AbandonWorkItem` |
| command behaviour | `agency.work.AbandonWorkItem` |
| command contract | `agency.work.AssignWorkItem` |
| command behaviour | `agency.work.AssignWorkItem` |
| command contract | `agency.work.ConfigureWorkInput` |
| command behaviour | `agency.work.ConfigureWorkInput` |
| command contract | `agency.work.PauseWorkInput` |
| command behaviour | `agency.work.PauseWorkInput` |
| command contract | `agency.work.RemoveWorkInput` |
| command behaviour | `agency.work.RemoveWorkInput` |
| command contract | `agency.work.ReturnWorkItem` |
| command behaviour | `agency.work.ReturnWorkItem` |
| command contract | `agency.work.SettleWorkItem` |
| command behaviour | `agency.work.SettleWorkItem` |
| command contract | `agency.work.SubmitWorkItem` |
| command behaviour | `agency.work.SubmitWorkItem` |
| event type | `agency.agents.AgentDefinitionCreated` |
| event type | `agency.agents.AgentDefinitionDeprecated` |
| event type | `agency.agents.AgentDefinitionPublished` |
| event type | `agency.agents.AgentProvisioned` |
| event type | `agency.agents.AgentRetired` |
| event type | `agency.agents.AgentSpecChecked` |
| event type | `agency.agents.AgentSpecRefused` |
| event type | `agency.auth.CredentialGranted` |
| event type | `agency.auth.CredentialRefreshed` |
| event type | `agency.auth.CredentialRevoked` |
| event type | `agency.auth.CredentialStored` |
| event type | `agency.auth.GrantRevoked` |
| event type | `agency.auth.PolicyActivated` |
| event type | `agency.auth.PolicyCreated` |
| event type | `agency.auth.PolicySuperseded` |
| event type | `agency.auth.PrincipalDisabled` |
| event type | `agency.auth.PrincipalInvited` |
| event type | `agency.auth.RoleAssigned` |
| event type | `agency.auth.TenantCreated` |
| event type | `agency.runs.RunAdmitted` |
| event type | `agency.runs.RunApprovalDenied` |
| event type | `agency.runs.RunApprovalGranted` |
| event type | `agency.runs.RunApprovalRequested` |
| event type | `agency.runs.RunBudgetSet` |
| event type | `agency.runs.RunFailed` |
| event type | `agency.runs.RunProposed` |
| event type | `agency.runs.RunRequested` |
| event type | `agency.runs.RunStarted` |
| event type | `agency.runs.RunStopped` |
| event type | `agency.service.SessionClosed` |
| event type | `agency.service.SessionOpened` |
| event type | `agency.teams.LeadSet` |
| event type | `agency.teams.MemberAdded` |
| event type | `agency.teams.MemberRemoved` |
| event type | `agency.teams.MessageSent` |
| event type | `agency.teams.QueueAttached` |
| event type | `agency.teams.ReportSent` |
| event type | `agency.teams.SwarmDeclared` |
| event type | `agency.teams.SwarmStopped` |
| event type | `agency.teams.TeamDisbanded` |
| event type | `agency.teams.TeamFormed` |
| event type | `agency.teams.WorkforceDefined` |
| event type | `agency.teams.WorkforceRetired` |
| event type | `agency.teams.WorkforceStaffed` |
| event type | `agency.tools.BindingRevoked` |
| event type | `agency.tools.ToolBound` |
| event type | `agency.tools.ToolRegistered` |
| event type | `agency.tools.ToolWithdrawn` |
| event type | `agency.work.WorkInputConfigured` |
| event type | `agency.work.WorkInputPaused` |
| event type | `agency.work.WorkInputRemoved` |
| event type | `agency.work.WorkItemAbandoned` |
| event type | `agency.work.WorkItemAssigned` |
| event type | `agency.work.WorkItemReturned` |
| event type | `agency.work.WorkItemSettled` |
| event type | `agency.work.WorkItemSubmitted` |
| error type | `agency.agents.NotFound` |
| error type | `agency.agents.StateConflict` |
| error type | `agency.auth.CredentialUnusable` |
| error type | `agency.auth.GranteeInvalid` |
| error type | `agency.auth.NotFound` |
| error type | `agency.auth.StateConflict` |
| error type | `agency.runs.NotFound` |
| error type | `agency.runs.RunEnded` |
| error type | `agency.runs.StateConflict` |
| error type | `agency.service.NotFound` |
| error type | `agency.service.StateConflict` |
| error type | `agency.teams.ForeignInstance` |
| error type | `agency.teams.NotFound` |
| error type | `agency.teams.NotOneLevelUp` |
| error type | `agency.teams.OutsideTeam` |
| error type | `agency.teams.StateConflict` |
| error type | `agency.tools.BindingTargetInvalid` |
| error type | `agency.tools.NotFound` |
| error type | `agency.tools.StateConflict` |
| error type | `agency.work.NotFound` |
| error type | `agency.work.StateConflict` |
| view type | `agency.agents.AgentDefinitionList` |
| view query | `agency.agents.AgentDefinitionList` |
| view type | `agency.agents.AgentList` |
| view query | `agency.agents.AgentList` |
| view type | `agency.auth.CredentialList` |
| view query | `agency.auth.CredentialList` |
| view type | `agency.auth.EffectivePolicy` |
| view type | `agency.auth.GrantMatrix` |
| view query | `agency.auth.GrantMatrix` |
| view type | `agency.auth.PolicyList` |
| view query | `agency.auth.PolicyList` |
| view type | `agency.auth.PrincipalList` |
| view query | `agency.auth.PrincipalList` |
| view type | `agency.runs.ApprovalInbox` |
| view query | `agency.runs.ApprovalInbox` |
| view type | `agency.runs.RunDetail` |
| view type | `agency.runs.RunList` |
| view query | `agency.runs.RunList` |
| view type | `agency.runs.UsageByAgent` |
| view query | `agency.runs.UsageByAgent` |
| view type | `agency.service.SessionList` |
| view query | `agency.service.SessionList` |
| view type | `agency.teams.SwarmList` |
| view query | `agency.teams.SwarmList` |
| view type | `agency.teams.TeamDetail` |
| view type | `agency.teams.TeamList` |
| view query | `agency.teams.TeamList` |
| view type | `agency.teams.WorkforceBoard` |
| view query | `agency.teams.WorkforceBoard` |
| view type | `agency.tools.BindingList` |
| view query | `agency.tools.BindingList` |
| view type | `agency.tools.ToolCatalogue` |
| view query | `agency.tools.ToolCatalogue` |
| view type | `agency.work.WorkInputList` |
| view query | `agency.work.WorkInputList` |
| view type | `agency.work.WorkItemDetail` |
| view type | `agency.work.WorkQueue` |
| actor grants | `agency.agents.SpecChecker` |
| actor grants | `agency.auth.Application` |
| actor grants | `agency.auth.Operator` |
| actor grants | `agency.auth.RunningAgent` |
| actor grants | `agency.auth.Supervisor` |
| actor grants | `agency.auth.Viewer` |
| actor grants | `agency.runs.LoomReadback` |
| actor grants | `agency.service.Gatekeeper` |
| component port | `agency` |
| component transport | `agency` |

## Ports — yours to provide

What the specification fully determines is generated; what it cannot determine is an obligation. A generated command behaviour or view query reads and writes through the ports below, and they are yours to provide: synthesis generates each port's contract and never an implementation of one, so where instances live stays your decision.

| port | what it answers |
| --- | --- |
| storage | one per entity a generated behaviour or query reads or writes: the instance stored under an identity; storing, replacing and removing one; and every stored instance, in the order the store keeps them |
| context | where a generated behaviour asks it: the caller's attributes, every identity and value the specification says the implementation assigns, and whether each `external:` branch is taken |

## Obligations — yours to implement

| capability | source | why not generated | contract |
| --- | --- | --- | --- |
| view query | `agency.auth.EffectivePolicy` | kept an obligation by a read through the reference `policy_id` to `agency.auth.Policy` (`field: {related: …}` or `policy_id.<field>` in `filter:`), which reads a second entity's rows | a query answering `agency.auth.EffectivePolicy` with rows projected from `agency.agents.AgentDefinition` at `read_your_writes` consistency, containing instances where `definition_id == param.definition_id` |
| view query | `agency.runs.RunDetail` | kept an obligation by a view parameter (`params:`), which a generated query does not apply | a query answering `agency.runs.RunDetail` with rows projected from `agency.runs.Run` at `read_your_writes` consistency, containing instances where `run_id == param.run_id` |
| view query | `agency.teams.TeamDetail` | kept an obligation by a view parameter (`params:`), which a generated query does not apply | a query answering `agency.teams.TeamDetail` with rows projected from `agency.teams.Team` at `read_your_writes` consistency, containing instances where `team_id == param.team_id` |
| view query | `agency.work.WorkItemDetail` | kept an obligation by a view parameter (`params:`), which a generated query does not apply | a query answering `agency.work.WorkItemDetail` with rows projected from `agency.work.WorkItem` at `read_your_writes` consistency, containing instances where `item_ref == param.item_ref` |
| view query | `agency.work.WorkQueue` | kept an obligation by a read through the reference `input_id` to `agency.work.WorkInput` (`field: {related: …}` or `input_id.<field>` in `filter:`), which reads a second entity's rows | a query answering `agency.work.WorkQueue` with rows projected from `agency.work.WorkItem` at `read_your_writes` consistency |

## Refused — not represented by this synthesis

| capability | source | stage | why |
| --- | --- | --- | --- |
