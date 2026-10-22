# Agency: product and system design

Working name: **Agency**. Draft 1, 2026-10-22, written by a Fable design agent for Timo. Read-only on
every repository; every claim carries a `path:line` source or says "I don't know". Lines marked
*(inferred)* are the author's reading, not a recorded decision.

Operator words this design serves:

- "the agency -> manage agents, work, teams, auth, tools | conductor -> application" (Timo,
  2026-10-22, `~/.cache/b10x-layers/decisions.md` row 98).
- "it shall become a sort of a deployment ... one service inside which offers an API ... in the back
  we have the agent runtime, Loom ... create agents, give them tools, describe them, form teams, we
  manage swarms, we manage work inputs ... work forces and work items. We might not store them, the
  work items, but we have to handle them." and "the cockpit of an agentic engineer" (Timo,
  2026-10-22, quoted in ADR 0105 draft lines 5-12,
  `~/.local/state/worktree/trees/b10x/atlas/wt-521012e8dceb/architecture/adr/0105-conductor-and-control-plane-converge-into-one-product.md`).

---

## 1. What Agency is

**One line.** Agency is the deployable service and cockpit from which an agentic engineer defines
agents, gives them tools and credentials, forms them into teams, feeds them work, and watches the
runs that Loom executes for them.

| Question | Answer | Source |
|---|---|---|
| Layer | Product layer, above Applications (ADR 0106 adds the Product layer: Agency, loom-coder, swarm, agentide) | `wt-bf9ed9bd217e/architecture/adr/0106-the-product-layer.md:27-32`; `~/beyond10x/agent-stack/stack.yaml:41-49` (`agency`, status proposed, "control-plane converges into") |
| Above it | **conductor** as an application that drives Agency's API (edge `conductor builds-on agency`, "sits on top of Agency") | `agent-stack/stack.yaml:77-85,263-277`; `decisions.md` row 98 |
| Above it, maybe | **takt**: an application on loom-sdk (ADR 0099). Whether takt talks to Agency: **I don't know**, nothing says so. *(inferred: takt is a peer application and can run with or without Agency; Agency can register a takt run as a work item, nothing more)* | `atlas/architecture/adr/0099-*.md:32-39`; `takt/README.md:141-145` |
| Below it | **Loom** (L3, agent runtime; `b10x-loom-sdk` is its library) for every model turn and every governed effect; **metaharness** (L2) only through Loom's session executor, never directly | ADR 0096 layer table `atlas/architecture/adr/0096-*.md:46-53`; ADR 0098 Decision 1-5 `0098-*.md:49-67` ("Loom calls llm for decisions, metaharness for agent work"); `loom/README.md:3-5,47`; `loom/crates/loom-session-executor/src/lib.rs:228-258` |
| What converges into it | **control-plane** (paused since 2026-10-21, "we don't need control plane") | `agent-stack/stack.yaml:86-94` (product, paused; edge `agency contains control-plane`); ADR 0092 §2 `0092-*.md:35-46` |
| What does not | conductor's CLI, store and guard stay their own repository and become a client | `decisions.md` row 98; `stack.yaml:77-85` |

### Agency is

