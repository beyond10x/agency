---
format: aep.planning-md/3
id: approval-record:agency-m0-and-gaps
kind: approval-record
status: draft
title: M0 specification wave and the G1-G8 gap filing accepted under the orchestrator's grant
summary: M0 spec, the eight conductor gaps as stories, and the epic moves, accepted under the orchestrator's grant on 2026-10-26.
relations:
- decides: epic:agency-spec
revision: 1
---
## Decision

The M0 specification wave and the filing of conductor's eight Agency gaps (G1 to G8,
conductor `docs/design/conductor-on-agency.md` § 3.2, conductor main `9fa423a6`) are accepted under
the orchestrator's grant for class-C items in this repository, item of 2026-10-26.

## What it covers

- `ess/` (eight domains, five authored scenarios), `generated/`, `conformance/`, `crates/`, the
  `task check` wiring and the CI `check` workflow.
- Stories `story:agency-program-agents` (G1), `story:agency-deployed-runs` (G2),
  `story:agency-settle-from-program-outcome` (G3), `story:agency-run-workspace` (G4),
  `story:agency-run-approvals` (G5), `story:agency-application-principal` (G6),
  `story:agency-credential-headroom` (G7), `story:agency-run-storage-capacity` (G8), and the new
  `epic:agency-capacity` for G7 and G8.
- Moves: `epic:agency-spec` to active; `epic:agency-runs`, `epic:agency-tools-policy`,
  `epic:agency-service`, `epic:agency-capacity` and `epic:conductor-on-agency` to proposed.

## Settled, not reopened

ADR 0105 (amended) and ADR 0108 (accepted 2026-10-26): Loom owns `loom.agent/1`, Agency holds agent
definitions by value, Loom serves A2A.
