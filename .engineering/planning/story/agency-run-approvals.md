---
format: aep.planning-md/3
id: story:agency-run-approvals
kind: story
status: draft
title: A run asks for and receives a run-level approval
summary: RunApproval requested by a run, shown in the ApprovalInbox and granted or denied; the answer reaches the run, as takt approve does (conductor design § 3.2 G5).
tags:
- conductor-gap:G5
relations:
- decomposes: epic:agency-tools-policy
revision: 1
---
## Outcome

A run can stop on a question that is about the run, not one tool call: a `RunApproval` (subject,
capability) is requested for the run, shown in the `ApprovalInbox`, and granted or denied by an
operator, supervisor or application; the answer reaches the run.

## Why

conductor `docs/design/conductor-on-agency.md` § 3.2 G5 (conductor main `9fa423a6`): a takt case
stops at `ApprovalRequired` (exit 4) and moves on `takt approve <case> <capability>`; Agency's inbox
carried only metaharness `Ask` decisions per tool call.

## Acceptance

Conformance scenarios (`ess/`, domain `agency.runs`) on the service target:

- `agency.runs.RequestRunApproval/outcome/requested`
- `agency.runs.GrantRunApproval/outcome/granted` and `agency.runs.DenyRunApproval/outcome/denied`
- `agency.runs.RunApproval/transition/deny/by/agency.runs.DenyRunApproval/denied`
- `agency.runs.GrantRunApproval/grant/denied` (a caller without the grant is refused)

## Scope

- agency: `ess/domains/runs.yaml` (`RunApproval`, the three commands, `ApprovalInbox`; specified
  in M0), `ess/domains/auth.yaml` (who may answer), the service route that hands the answer to
  the run.
- takt (not filed here): the pending approvals file of its root case
  (`design:continuous-orchestration` § 7) read by the run, and `takt approve` applied from
  Agency's answer.

## Not in scope

Per-tool-call `Ask` decisions from metaharness (M3 inbox).
