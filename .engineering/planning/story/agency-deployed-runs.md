---
format: aep.planning-md/3
id: story:agency-deployed-runs
kind: story
status: draft
title: StartRun writes a LoomAgent for a Command agent and records the run's end with its exit code
summary: 'A deployed run: StartRun writes the LoomAgent, reads its status back and records start, proposal or failure with the program''s exit code (conductor design § 3.2 G2).'
tags:
- conductor-gap:G2
relations:
- decomposes: epic:agency-runs
- depends_on: story:agency-program-agents
revision: 1
---
## Outcome

`StartRun` for an agent whose definition is a `Command` agent writes a `LoomAgent` resource, reads
its status back, and records the run's end with the program's exit code: `RecordRunStarted` when
the pod runs, `RecordRunProposal` or `RecordRunFailure` with the exit code when it completes.

## Why

conductor `docs/design/conductor-on-agency.md` § 3.2 G2 (conductor main `9fa423a6`): `StartRun`
drives an in-process `SessionExecutor` (Agency design § 5.4); conductor needs a deployed run and
its exit code.

## Acceptance

Conformance scenarios (`ess/`, domain `agency.runs`) on the service target:

- `agency.runs.StartRun/outcome/requested`
- `agency.runs.RecordRunStarted/outcome/started`
- `agency.runs.RecordRunProposal/outcome/proposed`
- `agency.runs.RecordRunFailure/outcome/failed`
- `agency.runs.RunDetail` publishing a new `exit_code` field, with a synthesized scenario per
  outcome that reads it back.

## Scope

- agency: `ess/domains/runs.yaml` (an `exit_code` on `Run` and on the record commands' input; not
  yet specified), the runs service module that writes the `LoomAgent` and reads its status.
- loom (not filed here): a `Command` run runs to completion instead of as a restarting
  Deployment, and its status reports completion and the exit code (two trial gaps, design § 3.2
  G2 "needs below Agency").

## Not in scope

Settling the work item from the outcome (`story:agency-settle-from-program-outcome`).
