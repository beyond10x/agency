---
format: aep.planning-md/3
id: story:agency-run-storage-capacity
kind: story
status: draft
title: A workforce's capacity counts the storage a run needs
summary: Workforce capacity gains storage, admission refuses a run whose workspace and build output do not fit, and the LoomAgent carries the limits (conductor design § 3.2 G8).
tags:
- conductor-gap:G8
relations:
- decomposes: epic:agency-capacity
revision: 1
---
## Outcome

A workforce's capacity counts storage as well as runs and tokens: a run's workspace and build
output size are part of its request, the workforce admits a run only while that storage is free,
and the run's `LoomAgent` carries the volume size and ephemeral-storage limit.

## Why

conductor `docs/design/conductor-on-agency.md` § 3.2 G8 (conductor main `9fa423a6`): the trial's
volume was 60Gi and its 3.0 GiB of build output was cleaned by hand; `Workforce.capacity` counted
runs and tokens only.

## Acceptance

Conformance scenarios on the service target, new in `ess/`:

- `agency.teams.DefineWorkforce/outcome/defined` with a storage capacity read back from
  `agency.teams.WorkforceBoard`
- an `AdmitRun` refusal when the requested storage exceeds what is free
- `agency.runs.StartRun/outcome/requested` with the requested storage read back

## Scope

- agency: `ess/domains/teams.yaml` (`Capacity` gains storage; not yet specified),
  `ess/domains/runs.yaml` (`RunWorkspace` or `RunBudget` gains a size), admission in the service.
- loom (not filed here): volume size and ephemeral-storage limits on the `LoomAgent`.
