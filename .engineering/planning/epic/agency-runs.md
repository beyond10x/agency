---
format: aep.planning-md/3
id: epic:agency-runs
kind: epic
status: draft
title: M2 Runs driven through loom-sdk
summary: StartRun drives a Loom SessionExecutor for Kind::Claude with budget and stop policy; RunDetail shows trace-ir/1 events; Loopback custody.
revision: 1
---
## Outcome

`StartRun` drives Loom through `loom-sdk` directly (`docs/design/agency.md` §8 Q8 A): `SessionExecutor` selected by the definition's kind, `RunBudget` and `StopPolicy` from definition and policy, Loopback custody for Claude and `ApiKey` copied into the scratch home as the Codex fallback with the custody used marked on the Run (Q6 B), and a work item settled from the run's `Proposal`.

## Acceptance

One unattended run from `SubmitWorkItem` to `SettleWorkItem` is traced end to end, and its `RunDetail` shows the `trace-ir/1` events and the custody used.

## Rule

Agency never runs a model loop; Loom does.
