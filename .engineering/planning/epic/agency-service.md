---
format: aep.planning-md/3
id: epic:agency-service
kind: epic
status: proposed
title: M1 Service skeleton
summary: 'agency serve on loopback: eventlog SQLite, every command accepted and logged, every view answered from projections, SSE events; single tenant, OperatorLogin only.'
revision: 2
transitions:
- {from: "draft", to: "proposed", at: "2026-10-26T14:54:20Z", actor: "human:timo", revision: 2}
---
## Outcome

A Rust binary `agency` (clap derive) whose `serve` subcommand binds loopback, appends every command to an Eventlog SQLite store, answers every view from projections rebuilt from the log, streams events on `/agency/events`, and authenticates a single local operator by session cookie and CSRF token.

## Acceptance

The synthesized conformance scenarios for the agents, teams and work areas run against the live binary as an `ExecuteCommand`/`QueryView` target with 0 failed and 0 errored.

## Rule

Single local operator until M5 (Timo, 2026-10-22, `docs/design/agency.md` §8 Q5 B).
