---
format: aep.planning-md/3
id: story:agency-run-workspace
kind: story
status: draft
title: A run names its repository, base branch and volume
summary: StartRun carries a RunWorkspace, a WorkInput of source AepStore names the repository, and the LoomAgent gets it as spec.workspace (conductor design § 3.2 G4).
tags:
- conductor-gap:G4
relations:
- decomposes: epic:agency-runs
revision: 1
---
## Outcome

A run names the repository, base branch and volume it works in: a `WorkInput` of source
`AepStore` names the repository, `StartRun` carries a `RunWorkspace` (repository, base branch,
optional volume claim), and the run's `LoomAgent` gets it as `spec.workspace`.

## Why

conductor `docs/design/conductor-on-agency.md` § 3.2 G4 (conductor main `9fa423a6`): no Agency noun
said which repository, base branch and volume a run works in; Loom has `spec.workspace`.

## Acceptance

Conformance scenarios on the service target:

- `agency.runs.StartRun/outcome/requested`, with `workspace` read back from `agency.runs.RunList`
- `agency.work.ConfigureWorkInput/outcome/configured` (source `AepStore`)
- `agency.work/authored/submit-assign-settle`

## Scope

- agency: `ess/domains/runs.yaml` (`RunWorkspace`, `Run.workspace`, `StartRun.workspace`;
  specified in M0), `ess/domains/work.yaml` (`AepStoreSource`), the runs service module mapping
  the workspace onto the `LoomAgent`.
- loom: none (`spec.workspace` exists since loom 0.42.0).
