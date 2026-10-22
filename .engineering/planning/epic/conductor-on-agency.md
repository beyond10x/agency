---
format: aep.planning-md/3
id: epic:conductor-on-agency
kind: epic
status: draft
title: M4 conductor as an application on Agency
summary: 'conductor gains an agency client: Spawn becomes StartRun, broker verbs become Send/Report, the team broker moves to Agency.'
revision: 1
---
## Outcome

conductor keeps its CLI, store and guard and drives Agency's API: an `agency-client` crate, `Spawn` replaced by `StartRun`, the team broker's `send`/`report` and `Team`/`Grant` nouns owned by Agency (`docs/design/agency.md` §8 Q4 A), and the dashboard modules the cockpit covers retired.

## Acceptance

conductor dispatches one session through Agency and its broker verbs reach Agency's `Send`/`Report`.

## Rule

Work in the conductor repository is that repository's; this epic records Agency's side and the contract.