- The place where the five nouns of the operator's sentence live: agents, work, teams, auth, tools.
- A single service with one HTTP API, one event stream and one browser cockpit (Timo: "one
  service inside which offers an API", ADR 0105 lines 5-12).
- The owner of **policy input** for runs: who may run what, with which tools and credentials, under
  which budget. Enforcement is L2/L1 (ADR 0104 draft, Decision 2-3, `wt-521012e8dceb/.../0104-*.md:29-43`).
- The owner of the scheduling and persistence that control-plane owns today ("control-plane owns
  scheduling, persistence, authority inputs and UI projections", `control-plane/README.md:43`).

### Agency is not

| Not | Who is | Source |
|---|---|---|
| A model loop or a harness | Loom runs turns; metaharness wraps harnesses; harness/Claude Code/Codex are L1 | ADR 0096 `0096-*.md:46-53`; `loom/README.md:30-43` |
| A credential holder for model calls inside a child | metaharness `CredentialSource::Loopback` keeps the token out of the child | `metaharness/crates/metaharness-protocol/src/spec.rs:110-134`; `metaharness/docs/design/loopback-provider-v0.1.md:19-30` |
| The per-call tool permission decider | metaharness (L2) decides per call, L1 enforces, Agency/Loom set policy | ADR 0104 draft `0104-*.md:29-43` |
| The thing that merges, pushes or deploys | Loom "never merges/pushes/deploys"; Commission admits effects; Connectors execute them | `loom/README.md:30-43`; `control-plane/README.md:43` |
| A tool contract registry | `beyond10x/tools` holds `tool-contract/1` contracts and default implementations | ADR 0095 Decision 1-4 `atlas/architecture/adr/0095-*.md` |
| An identity provider | Mandate (identity and authority for people, services, agents; tenants; delegation) | `mandate/README.md:3-7,20` |
| A coordinator loop over one goal | swarm (its own product since ADR 0106) | `swarm/README.md:21-26`; ADR 0106 lines 27-32 |

---

## 2. Domain nouns, ready for ESS

Domain name `agency`, one ESS specification with sub-domains `agency.agents`, `agency.tools`,
`agency.teams`, `agency.work`, `agency.auth`. The shape follows `loom/ess/domains/session.yaml`
(domain, summary, naming, types, entities with identity/fields/lifecycle, events, commands with
input/outcomes/creates/sets/emits, views with source/consistency/fields). Entity names follow Loom's
`<domain>.<area>.<Entity>` pattern (`loom.run.Selection`, `loom.authority.Answer`,
`loom/ess/domains/*.yaml`).

Field types below use ESS newtypes (`Id`, `Timestamp`, `Digest`) as in `session.yaml`'s `types`
block. Lifecycle states are given as `state -> state` chains. Relations are named.

### 2.1 Agents

| Entity | Identity | Fields | Relations | Lifecycle |
|---|---|---|---|---|
| `agency.agents.AgentDefinition` | `definition_id` | `name`, `description`, `kind: Kind{Claude, Codex, B10x, Pi, OpenCode}` (metaharness `Kind`, `spec.rs:19-31`), `model`, `brief` (system brief text), `tool_surface: {Native, Owned}` (`spec.rs:140-146`), `default_budget: RunBudget{max_tokens, deadline}` (loom CHANGELOG 0.32.0), `version`, `digest`, `created_by: principal_id` | `tool_bindings[]`, `policy_id`, `tenant_id` | `draft -> published -> deprecated`; a published version is immutable; a change makes the next version *(inferred, from metaharness's sealed `Frame` digest pattern, `frame.rs:310-340`)* |
| `agency.agents.Agent` | `agent_id` | `definition_id`, `definition_version`, `display_name`, `principal_id` (an Agent is also a principal, see 2.5), `state`, `current_run_id?`, `last_seen_at` | `definition`, `team_memberships[]`, `credential_grants[]` | `provisioned -> idle -> running -> idle`, `idle -> retired` |
| `agency.agents.Run` | `run_id` | `agent_id`, `work_item_ref`, `kind`, `model`, `budget: RunBudget`, `stop_policy` (loom `StopPolicy`, CHANGELOG 0.33.0), `started_at`, `ended_at?`, `outcome: {Proposal, Stopped{reason}, Failed{reason}}`, `usage: loom.metering.RunUsageReport` | `agent`, `work_item`, `session_id` (Loom `loom.session`) | `requested -> admitted -> running -> {proposed, stopped, failed}`; owned by Loom, Agency keeps a projection *(inferred)* |

A `Run` is Agency's record of one Loom `SessionExecutor` invocation
(`loom-session-executor/src/lib.rs:228-258`: `kind, workspace, files, brief, model, max_turns,
credentials, scratch, timeout, program, endpoint`). Agency does not re-model `loom.session`; it
stores the `session_id` and reads Loom's views.

### 2.2 Tools

| Entity | Identity | Fields | Relations | Lifecycle |
|---|---|---|---|---|
| `agency.tools.Tool` | `tool_id` | `contract_ref` (toolset + version from `beyond10x/tools`, ADR 0095 Decision 1-2), `verbs: List<Verb>`, `risk` (from the contract's `tool-contract/1` envelope: effects, risk, idempotency, ADR 0095 Decision 2), `implementation_ref?` (which crate or MCP server), `connector_instance_id?` (Connectors instance when the tool reaches an external service, `connectors/README.md:3-8`) | `bindings[]` | `registered -> available -> withdrawn` |
| `agency.tools.ToolBinding` | `binding_id` | `definition_id` (or `team_id`), `tool_id`, `verbs: List<Verb>` subset, `scope: {repositories[], paths[], subjects}` (conductor `Grant` fields `tools, repositories, paths, max_output_bytes, expires_at`, `conductor/spec/domains/tool.yaml:277-329`), `max_output_bytes`, `expires_at?`, `credential_id?` | `definition`/`team`, `tool`, `credential` | `granted -> active -> {expired, revoked}` |

A binding is what becomes the metaharness `Frame.operations` and `subjects: SubjectScope` when
Loom seals a run's frame (`metaharness/crates/metaharness-protocol/src/frame.rs:310-340`). The
metaharness `Decision{Allow, Deny{reason}, Replace{input}, Abstain}` per call
(`metaharness/crates/metaharness-protocol/src/command.rs:17-58`) is made against it (ADR 0104
Decision 2).

### 2.3 Teams, swarms, workforces

| Entity | Identity | Fields | Relations | Lifecycle |
|---|---|---|---|---|
| `agency.teams.Team` | `team_id` | `name`, `lead_agent_id`, `members: List<agent_id>`, `broker_ref` (conductor `conductor.tool.Team{instance, lead; relations broker, led_by}`, `tool.yaml:277-329`), `shared_bindings: List<binding_id>` | `lead`, `members[]`, `bindings[]`, `workforce_id?` | `formed -> active -> disbanded` |
| `agency.teams.Swarm` | `swarm_id` | `goal_ref`, `coordinator_agent_id`, `frame_digest` (swarm uses sealed frames, `swarm/README.md:42-80`), `max_workers`, `state` | `team_id`, `work_input_id` | `declared -> running -> {converged, stopped}`; the loop itself is the swarm product's; Agency holds the record and the stop lever *(inferred; ADR 0106 lines 36-41 say how swarm reaches lower layers is "Not yet known")* |
| `agency.teams.Workforce` | `workforce_id` | `name`, `teams: List<team_id>`, `solo_agents: List<agent_id>`, `capacity: {max_concurrent_runs, max_tokens_per_hour?}`, `queues: List<work_input_id>` | `teams[]`, `agents[]`, `work_inputs[]` | `defined -> staffed -> draining -> retired` |

A Workforce answers "who is on duty for which queue" (Timo: "work forces and work items"). It is the
scheduling unit control-plane's `goal.max_workers` approximated (`control-plane/ess/domains/host.yaml:103-141`).

### 2.4 Work

| Entity | Identity | Fields | Relations | Lifecycle |
|---|---|---|---|---|
| `agency.work.WorkInput` | `input_id` | `name`, `source: {Manual, AepStore{repo, filter}, GitHubIssues{repo, label}, Webhook{secret_ref}, A2aTask}` (A2A v1.0.1 exists as ESS in `a2a/README.md:3-11`; server/client "to follow"), `poll_interval?`, `workforce_id`, `transform_ref?` | `workforce`, `tenant` | `configured -> polling -> paused -> removed` |
| `agency.work.WorkItem` | `item_ref` (external id + source) | `input_id`, `title`, `payload_digest`, `assigned_agent_id?`, `run_ids[]`, `state`, `received_at`, `settled_at?` | `input`, `agent?`, `runs[]` | `received -> queued -> assigned -> running -> {done, returned, abandoned}` |

**Handled, not necessarily stored.** Timo: "We might not store them, the work items, but we have to
handle them." Design choice (open question Q3): the `WorkItem` row holds only `item_ref`,
`payload_digest`, state and timestamps; the body is read from the source system on demand and never
copied into Agency's store. That satisfies "handle" (route, assign, run, settle) without making Agency
a second tracker. *(inferred)*

Relation to conductor and control-plane: control-plane's `goal` + `assignment`
(`host.yaml:103-141`; views `AssignmentList`, `GoalList` in
`control-plane/generated/api/openapi/control-plane.yaml`) collapse into `WorkInput` + `WorkItem`.
conductor's `Goal`, `Dispatch`, `Need` command groups (`conductor/crates/conductor/src/cli.rs:460-582`)
map onto them from the client side.

### 2.5 Auth

| Entity | Identity | Fields | Relations | Lifecycle |
|---|---|---|---|---|
| `agency.auth.Tenant` | `tenant_id` | `name`, `mandate_tenant_ref?` | owns everything above | `created -> active -> suspended` |
| `agency.auth.Principal` | `principal_id` | `kind: {Human, Agent, Service}` (Mandate: "people, services, agents", `mandate/README.md:3-7`), `tenant_id`, `subject` (login or agent_id), `roles: List<Role>` with `Role{Operator, Supervisor, Viewer, Agent}` (control-plane distinguishes Operator vs Supervisor commands, `control-plane/README.md:77`), `mandate_ref?` | `tenant`, `agent?` | `invited -> active -> disabled` |
| `agency.auth.Credential` | `credential_id` | `kind: {OperatorLogin, ApiKey, OAuthToken, ConnectorInstance}` (metaharness `CredentialSource{OperatorLogin, ApiKey, Loopback, None}`, `spec.rs:110-134`), `custody: {AgencyVault, ConnectorsInstance{instance_id}, HostFile{path}}`, `scope: {models[], providers[], tools[]}`, `owner_principal_id`, `expires_at?`, `last_refreshed_at` | `owner`, `grants[]` | `stored -> active -> {expired, revoked}`; the secret value is never a field of the entity, only a custody pointer |
| `agency.auth.CredentialGrant` | `grant_id` | `credential_id`, `grantee: {agent_id, team_id, workforce_id}`, `purpose: {ModelCalls, ToolCalls{tool_id}}`, `expires_at?` | `credential`, `grantee` | `granted -> active -> {expired, revoked}` |
| `agency.auth.Policy` | `policy_id` | `name`, `rules: List<Rule>` with `Rule{subject: {definition, team, workforce, tenant}, tool_id?, verbs?, risk_ceiling: {low, medium, high}` (harness `--approve-up-to medium|high` ceiling, `harness/README.md:118-126`), `decision_mode: {Frame, Ask, Observe}` (metaharness `DecisionMode`, `spec.rs:57-61`), `budget_ceiling: RunBudget`, `hermetic: bool`}`, `version`, `digest` | `definitions[]`, `teams[]` | `draft -> active -> superseded` |

---

## 3. The five areas

Roles: **Operator** (a human who defines and runs), **Supervisor** (a human who may change
policy, credentials, tenancy), **Viewer** (read-only human), **Agent** (a running agent acting through
the API, limited to team verbs and work-item state), **Application** (conductor, takt, swarm, a
service principal). The Operator/Supervisor split is control-plane's (`control-plane/README.md:77`);
Agent and Application are new *(inferred)*.

### 3.1 Agents

| | |
|---|---|
| Commands | `CreateAgentDefinition`, `PublishAgentDefinition`, `DeprecateAgentDefinition`, `ProvisionAgent`, `RetireAgent`, `StartRun`, `StopRun`, `SetRunBudget` |
| Views | `AgentDefinitionList`, `AgentList` (state, current run, last seen), `RunList` (filter by agent/team/work item), `RunDetail` (Loom `loom.session` projection + metaharness `trace-ir/1` events, ADR 0096 L2 row `0096-*.md:46-53`), `UsageByAgent` (from `loom.metering.RunUsageReport`) |
| Lifecycle | definition `draft -> published -> deprecated`; agent `provisioned -> idle -> running -> retired` |
| Who | Operator: all but `DeprecateAgentDefinition`; Supervisor: all; Viewer: views; Agent: `StopRun` on its own run only; Application: `StartRun`, `StopRun`, views |

### 3.2 Work

| | |
|---|---|
| Commands | `ConfigureWorkInput`, `PauseWorkInput`, `RemoveWorkInput`, `SubmitWorkItem` (manual or webhook), `AssignWorkItem`, `ReturnWorkItem`, `SettleWorkItem`, `AbandonWorkItem` |
| Views | `WorkInputList`, `WorkQueue` (per workforce: received/queued/assigned/running), `WorkItemDetail` (refs, runs, timeline), `Throughput` (settled per hour, tokens per settled item; conductor dashboard's `usage`/`waste` modules do this today, `conductor/crates/conductor-dashboard/src/*`) |
| Lifecycle | item `received -> queued -> assigned -> running -> {done, returned, abandoned}` |
| Who | Operator: all; Agent: `ReturnWorkItem`, `SettleWorkItem` on its own assignment; Application: all item commands (conductor dispatches through here) |

### 3.3 Teams

| | |
|---|---|
| Commands | `FormTeam`, `AddMember`, `RemoveMember`, `SetLead`, `DisbandTeam`, `DeclareSwarm`, `StopSwarm`, `DefineWorkforce`, `StaffWorkforce`, `AttachQueue`, `RetireWorkforce`, `Send` and `Report` (team broker verbs, `conductor/crates/conductor-broker/src/team.rs:1-13`) |
| Views | `TeamList`, `TeamDetail` (members, lead, shared bindings, open messages), `SwarmList`, `WorkforceBoard` (capacity vs running, queues) |
| Lifecycle | team `formed -> active -> disbanded`; swarm `declared -> running -> {converged, stopped}`; workforce `defined -> staffed -> draining -> retired` |
| Who | Operator: all; Agent: `Send`, `Report` within its team; Application: all |

### 3.4 Auth

| | |
|---|---|
| Commands | `CreateTenant`, `InvitePrincipal`, `DisablePrincipal`, `AssignRole`, `StoreCredential` (value goes to custody, never into the event), `RefreshCredential`, `RevokeCredential`, `GrantCredential`, `RevokeGrant`, `CreatePolicy`, `ActivatePolicy`, `SupersedePolicy` |
| Views | `PrincipalList`, `CredentialList` (kind, custody, scope, expiry; never the value), `GrantMatrix` (who holds what for which purpose), `PolicyList`, `EffectivePolicy` (for one agent definition: the sealed rules Loom will hand to metaharness) |
| Lifecycle | see 2.5 |
| Who | Supervisor only for tenant, principal, credential, policy commands; Operator: `GrantCredential`/`RevokeGrant` on credentials they own; Agent: none; Application: views only |

### 3.5 Tools

| | |
|---|---|
| Commands | `RegisterTool` (from a `tool-contract/1` contract ref, ADR 0095), `WithdrawTool`, `BindTool` (to a definition or team, with verbs, scope, expiry, credential), `RevokeBinding` |
| Views | `ToolCatalogue` (contract, verbs, risk envelope), `BindingList`, `ToolCallLog` (per run: `Decision` outcomes allow/deny/replace, from metaharness events) |
| Lifecycle | tool `registered -> available -> withdrawn`; binding `granted -> active -> {expired, revoked}` |
| Who | Supervisor: `RegisterTool`, `WithdrawTool`; Operator: `BindTool`, `RevokeBinding` within policy; Agent: `ToolCatalogue` view of its own bindings (the owned surface's `search`/`describe`, `metaharness/crates/metaharness-tools/src/lib.rs:1-30`) |

---

## 4. Auth design

### 4.1 Identities

| Identity | How it authenticates to Agency | Source |
|---|---|---|
| Human operator (browser) | Session cookie + CSRF token, as control-plane does (`control-plane/README.md:62`); login via Mandate when deployed, local single-operator mode otherwise *(inferred)* | `control-plane/README.md:62`; `mandate/README.md:3-7` |
| Agent (running) | A per-run bearer minted at `StartRun`, scoped to `agent_id`, `run_id`, team verbs and own work item; expires with the run. Same pattern as metaharness's placeholder bearer `mh-run-<id>-<nonce>` *(inferred reuse)* | `metaharness/docs/design/loopback-provider-v0.1.md:23-25` |
| Application (conductor, takt, swarm) | Service principal with an API key or Mandate token exchange | `mandate/README.md:3-7,20` ("token exchange") |

### 4.2 Tenancy

One `Tenant` per deployment is M1; multi-tenant is a Mandate question (Mandate already models
tenant, delegation, approval consumption, `mandate/README.md:20`). Every entity in §2 carries
`tenant_id`; views filter by it. Whether Agency delegates tenancy to Mandate or keeps its own table:
open question Q5.

### 4.3 Credential holding and scoping

Three custodies, chosen per credential:

| Custody | When | How a run gets it | Source |
|---|---|---|---|
| `ConnectorsInstance` | Tool credentials for external services (GitHub, trackers) | Agency never sees the secret; the tool binding names the Connectors instance; Commission binds and invokes (ADR 0082/0083) | ADR 0095 Decision 4; `connectors/README.md:3-8` |
| `AgencyVault` | Model API keys, OAuth tokens for Claude/Codex logins | Loom's `SessionExecutor.credentials = CredentialSource::Loopback`: metaharness runs a per-run proxy on 127.0.0.1, injects the real token, the child holds only a placeholder; refresh in one place, serialized | `spec.rs:110-134`; `loopback-provider-v0.1.md:19-30`; hermetic rows H4/H6 `metaharness/docs/design/metaharness-protocol-v0.1.md:258,1215,1217` |
| `HostFile` | Single-operator dev mode: use the operator's own Claude/Codex login | `CredentialSource::OperatorLogin`; control-plane does this today ("planner uses Codex login", `control-plane/README.md:37,39`) | `spec.rs:110-134` |

Loopback is Claude-only today (`spec.rs:110-134`); for Codex, `ApiKey` copied into the scratch home
(H6) is the fallback. Agency marks the custody actually used on the `Run`.

### 4.4 Relation to metaharness per-call permission (ADR 0104 draft)

ADR 0104 Decision 2-3 (`0104-*.md:29-43`): metaharness is the one per-call decider, L1 enforces, L3
sets policy. Agency's part:

1. `Policy` + `ToolBinding` for the agent definition are resolved into an **effective policy** (view
   `EffectivePolicy`), digested.
2. Agency passes it to Loom at `StartRun` as the run's authority input (Loom's Mandate authority
   port and `loom.authority.*` entities, `loom/ess/domains/*.yaml`).
3. Loom seals the metaharness `Frame{workflow, node, step, operations, subjects, ...}`
   (`frame.rs:310-340`) and sets `RunSpec.decisions` (`spec.rs:167-230`) to `Frame` for unattended
   runs or `Ask` when the policy says a human decides above the risk ceiling.
4. metaharness answers each tool call with `Allow | Deny{reason} | Replace{input} | Abstain`
   (`command.rs:17-58`); `Ask` decisions route back to Agency as a pending approval the cockpit shows
   (Mandate "approval consumption", `mandate/README.md:20`).
5. Every decision lands in Agency's `ToolCallLog` via the event stream.

Agency never decides a single call itself. If ADR 0104 lands with option B or C
(`0104-*.md:60-64`), only step 3 changes.

---

## 5. Architecture

### 5.1 One deployable service

| Part | Choice | Why | Source |
|---|---|---|---|
| Binary | `agency` (Rust, clap derive), subcommands `serve`, `migrate`, `admin <...>`, `spec` (prints the ESS-derived OpenAPI) | Rust rule; control-plane and conductor are already Rust CLIs | `~/beyond10x/AGENTS.md` "Running code is Rust"; `control-plane/README.md:66-75`; `conductor/crates/conductor/src/cli.rs:460-582` |
| Crates | `agency-spec` (ESS domain + generated Rust types), `agency-core` (command handlers, projections), `agency-api` (HTTP + SSE), `agency-store` (eventlog adapter), `agency-runs` (Loom driver), `agency-ui` (embedded built Vue bundle), `agency` (bin) | mirrors control-plane's structure table | `control-plane/README.md:66-75` |
| Bind | loopback by default, `--bind` for a deployment behind a reverse proxy | control-plane binds loopback with one lock | `control-plane/README.md:79` |

### 5.2 API shape and name style

`POST /agency/commands/<Command>` and `GET /agency/views/<View>`, generated from the ESS
specification exactly as control-plane generates `/host/commands/<Command>` and
`/host/views/<View>` (`control-plane/generated/api/openapi/control-plane.yaml`). Why this and not
REST nouns: the ESS domain already names every command and view with input/outcomes; one generator
produces the OpenAPI, the Rust handlers' types and the TypeScript client types, and `task check`
drift-checks all three (`uilab/Taskfile.yml:25-26,34-38`: `ess generate types --target
rust|typescript` + diff). A REST surface would be a second vocabulary to keep in step. Conductor as a
client gets a stable, generated contract.

Event stream: `GET /agency/events` as SSE (control-plane `/events`, `control-plane/README.md:41`),
one event type per ESS `events` entry plus relayed Loom/metaharness run events (`trace-ir/1`).

### 5.3 Storage

Eventlog (`eventlog/README.md:3-16`: 0.9.1, File/SQLite/PostgreSQL) as the append-only command and
event store, SQLite for a single operator, PostgreSQL for a deployment; projections (views) rebuilt
from the log. control-plane already stores state in SQLite with "durable append"
(`control-plane/README.md:17,79`) and uses Eventlog storage. Secrets never enter the log: a
`StoreCredential` event carries the custody pointer only (§2.5).

### 5.4 Driving Loom runs

```
StartRun ──> agency-runs ──> loom-sdk: Commission + AgentExecutor
                              └─ SessionExecutor{kind, workspace, files, brief, model,
                                   max_turns, credentials: Loopback|OperatorLogin, scratch,
                                   timeout, program, endpoint}
                              └─ RunBudget{max_tokens, deadline}, StopPolicy
                              └─ metaharness RunSpec{kind, hermetic, frame, decisions,
                                   credentials, tool_surface, ...}
```

- `AgentExecutor::run / run_with_usage -> ExecutorStep`
  (`loom/crates/loom-commission/src/ports/executor.rs:20-53`) is the port; Loom's
  `SessionExecutor` is the implementation Agency selects per definition `kind`
  (`loom-session-executor/src/lib.rs:228-258`).
- Agency supplies what `loom/README.md:248-253` says an embedding application supplies: selector,
  argument generator, authority provider, effect port. The authority provider is §4.4 step 2; the
  effect port is Connectors.
- Agency reads back `Proposal{action, summary, files}` (`lib.rs`), `loom.metering.RunUsageReport`,
  and the session's `trace-ir/1` events, and projects them into `RunDetail`.
- Substrate confinement when present (Loom 0.33.0 `SubstrateRunner::spawn_child`, loom CHANGELOG).
- Whether products link `loom-sdk` directly or through an application layer: ADR 0106 "Not yet
  known" (lines 36-41). This design assumes direct `loom-sdk` linkage (edge `agency builds-on
  loom-sdk`, `stack.yaml:263-277`). The parallel loom-sdk design (same directory) decides the crate
  surface; `agency-runs` is its first consumer.

### 5.5 Cockpit UI

Vue 3 + TypeScript, Vite, pnpm, as `uilab/widget/` (`uilab/widget/package.json`: vue ^3.5, vite ^8,
vue-tsc; rule in `~/beyond10x/AGENTS.md` 2026-10-21 exception). Types come from `ess generate types
--target typescript`, drift-checked in `task check` (`uilab/Taskfile.yml:25-26,34-38`). The bundle
is embedded in the Rust binary as control-plane does (`control-plane/AGENTS.md:13`). control-plane's
`frontend/src/*.vue,*.js` is JS without generated types; it is a reference for the screens, not code
to lift.

Screens, one per §3 area plus a home: **Board** (workforces, queues, running agents, pending
approvals), **Agents**, **Work**, **Teams**, **Tools**, **Auth**, **Run** (live `trace-ir/1`
timeline, tool-call decisions, budget meter). conductor's dashboard modules
(answers/cache/decisions/health/overview/repo/series/usage/waste, port 7313,
`conductor/README.md:183`) are the model for Board and Throughput.

### 5.6 How conductor uses the API

conductor keeps its CLI, append-only store, PreToolUse guard and host loop
(`conductor/README.md:15-23,34-51,195-199`; `docs/design/conductor.md:5-32,71-81`) and replaces
three things with Agency calls:

| conductor today | Through Agency |
|---|---|
| `Spawn`, `Session`, `Dispatch` command groups start Claude Code sessions with `bypassPermissions` (`README.md:34-51`) | `StartRun` on an agent whose definition and policy Agency holds; permission is metaharness's per call (§4.4) |
| `Broker` team verbs `send`/`report`, `conductor.tool.Team`, `conductor.tool.Grant` (`tool.yaml:277-329`, `team.rs:1-13`) | `Send`/`Report`, `Team`, `ToolBinding` in Agency; conductor's broker becomes a thin client or is retired (option in §6) |
| Dashboard :7313 | Cockpit; conductor keeps its CLI `Status`/`Metric` views reading Agency views |

conductor's `Goal`, `Decision`, `Need`, `Finding`, `Rules`, `Trust` groups (`cli.rs:460-582`) stay
conductor's: they are the application's judgement about *what* to do next; Agency is *who runs it
with what*. The conductor-v2 draft's credential broker and placement
(`~/.cache/b10x-layers/conductor-v2/DESIGN.md`) move into Agency (credentials) and Loom
(placement); conductor-v2 should be re-cut against this split *(inferred, that draft predates row 98)*.

---

## 6. What happens to the repositories (options only; the call is Timo's)

### control-plane

| Option | What | For | Against |
|---|---|---|---|
| A | Rename `control-plane` -> `agency`, keep history, replace `host` domain with `agency` | Keeps the ESS/OpenAPI generator, SSE console, SQLite store, Vue embedding that already work (`control-plane/README.md:17,41,45-62,66-75`) | Carries a JS frontend, the `host` nouns and ADR 0092's `research` catalogue status (`0092-*.md:35-46`) |
| B | New `beyond10x/agency` repository; lift control-plane's generator and server skeleton by copy; archive control-plane | Clean ESS domain from day one, TS frontend from day one | New-repository creation is on the never-list for the orchestrator; Timo creates |
| C | Keep control-plane as is, Agency as a new repo, both paused/unpaused independently | No destruction | Two services with the same job; contradicts `stack.yaml:263-277` `agency contains control-plane` |

Recommendation: **B** (clean domain, generated TS types), with control-plane archived after M2.

### conductor

| Option | What | For | Against |
|---|---|---|---|
| A | conductor stays its own repository, adds an `agency` client crate, retires `Spawn`/`Broker` once Agency M3 ships | Matches row 98 and `stack.yaml:77-85`; conductor runs today | Two stores (conductor's append-only store and Agency's log) until the cut-over |
| B | conductor's Team/Grant/broker ESS entities move to Agency's spec now, conductor depends on them | One home for team nouns early | conductor 0.18.2 users break before Agency exists |
| C | Fold conductor into Agency | Draft ADR 0105 Decision 1 says this | Contradicts Timo's row 98 decision |

Recommendation: **A**.

### ADR 0105 must change

Draft at `wt-521012e8dceb/architecture/adr/0105-conductor-and-control-plane-converge-into-one-product.md`:

| Where | Now says | Must say |
|---|---|---|
| Title | conductor and control-plane converge into one product | control-plane converges into Agency; conductor becomes an application on it |
| Decision 1 (lines 32-52) | one product of both | Agency = control-plane's successor; conductor = application layer client (`decisions.md` row 98) |
| Context table row on conductor (lines 19-28) | a converging input | an application that keeps its CLI, store and guard |
| Nouns table (lines 44-52) | seven nouns | add `AgentDefinition`, `Tool` vs `ToolBinding`, `Workforce` distinct from `Team`, `Principal`, `Credential`, `CredentialGrant`, `Policy` (§2) |
| Consequences (lines 65-70) | README/ROADMAP rows for both | conductor README gains "application on Agency"; control-plane README gains the archive note |
| Not decided (lines 74-76) | repository fate | still Timo's; list the §6 options |
| Related | 0104 | add 0106 (Product layer), 0092 §2 (control-plane as research PoC), 0095 (tools) |

---

## 7. Milestones

| M | Name | Deliverable | Verified by |
|---|---|---|---|
| M0 | Spec | ESS domain `agency` (§2 entities, §3 commands/views), `ess validate`, OpenAPI + Rust + TS types generated, conformance suite synthesized (all scenarios skipped) | `task check` green on spec, drift diff empty |
| M1 | Service skeleton | `agency serve` on loopback, eventlog SQLite, every command accepted and logged, every view answers from projections, SSE `/agency/events`; single tenant, `OperatorLogin` only | conformance scenarios for Agents/Teams/Work pass against the live binary |
| M2 | Runs | `StartRun` drives Loom `SessionExecutor` for `Kind::Claude`, budget and stop policy honoured, `RunDetail` shows `trace-ir/1` events, `Loopback` custody | one unattended run from `SubmitWorkItem` to `SettleWorkItem` traced end to end (control-plane did 242 s unattended, `control-plane/README.md:107`) |
| M3 | Tools and policy | `RegisterTool` from a `tool-contract/1` ref, `BindTool`, `Policy` -> `EffectivePolicy` -> sealed Frame; `Ask` decisions surface as approvals | a `Deny` and a `Replace` observed in `ToolCallLog` on a real run |
| M4 | First usable cockpit | Vue Board, Agents, Work, Teams, Run screens; conductor `agency` client crate for `StartRun`/`Send`/`Report` | Timo runs one team on one queue from the browser; conductor dispatches one session through Agency |
| M5 | Deployment | PostgreSQL eventlog, `--bind`, Mandate login, multi-tenant filter, `Kind::Codex` | deployed instance behind a proxy with two principals |

### Draft epics and stories

| Epic | Stories |
|---|---|
| `epic:agency-spec` (M0) | agents sub-domain; tools sub-domain; teams sub-domain; work sub-domain; auth sub-domain; generators + drift check in `task check` |
| `epic:agency-service` (M1) | clap binary + `serve`; eventlog store adapter; command dispatch from generated types; view projections; SSE events; CSRF + session cookie; conformance target `ExecuteCommand`/`QueryView` |
| `epic:agency-runs` (M2) | `agency-runs` on loom-sdk; `SessionExecutor` selection by `Kind`; `RunBudget`/`StopPolicy` from definition + policy; run projection from `loom.session` and `trace-ir/1`; Loopback custody; work-item settle from `Proposal` |
| `epic:agency-tools-policy` (M3) | tool registration from contract ref; bindings with scope/expiry; policy rules and `EffectivePolicy`; authority provider for Loom; approval inbox for `Ask`; `ToolCallLog` |
| `epic:agency-cockpit` (M4) | Vite/pnpm project + generated TS types; Board; Agents; Work; Teams; Run timeline; embed bundle in binary |
| `epic:conductor-on-agency` (M4) | `agency-client` crate in conductor; `Spawn` -> `StartRun`; broker verbs -> `Send`/`Report`; retire :7313 modules the cockpit covers |
| `epic:agency-deploy` (M5) | PostgreSQL; `--bind`; Mandate login + tenant; Codex kind; archive control-plane (after Timo's call) |

Every story's acceptance is a named conformance scenario, per `~/beyond10x/AGENTS.md` "ESS drives
every product repository". Stories go into the AEP store of the Agency repository once it exists;
this file is the draft, not the plan.

---

## 8. Open questions for Timo

| Q | Question | Options | Recommendation |
|---|---|---|---|
| Q1 | Final name | A `agency` (working name, in `stack.yaml:41`); B `loom-agency`; C something else | **A**: it is already in the stack map and the ADR draft; rename later costs a repo rename |
| Q2 | Repository for Agency | A rename control-plane; B new `beyond10x/agency`, archive control-plane; C both live | **B** (§6) |
| Q3 | Work items: stored or handled | A store reference + state only, read body from source on demand; B store the full item; C store nothing, stateless routing | **A**: "handle" needs state to assign and settle; the body stays in the tracker |
| Q4 | Does conductor's broker move into Agency | A Agency owns `Team`/`Send`/`Report`, conductor's broker becomes a client; B conductor keeps the broker, Agency only records teams | **A**: one home for team nouns, and agents reach it with the per-run bearer |
| Q5 | Tenancy and login | A Mandate from M1; B local single operator until M5, Mandate at M5; C Agency's own user table | **B**: nothing in M1-M4 needs a second principal; Mandate 0.9.0 is a dev release (`mandate/README.md:3-7`) |
| Q6 | Model credentials for Codex runs | A wait for Loopback to cover Codex in metaharness; B `ApiKey` copied into scratch home (H6) as fallback; C Claude-only until M5 | **B** with the custody marked on the Run |
| Q7 | Does takt relate to Agency | A takt registers its runs as work items via the API; B no relation, peer application; C takt moves onto Agency later | **B** for now; nothing in ADR 0099 or `takt/README.md:141-145` connects them, and A is a one-story addition later |
| Q8 | Does Agency link `loom-sdk` directly or through an application-layer crate | A direct (`stack.yaml:263-277`); B through a thin application crate shared with loom-coder and swarm | **A**, revisit when the loom-sdk design (same directory) lands and ADR 0106's "Not yet known" (lines 36-41) is answered |
| Q9 | Swarm in Agency | A Agency records swarms and holds the stop lever only; B Agency runs the coordinator loop; C no swarm noun in Agency | **A**: swarm is its own product (ADR 0106); Agency manages, swarm runs |
| Q10 | Rewrite ADR 0105 now or supersede with 0107 | A amend the draft (it is unmerged, branch `docs/agent-stack-naming`); B leave 0105, write 0107 | **A**: an unmerged draft is cheaper to fix than to supersede |

### I don't know

- Where loom-coder lives (no repository found under `~/beyond10x/`); ADR 0106 lines 27-32 name it.
- Whether takt talks to Agency (Q7).
- How agentide and swarm reach lower layers (ADR 0106 lines 36-41 "Not yet known").
- Whether Loopback custody will cover Codex (metaharness `spec.rs:110-134` says Claude only today).
- Whether Mandate's tenant model is meant to be Agency's (nothing in `mandate/README.md` names Agency).
