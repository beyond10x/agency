---
format: aep.planning-md/3
id: epic:agency-spec
kind: epic
status: draft
title: 'M0 Spec: the ESS domain agency'
summary: The ESS domain agency (agents, tools, teams, work, auth) validated, compiled and projected, with Rust and TypeScript types and a synthesized conformance suite.
revision: 1
---
## Outcome

The ESS domain `agency` holds the 15 entities of `docs/design/agency.md` §2 (agents: AgentDefinition, Agent, Run; tools: Tool, ToolBinding; teams: Team, Swarm, Workforce; work: WorkInput, WorkItem; auth: Tenant, Principal, Credential, CredentialGrant, Policy) and the commands and views of `docs/design/agency.md` §3, and everything a client or the service needs is generated from it.

## Acceptance

`task check` validates `ess/` with 0 refusals, regenerates the OpenAPI, AsyncAPI and JSON Schema projections, the Rust and TypeScript types and the synthesized conformance suite, and finds no drift; CI `check` is green on main.

## Rule

No secret value and no work-item body is a field of any entity. Generated output is never edited by hand.
