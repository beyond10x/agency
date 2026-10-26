---
format: aep.planning-md/3
id: epic:agency-spec
kind: epic
status: active
title: 'M0 Spec: the ESS domain agency'
summary: The ESS system agency (shared, auth, agents, runs, tools, teams, work, service) validated, compiled and projected, with a synthesized Rust crate, TypeScript types and a conformance suite held in task check.
revision: 5
transitions:
- {from: "draft", to: "proposed", at: "2026-10-26T14:54:20Z", actor: "human:timo", revision: 4, decided_on: {"recorded":{"test_result":1,"verification":1}}}
- {from: "proposed", to: "active", at: "2026-10-26T14:54:20Z", actor: "human:timo", revision: 5, decided_on: {"recorded":{"test_result":1,"verification":1}}}
---
## Outcome

The ESS system `agency` (ess 0.69.2, format `ess/31`) holds the entities, commands, events and
views of `docs/design/agency.md` §2 and §3 in eight domains: `agency.shared`, `agency.auth`
(Tenant, Principal, Credential, CredentialGrant, Policy), `agency.agents` (AgentDefinition, Agent),
`agency.runs` (Run, RunApproval), `agency.tools` (Tool, ToolBinding), `agency.teams` (Team, Swarm,
Workforce), `agency.work` (WorkInput, WorkItem) and `agency.service` (Session). An agent
definition holds its `loom.agent/1` document by value, as a `Json` value Loom checks (ADR 0108);
Agency redeclares no Loom type. Everything a client or the service needs is generated from it.

## Acceptance

`task check` validates `ess/` with 0 refusals, holds the OpenAPI, AsyncAPI and JSON Schema
projections, the synthesized Rust crate, the TypeScript types and the synthesized conformance
suite to `ess/` with no drift, runs the suite against the interpreted target and holds its report
to `conformance/interpreted.baseline.json`; CI `check` is green on main.

## Rule

No secret value and no work-item body is a field of any entity. Generated output is never edited by hand.
