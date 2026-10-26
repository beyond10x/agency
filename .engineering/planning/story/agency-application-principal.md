---
format: aep.planning-md/3
id: story:agency-application-principal
kind: story
status: draft
title: An application principal calls Agency with a token before the deployment milestone
summary: An ApplicationToken session opened by a supervisor and held by digest lets an application such as conductor call unattended (conductor design § 3.2 G6).
tags:
- conductor-gap:G6
relations:
- decomposes: epic:agency-service
revision: 1
---
## Outcome

An application principal calls Agency without a person before the deployment milestone: a
supervisor opens an `ApplicationToken` session for it, held by its token's digest, and the
application may submit work, start runs and answer run approvals within its grants.

## Why

conductor `docs/design/conductor-on-agency.md` § 3.2 G6 (conductor main `9fa423a6`): M1 to M4
admitted one local operator by session cookie, and conductor calls unattended.

## Acceptance

Conformance scenarios (`ess/`, domains `agency.service` and `agency.auth`) on the service target:

- `agency.service.OpenSession/outcome/opened` (kind `ApplicationToken`)
- `agency.service.CloseSession/outcome/closed`
- `agency.service.SessionList/grant/read/admitted/agency.auth.Supervisor` and
  `agency.service.SessionList/grant/read/denied`
- `agency.service.OpenSession/grant/denied`

## Scope

- agency: `ess/domains/service.yaml` (`Session`, `OpenSession`, `CloseSession`, `SessionList`;
  specified in M0), `ess/domains/auth.yaml` (the `Application` actor's grants), the service's
  bearer-token authentication.
- none below Agency.
