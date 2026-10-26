---
format: aep.planning-md/3
id: story:agency-settle-from-program-outcome
kind: story
status: draft
title: 'A work item settles from a program''s outcome: exit code, case outcome, commit-bound evidence and branch'
summary: SettleWorkItem takes a takt run's exit code, case outcome, validator evidence bound to a commit and its branch (conductor design § 3.2 G3).
tags:
- conductor-gap:G3
relations:
- decomposes: epic:agency-runs
- depends_on: story:agency-deployed-runs
revision: 1
---
## Outcome

A `WorkItem` settles from a program's outcome, not only from a Loom `Proposal`: the run's exit
code (takt's registry, 0 to 8 built, 9 and 10 proposed), its case outcome, validator evidence
bound to a commit, and the branch it produced decide how the item settles, and the item records
which.

## Why

conductor `docs/design/conductor-on-agency.md` § 3.2 G3 (conductor main `9fa423a6`).

## Acceptance

Conformance scenarios (`ess/`, domain `agency.work`) on the service target:

- `agency.work.SettleWorkItem/outcome/settled`
- `agency.work/authored/submit-assign-settle`
- new outcome scenarios of `SettleWorkItem`, one per settled class an exit code maps to (for
  example delivered, needs approval, refused), each reading the recorded commit and branch back
  from `agency.work.WorkItemDetail`.

## Scope

- agency: `ess/domains/work.yaml` (`SettleWorkItem` input and `WorkItem` fields for the outcome,
  the commit and the branch; not yet specified), the settle path in the service.
- takt (not filed here): its outcome files stay a stable contract (takt PRD R13).

## Not in scope

Reading the outcome off the pod (`story:agency-deployed-runs`).
