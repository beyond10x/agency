---
format: aep.planning-md/3
id: story:agency-credential-headroom
kind: story
status: draft
title: Agency chooses and rotates among several subscription credentials by headroom
summary: A run stopped on a usage limit is recorded, and the next run takes a granted credential with headroom (conductor design § 3.2 G7).
tags:
- conductor-gap:G7
relations:
- decomposes: epic:agency-capacity
revision: 1
---
## Outcome

Agency chooses among several subscription credentials for a run and rotates them: a run stopped on
a usage limit is recorded as such, and the next run (or its resumption) takes a granted credential
with headroom. The run records which credential source served it.

## Why

conductor `docs/design/conductor-on-agency.md` § 3.2 G7 (conductor main `9fa423a6`): Agency had
grants and a workforce ceiling, no choice among credentials.

## Acceptance

Conformance scenarios on the service target, new in `ess/`:

- `agency.runs.RecordRunFailure` with a usage-limit reason, read back on `agency.runs.RunDetail`
- a credential-choice outcome on `StartRun` (or a new command) that picks the granted credential
  with headroom, and refuses when none has any
- `agency.auth/authored/store-grant-revoke` still passes

## Scope

- agency: `ess/domains/auth.yaml` (a credential's headroom or limit state; not yet specified),
  `ess/domains/runs.yaml` (`Run.credential_source`, a usage-limit end), the runs service module.
- connectors (not filed here): issue and refresh of subscription tokens (its OAuth2 work); custody
  stays in Connectors.
